use syn::Path;

use crate::{
    BoxStr,
    Field,
};

/// Represents a union defined in Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) addressable_paths: Vec<Path>,
    pub(crate) fields: Vec<Field>,
}

impl Union {
    /// Return the full path to the union item as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the union's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }
}
