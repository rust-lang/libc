#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]

//! # ctest2 - an FFI binding validator
//!
//! This library is intended to be used as a build dependency in a separate
//! project from the main repo to generate tests which can be used to validate
//! FFI bindings in Rust against the headers from which they come from.

#[cfg(test)]
mod tests;

mod ast;
mod ffi_items;
mod generator;
mod macro_expansion;
mod runner;
mod template;
mod translator;

pub use ast::{Abi, Const, Field, Fn, Parameter, Static, Struct, Type, Union};
pub use generator::TestGenerator;
pub use macro_expansion::expand;
pub use runner::{__compile_test, __run_test, generate_test};
pub use translator::TranslationError;

/// A possible error that can be encountered in our library.
pub type Error = Box<dyn std::error::Error>;
/// A type alias for `std::result::Result` that defaults to our error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
/// A boxed string for representing identifiers.
type BoxStr = Box<str>;
