use syn::{
    Path,
    Type,
};

use crate::BoxStr;
use crate::ast::{
    PathAggregateStore,
    PathUtils,
};

/// Represents a constant variable defined in Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) path_aggregate: PathAggregateStore,
    pub(crate) ty: Type,
}

impl Const {
    /// Return the full path to the constant variable as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the constant's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }
}

impl PathUtils for Const {
    fn path_aggregate(self) -> PathAggregateStore {
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
        path_aggregate: PathAggregateStore,
    ) -> Self {
        Self {
            cached_path,
            cached_ident,
            path,
            path_aggregate,
            ..self.clone()
        }
    }
}
