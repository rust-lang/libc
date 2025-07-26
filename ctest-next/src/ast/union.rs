use crate::{BoxStr, Field};

/// Represents a union defined in Rust.
#[derive(Debug, Clone)]
pub struct Union {
    #[expect(unused)]
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) fields: Vec<Field>,
}

impl Union {
    /// Return the identifier of the union as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
