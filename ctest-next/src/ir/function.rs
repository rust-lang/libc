use proc_macro2::TokenStream;
use syn::Ident;

use crate::ir::Parameter;

#[derive(Debug)]
pub struct Function {
    public: bool,
    ident: Ident,
    parameters: Vec<Parameter>,
    return_value: TokenStream,
}

impl Function {
    pub fn new(
        public: bool,
        ident: Ident,
        parameters: Vec<Parameter>,
        return_value: TokenStream,
    ) -> Self {
        Self {
            public,
            ident,
            parameters,
            return_value,
        }
    }

    pub fn public(&self) -> bool {
        self.public
    }

    pub fn ident(&self) -> &Ident {
        &self.ident
    }

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn return_value(&self) -> &TokenStream {
        &self.return_value
    }
}
