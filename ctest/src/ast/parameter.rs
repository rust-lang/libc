use crate::BoxStr;

/// Represents a parameter in a function signature defined in Rust.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub(crate) ident: BoxStr,
    #[expect(unused)]
    pub(crate) ty: syn::Type,
}

impl Parameter {
    /// Return the identifier of the parameter as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
