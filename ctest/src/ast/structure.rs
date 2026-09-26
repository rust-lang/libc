use crate::{
    BoxStr,
    Field,
};

/// Represents a struct defined in Rust.
#[derive(Debug, Clone)]
pub struct Struct {
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) path: syn::Path,
    pub(crate) fields: Vec<Field>,
}

impl Struct {
    /// Return the full path to the struct item as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.ident
    }

    /// Returns the last path of the identifier, from the absolute path returned
    /// by [`Struct::path`].
    pub fn ident(&self) -> String {
        let Some(syn::PathSegment { ident, .. }) = self.path.segments.last() else {
            unreachable!("all parsed items have at least one element in their path")
        };
        ident.to_string()
    }
}
