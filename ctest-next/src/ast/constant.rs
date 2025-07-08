use crate::BoxStr;

/// Represents a constant variable defined in Rust.
#[derive(Debug, Clone)]
pub struct Const {
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) ty: syn::Type,
    #[expect(unused)]
    pub(crate) expr: syn::Expr,
}

impl Const {
    /// Return the identifier of the constant as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
