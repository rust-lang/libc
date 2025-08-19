mod constant;
mod field;
mod function;
mod parameter;
mod static_variable;
mod structure;
mod type_alias;
mod union;

use std::fmt;

pub use constant::Const;
pub use field::Field;
pub use function::Fn;
pub use parameter::Parameter;
pub use static_variable::Static;
pub use structure::Struct;
pub use type_alias::Type;
pub use union::Union;

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
