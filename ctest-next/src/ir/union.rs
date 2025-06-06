use syn::Ident;

use crate::ir::Field;

#[derive(Debug)]
pub struct Union {
    public: bool,
    ident: Ident,
    fields: Vec<Field>,
}

impl Union {
    pub fn new(public: bool, ident: Ident, fields: Vec<Field>) -> Self {
        Self {
            public,
            ident,
            fields,
        }
    }

    pub fn public(&self) -> bool {
        self.public
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}
