//! Conversion of Rust code to a simplified abstract syntax tree.

use std::cmp::Ordering;
use std::iter;
use std::ops::{
    ControlFlow,
    Deref,
};

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::{
    Brace,
    Mod,
    PathSep,
};
use syn::visit::{
    self,
    Visit,
};
use syn::{
    File,
    Ident,
    Item,
    ItemMod,
    ItemUse,
    Path,
    PathArguments,
    PathSegment,
    UseName,
    UseRename,
    UseTree,
    Visibility,
};

use crate::ast::RenameUtils;
use crate::{
    Abi,
    BoxStr,
    Const,
    Field,
    Fn,
    Module,
    Parameter,
    Static,
    Struct,
    Type,
    Union,
    ast,
};

enum GenericItem {
    Type(Type),
    Struct(Struct),
    Union(Union),
    Const(Const),
    Fn(Fn),
    Static(Static),
    Module(Module),
}

/// Represents a collected set of top-level Rust items relevant to FFI
/// generation or analysis.
///
/// Includes foreign functions/statics, type aliases, structs, unions, and
/// constants. Modules are collected as recursive `FfiItems`, and currently used
/// to narrow down tested items to those in a specific module.
#[derive(Clone, Debug)]
pub(crate) struct FfiItems {
    pub(crate) aliases: Vec<Type>,
    pub(crate) structs: Vec<Struct>,
    pub(crate) unions: Vec<Union>,
    pub(crate) constants: Vec<Const>,
    pub(crate) foreign_functions: Vec<Fn>,
    pub(crate) foreign_statics: Vec<Static>,
    pub(crate) uses: Vec<RefinedUse>,
    pub(crate) modules: Vec<Module>,

    /// This is used while recursing through parsed modules to gather absolute
    /// paths to them as identifiers for both the modules and the items within
    /// them.
    pub(crate) current_module: syn::Path,
}

impl FfiItems {
    /// Creates a new blank FfiItems.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return whether the type has parsed a struct with the given identifier.
    pub(crate) fn contains_struct(&self, ident: &str) -> bool {
        self.structs()
            .iter()
            .any(|structure| structure.ident() == ident)
    }

    /// Return whether the type has parsed a union with the given identifier.
    pub(crate) fn contains_union(&self, ident: &str) -> bool {
        self.unions().iter().any(|union| union.ident() == ident)
    }

    /// Return a list of all type aliases found.
    pub(crate) fn aliases(&self) -> &Vec<Type> {
        &self.aliases
    }

    /// Return a list of all structs found.
    pub(crate) fn structs(&self) -> &Vec<Struct> {
        &self.structs
    }

    /// Return a list of all unions found.
    pub(crate) fn unions(&self) -> &Vec<Union> {
        &self.unions
    }

    /// Return a list of all constants found.
    pub(crate) fn constants(&self) -> &Vec<Const> {
        &self.constants
    }

    /// Return a list of all foreign functions found mapped by their ABI.
    pub(crate) fn foreign_functions(&self) -> &Vec<Fn> {
        &self.foreign_functions
    }

    /// Return a list of all foreign statics found mapped by their ABI.
    pub(crate) fn foreign_statics(&self) -> &Vec<Static> {
        &self.foreign_statics
    }

    /// Entry point to parse a [`syn::File`].
    ///
    /// This will also resolve `use`-trees such that the resulting `FfiItems`
    /// instance is always left with no reexports that are not sourced from
    /// third-party crates.
    pub(crate) fn visit_file(&mut self, file: &syn::File) {
        let File {
            attrs,
            items: mod_items,
            ..
        } = file;
        let file_mod = ItemMod {
            attrs: attrs.clone(),
            vis: Visibility::Public(syn::Token![pub](Span::call_site())),
            unsafety: None,
            mod_token: Mod::default(),
            ident: Ident::new("__ctest_root_mod", Span::call_site()),
            content: Some((Brace::default(), mod_items.clone())),
            semi: None,
        };
        self.visit_item_mod(&file_mod);
        let root_module = Module {
            items: self.clone(),
            ..Default::default()
        };
        *self = resolve_all(root_module.clone()).items.clone();
    }

    /// Performs a comparison for equality between FfiItems without taking field
    /// current_module into consideration.
    ///
    /// This is not an implementation of PartialEq for FfiItems because it would
    /// be unintuitive to have it ignore one field. This is only ever needed
    /// during use declaration resolution.
    ///
    /// The visibility restriction allows the Module type in ast/module.rs to
    /// have access to it.
    pub(super) fn custom_eq(&self, other: &Self) -> bool {
        self.aliases == other.aliases
            && self.structs == other.structs
            && self.unions == other.unions
            && self.constants == other.constants
            && self.foreign_functions == other.foreign_functions
            && self.foreign_statics == other.foreign_statics
            && self.uses == other.uses
            && self.modules == other.modules
    }

    /// Comparison function for use when sorting and deduplicating modules in
    /// [`FfiItems::dedup`].
    ///
    /// This is used in [`Module::custom_cmp`]. It trusts that the modules to
    /// compare have the same overall shape, and one will be a "natural"
    /// extension of the other.
    ///
    /// We consider "extension" here to mean that one module subtree is based on
    /// the other, and thus only contains both the items from the other and
    /// possibly some more items.
    pub(super) fn custom_cmp(&self, other: &Self) -> Ordering {
        let (ours, theirs) = (self.modules.len(), other.modules.len());
        // [NOTE]: the goal here is to have modules with "larger" subtrees
        // appear first in the item container, so declaring them to be "smaller"
        // is required for slice::sort_unstable_by to put them first in the
        // sorted list of modules in FfiItems::dedup. The same applies the other
        // way around with "smaller" subtrees that we would like to have appear
        // later in the sorted list of modules.
        match ours.cmp(&theirs) {
            Ordering::Equal => self
                .modules
                .clone()
                .into_iter()
                .zip(other.modules.clone())
                .fold(Ordering::Equal, |_, (our_m, their_m)| {
                    // [NOTE]: this is intended to not start anew a "regular"
                    // module comparison through Module::custom_cmp, but to
                    // rather have all subsequent comparisons be based off of
                    // the sizes of module subtrees rather than the
                    // lexicographical comparison of their paths.
                    our_m.items.custom_cmp(&their_m.items)
                }),
            other => other.reverse(),
        }
    }

    /// Searches for an item in the container, and returns an owned instance of
    /// that item.
    ///
    /// The generic item may be pattern-matched to find out specifically which
    /// item was found.
    fn search(&self, ident: syn::Ident) -> Option<GenericItem> {
        let aliases = self.aliases.clone().into_iter().map(GenericItem::Type);
        let structs = self.structs.clone().into_iter().map(GenericItem::Struct);
        let unions = self.unions.clone().into_iter().map(GenericItem::Union);
        let constants = self.constants.clone().into_iter().map(GenericItem::Const);
        let foreign_functions = self
            .foreign_functions
            .clone()
            .into_iter()
            .map(GenericItem::Fn);
        let foreign_statics = self
            .foreign_statics
            .clone()
            .into_iter()
            .map(GenericItem::Static);
        self.modules
            .clone()
            .into_iter()
            .map(GenericItem::Module)
            .chain(foreign_statics)
            .chain(foreign_functions)
            .chain(constants)
            .chain(unions)
            .chain(structs)
            .chain(aliases)
            .find(|it| {
                macro_rules! cmp {
                    ($($var:tt),+ $(,)?) => {
                        match it {$(
                            GenericItem::$var(it) => it.ident() == ident.to_string(),
                        )+}
                    };
                }
                cmp! {
                    Type,
                    Struct,
                    Union,
                    Const,
                    Fn,
                    Static,
                    Module,
                }
            })
    }

    /// Deduplicates the items in an item container by their path.
    ///
    /// Items of differing kind, like functions and constants, that may happen
    /// to have the same path will not get deduplicated. A consequence of
    /// calling this method is that the items are also sorted by their paths.
    fn dedup(self) -> FfiItems {
        macro_rules! key_fn {
            () => {
                |it| it.cached_path.clone()
            };
        }
        macro_rules! dedup {
            ($($it:tt),+ $(,)?) => {{
                let items = self.clone();
                $(
                    let mut $it = self.$it.clone();
                    $it.sort_unstable_by_key(key_fn!());
                    $it.dedup_by_key(key_fn!());
                    let items = FfiItems {
                        $it: $it.clone(),
                        ..items.clone()
                    };
                )+
                items
            }};
        }
        let new_items = dedup!(
            aliases,
            structs,
            unions,
            constants,
            foreign_functions,
            foreign_statics,
        );
        let mut new_modules = self.modules.clone();
        new_modules.sort_unstable_by(Module::custom_cmp);
        new_modules.dedup_by(|lhs, rhs| lhs.path == rhs.path);
        FfiItems {
            modules: new_modules.clone(),
            ..new_items.clone()
        }
    }
}

impl Default for FfiItems {
    fn default() -> Self {
        Self {
            aliases: Default::default(),
            structs: Default::default(),
            unions: Default::default(),
            constants: Default::default(),
            foreign_functions: Default::default(),
            foreign_statics: Default::default(),
            modules: Default::default(),
            uses: Default::default(),
            current_module: Path {
                leading_colon: None,
                segments: Default::default(),
            },
        }
    }
}

/// Refined type for [`syn::UsePath`] that does not consider group paths.
///
/// This is a lot like [`RefinedUse`] in that it only exists as replacement for
/// [`syn::UsePath`] but uses [`RefinedUseTree`] instead of [`syn::UseTree`].
/// See [`RefinedUseTree`] for details.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RefinedUsePath {
    ident: Ident,
    tree: Box<RefinedUseTree>,
}

impl Default for RefinedUsePath {
    fn default() -> Self {
        Self {
            ident: Ident::new("__dummy", Span::call_site()),
            tree: Box::default(),
        }
    }
}

/// Refined type for [`syn::UseTree`] that does not consider group paths.
///
/// This is the byproduct of normalizing a `use` declaration's path through
/// [`normalize_path`]. That function ensures some such path containing a group
/// as its tail end is converted into a set of paths without groups.
///
/// This enumeration is thus equivalent to `syn::UseTree` but does not consider
/// the case of `syn::UseTree::Group`.
#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) enum RefinedUseTree {
    Path(RefinedUsePath),
    Name(UseName),
    Rename(UseRename),
    #[default]
    Glob,
}

/// Refined type for `use` declarations that do not consider group paths.
///
/// This exists only for the purpose of refining [`syn::UseTree`], which is used
/// within [`syn::ItemUse`]. See type [`RefinedUseTree`] for the actual
/// refinement and details on the purpose of this type.
#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct RefinedUse {
    is_public: bool,
    tree: RefinedUseTree,
}

/// Gets rid of group reexports in a `use` declaration by flattenning them into
/// a set of individual reexports.
fn normalize_path(path: syn::UseTree) -> Vec<RefinedUseTree> {
    match path {
        UseTree::Name(n) => vec![RefinedUseTree::Name(n)],
        UseTree::Rename(r) => vec![RefinedUseTree::Rename(r)],
        UseTree::Glob(_) => vec![RefinedUseTree::Glob],

        UseTree::Path(syn::UsePath { ident, tree, .. }) => normalize_path(*tree)
            .into_iter()
            .map(Box::new)
            .map(|tree| {
                RefinedUseTree::Path(RefinedUsePath {
                    ident: ident.clone(),
                    tree,
                })
            })
            .collect(),

        UseTree::Group(syn::UseGroup { items, .. }) => {
            items.into_iter().map(normalize_path).flatten().collect()
        }
    }
}

/// Stateful pair carried through resolution passes.
///
/// Each `use` declaration requires access to either one of (1) the module where
/// it was sourced from, or (2) the crate root's subtree, or (3) any one
/// ascendant or descendant module of the module where it was sourced from.
///
/// Points (2) and (3) require having access to the entire crate's module tree.
/// Some ancestor item is looked up by the parent path to the item being
/// searched in respect of. Point (1) is satisfied by the `subtree` field.
#[derive(Debug, Clone)]
struct PassPair {
    subtree: Module,
    whole_crate: Module,
}

/// This is the resolution driver. It is the entrypoint for whole crate
/// resolution.
///
/// It performs passes on a given module. It diffs the results of a whole-crate
/// pass with the pre-pass state. Equality in that comparison determines
/// termination.
fn resolve_all(root: Module) -> Module {
    let (ControlFlow::Break(m) | ControlFlow::Continue(m)) =
        iter::repeat(()).try_fold(root.clone(), |root, _| {
            // [NOTE]: it holds that a pass over the crate root will yield the
            // crate root as its subtree. It is thus safe to ignore the
            // whole-crate module tree. They are the same at this point.
            let PassPair {
                subtree: new_root, ..
            } = resolve_pass(PassPair {
                subtree: root.clone(),
                whole_crate: root.clone(),
            });
            match new_root == root {
                true => ControlFlow::Break(new_root.clone()),
                false => ControlFlow::Continue(new_root.clone()),
            }
        });
    m
}

/// Represents a resolved or unresolved `use` declaration.
///
/// The state of resolution may change across resolution passes. It is used in
/// [`merge_module`] to merge whichever `use` declarations resolved to a set of
/// items back into the module they were sourced from.
#[derive(Debug, Clone)]
enum Resolution {
    Resolved(FfiItems),
    Unresolved,
}

/// Performs one pass through the provided pair of a subtree and the whole-crate
/// subtree.
///
/// The input pair's fields correspond with the subtree in which to perform the
/// resolution pass on, and the tree at the crate root. The latter is required
/// to resolve absolute imports that refer to some item in an ancestor module
/// that is more than one level up the tree than the subtree being resolved.
///
/// The returned pair contains updated snapshots of the input pair. The subtree
/// may have had new items added to it. The whole-crate module tree is
/// guaranteed to have integrated these changes.
fn resolve_pass(src: PassPair) -> PassPair {
    let (
        PassPair {
            whole_crate: whole_crate_with_resolved_children,
            ..
        },
        resolved_children,
    ) = src.subtree.items.modules.clone().into_iter().fold(
        (src.clone(), Vec::new()),
        |(PassPair { whole_crate, .. }, children), m| {
            let pair = resolve_pass(PassPair {
                subtree: m.clone(),
                whole_crate: whole_crate.clone(),
            });
            let mut new_children = children.clone();
            new_children.push(pair.subtree.clone());
            (pair.clone(), new_children.clone())
        },
    );
    let root_with_resolved_children = Module {
        items: FfiItems {
            modules: resolved_children.clone(),
            ..src.subtree.items.clone()
        },
        ..src.subtree.clone()
    };
    let resolved_uses = resolve_one(PassPair {
        subtree: root_with_resolved_children.clone(),
        whole_crate: whole_crate_with_resolved_children.clone(),
    });
    let new_root_with_duplicates =
        merge_module(root_with_resolved_children.clone(), resolved_uses.clone());
    let new_root = Module {
        items: new_root_with_duplicates.items.clone().dedup(),
        ..new_root_with_duplicates.clone()
    };
    let new_whole_crate = update_tree(new_root.clone(), whole_crate_with_resolved_children.clone());
    println!(
        "finished module pass on module {}",
        if &*new_root.cached_path == "" {
            "root".into()
        } else {
            new_root.cached_path.clone()
        },
    );
    PassPair {
        subtree: new_root.clone(),
        whole_crate: new_whole_crate.clone(),
    }
}

/// Attempts to resolve the `use` declarations in a given module.
///
/// The input pair consists of the module from which to source the `use`
/// declarations, and of the whole-crate module tree. The latter is used to
/// resolve certain types of paths in `use` declarations.
///
/// See [`resolve_pass`] for details.
fn resolve_one(state: PassPair) -> Vec<Resolution> {
    println!(
        "found {} use declarations in module {}",
        state.subtree.items.uses.len(),
        if &*state.subtree.cached_path == "" {
            "root".into()
        } else {
            state.subtree.cached_path.clone()
        },
    );
    state
        .subtree
        .items
        .uses
        .clone()
        .into_iter()
        .map(|u| resolve_use(u.clone(), state.clone()))
        .collect()
}

/// Attempts to resolve a single `use` declaration.
///
/// This can go one of two ways.
///
/// 1. Either the `use` declaration matches a specific module in the stateful
///    pair, or
///
/// 2. the `use` declaration is assumed to be unresolved in this resolution
///    pass.
///
/// The stateful pair initially consists of the module in which the `use`
/// declaration appears at. If the `use` declaration refers to an item by a path
/// with more than one segment, this function recursively munches through it.
///
/// These recursive calls will only be made if the extracted path segment
/// matches against some item in the stateful subtree. Currently, the only types
/// of items that can match on a path that still has another unmunched segment
/// are modules.
///
/// Upon finding the module, the new stateful subtree of the pair becomes this
/// module. This is then used in the recursive call with the path tail.
///
/// The other element of the pair is the whole-crate module tree. That is used
/// to resolve ancestor paths (when `super` is found) and absolute paths (when
/// `crate` is found.)
fn resolve_use(r#use: RefinedUse, state: PassPair) -> Resolution {
    macro_rules! single_item {
        ($field:ident: $it:expr) => {
            return Resolution::Resolved(FfiItems {
                $field: Vec::from([$it.clone()]),
                ..Default::default()
            })
        };
    }
    // [NOTE]: this ensures any returned item has the visibility of the use
    // declaration, and doesn't preserve its own visibility.
    macro_rules! change_vis {
        ($it:ident : $ty:tt) => {
            $ty {
                public: r#use.is_public,
                ..$it.clone()
            }
        };
    }
    let change_vis_in_mod = |m: Module| {
        macro_rules! c {
            ($($field:ident: $ty:tt),+ $(,)?) => {$(
                let $field: Vec<_> = m.items.$field
                    .clone()
                    .into_iter()
                    .map(|it| change_vis!(it: $ty))
                    .collect();
            )+};
        }
        c! {
            aliases: Type,
            constants: Const,
            structs: Struct,
            unions: Union,
            foreign_functions: Fn,
            foreign_statics: Static,
            modules: Module,
        }
        Module {
            items: FfiItems {
                aliases,
                constants,
                structs,
                unions,
                foreign_functions,
                foreign_statics,
                modules,
                ..m.items.clone()
            },
            ..m.clone()
        }
    };
    match r#use.tree.clone() {
        RefinedUseTree::Name(UseName { ident }) => {
            println!(
                "entered name resolution case with {} in module {}",
                ident.clone(),
                state.subtree.cached_path.clone()
            );
            // [NOTE]: self may not appear as the name basecase of a well-formed
            // path. And yet it is still considered here. That is because of
            // path normalization. self may appear as part of a tail group in a
            // use declaration's path. Path normalization flattens groups into
            // individual use statements. Path normalization is done through
            // normalize_path above. That happens at parse-time. That is why
            // this function must consider the case of self as a name basecase.
            // [NOTE]: crate does not appear as the tail end of a path in a use
            // declaration under no circumstances, so that case is not
            // considered.
            if is_self(ident.clone()) {
                single_item!(modules: change_vis_in_mod(state.subtree.clone()));
            } else if is_super(ident.clone()) {
                let parent_path = get_parent_path(state.subtree.clone());
                let Some(parent) = get_module(parent_path.clone(), state.whole_crate.clone())
                else {
                    unreachable!(
                        "The module path is guaranteed to be found in the \
                         whole crate module tree. It is the path of the \
                         stateful subtree, which is either the module in which \
                         the use declaration appears at, some descendant of \
                         that module, or some descendant module to the crate \
                         root after having stumbled on some ancestor-referring \
                         use declaration. The parent some such module must be \
                         below the crate root, as otherwise it would be \
                         considered malformed input."
                    );
                };
                single_item!(modules: change_vis_in_mod(parent.clone()));
            }
            match state.subtree.items.search(ident.clone()) {
                Some(GenericItem::Type(t)) => single_item!(aliases: change_vis!(t: Type)),
                Some(GenericItem::Struct(s)) => single_item!(structs: change_vis!(s: Struct)),
                Some(GenericItem::Union(u)) => single_item!(unions: change_vis!(u: Union)),
                Some(GenericItem::Const(c)) => single_item!(constants: change_vis!(c: Const)),
                Some(GenericItem::Fn(f)) => single_item!(foreign_functions: change_vis!(f: Fn)),
                Some(GenericItem::Static(s)) => {
                    single_item!(foreign_statics: change_vis!(s: Static))
                }
                Some(GenericItem::Module(m)) => single_item!(modules: change_vis!(m: Module)),
                None => Resolution::Unresolved,
            }
        }
        RefinedUseTree::Glob => {
            println!(
                "entered glob resolution case in module {}",
                if &*state.subtree.cached_path == "" {
                    "root".into()
                } else {
                    state.subtree.cached_path.clone()
                },
            );
            Resolution::Resolved(change_vis_in_mod(state.subtree.clone()).items)
        }
        RefinedUseTree::Rename(UseRename { ident, rename, .. }) => {
            println!(
                "entered rename resolution case with {:?} in module {}",
                UseRename {
                    ident: ident.clone(),
                    as_token: syn::parse_quote! {as},
                    rename: rename.clone(),
                },
                state.subtree.cached_path.clone()
            );
            macro_rules! new_path {
                ($it:expr) => {{
                    let new_segments = $it
                        .path
                        .segments
                        .clone()
                        .into_iter()
                        .skip($it.path.segments.len() - 1)
                        .fold(Punctuated::new(), |p, _| {
                            let mut p = p.clone();
                            p.push(PathSegment {
                                ident: rename.clone(),
                                arguments: PathArguments::None,
                            });
                            p.clone()
                        });
                    Path {
                        segments: new_segments.clone(),
                        ..$it.path.clone()
                    }
                }};
            }
            macro_rules! renamed_single_item {
                ($field:ident , $ty:tt : $it:expr) => {{
                    let new_path = new_path!($it.clone());
                    let new_item = $it.rename(new_path.clone());
                    single_item!($field: new_item.clone());
                }};
            }
            let renamer = |m: Module| {
                let new_path = new_path!(m.clone());
                // [NOTE]: it may seem better to use the segment length of the
                // old path of the module, much like we do in function
                // RenameUtils::rename, but that is not necessary here. We know
                // the segment length will be the same before and after the
                // rename because use declarations can only rename the module's
                // identifier. The module identifier only accounts for one
                // segment of the item's path.
                let new_path_len = new_path.segments.len();
                m.clone().rename_with(|p| {
                    let new_path = p.segments.clone().into_iter().skip(new_path_len).fold(
                        new_path.clone(),
                        |p, s| {
                            let mut p = p.clone();
                            p.segments.push(s);
                            p.clone()
                        },
                    );
                    Path {
                        leading_colon: p.leading_colon,
                        ..new_path.clone()
                    }
                })
            };
            if is_self(ident.clone()) {
                single_item!(modules: change_vis_in_mod(renamer(state.subtree.clone())));
            } else if is_super(ident.clone()) {
                let parent_path = get_parent_path(state.subtree.clone());
                let Some(parent) = get_module(parent_path.clone(), state.whole_crate.clone())
                else {
                    unreachable!(
                        "See the note on super in the name basecase. The same \
                         applies here."
                    );
                };
                single_item!(modules: change_vis_in_mod(renamer(parent.clone())));
            } else if is_crate(ident.clone()) {
                single_item!(modules: change_vis_in_mod(renamer(state.whole_crate.clone())));
            }
            match state.subtree.items.search(ident.clone()) {
                Some(GenericItem::Module(m)) => {
                    single_item!(modules: change_vis_in_mod(renamer(m.clone())))
                }
                Some(GenericItem::Type(t)) => renamed_single_item!(aliases, Type: t.clone()),
                Some(GenericItem::Struct(s)) => renamed_single_item!(structs, Struct: s.clone()),
                Some(GenericItem::Union(u)) => renamed_single_item!(unions, Union: u.clone()),
                Some(GenericItem::Const(c)) => renamed_single_item!(constants, Const: c.clone()),
                Some(GenericItem::Fn(f)) => renamed_single_item!(foreign_functions, Fn: f.clone()),
                Some(GenericItem::Static(s)) => {
                    renamed_single_item!(foreign_statics, Static: s.clone())
                }
                None => Resolution::Unresolved,
            }
        }

        RefinedUseTree::Path(RefinedUsePath { ident, tree }) => {
            println!(
                "entered path resolution case with {} in module {}",
                ident.clone(),
                state.subtree.cached_path.clone()
            );
            let new_subtree = if is_self(ident.clone()) {
                state.subtree.clone()
            } else if is_super(ident.clone()) {
                let parent_path = get_parent_path(state.subtree.clone());
                let Some(parent) = get_module(parent_path.clone(), state.whole_crate.clone())
                else {
                    unreachable!(
                        "See the note on super in the name basecase. The same \
                         applies here."
                    );
                };
                parent.clone()
            } else if is_crate(ident.clone()) {
                state.whole_crate.clone()
            } else {
                match state.subtree.items.search(ident.clone()) {
                    Some(GenericItem::Module(m)) => m.clone(),
                    Some(_) | None => return Resolution::Unresolved,
                }
            };
            resolve_use(
                RefinedUse {
                    tree: *tree.clone(),
                    ..r#use
                },
                PassPair {
                    subtree: new_subtree.clone(),
                    ..state.clone()
                },
            )
        }
    }
}

/// Attempts to find a given module path (`target`) within a module subtree
/// (`root`.)
fn get_module(target: Path, root: Module) -> Option<Module> {
    if root.path == target {
        return root.clone().into();
    }
    root.items
        .modules
        .clone()
        .into_iter()
        .find_map(|m| get_module(target.clone(), m.clone()))
}

/// Fetches the parent path to a given module.
///
/// If the module is a top-level item (i.e. it appears at the crate root,) the
/// returned path will be empty. This follows from the parsing strategy for
/// paths in `FfiItems`. The absolute paths are all `crate`-_unprepended_.
fn get_parent_path(root: Module) -> Path {
    // [NOTE]: this performs a saturating_sub because there is one and only one
    // item in the crate's module tree that has an empty path; The crate root.
    // That is only wrapped in a Module of its own only after parsing.  This
    // function currently only gets used during parsing, so there is a chance
    // this gets called with the virtual module that wraps the crate root's
    // FfiItems. That is not logically possible, because well-formed input can
    // not have a use declaration with super in the path at the crate root. This
    // function is used when stumbling upon super in function resolve_use.
    //
    // The above assumption does not change the way this is implemented, except
    // you could very well change the saturating_sub for overflowing
    // subtraction. It is best if this function is protected against what would
    // currently be misuse.
    let new_segments = root
        .path
        .segments
        .clone()
        .into_iter()
        .take(root.path.segments.len().saturating_sub(1))
        .fold(Punctuated::new(), |p, s| {
            let mut p = p.clone();
            p.push(s);
            p.clone()
        });
    Path {
        leading_colon: None,
        segments: new_segments.clone(),
    }
}

/// Checks if the given identifier was parsed as the Rust keyword `self`.
fn is_self(ident: syn::Ident) -> bool {
    ident.to_string() == "self"
}

/// Checks if the given identifier was parsed as the Rust keyword `crate`.
fn is_crate(ident: syn::Ident) -> bool {
    ident.to_string() == "crate"
}

/// Checks if the given identifier was parsed as the Rust keyword `super`.
fn is_super(ident: syn::Ident) -> bool {
    ident.to_string() == "super"
}

/// Integrates module subtree `src` into module subtree `dst`.
///
/// It attempts to find `src` in `dst`. It returns an "updated" snapshot of
/// `dst` with the contents of `src` integrated. Note `src` may or may not have
/// been found within some subtree of `dst`.
///
/// If not found, `dst` is returned unmodified. `src` may also match `dst`
/// itself. That would return `src`.
fn update_tree(src: Module, dst: Module) -> Module {
    if src.path == dst.path {
        return src.clone().into();
    }
    let new_children: Vec<_> = dst
        .items
        .modules
        .clone()
        .into_iter()
        .map(|m| update_tree(src.clone(), m.clone()))
        .collect();
    Module {
        items: FfiItems {
            modules: new_children.clone(),
            ..dst.items.clone()
        },
        ..dst.clone()
    }
}

/// Integrates the resolution items of `src` into `dst`.
///
/// Each resolution item is either one of an `Unresolved` or a `Resolved`
/// variant. The former are essentially ignored. The latter pack an `FfiItems`
/// that is integrated into `dst`'s own items. That is the merging part.
///
/// Note that, during merging, the items to be merged may end up duplicating
/// existing items on `dst`. It is call-site's responsibility to deduplicate
/// those through [`FfiItems::dedup`].
fn merge_module(dst: Module, src: Vec<Resolution>) -> Module {
    let new_path = dst.path.clone();
    src.clone()
        .into_iter()
        .filter_map(|r| {
            if let Resolution::Resolved(items) = r.clone() {
                Some(items.clone())
            } else {
                None
            }
        })
        .map(
            |FfiItems {
                 aliases,
                 structs,
                 unions,
                 constants,
                 foreign_functions,
                 foreign_statics,
                 modules,
                 ..
             }| {
                macro_rules! seek_parent {
                    ($field:ident) => {{
                        $field.first().map(|it| {
                            it.path
                                .segments
                                .clone()
                                .into_iter()
                                .take(it.path.segments.len() - 1)
                                .collect::<Punctuated<PathSegment, PathSep>>()
                        })
                    }};
                }
                // [NOTE]: this ensures we find the immediate parent to any one
                // of the items about to be merged. If there really are no items
                // about to be merged, then we can simply return an empty
                // FfiItems from the mapping, which would essentially merge no
                // items down in the fold that finishes up the merging function.
                // We don't invoke unreachable! in the diverging branch because
                // a resolution item could pack an empty item container if a use
                // declaration consists a glob import of all items in a module
                // with no items.
                let Some(source_module_path) = seek_parent!(aliases)
                    .or_else(|| seek_parent!(structs))
                    .or_else(|| seek_parent!(unions))
                    .or_else(|| seek_parent!(constants))
                    .or_else(|| seek_parent!(foreign_functions))
                    .or_else(|| seek_parent!(foreign_statics))
                    .or_else(|| seek_parent!(modules))
                else {
                    return FfiItems::default();
                };
                let dummy_module = Module {
                    path: Path {
                        leading_colon: None,
                        segments: {
                            let mut base = Punctuated::new();
                            base.extend(source_module_path);
                            base.clone()
                        },
                    },
                    items: FfiItems {
                        aliases: aliases.clone(),
                        structs: structs.clone(),
                        unions: unions.clone(),
                        constants: constants.clone(),
                        foreign_functions: foreign_functions.clone(),
                        foreign_statics: foreign_statics.clone(),
                        modules: modules.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let new_module = dummy_module.clone().rename(new_path.clone());
                fn remove_uses(m: Module) -> Module {
                    let new_modules: Vec<_> = m
                        .items
                        .modules
                        .clone()
                        .into_iter()
                        .map(remove_uses)
                        .collect();
                    Module {
                        items: FfiItems {
                            uses: vec![],
                            modules: new_modules.clone(),
                            ..m.items.clone()
                        },
                        ..m.clone()
                    }
                }
                // [NOTE]: this removes all use declarations in all descendant
                // modules of the subtrees to merge because otherwise merged
                // modules can increase the number of use declarations in the
                // crate's module tree, which is one cause of non-termination.
                let new_modules_without_uses: Vec<_> = new_module
                    .items
                    .modules
                    .clone()
                    .into_iter()
                    .map(remove_uses)
                    .collect();
                FfiItems {
                    modules: new_modules_without_uses.clone(),
                    ..new_module.items.clone()
                }
            },
        )
        .fold(
            dst.clone(),
            |dst,
             FfiItems {
                 aliases,
                 structs,
                 unions,
                 constants,
                 foreign_functions,
                 foreign_statics,
                 modules,
                 ..
             }| {
                let mut dst = dst.clone();
                dst.items.aliases.extend_from_slice(&aliases);
                dst.items.structs.extend_from_slice(&structs);
                dst.items.unions.extend_from_slice(&unions);
                dst.items.constants.extend_from_slice(&constants);
                dst.items
                    .foreign_functions
                    .extend_from_slice(&foreign_functions);
                dst.items
                    .foreign_statics
                    .extend_from_slice(&foreign_statics);
                dst.items.modules.extend_from_slice(&modules);
                dst.clone()
            },
        )
}

/// Appends a new module-local item to an absolute path that does _not_ start
/// with `crate`.
///
/// This is used whenever we need to create paths for either items or modules.
fn append_path(base: Path, new: Ident) -> Path {
    if base.segments.is_empty() {
        syn::parse_quote! { #new }
    } else {
        syn::parse_quote! { #base::#new }
    }
}

/// Determine whether an item is visible to other crates.
///
/// This function assumes that if the visibility is restricted then it is not
/// meant to be accessed.
fn is_visible(vis: &syn::Visibility) -> bool {
    match vis {
        syn::Visibility::Public(_) => true,
        syn::Visibility::Inherited | syn::Visibility::Restricted(_) => false,
    }
}

/// Collect fields in a syn grammar into ctest's equivalent structure.
fn collect_fields(fields: &Punctuated<syn::Field, syn::Token![,]>) -> Vec<Field> {
    fields
        .iter()
        .filter_map(|field| {
            field.ident.as_ref().map(|ident| {
                let ident = ident.to_string();
                Field {
                    public: is_visible(&field.vis),
                    ident: ident
                        .strip_prefix("r#")
                        .unwrap_or(&ident)
                        .to_string()
                        .into_boxed_str(),
                    ty: field.ty.clone(),
                }
            })
        })
        .collect()
}

fn extract_single_link_name(attrs: &[syn::Attribute]) -> Option<BoxStr> {
    let mut link_name_iter = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("link_name"));

    let link_name = link_name_iter.next()?;
    if let Some(attr) = link_name_iter.next() {
        panic!("multiple `#[link_name = ...]` attributes found: {attr:?}");
    }

    if let syn::Meta::NameValue(nv) = &link_name.meta
        && let syn::Expr::Lit(expr_lit) = &nv.value
        && let syn::Lit::Str(lit_str) = &expr_lit.lit
    {
        return Some(lit_str.value().into_boxed_str());
    }

    panic!("unrecognized `link_name` syntax: {link_name:?}");
}

fn visit_foreign_item_fn(table: &mut FfiItems, i: &syn::ForeignItemFn, abi: &Abi) {
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let path = append_path(table.current_module.clone(), i.sig.ident.clone());
    let cached_path = ast::path_to_string(path.clone());
    let cached_ident = i.sig.ident.to_string().into_boxed_str();
    let parameters = i
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(arg) => Parameter {
                ident: match arg.pat.deref() {
                    syn::Pat::Ident(i) => i.ident.to_string().into_boxed_str(),
                    _ => {
                        unimplemented!(
                            "Foreign functions are unlikely to have any other \
                             pattern."
                        )
                    }
                },
                ty: arg.ty.deref().clone(),
            },
            syn::FnArg::Receiver(_) => {
                unreachable!(
                    "Foreign functions can't have self/receiver \
                     parameters."
                )
            }
        })
        .collect::<Vec<_>>();
    let return_type = match &i.sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(ty.deref().clone()),
    };
    let link_name = extract_single_link_name(&i.attrs);

    table.foreign_functions.push(Fn {
        public,
        abi,
        cached_path,
        cached_ident,
        path: path.clone(),
        addressable_paths: vec![path.clone()],
        link_name,
        parameters,
        return_type,
    });
}

fn visit_foreign_item_static(table: &mut FfiItems, i: &syn::ForeignItemStatic, abi: &Abi) {
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let path = append_path(table.current_module.clone(), i.ident.clone());
    let cached_path = ast::path_to_string(path.clone());
    let cached_ident = i.ident.to_string().into_boxed_str();
    let ty = i.ty.deref().clone();
    let link_name = extract_single_link_name(&i.attrs);

    table.foreign_statics.push(Static {
        public,
        abi,
        cached_path,
        cached_ident,
        path: path.clone(),
        addressable_paths: vec![path.clone()],
        link_name,
        ty,
    });
}

fn visit_item_type(table: &mut FfiItems, i: &syn::ItemType) {
    let public = is_visible(&i.vis);
    let path = append_path(table.current_module.clone(), i.ident.clone());
    let cached_path = ast::path_to_string(path.clone());
    let cached_ident = i.ident.to_string().into_boxed_str();
    let ty = i.ty.deref().clone();

    table.aliases.push(Type {
        public,
        cached_path,
        cached_ident,
        path: path.clone(),
        addressable_paths: vec![path.clone()],
        ty,
    });
}

fn visit_item_struct(table: &mut FfiItems, i: &syn::ItemStruct) {
    let public = is_visible(&i.vis);
    let path = append_path(table.current_module.clone(), i.ident.clone());
    let cached_path = ast::path_to_string(path.clone());
    let cached_ident = i.ident.to_string().into_boxed_str();
    let fields = match &i.fields {
        syn::Fields::Named(fields) => collect_fields(&fields.named),
        syn::Fields::Unnamed(fields) => collect_fields(&fields.unnamed),
        syn::Fields::Unit => Vec::new(),
    };

    table.structs.push(Struct {
        public,
        cached_path,
        cached_ident,
        path: path.clone(),
        addressable_paths: vec![path.clone()],
        fields,
    });
}

fn visit_item_union(table: &mut FfiItems, i: &syn::ItemUnion) {
    let public = is_visible(&i.vis);
    let path = append_path(table.current_module.clone(), i.ident.clone());
    let cached_path = ast::path_to_string(path.clone());
    let cached_ident = i.ident.to_string().into_boxed_str();
    let fields = collect_fields(&i.fields.named);

    table.unions.push(Union {
        public,
        cached_path,
        cached_ident,
        path: path.clone(),
        addressable_paths: vec![path.clone()],
        fields,
    });
}

fn visit_item_const(table: &mut FfiItems, i: &syn::ItemConst) {
    let public = is_visible(&i.vis);
    let path = append_path(table.current_module.clone(), i.ident.clone());
    let cached_path = ast::path_to_string(path.clone());
    let cached_ident = i.ident.to_string().into_boxed_str();
    let ty = i.ty.deref().clone();

    table.constants.push(Const {
        public,
        cached_path,
        cached_ident,
        path: path.clone(),
        addressable_paths: vec![path.clone()],
        ty,
    });
}

fn visit_item_foreign_mod(table: &mut FfiItems, i: &syn::ItemForeignMod) {
    // Because we need to store the ABI we can't directly visit the foreign
    // functions/statics.

    // Since this is an extern block, assume extern "C" by default.
    let abi = i
        .abi
        .name
        .clone()
        .map_or(Abi::C, |s| Abi::from(s.value().as_str()));

    for item in &i.items {
        match item {
            syn::ForeignItem::Fn(function) => visit_foreign_item_fn(table, function, &abi),
            syn::ForeignItem::Static(static_variable) => {
                visit_foreign_item_static(table, static_variable, &abi)
            }
            _ => (),
        }
    }
}

impl<'ast> Visit<'ast> for FfiItems {
    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        let ItemMod {
            vis,
            ident,
            content: Some((_, mod_items)),
            ..
        } = i.clone()
        else {
            unreachable!(
                "This runs post cargo-expand, which inlines all \
                 modules."
            );
        };
        let is_crate_root = ident == "__ctest_root_mod";

        let mut running_items = if is_crate_root {
            self.clone()
        } else {
            FfiItems {
                current_module: append_path(self.current_module.clone(), ident.clone()),
                ..Default::default()
            }
        };

        let uses: Vec<_> = mod_items
            .iter()
            .cloned()
            .filter_map(|it| match it {
                Item::Type(t) => {
                    visit_item_type(&mut running_items, &t);
                    None
                }
                Item::Struct(s) => {
                    visit_item_struct(&mut running_items, &s);
                    None
                }
                Item::Union(u) => {
                    visit_item_union(&mut running_items, &u);
                    None
                }
                Item::Const(c) => {
                    visit_item_const(&mut running_items, &c);
                    None
                }
                Item::ForeignMod(m) => {
                    visit_item_foreign_mod(&mut running_items, &m);
                    None
                }
                Item::Use(ItemUse { vis, tree, .. }) => normalize_path(tree)
                    .into_iter()
                    .map(move |tree| RefinedUse {
                        is_public: matches!(vis, Visibility::Public(_)),
                        tree,
                    })
                    .into(),
                _ => None,
            })
            .flatten()
            .collect();

        visit::visit_item_mod(&mut running_items, i);
        running_items.uses = uses.clone();

        if is_crate_root {
            *self = running_items.clone();
        } else {
            let path = running_items.current_module.clone();
            self.modules.push(Module {
                public: matches!(vis, Visibility::Public(_)),
                cached_path: ast::path_to_string(running_items.current_module.clone()),
                cached_ident: ident.to_string().into_boxed_str(),
                path: path.clone(),
                addressable_paths: vec![path.clone()],
                items: running_items.clone(),
            });
        }
    }
}

#[test]
fn tmp() {
    let source = r#"
use test2::test3;

use test2::*;

use test1::{*, test3::*};

mod test1 {
    use test2::{*, test3::*};

    mod test2 {
        use test3::*;

        extern "C" {
            fn time();
        }

        mod test3 {
            pub struct Foo;

            fn foo() {
                extern "C" {
                    fn ctime();
                }
            }
        }
    }
}
    "#;
    let file = syn::parse_file(source).unwrap();
    let items = FfiItems::default();
    let mut items = items.clone();
    items.visit_file(&file);
    let items = items.clone();
    println!("{:#?}", items);
}

#[test]
#[ignore]
fn tmp1() {
    let source = r#"
pub struct Foo;

mod test1 { use super::*; }
    "#;
    let file = syn::parse_file(source).unwrap();
    let mut items = FfiItems::default();
    items.visit_file(&file);
    println!("{:#?}", items.clone());
}

#[test]
#[ignore]
fn tmp2() {
    let m1 = Module {
        cached_path: String::from("a").into_boxed_str(),
        items: FfiItems::default(),
        ..Default::default()
    };
    let m2 = Module {
        cached_path: String::from("a").into_boxed_str(),
        items: FfiItems {
            modules: vec![Module::default()],
            ..Default::default()
        },
        ..Default::default()
    };
    let mut ms = vec![m1, m2];
    ms.sort_unstable_by(Module::custom_cmp);
    println!("{:#?}", ms.clone());
}

#[test]
#[ignore]
fn tmp3() {
    let dst_child_path = Path {
        leading_colon: None,
        segments: {
            let mut out = Punctuated::new();
            out.push(PathSegment {
                ident: Ident::new("foo", Span::call_site()),
                arguments: PathArguments::None,
            });
            out.clone()
        },
    };
    let dst = Module {
        items: FfiItems {
            modules: Vec::from([Module {
                path: dst_child_path.clone(),
                cached_path: ast::path_to_string(dst_child_path.clone()),
                items: FfiItems {
                    structs: Vec::from([{
                        let path = Path {
                            leading_colon: None,
                            segments: Punctuated::new(),
                        };
                        Struct {
                            path: path.clone(),
                            cached_path: ast::path_to_string(path.clone()),
                            ..Default::default()
                        }
                    }]),
                    ..Default::default()
                },
                ..Default::default()
            }]),
            ..Default::default()
        },
        ..Default::default()
    };
    let src = Vec::from([Resolution::Resolved(FfiItems {
        modules: Vec::from([Module {
            path: dst_child_path.clone(),
            cached_path: ast::path_to_string(dst_child_path.clone()),
            items: FfiItems {
                modules: Vec::from([Module::default()]),
                uses: Vec::from([RefinedUse::default()]),
                ..Default::default()
            },
            ..Default::default()
        }]),
        ..Default::default()
    })]);
    let merged = merge_module(dst.clone(), src.clone());
    println!("{:#?}", merged.clone());
}
