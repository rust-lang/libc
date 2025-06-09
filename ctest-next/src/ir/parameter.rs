use proc_macro2::TokenStream;

#[derive(Debug)]
pub struct Parameter {
    pattern: TokenStream,
    ty: TokenStream,
}

impl Parameter {
    pub fn new(pattern: TokenStream, ty: TokenStream) -> Self {
        Self { pattern, ty }
    }

    pub fn pattern(&self) -> &TokenStream {
        &self.pattern
    }

    pub fn ty(&self) -> &TokenStream {
        &self.ty
    }
}
