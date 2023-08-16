//! # Derive
use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod args;
mod provider;
mod utils;

/// Derive the `Provider<Dependency>` trait.
#[proc_macro_derive(Provider, attributes(inject))]
pub fn derive_any_provider_impl(input: TokenStream) -> TokenStream {
    let object_args =
        match args::Provider::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
            Ok(object_args) => object_args,
            Err(err) => return TokenStream::from(err.write_errors()),
        };
    match provider::generate(&object_args) {
        Ok(expanded) => expanded,
        Err(err) => err.write_errors().into(),
    }
}
