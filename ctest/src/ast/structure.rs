use syn::Path;
use syn::punctuated::Punctuated;

use crate::ast::{
    PathAggregateStore,
    PathUtils,
};
use crate::{
    BoxStr,
    Field,
};

/// Represents a struct defined in Rust.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) path_aggregate: PathAggregateStore,
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
        &self.cached_path
    }

    /// Returns the last segment of the struct's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }
}

impl PathUtils for Struct {
    fn path_aggregate(&self) -> &PathAggregateStore {
        &self.path_aggregate
    }

    fn path(&self) -> &Path {
        &self.path
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
            ..self
        }
    }
}

impl Default for Struct {
    fn default() -> Self {
        let dpath = Path {
            leading_colon: None,
            segments: Punctuated::new(),
        };
        Self {
            public: Default::default(),
            cached_path: Default::default(),
            cached_ident: Default::default(),
            path: dpath.clone(),
            path_aggregate: PathAggregateStore::from_iter([dpath]),
            fields: vec![],
        }
    }
}
