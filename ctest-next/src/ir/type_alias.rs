use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct TypeAlias {
    public: bool,
    ident: Ident,
    ty: TokenStream,
}

impl TypeAlias {
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
