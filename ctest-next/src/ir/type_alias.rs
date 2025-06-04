use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct TypeAlias {
    ident: Ident,
    ty: TokenStream,
}

impl TypeAlias {
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
