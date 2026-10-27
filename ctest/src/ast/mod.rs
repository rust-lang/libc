mod constant;
mod field;
mod function;
mod module;
mod parameter;
mod static_variable;
mod structure;
mod type_alias;
mod union;

use std::fmt;
use std::ops::{
    self,
    Not,
};
use std::vec::IntoIter;

pub use constant::Const;
pub use field::Field;
pub use function::Fn;
pub use module::Module;
pub use parameter::Parameter;
use quote::ToTokens;
pub use static_variable::Static;
pub use structure::Struct;
use syn::Path;
pub use type_alias::Type;
pub use union::Union;

use crate::{
    BoxStr,
    MapInput,
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PathAggregateStore {
    value: Vec<Path>,
}

impl PathAggregateStore {
    pub(crate) const fn new() -> Self {
        Self { value: Vec::new() }
    }

    pub(crate) fn push(self, p: Path) -> Self {
        let PathAggregateStore { mut value } = self;
        value.push(p);
        Self { value }
    }
}

pub(crate) type AddressablePaths = IntoIter<Path>;

impl IntoIterator for PathAggregateStore {
    type IntoIter = AddressablePaths;
    type Item = Path;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl Extend<Path> for PathAggregateStore {
    fn extend<I: IntoIterator<Item = Path>>(&mut self, iter: I) {
        self.value.extend(iter);
    }
}

impl Default for PathAggregateStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility interface for keeping a single renaming function for all parsed
/// items.
///
/// They all share the same set of fields relative to their identifier and path,
/// all of which need updating whenever the path is renamed. We ensure that the
/// only place where this happens is in this function for convenience's sake.
pub(crate) trait PathUtils: Clone {
    fn path(self) -> Path;
    fn path_aggregate(self) -> PathAggregateStore;
    fn mk(
        self,
        cached_path: BoxStr,
        cached_ident: BoxStr,
        path: Path,
        addressable_paths: PathAggregateStore,
    ) -> Self;

    // [TODO]: override the default impelementation for modules, so as to allow
    // the items living within the subtree rooted at the module to also adopt the
    // new path in their aggreagtes.
    // [TODO]: change all call sites where the path aggregate is modified
    // manually with uses of this method.
    fn add_path(self, path: Path) -> Self {
        let aggregate = self.clone().path_aggregate();
        let new_aggregate = aggregate.push(path);
        self.clone().mk(
            path_to_string(self.clone().path()),
            ident_to_string(self.clone().path()),
            self.clone().path(),
            new_aggregate,
        )
    }

    /// Diffs the path aggregates of two interface implementors of the same
    /// type.
    ///
    /// The resulting collection contains the paths in the aggregate of `other`
    /// that are missing from the aggregate of the receiver.
    fn diff_aggregates(self, other: Self) -> impl Iterator<Item = Path> {
        other.clone().path_aggregate().into_iter().filter(move |p| {
            self.clone()
                .path_aggregate()
                .into_iter()
                .any(|ip| *p == ip)
                .not()
        })
    }

    /// Diffs the path aggregates of both `self` and `other`, and yields a
    /// unified path aggregate integrating the paths from both.
    ///
    /// This uses the `diff_aggregates` trait method to compute the differences
    /// between the receiver and `other`, and then yields the path aggregate
    /// resulting from adding those missing paths to the the receiver's path
    /// aggregate.
    fn merge_aggregates(self, other: Self) -> PathAggregateStore {
        let we_need = self.clone().diff_aggregates(other.clone());
        let mut ours = self.clone().path_aggregate();
        ours.extend(we_need);
        ours
    }

    /// Checks whether two interface implementors contain some common path in
    /// their path aggregate.
    fn akin_aggregates(self, other: Self) -> bool {
        self.clone()
            .path_aggregate()
            .into_iter()
            .any(|p| other.clone().path_aggregate().into_iter().any(|p2| p == p2))
    }

    /// Provided some parsed type, this method will run the given closure on
    /// each variation of a term of that type (`self`.)
    ///
    /// The variations of a given term are given by their addressable paths.
    /// This is an append-only aggregate all implementors of this interface have
    /// as one of their projections. The variations are themselves the possible
    /// paths by which an item may be referred to as a consequence of having
    /// been imported over to other modules of the crate.
    ///
    /// This interface gets access to it through the
    /// [`RenameUtils::addressable_paths`] projection.
    ///
    /// This is a utility to avoid having to go through the variations, as well
    /// as updating the identifier and paths caches whenever some operation
    /// needs to be performed on all variations.
    fn with_variations<T>(&self, f: impl ops::Fn(Self) -> T + Clone) -> Vec<T> {
        self.clone()
            .path_aggregate()
            .into_iter()
            .map(|p| f(self.clone().rename(p.clone())))
            .collect()
    }

    fn rename_with(self, new_path: impl ops::Fn(Path) -> Path + Clone) -> Self {
        self.clone().rename(new_path(self.clone().path()))
    }

    fn rename(self, new_path: Path) -> Self {
        self.clone().mk(
            path_to_string(new_path.clone()),
            ident_to_string(new_path.clone()),
            new_path.clone(),
            self.path_aggregate().push(new_path),
        )
    }
}

/// Transforms an item's absolute path to use `_` as path separator instead of
/// `::`.
pub(crate) fn escape_item_path<'a>(item: impl Into<MapInput<'a>>) -> String {
    let path = match item.into() {
        MapInput::Struct(s) => s.path(),
        MapInput::Union(u) => u.path(),
        MapInput::Fn(f) => f.path(),
        MapInput::Alias(a) => a.path(),
        MapInput::Const(c) => c.path(),
        MapInput::Static(s) => s.path(),

        _ => unimplemented!("other MapInput data constructors do not represent rust items"),
    };
    path.replace("::", "_")
}

/// Returns a stringified `syn::Path` that is meant to match one-to-one the path
/// to the item from the crate root.
pub(crate) fn path_to_string(path: Path) -> BoxStr {
    path.into_token_stream()
        .to_string()
        .replace(|c: char| c.is_ascii_whitespace(), "")
        .into_boxed_str()
}

/// Returns a stringified representation of the last segment in the passed
/// identifier.
pub(crate) fn ident_to_string(path: Path) -> BoxStr {
    path.segments
        .last()
        .map(|it| it.ident.to_string().into_boxed_str())
        .unwrap_or_default()
}

/// The ABI as defined by the extern block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Abi {
    /// The C ABI.
    C,
    /// The Rust ABI.
    Rust,
    /// Any other ABI.
    Other(String),
}

impl From<&str> for Abi {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "c" => Abi::C,
            "rust" => Abi::Rust,
            s => Abi::Other(s.to_string()),
        }
    }
}

impl fmt::Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Abi::C => write!(f, "C"),
            Abi::Rust => write!(f, "Rust"),
            Abi::Other(s) => write!(f, "{s}"),
        }
    }
}
