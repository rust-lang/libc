use syn::Ident;

use crate::ir::Field;

#[derive(Debug)]
pub struct Union {
    ident: Ident,
    fields: Vec<Field>,
}

impl Union {
    pub fn new(ident: Ident, fields: Vec<Field>) -> Self {
        Self { ident, fields }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn fields(&self) -> &Vec<Field> {
        &self.fields
    }
}
