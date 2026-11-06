//! Conversion of Rust code to a simplified abstract syntax tree.

use std::cmp::Ordering;
use std::fmt::{
    self,
    Debug,
    Formatter,
};
use std::ops::{
    self,
    ControlFlow,
    Deref,
    Not,
};
use std::{
    iter,
    mem,
    vec,
};

use proc_macro2::Span;
use syn::punctuated::Punctuated;
use syn::token::{
    Brace,
    Mod,
    PathSep,
};
use syn::visit::Visit;
use syn::{
    File,
    FnArg,
    ForeignItem,
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

use crate::ast::{
    PathAggregateStore,
    PathUtils,
};
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

// [NOTE]: for use only in printf-debugging. It doesn't automatically derive
// Debug to selectively print out the fields I'm interested in while debugging.
#[derive(Clone)]
struct DebugFfiItems {
    aliases: Vec<Type>,
    structs: Vec<Struct>,
    unions: Vec<Union>,
    constants: Vec<Const>,
    foreign_functions: Vec<Fn>,
    foreign_statics: Vec<Static>,
    modules: Vec<Module>,
}

impl Debug for DebugFfiItems {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut dbg_struct = f.debug_struct("DebugFfiItems");

        #[derive(Clone)]
        struct PrettyPath<'a>(&'a Path);

        impl Debug for PrettyPath<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.pad(&ast::path_to_string(self.0))
            }
        }

        macro_rules! pretty_print {
            ($($field:tt: $ty:tt),+ $(,)?) => {$(
                dbg_struct.field_with(stringify!($field), |f| {
                    f.pad("[ ")
                        .and_then(|_| {
                            self.$field
                                .iter()
                                .map(|it| {
                                    f.pad(&format!(
                                        concat!(
                                            stringify!($ty),
                                            " {{ path: {:?}, path_aggregate: {:?} }}",
                                        ),
                                        PrettyPath(&it.path),
                                        (&it.path_aggregate)
                                            .into_iter()
                                            .map(PrettyPath)
                                            .collect::<Vec<_>>(),
                                    ))
                                })
                                .collect::<fmt::Result>()
                        })
                        .and_then(|_| f.pad(" ]"))
                });
            )+};
        }

        pretty_print! {
            aliases: Type,
            structs: Struct,
            unions: Union,
            constants: Const,
            foreign_functions: Fn,
            foreign_statics: Static,
        }

        dbg_struct.field_with("modules", |f| {
            f.pad("[ ")
                .and_then(|_| {
                    self.modules
                        .iter()
                        .map(|m| {
                            f.pad("[ ")
                                .and_then(|_| {
                                    f.pad(&format!(
                                        "path: {:?}, path_aggregate: {:?}, items: {{ {:?} }}",
                                        PrettyPath(&m.path),
                                        (&m.path_aggregate)
                                            .into_iter()
                                            .map(PrettyPath)
                                            .collect::<Vec<_>>(),
                                        DebugFfiItems::from(m.items.clone()),
                                    ))
                                })
                                .and_then(|_| f.pad(" ]"))
                        })
                        .collect::<fmt::Result>()
                })
                .and_then(|_| f.pad(" ]"))
        });

        dbg_struct.finish()
    }
}

// [NOTE]: for debugging purposes only. Pretty prints a refined type for
// syn::UseTree.
#[derive(Clone)]
struct PrettyUseTree<'a>(&'a RefinedUseTree);

impl Debug for PrettyUseTree<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.pad(&refined_tree_to_string(&self.0))
    }
}

fn refined_tree_to_string(t: &RefinedUseTree) -> String {
    match t {
        RefinedUseTree::Path(p) => {
            format!("{}::{}", p.ident, refined_tree_to_string(&p.tree))
        }
        RefinedUseTree::Name(n) => format!("{}", n.ident),
        RefinedUseTree::Rename(r) => format!("{} as {}", r.ident, r.rename),
        RefinedUseTree::Glob => format!("*"),
    }
}

impl From<FfiItems> for DebugFfiItems {
    fn from(
        FfiItems {
            aliases,
            structs,
            unions,
            constants,
            foreign_functions,
            foreign_statics,
            modules,
            ..
        }: FfiItems,
    ) -> Self {
        Self {
            aliases,
            structs,
            unions,
            constants,
            foreign_functions,
            foreign_statics,
            modules,
        }
    }
}

/// Tuple-based state monad used for keeping track of mutable state while parsing items for
/// `FfiItems`.
///
/// The monad is currently used for tracking the current module of the item being parsed, as well as
/// the unique ID of all parsed items. The monad in those cases corresponds with the concrete type
/// `StateMonad<ParsingState, FfiItems>`. The implementation of `syn::visit::Visit` doesn't avoid
/// the impure mutable state of that interface but scopes it to polymorphic parameter `T` in a
/// separate copy of the state monad. After the recursive visitor traversal is done in the copy, the
/// original and mutated state monads are reconciled by binding the former to the latter.
// [FIXME]: it may be a better idea to change this into a cps state monad just
// to see whether they work out better with linear terms.
struct StateMonad<S, T> {
    seq: Box<dyn ops::FnOnce(S) -> (S, T)>,
}

impl<S, T> StateMonad<S, T> {
    fn pure(value: T) -> Self
    where
        for<'a> T: 'a,
    {
        Self {
            seq: Box::new(|s| (s, value)),
        }
    }

    fn bind<U, F>(self, f: F) -> StateMonad<S, U>
    where
        for<'a> S: 'a,
        for<'a> T: 'a,
        for<'a> F: ops::FnOnce(T) -> StateMonad<S, U> + 'a,
    {
        StateMonad {
            seq: Box::new(|s| {
                let (state, value) = (self.seq)(s);

                (f(value).seq)(state)
            }),
        }
    }

    fn set<F>(f: F) -> StateMonad<S, T>
    where
        for<'a> F: ops::FnOnce(S) -> (S, T) + 'a,
    {
        StateMonad { seq: Box::new(f) }
    }

    fn run(self, init: S) -> (S, T) {
        (self.seq)(init)
    }
}

type ParsingMonad = StateMonad<ParsingState, FfiItems>;

#[test]
fn tmpmonad() {
    use StateMonad;

    let (state, value) = StateMonad::pure(2)
        .bind(|x| StateMonad::set(move |s| (s * x, x)))
        .run(3);

    assert_eq!((state, value), (6, 2));
}

/* FIXME: maybe further separate the state that is "shared" among items (i.e.
 * the uid field at the time of writing) from the state that is particular to
 * the current module being parsed (i.e. fields current_module and
 * module_is_public at the time of writing.) */
#[derive(Debug, Clone)]
struct ParsingState {
    uid: usize,
    current_module: Path,
}

impl Default for ParsingState {
    fn default() -> Self {
        Self {
            uid: Default::default(),
            current_module: Path {
                leading_colon: None,
                segments: Default::default(),
            },
        }
    }
}

/// Represents a collected set of top-level Rust items relevant to FFI generation or analysis.
///
/// Includes foreign functions/statics, type aliases, structs, unions, and constants. Modules are
/// collected as recursive `FfiItems`, and currently used to narrow down tested items to those in a
/// specific module.
#[derive(Clone, Debug, Default)]
pub(crate) struct FfiItems {
    pub(crate) aliases: Vec<Type>,
    pub(crate) structs: Vec<Struct>,
    pub(crate) unions: Vec<Union>,
    pub(crate) constants: Vec<Const>,
    pub(crate) foreign_functions: Vec<Fn>,
    pub(crate) foreign_statics: Vec<Static>,
    pub(crate) uses: Vec<RefinedUse>,
    pub(crate) modules: Vec<Module>,
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
    /// This will also resolve `use` declarations, copying items over from one module to another.
    pub(crate) fn visit_file(self, file: &syn::File) -> FfiItems {
        let file_mod = ItemMod {
            attrs: file.attrs.clone(),
            vis: syn::parse_quote! { pub },
            unsafety: None,
            mod_token: Mod::default(),
            ident: Ident::new("__ctest_root_mod", Span::call_site()),
            content: (Brace::default(), file.items.clone()).into(),
            semi: None,
        };

        /* TODO: change relevant areas of module template to ensure they use
         * both the FfiItems from the module they're generating tests for as
         * well as the FfiItems corresponding to the entire crate. The latter
         * should be used for performing the whole-crate traversals in search
         * of items with the same unique IDs as those used in each of the items
         * we're testing the skips against. */
        let mut parsing_monad = ParsingMonad::pure(self);

        parsing_monad.visit_item_mod(&file_mod);

        let (_, items) = parsing_monad.run(ParsingState::default());
        let Module { items, .. } = resolve_all(Module {
            items,
            ..Default::default()
        });

        items
    }

    /// Yields an iterator over all items in the calling `FfiItems`.
    ///
    /// The items in the item container do not include `use` declarations.
    pub(crate) fn items(&self) -> ItemIterator {
        let base = self
            .aliases
            .clone()
            .into_iter()
            .map(GenericItem::Type)
            .chain(self.structs.clone().into_iter().map(GenericItem::Struct))
            .chain(self.unions.clone().into_iter().map(GenericItem::Union))
            .chain(self.constants.clone().into_iter().map(GenericItem::Const))
            .chain(
                self.foreign_functions
                    .clone()
                    .into_iter()
                    .map(GenericItem::Fn),
            )
            .chain(
                self.foreign_statics
                    .clone()
                    .into_iter()
                    .map(GenericItem::Static),
            )
            .chain(self.modules.clone().into_iter().map(GenericItem::Module));

        Box::new(base)
    }

    /// Performs a comparison for equality between `FfiItems`, without taking fields
    /// `current_module` nor `uses` into consideration.
    ///
    /// This is not an implementation of `PartialEq` for `FfiItems` because, at the time of writing,
    /// the logic for use declaration resolution is not yet finished.
    pub(super) fn custom_eq(&self, other: &Self) -> bool {
        self.aliases == other.aliases
            && self.structs == other.structs
            && self.unions == other.unions
            && self.constants == other.constants
            && self.foreign_functions == other.foreign_functions
            && self.foreign_statics == other.foreign_statics
            && self.modules == other.modules
    }

    /// Comparison function for use when sorting and deduplicating modules in [`FfiItems::dedup`].
    ///
    /// This is used in [`Module::custom_cmp`]. It trusts that the modules to compare have the same
    /// overall shape, and one will be a "natural" extension of the other.
    ///
    /// We consider "extension" here to mean that one module subtree is based on the other, and thus
    /// only contains both the items from the other and possibly some more items.
    pub(super) fn custom_cmp(&self, other: &Self) -> Ordering {
        let (ours, theirs) = (self.modules.len(), other.modules.len());

        // [NOTE]: the goal here is to have modules with "larger" subtrees
        // appear first in the item container, so declaring them to be "smaller"
        // is required for slice::sort_unstable_by to put them first in the
        // sorted list of modules in FfiItems::dedup. The same applies the other
        // way around with "smaller" subtrees that we would like to have appear
        // later in the sorted list of modules.
        match ours.cmp(&theirs) {
            Ordering::Equal => self.modules.iter().zip(&other.modules).fold(
                Ordering::Equal,
                |_, (our_m, their_m)| {
                    // [NOTE]: this is intended to not start anew a "regular"
                    // module comparison through Module::custom_cmp, but to
                    // rather have all subsequent comparisons be based off of
                    // the sizes of module subtrees rather than the
                    // lexicographical comparison of their paths.
                    our_m.items.custom_cmp(&their_m.items)
                },
            ),
            other => other.reverse(),
        }
    }

    /// Searches for an item in the container, and returns an owned instance of that item.
    ///
    /// The generic item may be pattern-matched to find out specifically which item was found.
    fn search(&self, ident: &Ident) -> Option<GenericItem> {
        self.items().find(|it| {
            macro_rules! cmp {
                ($($var:tt),+ $(,)?) => {
                    match it {$(
                        GenericItem::$var(it) => ident == it.ident(),
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
    /// Items of differing kind, like functions and constants, that may happen to have the same path
    /// will not get deduplicated. A consequence of calling this method is that the items are also
    /// sorted by their paths.
    fn dedup(self) -> FfiItems {
        macro_rules! dedup {
            ($($it:tt),+ $(,)?) => {{
                macro_rules! key_fn {
                    () => {
                        |it| it.cached_path.clone()
                    };
                }

                let items = self.clone();

                $(
                    let mut $it = self.$it;

                    $it.sort_unstable_by_key(key_fn!());
                    $it.dedup_by_key(key_fn!());

                    let items = FfiItems {
                        $it: $it,
                        ..items
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

        let mut new_modules = self.modules;
        new_modules.sort_unstable_by(Module::custom_cmp);
        new_modules.dedup_by(|Module { path: lhs, .. }, Module { path: rhs, .. }| *lhs == *rhs);

        FfiItems {
            modules: new_modules,
            ..new_items
        }
    }
}

impl FromIterator<GenericItem> for FfiItems {
    fn from_iter<I: IntoIterator<Item = GenericItem>>(iter: I) -> FfiItems {
        let mut partitioned_items: Vec<_> = iter.into_iter().collect();

        println!(
            "[FfiItems::from_iter] partitioned_items: {:?}",
            partitioned_items
                .iter()
                .map(|it| {
                    macro_rules! exe {
                    ($($ty:tt),+ $(,)?) => {
                        match it {$(
                            GenericItem::$ty(it) => it.cached_path.clone(),
                        )+}
                    };
                }

                    exe! {
                        Type,
                        Struct,
                        Union,
                        Const,
                        Fn,
                        Static,
                        Module,
                    }
                })
                .collect::<Vec<_>>()
        );

        // [NOTE]: this is expected to sort all items by their kind, such that
        // we end up with the following ordering:
        //
        // 1. Type aliases.
        // 2. Stucts.
        // 3. Unions.
        // 4. Constants.
        // 5. Foreign functions.
        // 6. Foreign statics.
        // 7. Modules.
        //
        // This is then used to partition over the resulting collection and get
        // "buckets" of GenericItems that can be mapped to a concrete type
        // through partial mapping.
        partitioned_items.sort_by(|it1, it2| match (it1, it2) {
            (GenericItem::Type(_), GenericItem::Type(_))
            | (GenericItem::Struct(_), GenericItem::Struct(_))
            | (GenericItem::Union(_), GenericItem::Union(_))
            | (GenericItem::Const(_), GenericItem::Const(_))
            | (GenericItem::Fn(_), GenericItem::Fn(_))
            | (GenericItem::Static(_), GenericItem::Static(_))
            | (GenericItem::Module(_), GenericItem::Module(_)) => Ordering::Equal,
            (GenericItem::Type(_), GenericItem::Struct(_))
            | (GenericItem::Type(_), GenericItem::Union(_))
            | (GenericItem::Type(_), GenericItem::Const(_))
            | (GenericItem::Type(_), GenericItem::Fn(_))
            | (GenericItem::Type(_), GenericItem::Static(_))
            | (GenericItem::Type(_), GenericItem::Module(_))
            | (GenericItem::Struct(_), GenericItem::Union(_))
            | (GenericItem::Struct(_), GenericItem::Const(_))
            | (GenericItem::Struct(_), GenericItem::Fn(_))
            | (GenericItem::Struct(_), GenericItem::Static(_))
            | (GenericItem::Struct(_), GenericItem::Module(_))
            | (GenericItem::Union(_), GenericItem::Const(_))
            | (GenericItem::Union(_), GenericItem::Fn(_))
            | (GenericItem::Union(_), GenericItem::Static(_))
            | (GenericItem::Union(_), GenericItem::Module(_))
            | (GenericItem::Const(_), GenericItem::Fn(_))
            | (GenericItem::Const(_), GenericItem::Static(_))
            | (GenericItem::Const(_), GenericItem::Module(_))
            | (GenericItem::Fn(_), GenericItem::Static(_))
            | (GenericItem::Fn(_), GenericItem::Module(_))
            | (GenericItem::Static(_), GenericItem::Module(_)) => Ordering::Less,
            (GenericItem::Struct(_), GenericItem::Type(_))
            | (GenericItem::Union(_), GenericItem::Type(_))
            | (GenericItem::Union(_), GenericItem::Struct(_))
            | (GenericItem::Const(_), GenericItem::Type(_))
            | (GenericItem::Const(_), GenericItem::Struct(_))
            | (GenericItem::Const(_), GenericItem::Union(_))
            | (GenericItem::Fn(_), GenericItem::Type(_))
            | (GenericItem::Fn(_), GenericItem::Struct(_))
            | (GenericItem::Fn(_), GenericItem::Union(_))
            | (GenericItem::Fn(_), GenericItem::Const(_))
            | (GenericItem::Static(_), GenericItem::Type(_))
            | (GenericItem::Static(_), GenericItem::Struct(_))
            | (GenericItem::Static(_), GenericItem::Union(_))
            | (GenericItem::Static(_), GenericItem::Const(_))
            | (GenericItem::Static(_), GenericItem::Fn(_))
            | (GenericItem::Module(_), GenericItem::Type(_))
            | (GenericItem::Module(_), GenericItem::Struct(_))
            | (GenericItem::Module(_), GenericItem::Union(_))
            | (GenericItem::Module(_), GenericItem::Const(_))
            | (GenericItem::Module(_), GenericItem::Fn(_))
            | (GenericItem::Module(_), GenericItem::Static(_)) => Ordering::Greater,
        });

        macro_rules! get_partitions {
            ($($field:ident: $ty:tt),+ $(,)?) => {$(
                macro_rules! search_idx {
                    ($start:expr, $pred:expr) => {{
                        partitioned_items
                            .iter()
                            .enumerate()
                            .skip($start)
                            .skip_while(|(_, it)| $pred(it))
                            .next()
                            .map(|(idx, _)| idx)
                            .unwrap_or(partitioned_items.len())
                    }};
                }

                let start =
                    search_idx!(0, |it: &&GenericItem| matches!(it, GenericItem::$ty(_)).not());
                let end = search_idx!(start, |it: &&GenericItem| matches!(it, GenericItem::$ty(_)));

                let $field = if start >= end {
                    Vec::new()
                } else {
                    let partition = partitioned_items
                        .get(start..end)
                        .map(|it| {
                            it.into_iter()
                                .map(|it| match it {
                                    GenericItem::$ty(it) => it.clone(),
                                    _ => unreachable!(concat!(
                                        "Items have been sorted by their kind \
                                         before. This is expected to \
                                         correspond with the bucket ",
                                        stringify!($field),
                                        ".",
                                    )),
                                })
                                .collect::<Vec<_>>()
                        })
                        .expect(
                            "The slice must be valid because the range is \
                             logical, and the components of the range are \
                             indices into the collection.",
                        );

                    println!(
                        concat!(
                            "[FfiItems::from_iter] ",
                            stringify!($field),
                            "_partition: {:?}"
                        ),
                        partition
                            .iter()
                            .map(|it| &it.cached_path)
                            .collect::<Vec<_>>(),
                    );

                    partition
                };
            )+};
        }

        get_partitions! {
            aliases: Type,
            structs: Struct,
            unions: Union,
            constants: Const,
            foreign_functions: Fn,
            foreign_statics: Static,
            modules: Module,
        }

        FfiItems {
            aliases,
            structs,
            unions,
            constants,
            foreign_functions,
            foreign_statics,
            modules,
            ..Default::default()
        }
    }
}

/// Sum over any item held within an `FfiItems` term.
#[derive(Debug, Clone)]
pub(crate) enum GenericItem {
    Type(Type),
    Struct(Struct),
    Union(Union),
    Const(Const),
    Fn(Fn),
    Static(Static),
    Module(Module),
}

/// An iterator over all items but `use` declarations in an `FfiItems` term.
type ItemIterator = Box<dyn Iterator<Item = GenericItem>>;

/// Refined type for [`syn::UsePath`] that does not consider group paths.
///
/// This is a lot like [`RefinedUse`] in that it only exists as replacement for [`syn::UsePath`] but
/// uses [`RefinedUseTree`] instead of [`syn::UseTree`]. See [`RefinedUseTree`] for details.
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
/// This is the byproduct of normalizing a `use` declaration's path through [`normalize_path`]. That
/// function ensures some such path containing a group as its tail end is converted into a set of
/// paths without groups.
///
/// This enumeration is thus equivalent to `syn::UseTree` but does not consider the case of
/// [`syn::UseTree::Group`].
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
/// This exists only for the purpose of refining [`syn::UseTree`], which is used within
/// [`syn::ItemUse`]. See type [`RefinedUseTree`] for the actual refinement and details on the
/// purpose of this type.
#[derive(Clone, Debug, PartialEq, Default)]
pub(crate) struct RefinedUse {
    public: bool,
    tree: RefinedUseTree,
}

/// Gets rid of group reexports in a `use` declaration by flattenning them into a set of individual
/// reexports.
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
/// Each `use` declaration requires access to either one of (1) the module where it was sourced
/// from, or (2) the crate root's subtree, or (3) any one ascendant or descendant module of the
/// module where it was sourced from.
///
/// Points (2) and (3) require having access to the entire crate's module tree. Some ancestor item
/// is looked up by the parent path to the item being searched with respect to. Point (1) is
/// satisfied by the `subtree` field.
#[derive(Debug, Clone)]
struct PassPair {
    subtree: Module,
    whole_crate: Module,
}

/// This is the resolution driver. It is the entrypoint for whole crate resolution.
///
/// It performs passes on a given module. It diffs the results of a whole-crate pass with the
/// pre-pass state. Equality in that comparison determines termination.
fn resolve_all(root: Module) -> Module {
    let (ControlFlow::Break(m) | ControlFlow::Continue(m)) =
        iter::repeat(()).try_fold(root, |root, _| {
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
                true => ControlFlow::Break(new_root),
                false => ControlFlow::Continue(new_root),
            }
        });
    m
}

/// Represents a resolved or unresolved `use` declaration.
///
/// The state of resolution may change across resolution passes. It is used in [`merge_module`] to
/// merge whichever `use` declarations resolved to a set of items back into the module they were
/// sourced from.
#[derive(Debug, Clone)]
enum Resolution {
    Resolved(FfiItems),
    Unresolved,
}

/// Performs one pass through the provided pair of a subtree and the whole-crate subtree.
///
/// The input pair's fields correspond with the subtree in which to perform the resolution pass on,
/// and the tree at the crate root. The latter is required to resolve absolute imports that refer to
/// some item in an ancestor module that is more than one level up the tree than the subtree being
/// resolved.
///
/// The returned pair contains updated snapshots of the input pair. The subtree may have had new
/// items added to it. The whole-crate module tree is guaranteed to have integrated these changes.
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
                subtree: m,
                whole_crate: whole_crate,
            });

            let mut new_children = children;
            new_children.push(pair.subtree.clone());

            (pair, new_children)
        },
    );

    let root_with_resolved_children = Module {
        items: FfiItems {
            modules: resolved_children,
            ..src.subtree.items
        },
        ..src.subtree
    };

    let resolved_uses = resolve_one(PassPair {
        subtree: root_with_resolved_children.clone(),
        whole_crate: whole_crate_with_resolved_children.clone(),
    });

    let PassPair {
        subtree: new_root_with_duplicates,
        whole_crate: new_whole_crate_with_resolved_children,
    } = merge_module(
        PassPair {
            subtree: root_with_resolved_children,
            whole_crate: whole_crate_with_resolved_children,
        },
        resolved_uses,
    );

    let new_root = Module {
        items: new_root_with_duplicates.items.dedup(),
        ..new_root_with_duplicates
    };
    let new_whole_crate = update_tree(
        &new_root.path,
        |_| GenericItem::Module(new_root.clone()),
        new_whole_crate_with_resolved_children,
    );

    println!(
        "[resolve_pass] finished module pass on module {}",
        if &*new_root.cached_path == "" {
            "root".into()
        } else {
            &*new_root.cached_path
        },
    );

    PassPair {
        subtree: new_root,
        whole_crate: new_whole_crate,
    }
}

/// Attempts to resolve the `use` declarations in a given module.
///
/// The input pair consists of the module from which to source the `use` declarations, and of the
/// whole-crate module tree. The latter is used to resolve certain types of paths in `use`
/// declarations.
///
/// See [`resolve_pass`] for details.
fn resolve_one(state: PassPair) -> Vec<Resolution> {
    println!(
        "[resolve_one] found {} use declarations in module {}: {:?}",
        state.subtree.items.uses.len(),
        if &*state.subtree.cached_path == "" {
            "root"
        } else {
            &*state.subtree.cached_path
        },
        state
            .subtree
            .items
            .uses
            .iter()
            .map(|RefinedUse { tree, .. }| tree)
            .map(PrettyUseTree)
            .collect::<Vec<_>>(),
    );

    state
        .subtree
        .items
        .uses
        .clone()
        .into_iter()
        .map(|u| resolve_use(u, state.clone()))
        .collect()
}

/// Attempts to resolve a single `use` declaration.
///
/// This can go one of two ways.
///
/// 1. Either the `use` declaration matches a specific module in the stateful pair, or
/// 2. the `use` declaration is assumed to be unresolved in this resolution pass.
///
/// The stateful pair initially consists of the module in which the `use` declaration appears at. If
/// the `use` declaration refers to an item by a path with more than one segment, this function
/// recursively munches through it.
///
/// These recursive calls will only be made if the extracted path segment matches against some item
/// in the stateful subtree. Currently, the only types of items that can match on a path that still
/// has another unmunched segment are modules.
///
/// Upon finding the module, the new stateful subtree of the pair becomes this module. This is then
/// used in the recursive call with the path tail.
///
/// The other element of the pair is the whole-crate module tree. That is used to resolve ancestor
/// paths (when `super` is found) and absolute paths (when `crate` is found.)
fn resolve_use(r#use: RefinedUse, state: PassPair) -> Resolution {
    macro_rules! log {
        ($msg:literal $(, $complement:expr)* $(,)?) => {
            println!(
                "[resolve_use {:?}] {}",
                PrettyUseTree(&(r#use.tree)),
                format_args!($msg $(, $complement)*),
            );
        };
    }

    macro_rules! single_item {
        ($field:ident: $it:expr) => {
            return Resolution::Resolved(FfiItems {
                $field: vec![$it],
                ..Default::default()
            })
        };
    }

    match r#use.tree {
        RefinedUseTree::Name(UseName { ref ident }) => {
            log!(
                "entered name resolution case with {} in module {}",
                ident,
                state.subtree.cached_path,
            );

            // [NOTE]: self may not appear as the name basecase of a well-formed
            // path. And yet it is still considered here. That is because of
            // path normalization. self may appear as part of a tail group in a
            // use declaration's path. Path normalization flattens groups into
            // individual use statements. Path normalization is done through
            // normalize_path above. That happens at parse-time. That is why
            // this function must consider the case of self as a name basecase.
            // [NOTE]: crate does not appear as the tail end of a path in a use
            // declaration (that is not a rename) under no circumstances, so
            // that case is not considered here.
            if is_self(&ident) {
                single_item!(modules: state.subtree);
            } else if is_super(&ident) {
                let parent_path = get_parent_path(&state.subtree);
                let Some(parent) = get_module(&parent_path, &state.whole_crate) else {
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

                single_item!(modules: parent.clone());
            }

            match state.subtree.items.search(&ident) {
                Some(GenericItem::Type(t)) => single_item!(aliases: t),
                Some(GenericItem::Struct(s)) => single_item!(structs: s),
                Some(GenericItem::Union(u)) => single_item!(unions: u),
                Some(GenericItem::Const(c)) => single_item!(constants: c),
                Some(GenericItem::Fn(f)) => single_item!(foreign_functions: f),
                Some(GenericItem::Static(s)) => single_item!(foreign_statics: s),
                Some(GenericItem::Module(m)) => single_item!(modules: m),
                None => Resolution::Unresolved,
            }
        }
        RefinedUseTree::Glob => {
            log!(
                "entered glob resolution case in module {}",
                if &*state.subtree.cached_path == "" {
                    "root"
                } else {
                    &*state.subtree.cached_path
                },
            );

            Resolution::Resolved(state.subtree.items)
        }
        RefinedUseTree::Rename(UseRename {
            ref ident,
            ref rename,
            ..
        }) => {
            log!(
                "entered rename resolution case with {:?} in module {}",
                UseRename {
                    ident: ident.clone(),
                    as_token: syn::parse_quote! { as },
                    rename: rename.clone(),
                },
                state.subtree.cached_path,
            );

            macro_rules! new_path {
                ($it:expr) => {{
                    let new_ident = $it.path.segments.iter().rev().take(1).map(|_| PathSegment {
                        ident: rename.clone(),
                        arguments: PathArguments::None,
                    });
                    let new_segments = $it
                        .path
                        .segments
                        .clone()
                        .into_iter()
                        .chain(new_ident)
                        .collect();

                    Path {
                        segments: new_segments,
                        ..$it.path
                    }
                }};
            }

            // [NOTE]: it may seem odd that borrowck doesn't mention the fact
            // the linear resource in the item at $it hasn't already been used
            // by the time the linear PathUtils::rename trait method is used.
            // Notice how new_path! does not make use of any linear resources in
            // $it; The remaining fields filled in by $it in the syn::Path it
            // yields have an unrestricted multiplicity (i.e. they're Copy.)
            macro_rules! rename_single_item {
                ($field:ident , $it:expr , $ty:tt) => {{
                    let new_path = new_path!($it);

                    single_item!($field: $it.rename(new_path));
                }};
            }

            if is_self(&ident) {
                rename_single_item!(modules, state.subtree, Module);
            } else if is_super(&ident) {
                let parent_path = get_parent_path(&state.subtree);
                let Some(parent) = get_module(&parent_path, &state.whole_crate).cloned() else {
                    unreachable!(
                        "See the note on super in the name basecase. The same \
                         applies here."
                    );
                };

                rename_single_item!(modules, parent, Module);
            } else if is_crate(&ident) {
                rename_single_item!(modules, state.whole_crate, Module);
            }

            match state.subtree.items.search(&ident) {
                Some(GenericItem::Type(t)) => rename_single_item!(aliases, t, Type),
                Some(GenericItem::Struct(s)) => rename_single_item!(structs, s, Struct),
                Some(GenericItem::Union(u)) => rename_single_item!(unions, u, Union),
                Some(GenericItem::Const(c)) => rename_single_item!(constants, c, Const),
                Some(GenericItem::Fn(f)) => rename_single_item!(foreign_functions, f, Fn),
                Some(GenericItem::Static(s)) => rename_single_item!(foreign_statics, s, Static),
                Some(GenericItem::Module(m)) => rename_single_item!(modules, m, Module),
                None => Resolution::Unresolved,
            }
        }

        RefinedUseTree::Path(RefinedUsePath {
            ref ident,
            ref tree,
        }) => {
            log!(
                "entered path resolution case with {} in module {}",
                ident,
                state.subtree.cached_path,
            );

            let new_state = if is_self(&ident) {
                state
            } else if is_super(&ident) {
                let parent_path = get_parent_path(&state.subtree);
                let Some(parent) = get_module(&parent_path, &state.whole_crate).cloned() else {
                    unreachable!(
                        "See the note on super in the name basecase. The same \
                         applies here."
                    );
                };

                PassPair {
                    subtree: parent,
                    whole_crate: state.whole_crate,
                }
            } else if is_crate(&ident) {
                PassPair {
                    subtree: state.whole_crate.clone(),
                    whole_crate: state.whole_crate,
                }
            } else if let Some(GenericItem::Module(m)) = state.subtree.items.search(&ident) {
                PassPair {
                    subtree: m,
                    whole_crate: state.whole_crate,
                }
            } else {
                return Resolution::Unresolved;
            };

            // [FIXME]: remove the below clone once debugging is done and the
            // logging macro invocation above is removed.
            resolve_use(
                RefinedUse {
                    tree: *tree.clone(),
                    ..r#use
                },
                new_state,
            )
        }
    }
}

/// Attempts to find a given module, provided its path (`target`,) within a module subtree (`root`.)
fn get_module<'m>(target: &Path, root: &'m Module) -> Option<&'m Module> {
    if root.path == *target {
        return root.into();
    }

    root.items
        .modules
        .iter()
        .find_map(|m| get_module(target, m))
}

/// Fetches the parent path to a given module.
///
/// If the module is a top-level item (i.e. it appears at the crate root,) the returned path will be
/// empty. This follows from the parsing strategy for paths in `FfiItems`. The absolute paths are
/// all `crate`-_unprepended_.
fn get_parent_path(root: &Module) -> Path {
    // [NOTE]: this performs a saturating_sub because there is one and only one
    // item in the crate's module tree that has an empty path; The crate root.
    // That is only wrapped in a Module of its own only after parsing. This
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
    let segments_before_ident = root.path.segments.len().saturating_sub(1);

    Path {
        leading_colon: None,
        segments: root
            .path
            .segments
            .iter()
            .take(segments_before_ident)
            .cloned()
            .collect(),
    }
}

/// Checks if the given identifier was parsed as the Rust keyword `self`.
fn is_self(ident: &Ident) -> bool {
    ident.to_string() == "self"
}

/// Checks if the given identifier was parsed as the Rust keyword `crate`.
fn is_crate(ident: &Ident) -> bool {
    ident.to_string() == "crate"
}

/// Checks if the given identifier was parsed as the Rust keyword `super`.
fn is_super(ident: &Ident) -> bool {
    ident.to_string() == "super"
}

/// Attempts to find some item in module tree `root` with path `target`, then runs the passed
/// closure on the item, and replaces it with the result.
///
/// # Panics
///
/// The function will panic if the specific injection of the returned `GenericItem` does not match
/// that of the input `GenericItem` to closure `action`.
fn update_tree(
    target: &Path,
    action: impl ops::Fn(GenericItem) -> GenericItem + Clone,
    root: Module,
) -> Module {
    macro_rules! ignored {
        () => {
            unimplemented!(
                "The action is not meant to return an item of a differing kind \
                 than what was provided."
            );
        };
    }

    if *target == root.path {
        let GenericItem::Module(new_root) = action(GenericItem::Module(root)) else {
            ignored!();
        };

        return new_root;
    }

    macro_rules! exe {
        ($($field:tt: $ty:tt),+ $(,)?) => {$(
            let $field: Vec<_> = root
                .items
                .$field
                .into_iter()
                .map(|it| {
                    if it.path == *target {
                        let GenericItem::$ty(it) = action(GenericItem::$ty(it)) else {
                            ignored!();
                        };

                        it
                    } else {
                        it
                    }
                })
                .collect();
        )+};
    }

    exe! {
        aliases: Type,
        structs: Struct,
        unions: Union,
        constants: Const,
        foreign_functions: Fn,
        foreign_statics: Static,
        modules: Module,
    }

    let modules = modules
        .into_iter()
        .map(|m| update_tree(target, action.clone(), m))
        .collect();

    Module {
        items: FfiItems {
            aliases,
            structs,
            unions,
            constants,
            foreign_functions,
            foreign_statics,
            modules,
            ..root.items
        },
        ..root
    }
}

/// Integrates the resolution items of `src` into `dst`.
///
/// Each resolution item is either one of an `Unresolved` or a `Resolved` variant. The former are
/// essentially ignored. The latter pack an `FfiItems` that is integrated into `dst`'s own items.
/// That is the merging part.
///
/// Note that, during merging, the items to be merged may end up duplicating existing items on
/// `dst`. It is call-site's responsibility to deduplicate those through [`FfiItems::dedup`].
fn merge_module(state: PassPair, src: Vec<Resolution>) -> PassPair {
    println!(
        "[merge_module] preprocessed_items: {:#?}",
        src.iter()
            .map(|r| {
                if let Resolution::Resolved(it) = r {
                    format!("resolved: {:?}", DebugFfiItems::from(it.clone()))
                } else {
                    format!("unresolved")
                }
            })
            .collect::<Vec<_>>(),
    );

    // [NOTE]: we collect all items here because we want the actions in the
    // mapping to take place in whole before we start folding over the items to
    // merge.
    let processed_items: Vec<_> = src
        .into_iter()
        .filter_map(|r| {
            if let Resolution::Resolved(items) = r {
                Some(items)
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
                    ($($it:tt),+ $(,)?) => { seek_parent!(@@ $($it)+) };

                    (@@ $head:tt $($tail:tt)*) => {
                        seek_parent!(@ $head).or_else(|| seek_parent!(@@ $($tail)*))
                    };
                    (@@) => { None };

                    (@ $field:tt) => {{
                        $field.first().map(|it| {
                            it.path
                                .segments
                                .iter()
                                .take(it.path.segments.len() - 1)
                                .cloned()
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
                // declaration consists of a glob import of all items in a
                // module with no items. We can be sure that all items about to
                // be merged share the same parent because use declarations are
                // cleansed of groups at parse-time.
                let Some(source_module_path) = seek_parent!(
                    aliases,
                    structs,
                    unions,
                    constants,
                    foreign_functions,
                    foreign_statics,
                    modules,
                ) else {
                    return FfiItems::default();
                };

                // [NOTE]: the dummy module is only in charge of wrapping all
                // items we want to rename with a path that makes up some
                // initial portion of the path segments of all descendant items'
                // path. This is so that the below call to function
                // RenameUtils::rename can use it as reference for the subtree
                // items' path.
                let dummy_module = Module {
                    path: Path {
                        leading_colon: None,
                        segments: source_module_path,
                    },
                    items: FfiItems {
                        aliases,
                        structs,
                        unions,
                        constants,
                        foreign_functions,
                        foreign_statics,
                        modules,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let Module {
                    items: new_items, ..
                } = dummy_module.rename(state.subtree.path.clone());

                fn remove_uses(m: Module) -> Module {
                    Module {
                        items: FfiItems {
                            uses: vec![],
                            modules: m.items.modules.into_iter().map(remove_uses).collect(),
                            ..m.items
                        },
                        ..m
                    }
                }

                // [NOTE]: this removes all use declarations in all descendant
                // modules of the subtrees to merge because otherwise merged
                // modules can increase the number of use declarations in the
                // crate's module tree, which is one cause of non-termination.
                let new_modules_without_uses =
                    new_items.modules.into_iter().map(remove_uses).collect();

                FfiItems {
                    modules: new_modules_without_uses,
                    ..new_items
                }
            },
        )
        .collect();

    println!(
        "[merge_module] processed_items: {:#?}",
        processed_items
            .iter()
            .cloned()
            .map(|it| format!("{:?}", DebugFfiItems::from(it)))
            .collect::<Vec<_>>(),
    );

    processed_items.into_iter().fold(
        state,
        |PassPair {
             subtree,
             whole_crate,
         },
         it| {
            // [TODO]: this is still wrong but it also seems to be headed in the
            // right direction.
            // [FIXME]: get rid of the below clone once debugging is done.
            let new_whole_crate = sync_driver(&it, whole_crate.clone());
            let Some(new_subtree) = get_module(&subtree.path, &new_whole_crate).cloned() else {
                unreachable!(
                    "The subtree is bound to be found in the whole-crate \
                     module tree because it was sourced from it, and syncing \
                     the path aggregates of items should not suddently remove \
                     modules."
                );
            };

            assert_ne!(
                whole_crate, new_whole_crate,
                "old whole-crate module tree matches new whole-crate module \
                 tree even though there were items to merge",
            );

            println!(
                "[merge_module] new_whole_crate: {:?}",
                DebugFfiItems::from(new_whole_crate.items.clone())
            );

            PassPair {
                subtree: Module {
                    items: new_subtree.items.items().chain(it.items()).collect(),
                    ..new_subtree
                },
                whole_crate: new_whole_crate,
            }
        },
    )
}

/// Provided an item container with the most up-to-date occurrences of some item(s) (`items`,) this
/// function will propagate the state of the path aggregate of each of those items throughout the
/// entirety of the subtree rooted at the `tree` module tree.
fn sync_driver(items: &FfiItems, tree: Module) -> Module {
    items.items().fold(tree, |m, it| {
        macro_rules! exe {
            ($($ty:tt),+ $(,)?) => {
                match it {$(
                    GenericItem::$ty(it) => sync_aggregates(&it.path_aggregate, m),
                )+}
            };
        }

        exe! {
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

/// Provided a reference path aggregate corresponding with that of an item, this function traverses
/// module `m` in search of all occurrences of such an item and "updates" such ocurrences to
/// integrate the paths from `reference`'s path aggregate store that are not part of any one of
/// those item occurrences.
///
/// # Implementation notes
///
/// The strategy follows that for any given item `a`, its path aggregate is bound to be unique with
/// respect to that of another item `b` so long as `a` is not an occurrence of `b` in another module
/// of the crate's module tree. We thus define the relation `a ~ b` as one where `b` contains some
/// path in its path aggregate that is also part of `a`'s path aggregate.
///
/// If we fix `a` and it holds that `a ~ b`, then it follows that there exists some non-empty set
/// `B` of all occurrences `b` of `a` in the crate's module tree. Under the assumption that some two
/// items `a` and `b` may only share path aggregates it they are of the same kind, the below
/// strategy attempts to find akin path aggregates where `a ~ b` holds. This requires not for a
/// single kind of item to be checked against the fixed point (here `reference`) because of the
/// above assumption; Akin path aggregates will only be found for occurrences of the same item, and
/// thus the same kind of item, in the crate's module tree.
fn sync_aggregates(reference: &PathAggregateStore, m: Module) -> Module {
    let akin = |agg: &PathAggregateStore| agg.akin(reference);

    println!(
        "[sync_aggregates] preprocessed_module {}: {:?}",
        if m.cached_path.is_empty() {
            "root"
        } else {
            &m.cached_path
        },
        DebugFfiItems::from(m.items.clone()),
    );

    let new_m = if akin(&m.path_aggregate) {
        // [NOTE]: this we diff instead of merging because merging will get us a
        // set of path aggregates that contain both the ones already in m, as
        // well as the ones that were part of reference but not part of m.
        // Diffing gets us only the paths in reference that are not part of m's
        // path aggregate. We don't just merge the path aggregates and create
        // a new m with the result because adding a path aggregate to a module
        // has an effect on the subtree of items rooted at the module in
        // question (see the Module::add_path trait method override.)
        let diff = m.path_aggregate.diff(reference);

        diff.into_iter().fold(m, Module::add_path)
    } else {
        m
    };

    let new_items: FfiItems = new_m
        .items
        .items()
        .map(|it| {
            macro_rules! exe {
                ($($ty:tt),+ $(,)?) => {
                    match it {
                        $(
                        GenericItem::$ty(it) if akin(&it.path_aggregate) => {
                            GenericItem::$ty($ty {
                                path_aggregate: reference.clone(),
                                ..it
                            })
                        }
                        )+
                        GenericItem::Module(it) => {
                            GenericItem::Module(sync_aggregates(reference, it))
                        }
                        non_matching => non_matching,
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

    println!(
        "[sync_aggregates] processed_module {}: {:?}",
        if new_m.cached_path.is_empty() {
            "root"
        } else {
            &new_m.cached_path
        },
        DebugFfiItems::from(new_items.clone()),
    );

    Module {
        items: new_items,
        ..new_m
    }
}

/// Appends a new module-local item to an absolute path that does _not_ start with `crate`.
///
/// This is used whenever we need to create paths for either items or modules.
fn append_path(base: Path, new: &Ident) -> Path {
    if base.segments.is_empty() {
        syn::parse_quote! { #new }
    } else {
        syn::parse_quote! { #base::#new }
    }
}

/// Determine whether an item is visible to other crates.
///
/// This function assumes that if the visibility is restricted then it is not meant to be accessed.
fn is_visible(vis: &Visibility) -> bool {
    match vis {
        Visibility::Public(_) => true,
        Visibility::Inherited | Visibility::Restricted(_) => false,
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

/* FIXME: maybe we can automate all of the below parsing machinery into a macro.
 * The logic that produces the new state monad is especially verbose to have it
 * appearing multiple times. */

fn visit_foreign_item_fn(monad: ParsingMonad, i: &syn::ForeignItemFn, abi: &Abi) -> ParsingMonad {
    let link_name = extract_single_link_name(&i.attrs);
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let ident = i.sig.ident.clone();

    let parameters = i
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(arg) => Parameter {
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
            FnArg::Receiver(_) => {
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

    monad.bind(move |items| {
        StateMonad::set(
            move |ParsingState {
                      uid,
                      current_module,
                      module_is_public,
                  }| {
                let path = append_path(current_module.clone(), &ident);
                let new_uid = uid.strict_add(1);
                let new_items = FfiItems {
                    foreign_functions: items
                        .foreign_functions
                        .into_iter()
                        .chain([Fn {
                            uid: new_uid,
                            public,
                            abi,
                            cached_path: ast::path_to_string(&path),
                            cached_ident: ident.to_string().into_boxed_str(),
                            path: path,
                            link_name,
                            parameters,
                            return_type,
                        }])
                        .collect(),
                    ..items
                };
                let new_state = ParsingState {
                    uid: new_uid,
                    current_module,
                    module_is_public,
                };

                (new_state, new_items)
            },
        )
    })
}

fn visit_foreign_item_static(
    monad: ParsingMonad,
    i: &syn::ForeignItemStatic,
    abi: &Abi,
) -> ParsingMonad {
    let link_name = extract_single_link_name(&i.attrs);
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let ident = i.ident.clone();
    let ty = i.ty.deref().clone();

    monad.bind(move |items| {
        StateMonad::set(
            move |ParsingState {
                      uid,
                      current_module,
                      module_is_public,
                  }| {
                let path = append_path(current_module.clone(), &ident);
                let new_uid = uid.strict_add(1);
                let new_items = FfiItems {
                    foreign_statics: items
                        .foreign_statics
                        .into_iter()
                        .chain([Static {
                            uid: new_uid,
                            public,
                            abi,
                            cached_path: ast::path_to_string(&path),
                            cached_ident: ident.to_string().into_boxed_str(),
                            path: path,
                            link_name,
                            ty,
                        }])
                        .collect(),
                    ..items
                };
                let new_state = ParsingState {
                    uid: new_uid,
                    current_module,
                    module_is_public,
                };

                (new_state, new_items)
            },
        )
    })
}

fn visit_item_type(monad: ParsingMonad, i: &syn::ItemType) -> ParsingMonad {
    let public = is_visible(&i.vis);
    let ty = i.ty.deref().clone();
    let ident = i.ident.clone();

    monad.bind(move |items| {
        StateMonad::set(
            move |ParsingState {
                      uid,
                      current_module,
                      module_is_public,
                  }| {
                let path = append_path(current_module.clone(), &ident);
                let new_uid = uid.strict_add(1);
                let new_items = FfiItems {
                    aliases: items
                        .aliases
                        .into_iter()
                        .chain([Type {
                            uid: new_uid,
                            public,
                            cached_path: ast::path_to_string(&path),
                            cached_ident: ident.to_string().into_boxed_str(),
                            path: path,
                            ty,
                        }])
                        .collect(),
                    ..items
                };
                let new_state = ParsingState {
                    uid: new_uid,
                    current_module,
                    module_is_public,
                };

                (new_state, new_items)
            },
        )
    })
}

fn visit_item_struct(monad: ParsingMonad, i: &syn::ItemStruct) -> ParsingMonad {
    let public = is_visible(&i.vis);
    let ident = i.ident.clone();
    let fields = match &i.fields {
        syn::Fields::Named(fields) => collect_fields(&fields.named),
        syn::Fields::Unnamed(fields) => collect_fields(&fields.unnamed),
        syn::Fields::Unit => Vec::new(),
    };

    monad.bind(move |items| {
        StateMonad::set(
            move |ParsingState {
                      uid,
                      current_module,
                      module_is_public,
                  }| {
                let path = append_path(current_module.clone(), &ident);
                let new_uid = uid.strict_add(1);
                let new_items = FfiItems {
                    structs: items
                        .structs
                        .into_iter()
                        .chain([Struct {
                            uid: new_uid,
                            public,
                            cached_path: ast::path_to_string(&path),
                            cached_ident: ident.to_string().into_boxed_str(),
                            path: path,
                            fields,
                        }])
                        .collect(),
                    ..items
                };
                let new_state = ParsingState {
                    uid: new_uid,
                    current_module,
                    module_is_public,
                };

                (new_state, new_items)
            },
        )
    })
}

fn visit_item_union(monad: ParsingMonad, i: &syn::ItemUnion) -> ParsingMonad {
    let public = is_visible(&i.vis);
    let ident = i.ident.clone();
    let fields = collect_fields(&i.fields.named);

    monad.bind(move |items| {
        StateMonad::set(
            move |ParsingState {
                      uid,
                      current_module,
                      module_is_public,
                  }| {
                let path = append_path(current_module.clone(), &ident);
                let new_uid = uid.strict_add(1);
                let new_items = FfiItems {
                    unions: items
                        .unions
                        .into_iter()
                        .chain([Union {
                            uid: new_uid,
                            public,
                            cached_path: ast::path_to_string(&path),
                            cached_ident: ident.to_string().into_boxed_str(),
                            path: path,
                            fields,
                        }])
                        .collect(),
                    ..items
                };
                let new_state = ParsingState {
                    uid: new_uid,
                    current_module,
                    module_is_public,
                };

                (new_state, new_items)
            },
        )
    })
}

fn visit_item_const(monad: ParsingMonad, i: &syn::ItemConst) -> ParsingMonad {
    let public = is_visible(&i.vis);
    let ident = i.ident.clone();
    let ty = i.ty.deref().clone();

    monad.bind(move |items| {
        StateMonad::set(
            move |ParsingState {
                      uid,
                      current_module,
                      module_is_public,
                  }| {
                let path = append_path(current_module.clone(), &ident);
                let new_uid = uid.strict_add(1);
                let new_items = FfiItems {
                    constants: items
                        .constants
                        .into_iter()
                        .chain([Const {
                            uid: new_uid,
                            public,
                            cached_path: ast::path_to_string(&path),
                            cached_ident: ident.to_string().into_boxed_str(),
                            path: path,
                            ty,
                        }])
                        .collect(),
                    ..items
                };
                let new_state = ParsingState {
                    uid: new_uid,
                    current_module,
                    module_is_public,
                };

                (new_state, new_items)
            },
        )
    })
}

fn visit_item_foreign_mod(monad: ParsingMonad, i: &syn::ItemForeignMod) -> ParsingMonad {
    // Because we need to store the ABI we can't directly visit the foreign
    // functions/statics.

    // Since this is an extern block, assume extern "C" by default.
    let abi = i
        .abi
        .name
        .clone()
        .map_or(Abi::C, |s| Abi::from(s.value().as_str()));

    i.items.iter().fold(monad, |monad, it| match it {
        ForeignItem::Fn(function) => visit_foreign_item_fn(monad, function, &abi),
        ForeignItem::Static(static_variable) => {
            visit_foreign_item_static(monad, static_variable, &abi)
        }
        _ => monad,
    })
}

fn visit_item_mod(monad: ParsingMonad, i: &syn::ItemMod) -> ParsingMonad {
    let vis = is_visible(&i.vis);
    let ident = i.ident.clone();

    let mut child_monad = StateMonad::pure(FfiItems::default());
    child_monad.visit_item_mod(&i);

    monad.bind(move |items| {
        StateMonad::set(move |state: ParsingState| {
            let (
                ParsingState {
                    uid: uid_after_child,
                    current_module: child_path,
                },
                child_items,
            ) = child_monad.run(state.clone());

            /* NOTE: here we just assume that the child module was non-empty and
             * thus the UID ought be incremented anew for the module itself. */
            let new_uid = uid_after_child.strict_add(1);
            let new_items = FfiItems {
                modules: items
                    .modules
                    .into_iter()
                    .chain([Module {
                        uid: new_uid,
                        public: vis,
                        cached_ident: ident.to_string().into_boxed_str(),
                        cached_path: ast::path_to_string(&child_path),
                        path: child_path,
                        items: child_items,
                    }])
                    .collect(),
                ..items
            };
            let new_state = ParsingState {
                uid: new_uid,
                ..state
            };

            (new_state, new_items)
        })
    })
}

fn visit_item_use(monad: ParsingMonad, ItemUse { vis, tree, .. }: &syn::ItemUse) -> ParsingMonad {
    let vis = vis.clone();
    let base = normalize_path(tree.clone())
        .into_iter()
        .map(move |tree| RefinedUse {
            public: matches!(vis, Visibility::Public(_)),
            tree,
        });

    monad.bind(|items| {
        StateMonad::pure(FfiItems {
            uses: items.uses.into_iter().chain(base).collect(),
            ..items
        })
    })
}

impl<'ast> Visit<'ast> for ParsingMonad {
    fn visit_item_mod(&mut self, i: &'ast ItemMod) {
        let ItemMod {
            ident,
            content: Some((_, items)),
            ..
        } = i
        else {
            unreachable!(
                "This runs post cargo-expand, which inlines all \
                 modules."
            )
        };

        /* NOTE: here we replace the calling state monad with an empty monad so
         * that we can get ownership over a linear term. The calling monad in
         * recursive traversals is unaffected because we recurse down to child
         * modules with a new monad whose sequencing takes place through the
         * state of the calling monad. See the Item::Mod branch in the folding
         * further down below for details. */
        let base_monad = mem::replace(self, StateMonad::set(|s| (s, FfiItems::default())));

        let current_monad = if ident == "__ctest_root_mod" {
            base_monad
        } else {
            let ident = ident.clone();

            base_monad.bind(move |items| {
                StateMonad::set(move |state: ParsingState| {
                    let new_state = ParsingState {
                        current_module: append_path(state.current_module, &ident),
                        ..state
                    };

                    (new_state, items)
                })
            })
        };

        *self = items
            .iter()
            .filter(|it| match it {
                Item::Type(_)
                | Item::Struct(_)
                | Item::Union(_)
                | Item::Const(_)
                | Item::ForeignMod(_)
                | Item::Mod(_)
                | Item::Use(_) => true,
                _ => false,
            })
            .fold(current_monad, |monad, it| {
                macro_rules! exe {
                    ($($f:tt: $ty:tt),+ $(,)?) => {
                        match it {
                            $(Item::$ty(it) => $f(monad, it),)+
                            _ => monad,
                        }
                    };
                }

                exe! {
                    visit_item_type: Type,
                    visit_item_struct: Struct,
                    visit_item_union: Union,
                    visit_item_const: Const,
                    visit_item_foreign_mod: ForeignMod,
                    visit_item_mod: Mod,
                    visit_item_use: Use,
                }
            });
    }
}

#[test]
fn tmp() {
    /*
     * time should end up with the following paths:
     * [time, test1::time, test1::test2::time, test2::time]
     * Foo should end up with the following paths:
     * [
     *   Foo,
     *   test1::Foo,
     *   test1::test3::Foo,
     *   test1::test2::Foo,
     *   test1::test2::test3::Foo,
     *   test2::Foo,
     *   test2::test3::Foo,
     *   test3::Foo,
     * ]
     */
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

    fn remove_uses(it: FfiItems) -> FfiItems {
        let modules: Vec<_> = it
            .modules
            .into_iter()
            .map(|m| Module {
                items: remove_uses(m.items),
                ..m
            })
            .collect();

        FfiItems {
            modules,
            uses: vec![],
            ..it
        }
    }

    let mut items = FfiItems::default();
    items.visit_file(&file);

    println!("{:#?}", remove_uses(items));
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

// #[test]
// #[ignore]
// fn tmp3() {
//     let dst_child_path = Path {
//         leading_colon: None,
//         segments: {
//             let mut out = Punctuated::new();
//             out.push(PathSegment {
//                 ident: Ident::new("foo", Span::call_site()),
//                 arguments: PathArguments::None,
//             });
//             out.clone()
//         },
//     };
//     let dst = Module {
//         items: FfiItems {
//             modules: Vec::from([Module {
//                 path: dst_child_path.clone(),
//                 cached_path: ast::path_to_string(dst_child_path.clone()),
//                 items: FfiItems {
//                     structs: Vec::from([{
//                         let path = Path {
//                             leading_colon: None,
//                             segments: Punctuated::new(),
//                         };
//                         Struct {
//                             path: path.clone(),
//                             cached_path: ast::path_to_string(path.clone()),
//                             ..Default::default()
//                         }
//                     }]),
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             }]),
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//     let src = Vec::from([Resolution::Resolved(FfiItems {
//         modules: Vec::from([Module {
//             path: dst_child_path.clone(),
//             cached_path: ast::path_to_string(dst_child_path.clone()),
//             items: FfiItems {
//                 modules: Vec::from([Module::default()]),
//                 uses: Vec::from([RefinedUse::default()]),
//                 ..Default::default()
//             },
//             ..Default::default()
//         }]),
//         ..Default::default()
//     })]);
//     let merged = merge_module(dst.clone(), src.clone());
//     println!("{:#?}", merged.clone());
// }
