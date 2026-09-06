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

pub use constant::Const;
pub use field::Field;
pub use function::Fn;
pub use module::Module;
pub use parameter::Parameter;
pub use static_variable::Static;
pub use structure::Struct;
pub use type_alias::Type;
pub use union::Union;

use crate::MapInput;

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
