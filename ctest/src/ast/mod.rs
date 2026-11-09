mod constant;
mod field;
mod function;
mod module;
mod parameter;
mod static_variable;
mod structure;
mod type_alias;
mod union;

use std::slice::Iter;
use std::vec::IntoIter;
use std::{
    fmt,
    ops,
};

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

/// Container for all paths by which a given item may be addressed throughout the crate.
///
/// This is used to store all paths by which an item may be referred to, after it has potentially
/// been imported to modules other than the one it was declared in.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PathAggregateStore {
    value: Vec<Path>,
}

impl PathAggregateStore {
    pub(crate) const fn new() -> Self {
        Self { value: Vec::new() }
    }

    /// Appends a new path to the path aggregate store.
    pub(crate) fn push(self, p: Path) -> Self {
        let PathAggregateStore { mut value } = self;
        value.push(p);

        Self { value }
    }

    /// Checks if any two path aggregates are known to correspond with those of the same item.
    ///
    /// In other words, it checks if the item owning the receiver path aggregate and the item owning
    /// the `other` path aggregate are different occurrences of the same item across the crate's
    /// module tree.
    #[expect(unused)]
    pub(crate) fn akin(&self, other: &Self) -> bool {
        self.value
            .iter()
            .any(|p1| other.value.iter().any(|p2| p1 == p2))
    }

    /// Diffs any two given path aggregates and returns the set of paths that were present in
    /// `other` but missing in the receiver.
    ///
    /// Note that if the receiver contains more paths than `other`, the returned collection will be
    /// empty. In other words, the operation is not reflexive
    /// (`self.diff(other) != other.diff(self)`.)
    pub(crate) fn diff(&self, other: &Self) -> Vec<Path> {
        let ours = &self.value;
        let theirs = &other.value;
        theirs
            .clone()
            .into_iter()
            .filter(|p1| ours.iter().any(|p2| p1 == p2))
            .collect()
    }

    /// Yields a new path aggregate that integrates the paths of the receiver with those of `other`.
    ///
    /// Unlike diffing, the resulting path aggregate store unifies the paths from both parties; The
    /// receiver and `other`.
    #[expect(unused)]
    pub(crate) fn merge(self, other: &Self) -> Self {
        self.diff(other)
            .into_iter()
            .fold(self, PathAggregateStore::push)
    }
}

pub(crate) type AddressablePaths = IntoIter<Path>;
pub(crate) type RefAddressablePaths<'a> = Iter<'a, Path>;

impl IntoIterator for PathAggregateStore {
    type IntoIter = AddressablePaths;
    type Item = Path;

    fn into_iter(self) -> Self::IntoIter {
        self.value.into_iter()
    }
}

impl<'a> IntoIterator for &'a PathAggregateStore {
    type IntoIter = RefAddressablePaths<'a>;
    type Item = &'a Path;

    fn into_iter(self) -> Self::IntoIter {
        self.value.iter()
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

impl From<Vec<Path>> for PathAggregateStore {
    fn from(value: Vec<Path>) -> Self {
        Self { value }
    }
}

impl FromIterator<Path> for PathAggregateStore {
    fn from_iter<I: IntoIterator<Item = Path>>(iter: I) -> Self {
        Self {
            value: iter.into_iter().collect(),
        }
    }
}

/// Utility interface for keeping a single renaming function for all parsed items.
///
/// They all share the same set of fields relative to their identifier and path, all of which need
/// updating whenever the path is renamed. We ensure that the only place where this happens is in
/// this function for convenience's sake.
pub(crate) trait PathUtils: Clone {
    fn path(&self) -> &Path;
    fn mk(self, cached_path: BoxStr, cached_ident: BoxStr, path: Path) -> Self;

    fn rename_with(self, f: impl ops::Fn(&Path) -> Path + Clone) -> Self {
        let new_path = f(&self.path());

        self.rename(new_path)
    }

    fn rename(self, new_path: Path) -> Self {
        self.mk(
            path_to_string(&new_path),
            ident_to_string(&new_path),
            new_path,
        )
    }
}

/// Transforms an item's absolute path to use `_` as path separator instead of `::`.
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

/// Returns a stringified `syn::Path` that is meant to match one-to-one the path to the item from
/// the crate root.
pub(crate) fn path_to_string(path: &Path) -> BoxStr {
    path.into_token_stream()
        .to_string()
        .replace(|c: char| c.is_ascii_whitespace(), "")
        .into_boxed_str()
}

/// Returns a stringified representation of the last segment in the passed identifier.
pub(crate) fn ident_to_string(path: &Path) -> BoxStr {
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
