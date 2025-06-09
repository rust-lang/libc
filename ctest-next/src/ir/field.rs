use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct Field {
    public: bool,
    ident: Option<Ident>,
    ty: TokenStream,
}

impl Field {
    pub fn new(public: bool, ident: Option<Ident>, ty: TokenStream) -> Self {
        Self { public, ident, ty }
    }

    pub fn public(&self) -> bool {
        self.public
    }

    pub fn ident(&self) -> &Option<Ident> {
        &self.ident
    }

    pub fn ty(&self) -> &TokenStream {
        &self.ty
    }
}
