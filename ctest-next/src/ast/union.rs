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

    /// Return the public fields of the union.
    pub(crate) fn public_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter().filter(|f| f.public)
    }
}
