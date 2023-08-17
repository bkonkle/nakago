//! # Derive
use darling::FromMeta;
use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemImpl};

mod args;
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
