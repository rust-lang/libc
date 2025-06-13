/// Represents a parameter in a function signature defined in Rust.
pub struct Parameter {
    pattern: syn::Pat,
    ty: syn::Type,
}

impl Parameter {
    /// Creates a new parameter.
    pub fn new(pattern: syn::Pat, ty: syn::Type) -> Self {
        Self { pattern, ty }
    }

    /// Return the pattern to match for the parameter.
    #[expect(unused)]
    pub(crate) fn pattern(&self) -> &syn::Pat {
        &self.pattern
    }

    /// Return the type of the parameter.
    #[expect(unused)]
    pub(crate) fn ty(&self) -> &syn::Type {
        &self.ty
    }
}
