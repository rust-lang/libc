use crate::BoxStr;
use crate::ffi_items::FfiItems;

/// Represents a Rust module. `ctest` only considers items for which there is a
/// corresponding type in this crate.
#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) path: syn::Path,
    pub(crate) items: FfiItems,
}

impl Module {
    /// Returns the identifier of the parsed module.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
