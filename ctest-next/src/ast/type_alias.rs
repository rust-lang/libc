/// Represents a type alias defined in Rust.
pub struct Type {
    public: bool,
    ident: syn::Ident,
    ty: syn::Type,
}

impl Type {
    /// Creates a new type alias.
    pub fn new(public: bool, ident: syn::Ident, ty: syn::Type) -> Self {
        Self { public, ident, ty }
    }

    /// Return whether the type alias is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the type alias as a string.
    pub fn ident(&self) -> String {
        self.ident.to_string()
    }

    /// Return the type that the type alias aliases.
    #[expect(unused)]
    pub(crate) fn ty(&self) -> &syn::Type {
        &self.ty
    }
}
