use crate::Field;

/// Represents a struct defined in Rust.
pub struct Struct {
    public: bool,
    ident: syn::Ident,
    fields: Vec<Field>,
}

impl Struct {
    /// Creates a new struct.
    pub fn new(public: bool, ident: syn::Ident, fields: Vec<Field>) -> Self {
        Self {
            public,
            ident,
            fields,
        }
    }

    /// Return whether the struct is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the struct as a string.
    pub fn ident(&self) -> String {
        self.ident.to_string()
    }

    /// Return the fields of the struct.
    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}
