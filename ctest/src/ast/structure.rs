use syn::Path;
use syn::punctuated::Punctuated;

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
    pub(crate) addressable_paths: Vec<Path>,
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

impl Default for Struct {
    fn default() -> Self {
        let dpath = Path {
            leading_colon: None,
            segments: Punctuated::new(),
        };
        Self {
            public: bool::default(),
            cached_path: String::new().into_boxed_str(),
            cached_ident: String::new().into_boxed_str(),
            path: dpath.clone(),
            addressable_paths: vec![dpath.clone()],
            fields: vec![],
        }
    }
}
