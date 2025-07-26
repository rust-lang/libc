use crate::{BoxStr, Field};

/// Represents a struct defined in Rust.
#[derive(Debug, Clone)]
pub struct Struct {
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) fields: Vec<Field>,
}

impl Struct {
    /// Return the identifier of the struct as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
