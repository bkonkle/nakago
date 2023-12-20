//! # Derive
use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

use crate::utils::expand_with;

mod args;
mod from_ref;
mod provider;
mod utils;

macro_rules! parse_nested_meta {
    ($ty:ty, $args:expr) => {{
        let meta = match darling::ast::NestedMeta::parse_meta_list(proc_macro2::TokenStream::from(
            $args,
        )) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(darling::Error::from(e).write_errors());
            }
        };

        match <$ty>::from_list(&meta) {
            Ok(object_args) => object_args,
            Err(err) => return TokenStream::from(err.write_errors()),
        }
    }};
}

/// Derive `Provider` trait for a struct.
#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Provider(args: TokenStream, input: TokenStream) -> TokenStream {
    let object_args = parse_nested_meta!(args::Provider, args);
    let mut item_impl = parse_macro_input!(input as ItemImpl);
    match provider::generate(&object_args, &mut item_impl) {
        Ok(expanded) => expanded,
        Err(err) => err.write_errors().into(),
    }
}

/// Derive an implementation of [`FromRef`] for each field in a struct.
///
/// [`FromRef`]: https://docs.rs/axum/0.7/axum/extract/trait.FromRef.html
#[proc_macro_derive(FromRef, attributes(from_ref))]
pub fn derive_from_ref(item: TokenStream) -> TokenStream {
    expand_with(item, from_ref::expand)
}
