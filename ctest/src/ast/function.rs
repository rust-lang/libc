use crate::{
    Abi,
    BoxStr,
    Parameter,
};

/// Represents a function signature defined in Rust.
///
/// This structure is only used for parsing functions in extern blocks.
#[derive(Debug, Clone)]
pub struct Fn {
    pub(crate) public: bool,
    #[expect(unused)]
    pub(crate) abi: Abi,
    pub(crate) ident: BoxStr,
    pub(crate) path: syn::Path,
    pub(crate) link_name: Option<BoxStr>,
    #[expect(unused)]
    pub(crate) parameters: Vec<Parameter>,
    #[expect(unused)]
    pub(crate) return_type: Option<syn::Type>,
}

impl Fn {
    /// Return the full path to the function item as a string.
    ///
    /// If inside a nested module, this will return a top-level-relative path,
    /// but not a crate-relative path. For some item `foo` in module
    /// `crate::bar`, the returned string will be `bar::foo`, and not
    /// `crate::bar::foo`.
    pub fn path(&self) -> &str {
        &self.ident
    }

    /// Returns the last path of the identifier, from the absolute path returned
    /// by [`Fn::path`].
    pub fn ident(&self) -> String {
        let Some(syn::PathSegment { ident, .. }) = self.path.segments.last() else {
            unreachable!("all parsed items have at least one element in their path")
        };
        ident.to_string()
    }

    /// Return the name of the function to be linked C side with.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }
}
