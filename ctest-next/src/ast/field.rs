use crate::BoxStr;

/// Represents a field in a struct or union defined in Rust.
#[derive(Debug, Clone)]
pub struct Field {
    #[expect(unused)]
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    #[expect(unused)]
    pub(crate) ty: syn::Type,
}

impl Field {
    /// Return the identifier of the field as a string if it exists.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
