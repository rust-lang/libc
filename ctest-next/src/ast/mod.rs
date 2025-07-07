mod constant;
mod field;
mod function;
mod parameter;
mod static_variable;
mod structure;
mod type_alias;
mod union;

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

/// Things that can appear directly inside of a module or scope.
///
/// This is not an exhaustive list and only contains variants directly useful
/// for our purposes.
#[derive(Debug, Clone)]
#[expect(unused)]
pub(crate) enum Item {
    /// Represents a constant defined in Rust.
    Const(Const),
    /// Represents a function defined in Rust.
    Fn(Fn),
    /// Represents a static variable defined in Rust.
    Static(Static),
    /// Represents a type alias defined in Rust.
    Type(Type),
    /// Represents a struct defined in Rust.
    Struct(Struct),
    /// Represents a union defined in Rust.
    Union(Union),
}
