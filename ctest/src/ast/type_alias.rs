use syn::Path;

use crate::BoxStr;
use crate::ast::PathUtils;

/// Represents a type alias defined in Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub(crate) uid: usize,
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
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
        &self.cached_path
    }

    /// Returns the last segment of the type alias's path; Namely, its
    /// identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }
}

impl PathUtils for Type {
    fn path(&self) -> &Path {
        &self.path
    }

    fn mk(self, cached_path: BoxStr, cached_ident: BoxStr, path: Path) -> Self {
        Self {
            cached_path,
            cached_ident,
            path,
            ..self
        }
    }
}
