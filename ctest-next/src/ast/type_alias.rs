use crate::BoxStr;

/// Represents a type alias defined in Rust.
#[derive(Debug, Clone)]
pub struct Type {
    #[expect(unused)]
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) ty: syn::Type,
}

impl Type {
    /// Return the identifier of the type alias as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
