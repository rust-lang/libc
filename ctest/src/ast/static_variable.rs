use syn::{
    Path,
    Type,
};

use crate::ast::PathUtils;
use crate::{
    Abi,
    BoxStr,
};

/// Represents a static variable in Rust.
///
/// This structure is only used for parsing statics in extern blocks,
/// as a result it does not have a field for storing the expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Static {
    pub(crate) public: bool,
    #[expect(unused)]
    pub(crate) abi: Abi,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) path_aggregate: Vec<Path>,
    pub(crate) link_name: Option<BoxStr>,
    pub(crate) ty: Type,
}

impl Static {
    /// Return the full path to the static variable as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the static's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }

    /// Return the name of the function to be linked C side with.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }
}

impl PathUtils for Static {
    fn path_aggregate(self) -> Vec<Path> {
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
