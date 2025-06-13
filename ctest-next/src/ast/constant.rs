/// Represents a constant variable defined in Rust.
pub struct Const {
    public: bool,
    ident: syn::Ident,
    ty: syn::Type,
    expr: syn::Expr,
}

impl Const {
    /// Creates a new constant.
    pub fn new(public: bool, ident: syn::Ident, ty: syn::Type, expr: syn::Expr) -> Self {
        Self {
            public,
            ident,
            ty,
            expr,
        }
    }

    /// Return whether the constant is visible to other crates.
    pub fn public(&self) -> bool {
        self.public
    }

    /// Return the identifier of the constant as a string.
    pub fn ident(&self) -> String {
        self.ident.to_string()
    }

    /// Return the type of the constant.
    #[expect(unused)]
    pub(crate) fn ty(&self) -> &syn::Type {
        &self.ty
    }

    /// Return the expression stored as a constant.
    #[expect(unused)]
    pub(crate) fn expr(&self) -> &syn::Expr {
        &self.expr
    }
}
