use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct Field {
    ident: Option<Ident>,
    ty: TokenStream,
}

impl Field {
    pub fn new(ident: Option<Ident>, ty: TokenStream) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &Option<Ident> {
        &self.ident
    }

    pub fn ty(&self) -> &TokenStream {
        &self.ty
    }
}
