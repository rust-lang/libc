use crate::BoxStr;

/// Represents a type alias defined in Rust.
#[derive(Debug, Clone)]
pub struct Type {
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) path: syn::Path,
    pub(crate) ty: syn::Type,
}

impl Type {
    /// Return the full path to the type alias as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.ident
    }

    /// Returns the last path of the identifier, from the absolute path returned
    /// by [`Type::path`].
    pub fn ident(&self) -> String {
        let Some(syn::PathSegment { ident, .. }) = self.path.segments.last() else {
            unreachable!("all parsed items have at least one element in their path")
        };
        ident.to_string()
    }
}
