/// Represents a field in a struct or union defined in Rust.
pub struct Field {
    public: bool,
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

impl Field {
    /// Creates a new field.
    pub fn new(public: bool, ident: Option<syn::Ident>, ty: syn::Type) -> Self {
        Self { public, ident, ty }
    }

    /// Return whether the field is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the field as a string if it exists.
    pub fn ident(&self) -> Option<String> {
        self.ident.as_ref().map(|i| i.to_string())
    }

    /// Return the type of the field.
    #[expect(unused)]
    pub(crate) fn ty(&self) -> &syn::Type {
        &self.ty
    }
}
