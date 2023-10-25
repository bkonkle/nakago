use proc_macro::TokenStream;
use quote::quote;
use syn::ItemImpl;

use crate::{
    args,
    utils::{get_crate_name, get_trait_path, get_type_path_and_name, GeneratorResult},
};

pub fn generate(
    object_args: &args::Provider,
    item_impl: &mut ItemImpl,
) -> GeneratorResult<TokenStream> {
    let crate_name = get_crate_name(object_args.internal);
    let (self_ty, _self_name) = get_type_path_and_name(item_impl.self_ty.as_ref())?;
    let (impl_generics, _ty_generics, where_clause) = item_impl.generics.split_for_impl();
    let trait_path = get_trait_path(item_impl)?;

    let expanded = quote! {
        #item_impl

        #[allow(clippy::all, clippy::pedantic)]
        #[#crate_name::async_trait::async_trait]
        impl #impl_generics #crate_name::Provider<#crate_name::Dependency> for #self_ty #where_clause {
            async fn provide(self: Arc<Self>, i: Inject) -> #crate_name::inject::Result<Arc<#crate_name::Dependency>> {
                let provider = self as Arc<dyn #trait_path>;

                Ok(provider.provide(i).await?)
            }
        }
    };

    Ok(expanded.into())
}
