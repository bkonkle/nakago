use proc_macro2::{Ident, Span, TokenStream, TokenTree};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{Error, ItemImpl, Path, Type, TypeGroup, TypeParamBound};
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

pub fn get_crate_name(internal: bool) -> TokenStream {
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
