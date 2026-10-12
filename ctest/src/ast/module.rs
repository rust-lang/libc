use syn::Path;
use syn::punctuated::Punctuated;

use crate::BoxStr;
use crate::ffi_items::FfiItems;

/// Represents a Rust module. `ctest` only considers items for which there is a
/// corresponding type in this crate.
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) path: Path,
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

    /// Returns the last path of the identifier, from the absolute path returned
    /// by [`Module::path`].
    pub fn ident(&self) -> String {
        let Some(syn::PathSegment { ident, .. }) = self.path.segments.last() else {
            unreachable!("all parsed items have at least one element in their path")
        };
        ident.to_string()
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            public: bool::default(),
            cached_path: String::new().into_boxed_str(),
            path: Path {
                leading_colon: None,
                segments: Punctuated::new(),
            },
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
