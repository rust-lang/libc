use syn::Path;

use crate::BoxStr;
use crate::ast::RenameUtils;

/// Represents a type alias defined in Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) path_aggregate: Vec<Path>,
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

impl RenameUtils for Type {
    fn aggregate_path(self) -> Vec<Path> {
        self.path_aggregate.clone()
    }

    fn path(self) -> Path {
        self.path.clone()
    }

    fn mk(
        self,
        cached_path: BoxStr,
        cached_ident: BoxStr,
        path: Path,
        addressable_paths: Vec<Path>,
    ) -> Self {
        Self {
            cached_path,
            cached_ident,
            path,
            path_aggregate: addressable_paths,
            ..self.clone()
        }
    }
}
