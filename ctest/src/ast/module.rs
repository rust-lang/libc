use std::cmp::Ordering;

use proc_macro2::Punct;
use syn::Path;
use syn::punctuated::Punctuated;

use crate::ffi_items::FfiItems;
use crate::{
    BoxStr,
    ast,
};

/// Represents a Rust module. `ctest` only considers items for which there is a
/// corresponding type in this crate.
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) addressable_paths: Vec<Path>,
    pub(crate) items: FfiItems,
}

impl Module {
    /// Returns the full path to the module item.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the module's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }

    /// Renames the receiver's path with `new_path`, ensuring that both the
    /// caches and the aggregate of addressable paths are updated.
    // [TODO]: use this throughout the ffi_items module to replace the repeated
    // occurrences of manual renaming implementations.
    pub(crate) fn rename_path(self, new_path: Path) -> Self {
        let Module {
            addressable_paths, ..
        } = self.clone();
        Self {
            cached_path: ast::path_to_string(new_path.clone()),
            cached_ident: new_path
                .segments
                .clone()
                .last()
                .expect("msg")
                .ident
                .to_string()
                .into_boxed_str(),
            path: new_path.clone(),
            addressable_paths: addressable_paths
                .clone()
                .into_iter()
                .chain([new_path])
                .collect(),
            ..self.clone()
        }
    }

    /// Comparison function for use when sorting modules in [`FfiItems::dedup`].
    ///
    /// This is not implemented as part of [`Ord`] because the implementation is
    /// inconsistent with the [`PartialEq`] implementation.
    pub(crate) fn custom_cmp(&self, other: &Self) -> Ordering {
        match self.cached_path.cmp(&other.cached_path) {
            Ordering::Equal => self.items.custom_cmp(&other.items),
            other => other,
        }
    }
}

impl Default for Module {
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
            items: FfiItems::default(),
        }
    }
}

// [NOTE]: a custom implementation is used because FfiItems does not have a
// PartialEq implementation. It is best if it does not have one. The
// current_module field in FfiItems is not meant for use post-parse-time. It
// would likely be unintuitive to have its PartialEq not take into account.
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.public == other.public
            && self.cached_path == other.cached_path
            && self.path == other.path
            && self.items.custom_eq(&other.items)
    }
}
