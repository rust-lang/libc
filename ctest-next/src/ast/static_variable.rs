/// Represents a static variable in Rust.
///
/// This structure is only used for parsing statics in extern blocks,
/// as a result it does not have a field for storing the expression.
pub struct Static {
    public: bool,
    ident: syn::Ident,
    ty: syn::Type,
}

impl Static {
    /// Creates a new static variable.
    pub fn new(public: bool, ident: syn::Ident, ty: syn::Type) -> Self {
        Self { public, ident, ty }
    }

    /// Return whether the static variable is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the static variable as a string.
    pub fn ident(&self) -> String {
        self.ident.to_string()
    }

    /// Return the type of the static variable.
    #[expect(unused)]
    pub(crate) fn ty(&self) -> &syn::Type {
        &self.ty
    }
}
