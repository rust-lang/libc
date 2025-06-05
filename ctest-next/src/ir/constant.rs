use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct Constant {
    ident: Ident,
    ty: TokenStream,
    value: TokenStream,
}

impl Constant {
    pub fn new(ident: Ident, ty: TokenStream, value: TokenStream) -> Self {
        Self { ident, ty, value }
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &TokenStream {
        &self.ty
    }

    pub fn value(&self) -> &TokenStream {
        &self.value
    }
}
