use darling::FromMeta;

#[derive(FromMeta)]
pub struct Provider {
    #[darling(default)]
    pub internal: bool,
}
