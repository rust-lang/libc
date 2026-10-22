use std::cmp::Ordering;
use std::ops;

use syn::punctuated::Punctuated;
use syn::{
    Path,
    PathSegment,
};

use crate::ast::RenameUtils;
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

impl RenameUtils for Module {
    fn addressable_paths(self) -> Vec<Path> {
        self.addressable_paths.clone()
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
            addressable_paths,
            ..self.clone()
        }
    }

    // [NOTE]: this is implemented in terms of rename_with but it doesn't just
    // yield the passed path, because that would mean all items would end up
    // with the same path. Instead, it calls into rename_with and skips as many
    // segments of all paths in the subtree as the current module has. Then it
    // appends whatever segments are left to the new path (new_path) we got
    // passed here. In the case of the module on which this was called, that
    // means all segments will be skipped and nothing will be appended to the
    // new path. In all other cases of items within its subtree, they will have
    // the previous identifier of the ancestor in which this rename request was
    // originally called trimmed, and whatever segments are left will be
    // appended to the new path of that (potentially grand-) ancestor.
    fn rename(self, new_path: Path) -> Self {
        let old_path_len = self.path.segments.len();
        self.clone().rename_with(|p| {
            let segments_post_ancestor = p.segments.clone().into_iter().skip(old_path_len);
            let mut new_path = new_path.clone();
            new_path.segments.extend(segments_post_ancestor);
            new_path.clone()
        })
    }

    fn rename_with(self, f: impl ops::Fn(Path) -> Path + Clone) -> Self {
        macro_rules! map_non_modules {
            ($($field:ident: $ty:tt),+ $(,)?) => {$(
                let $field = self.items
                    .$field
                    .clone()
                    .into_iter()
                    .map(|it| it.clone().rename(f(it.path.clone())))
                    .collect::<Vec<_>>();
            )+};
        }
        map_non_modules! {
            aliases: Type,
            structs: Struct,
            unions: Union,
            constants: Const,
            foreign_functions: Fn,
            foreign_statics: Static,
        }
        let modules: Vec<_> = self
            .items
            .modules
            .clone()
            .into_iter()
            .map(|m| m.clone().rename_with(f.clone()))
            .collect();
        let new_items = FfiItems {
            aliases: aliases.clone(),
            structs: structs.clone(),
            unions: unions.clone(),
            constants: constants.clone(),
            foreign_functions: foreign_functions.clone(),
            foreign_statics: foreign_statics.clone(),
            modules: modules.clone(),
            ..Default::default()
        };
        let new_path = f(self.path.clone());
        Module {
            cached_path: ast::path_to_string(new_path.clone()),
            cached_ident: new_path
                .segments
                .last()
                .map(|PathSegment { ident, .. }| ident.to_string().into_boxed_str())
                .unwrap_or_default(),
            path: new_path.clone(),
            addressable_paths: self
                .addressable_paths
                .clone()
                .into_iter()
                .chain([new_path.clone()])
                .collect(),
            items: new_items.clone(),
            ..self.clone()
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
