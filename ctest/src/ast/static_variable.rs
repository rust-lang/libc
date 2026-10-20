use crate::{
    Abi,
    BoxStr,
};

/// Represents a static variable in Rust.
///
/// This structure is only used for parsing statics in extern blocks,
/// as a result it does not have a field for storing the expression.
#[derive(Debug, Clone, PartialEq)]
pub struct Static {
    pub(crate) public: bool,
    #[expect(unused)]
    pub(crate) abi: Abi,
    pub(crate) cached_path: BoxStr,
    pub(crate) cached_ident: BoxStr,
    pub(crate) path: syn::Path,
    pub(crate) link_name: Option<BoxStr>,
    pub(crate) ty: syn::Type,
}

impl Static {
    /// Return the full path to the static variable as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.cached_path
    }

    /// Returns the last segment of the static's path; Namely, its identifier.
    pub fn ident(&self) -> &str {
        &self.cached_ident
    }

    /// Return the name of the function to be linked C side with.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }
}
