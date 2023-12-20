use proc_macro2::{Ident, Span, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens};
use syn::{parse::Parse, Error, ItemImpl, Path, Type, TypeGroup, TypeParamBound};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("{0}")]
    Syn(#[from] syn::Error),

    #[error("{0}")]
    Darling(#[from] darling::Error),
}

impl GeneratorError {
    pub fn write_errors(self) -> proc_macro2::TokenStream {
        match self {
            GeneratorError::Syn(err) => err.to_compile_error(),
            GeneratorError::Darling(err) => err.write_errors(),
        }
    }
}

pub type GeneratorResult<T> = std::result::Result<T, GeneratorError>;

pub fn get_crate_name(internal: bool) -> proc_macro2::TokenStream {
    if internal {
        quote! { crate }
    } else {
        let name = match crate_name("nakago") {
            Ok(FoundCrate::Name(name)) => name,
            Ok(FoundCrate::Itself) | Err(_) => "nakago".to_string(),
        };
        TokenTree::from(Ident::new(&name, Span::call_site())).into()
    }
}

pub fn get_type_path_and_name(ty: &Type) -> GeneratorResult<(&Type, String)> {
    match ty {
        Type::Path(path) => Ok((
            ty,
            path.path
                .segments
                .last()
                .map(|s| s.ident.to_string())
                .unwrap(),
        )),
        Type::Group(TypeGroup { elem, .. }) => get_type_path_and_name(elem),
        Type::TraitObject(trait_object) => Ok((
            ty,
            trait_object
                .bounds
                .iter()
                .find_map(|bound| match bound {
                    TypeParamBound::Trait(t) => {
                        Some(t.path.segments.last().map(|s| s.ident.to_string()).unwrap())
                    }
                    _ => None,
                })
                .unwrap(),
        )),
        _ => Err(Error::new_spanned(ty, "Invalid type").into()),
    }
}

pub fn get_trait_path(item_impl: &ItemImpl) -> GeneratorResult<&Path> {
    match item_impl.trait_.as_ref() {
        Some((_, path, _)) => Ok(path),
        None => Err(Error::new_spanned(item_impl, "Missing trait for Provider").into()),
    }
}

pub fn expand<T>(result: syn::Result<T>) -> proc_macro::TokenStream
where
    T: ToTokens,
{
    match result {
        Ok(tokens) => {
            let tokens = (quote! { #tokens }).into();
            if std::env::var_os("NAKAGO_MACROS_DEBUG").is_some() {
                eprintln!("{tokens}");
            }
            tokens
        }
        Err(err) => err.into_compile_error().into(),
    }
}

pub fn expand_with<F, I, K>(input: proc_macro::TokenStream, f: F) -> proc_macro::TokenStream
where
    F: FnOnce(I) -> syn::Result<K>,
    I: Parse,
    K: ToTokens,
{
    expand(syn::parse(input).and_then(f))
}

pub(crate) trait Combine: Sized {
    fn combine(self, other: Self) -> syn::Result<Self>;
}

pub(crate) fn combine_unary_attribute<K>(a: &mut Option<K>, b: Option<K>) -> syn::Result<()>
where
    K: ToTokens,
{
    if let Some(kw) = b {
        if a.is_some() {
            let kw_name = std::any::type_name::<K>().split("::").last().unwrap();
            let msg = format!("`{kw_name}` specified more than once");
            return Err(syn::Error::new_spanned(kw, msg));
        }
        *a = Some(kw);
    }
    Ok(())
}

pub(crate) fn parse_attrs<T>(ident: &str, attrs: &[syn::Attribute]) -> syn::Result<T>
where
    T: Combine + Default + Parse,
{
    attrs
        .iter()
        .filter(|attr| attr.meta.path().is_ident(ident))
        .map(|attr| attr.parse_args::<T>())
        .try_fold(T::default(), |out, next| out.combine(next?))
}
