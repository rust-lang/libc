use crate::Field;

/// Represents a union defined in Rust.
pub struct Union {
    public: bool,
    ident: syn::Ident,
    fields: Vec<Field>,
}

impl Union {
    /// Creates a new union.
    pub fn new(public: bool, ident: syn::Ident, fields: Vec<Field>) -> Self {
        Self {
            public,
            ident,
            fields,
        }
    }

    /// Return whether the union is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the union as a string.
    pub fn ident(&self) -> String {
        self.ident.to_string()
    }

    /// Return the fields of the union.
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}
