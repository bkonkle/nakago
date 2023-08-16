use proc_macro::TokenStream;
use quote::quote;

use crate::{
    args,
    utils::{get_crate_name, GeneratorResult},
};

pub fn generate(object_args: &args::Provider) -> GeneratorResult<TokenStream> {
    let ident = &object_args.ident;
    let (impl_generics, ty_generics, where_clause) = object_args.generics.split_for_impl();

    let crate_name = get_crate_name(object_args.internal);

    let expanded = quote! {
        #[allow(clippy::all, clippy::pedantic)]
        #[#crate_name::async_trait::async_trait]
        impl #impl_generics Provider<#crate_name::Dependency> for #ident #ty_generics #where_clause {
            async fn provide(self: Arc<Self>, i: Inject) -> #crate_name::InjectResult<Arc<#crate_name::Dependency>> {
                let provider = self as Arc<#ident>;

                Ok(provider.provide(i).await?)
            }
        }
    };

    Ok(expanded.into())
}
