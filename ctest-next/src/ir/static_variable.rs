use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct Static {
    public: bool,
    ident: Ident,
    ty: TokenStream,
    // We do not care about the value as we only parse foreign statics.
}

impl Static {
    pub fn new(public: bool, ident: Ident, ty: TokenStream) -> Self {
        Self { public, ident, ty }
    }

    pub fn public(&self) -> bool {
        self.public
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn ty(&self) -> &TokenStream {
        &self.ty
    }
}
