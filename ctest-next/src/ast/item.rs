use crate::{Const, Fn, Static, Struct, Type, Union};

/// Things that can appear directly inside of a module or scope.
///
/// This is not an exhaustive list and only contains variants directly useful
/// for our purposes.
pub enum Item {
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
