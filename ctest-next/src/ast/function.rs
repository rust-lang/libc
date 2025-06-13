use crate::Parameter;

/// Represents a function signature defined in Rust.
///
/// This structure is only used for parsing functions in extern blocks.
pub struct Fn {
    public: bool,
    ident: syn::Ident,
    parameters: Vec<Parameter>,
    return_type: Option<syn::Type>,
}

impl Fn {
    /// Creates a new function.
    pub fn new(
        public: bool,
        ident: syn::Ident,
        parameters: Vec<Parameter>,
        return_type: Option<syn::Type>,
    ) -> Self {
        Self {
            public,
            ident,
            parameters,
            return_type,
        }
    }

    /// Return whether the function is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the function as a string.
    pub fn ident(&self) -> String {
        self.ident.to_string()
    }

    /// Return the parameters of the function.
    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    /// Return the return type of the function.
    #[expect(unused)]
    pub(crate) fn return_type(&self) -> &Option<syn::Type> {
        &self.return_type
    }
}
