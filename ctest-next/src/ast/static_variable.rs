use crate::{Abi, BoxStr};

/// Represents a static variable in Rust.
///
/// This structure is only used for parsing statics in extern blocks,
/// as a result it does not have a field for storing the expression.
#[derive(Debug, Clone)]
pub struct Static {
    pub(crate) public: bool,
    #[expect(unused)]
    pub(crate) abi: Abi,
    pub(crate) ident: BoxStr,
    pub(crate) link_name: Option<BoxStr>,
    pub(crate) ty: syn::Type,
}

impl Static {
    /// Return the identifier of the static variable as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Return the name of the function to be linked C side with.
    pub fn link_name(&self) -> Option<&str> {
        self.link_name.as_deref()
    }
}
