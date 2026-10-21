use std::cmp::Ordering;

use syn::Path;
use syn::punctuated::Punctuated;

use crate::ast::RenameUtils;
use crate::ffi_items::FfiItems;
use crate::{
    BoxStr,
    Const,
    Fn,
    Static,
    Struct,
    Type,
    Union,
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

    fn rename(self, new_path: Path) -> Self {
        let new_path_len = new_path.segments.len();
        macro_rules! rename_non_modules {
            ($($field:ident),+ $(,)?) => {$(
                let $field = self.items
                    .$field
                    .clone()
                    .into_iter()
                    .map(|it| {
                        let mut new_path = new_path.clone();
                        new_path
                            .segments
                            .extend(it.path.segments.clone().into_iter().skip(new_path_len));
                        it.clone().rename(new_path)
                    })
                    .collect();
            )+};
        }
        rename_non_modules! {
            aliases,
            structs,
            constants,
            foreign_functions,
            foreign_statics,
            unions,
        }
        let modules = self
            .items
            .modules
            .clone()
            .into_iter()
            .map(|it| {
                it.rename_with(|it| {
                    let mut new_path = new_path.clone();
                    new_path
                        .segments
                        .extend(it.segments.clone().into_iter().skip(new_path_len));
                    new_path
                })
            })
            .collect();
        Module {
            cached_path: ast::path_to_string(new_path.clone()),
            cached_ident: new_path
                .segments
                .last()
                .map(|s| s.ident.to_string().into_boxed_str())
                .unwrap_or_default(),
            path: new_path.clone(),
            addressable_paths: self
                .addressable_paths
                .clone()
                .into_iter()
                .chain([new_path.clone()])
                .collect(),
            items: FfiItems {
                aliases,
                structs,
                unions,
                constants,
                foreign_functions,
                foreign_statics,
                modules,
                ..self.items.clone()
            },
            ..self.clone()
        }
    }

    fn rename_with(self, f: impl std::ops::Fn(Path) -> Path + Clone) -> Self {
        let root_path = f(self.path.clone());
        let root_cache = ast::path_to_string(root_path.clone());
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
            .map(|m| m.rename_with(f.clone()))
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
        Module {
            cached_path: root_cache.clone(),
            path: root_path.clone(),
            addressable_paths: self
                .addressable_paths
                .clone()
                .into_iter()
                .chain([root_path.clone()])
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
