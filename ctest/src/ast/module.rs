use std::cmp::Ordering;
use std::ops;

use syn::Path;
use syn::punctuated::Punctuated;

use crate::ast::PathUtils;
use crate::ffi_items::{
    FfiItems,
    GenericItem,
};
use crate::{
    BoxStr,
    ast,
};

/// Represents a Rust module. `ctest` only considers items for which there is a corresponding type
/// in this crate.
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) uid: usize,
    pub(crate) public: bool,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: Path,
    pub(crate) items: FfiItems,
}

impl Module {
    /// Returns the full path to the module item.
    ///
    /// If inside a nested module, this will return a top-level-relative path, but not a
    /// crate-relative path. For some item `foo` in module `crate::bar`, the returned string will be
    /// `bar::foo`, and not `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the module's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }

    /// Comparison function for use when sorting modules in [`FfiItems::dedup`].
    ///
    /// This is not implemented as part of [`Ord`] because the implementation is inconsistent with
    /// the [`PartialEq`] implementation.
    pub(crate) fn custom_cmp(&self, other: &Self) -> Ordering {
        match self.cached_path.cmp(&other.cached_path) {
            Ordering::Equal => self.items.custom_cmp(&other.items),
            other => other,
        }
    }
}

impl PathUtils for Module {
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

    /** Override to ensure the effects of renaming a module cascade down to the subtree of items
     * rooted at it.
     *
     * Implementation-wise, it trims off the suffix of all items in the subtree rooted at the module
     * whose length matches that of the old module. Then it appends whatever path segments remain to
     * the new path of the module on which this rename request was originally made (`new_path`.) */
    fn rename(self, new_path: Path) -> Self {
        let old_path_len = self.path.segments.len();

        self.rename_with(|p| {
            p.segments
                .iter()
                .skip(old_path_len)
                .cloned()
                .fold(new_path.clone(), |new_path, s| Path {
                    segments: new_path.segments.into_iter().chain([s]).collect(),
                    leading_colon: new_path.leading_colon,
                })
        })
    }

    fn rename_with(self, f: impl ops::Fn(&Path) -> Path + Clone) -> Self {
        let new_items = self
            .items
            .items()
            .map(|it| {
                macro_rules! exe {
                    ($($ty:tt),+ $(,)?) => {
                        match it {
                            $(
                            GenericItem::$ty(it) => {
                                let new_path = f(&it.path);

                                GenericItem::$ty(it.rename(new_path))
                            }
                            )+
                            GenericItem::Module(it) => {
                                GenericItem::Module(it.rename_with(f.clone()))
                            }
                        }
                    };
                }

                exe! {
                    Type,
                    Struct,
                    Union,
                    Const,
                    Fn,
                    Static,
                }
            })
            .collect();

        let new_path = f(&self.path);

        Module {
            cached_path: ast::path_to_string(&new_path),
            cached_ident: ast::ident_to_string(&new_path),
            path: new_path,
            items: new_items,
            ..self
        }
    }
}

impl Default for Module {
    fn default() -> Self {
        Self {
            uid: Default::default(),
            public: Default::default(),
            cached_path: Default::default(),
            cached_ident: Default::default(),
            path: Path {
                leading_colon: None,
                segments: Punctuated::new(),
            },
            items: Default::default(),
        }
    }
}

/* TODO: this is either wrong or needs a refactor after the refactor that
 * removed the current_module field of FfiItems. */
/* NOTE: a custom implementation is used because FfiItems does not have a
 * PartialEq implementation. It is best if it does not have one. The
 * current_module field in FfiItems is not meant for use post-parse-time. It
 * would likely be unintuitive to have its PartialEq not take into account. */
impl PartialEq for Module {
    fn eq(&self, other: &Self) -> bool {
        self.public == other.public && self.path == other.path && self.items.custom_eq(&other.items)
    }
}
