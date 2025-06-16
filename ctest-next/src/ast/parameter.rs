/// Represents a parameter in a function signature defined in Rust.
#[derive(Debug, Clone)]
pub struct Parameter {
    #[expect(unused)]
    pub(crate) pattern: syn::Pat,
    #[expect(unused)]
    pub(crate) ty: syn::Type,
}
