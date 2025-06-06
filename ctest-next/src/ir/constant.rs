use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct Constant {
    public: bool,
    ident: Ident,
    ty: TokenStream,
    value: TokenStream,
}

impl Constant {
    pub fn new(public: bool, ident: Ident, ty: TokenStream, value: TokenStream) -> Self {
        Self {
            public,
            ident,
            ty,
            value,
        }
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

    pub fn value(&self) -> &TokenStream {
        &self.value
    }
}
