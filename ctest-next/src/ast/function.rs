use crate::{Abi, BoxStr, Parameter};

/// Represents a function signature defined in Rust.
///
/// This structure is only used for parsing functions in extern blocks.
#[derive(Debug, Clone)]
pub struct Fn {
    pub(crate) public: bool,
    #[expect(unused)]
    pub(crate) abi: Abi,
    pub(crate) ident: BoxStr,
    #[expect(unused)]
    pub(crate) parameters: Vec<Parameter>,
    #[expect(unused)]
    pub(crate) return_type: Option<syn::Type>,
}

impl Fn {
    /// Return the identifier of the function as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
