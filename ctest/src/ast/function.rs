use syn::Path;

use crate::ast::{
    PathAggregateStore,
    PathUtils,
};
use crate::{
    Abi,
    BoxStr,
    Parameter,
};

/// Represents a function signature defined in Rust.
///
/// This structure is only used for parsing functions in extern blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct Fn {
    pub(crate) public: bool,
    #[expect(unused)]
    pub(crate) abi: Abi,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) path_aggregate: PathAggregateStore,
    pub(crate) link_name: Option<BoxStr>,
    #[expect(unused)]
    pub(crate) parameters: Vec<Parameter>,
    #[expect(unused)]
    pub(crate) return_type: Option<syn::Type>,
}

impl Fn {
    /// Return the full path to the function item as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the function's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }

    /// Return the name of the function to be linked C side with.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }
}

impl PathUtils for Fn {
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
        addressable_paths: PathAggregateStore,
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
