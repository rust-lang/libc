use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct Static {
    ident: Ident,
    ty: TokenStream,
    // We do not care about the value as we only parse foreign statics.
}

impl Static {
    pub fn new(ident: Ident, ty: TokenStream) -> Self {
        Self { ident, ty }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &TokenStream {
        &self.ty
    }
}
