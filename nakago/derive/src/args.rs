use darling::FromDeriveInput;
use proc_macro2::Ident;
use syn::{Attribute, Generics};

#[derive(FromDeriveInput)]
#[darling(attributes(inject), forward_attrs(doc))]
pub struct Provider {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub internal: bool,
}
