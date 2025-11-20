#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]

//! # ctest - an FFI binding validator
//!
//! This library is intended to be used as a build dependency in a separate
//! project from the main repo to generate tests which can be used to validate
//! FFI bindings in Rust against the headers from which they come from.

#[cfg(test)]
mod tests;

mod ast;
mod cdecl;
mod ffi_items;
mod generator;
mod macro_expansion;
mod runner;
mod template;
mod translator;

use std::env;

pub use ast::{
    Abi,
    Const,
    Field,
    Fn,
    Parameter,
    Static,
    Struct,
    Type,
    Union,
};
pub use generator::TestGenerator;
pub use macro_expansion::expand;
pub use runner::{
    __compile_test,
    __run_test,
    generate_test,
};
pub use translator::TranslationError;

use crate::generator::GenerationError;

/// A possible error that can be encountered in our library.
pub type Error = Box<dyn std::error::Error>;
/// A type alias for `std::result::Result` that defaults to our error type.
pub type Result<T, E = Error> = std::result::Result<T, E>;
/// A boxed string for representing identifiers.
type BoxStr = Box<str>;

/// The edition used by rustc when expanding macros and compiling tests.
pub(crate) const EDITION: &str = "2021";

/// A kind of item to which the C volatile qualifier could apply.
///
/// This is necessary because `ctest` does not parse the header file, so it
/// does not know which items are volatile.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum VolatileItemKind {
    /// A struct field.
    StructField(Struct, Field),
    /// An extern static.
    Static(Static),
    /// A function argument.
    FnArgument(Fn, Box<Parameter>),
    /// Function return type.
    FnReturnType(Fn),
}

/// Inputs needed to rename or skip a field.
#[derive(Debug, Clone)]
pub(crate) enum MapInput<'a> {
    /// This variant is used for renaming the struct identifier.
    Struct(&'a Struct),
    Union(&'a Union),
    Fn(&'a crate::Fn),
    StructField(&'a Struct, &'a Field),
    UnionField(&'a Union, &'a Field),
    Alias(&'a Type),
    Const(&'a Const),
    Static(&'a Static),
    Type(&'a str),
    /// This variant is used for renaming the struct type.
    StructType(&'a str),
    UnionType(&'a str),
    CEnumType(&'a str),
    StructFieldType(&'a Struct, &'a Field),
    UnionFieldType(&'a Union, &'a Field),
}

/// The language used to generate the tests.
#[derive(Debug, Default, Clone)]
#[non_exhaustive]
pub enum Language {
    /// The C Programming Language.
    #[default]
    C,
    /// The C++ Programming Language.
    CXX,
}

impl Language {
    /// Return the file extension of the programming language.
    pub(crate) fn extension(&self) -> &str {
        match self {
            Self::C => "c",
            Self::CXX => "cpp",
        }
    }
}

/// Search for the target to build for, specified manually or through an environment variable.
///
/// This function will check the following places for the target name:
/// - TestGenerator.target
/// - TARGET environment variable.
/// - TARGET_PLATFORM environment variable.
fn get_build_target(generator: &TestGenerator) -> Result<String, GenerationError> {
    generator
        .target
        .clone()
        .or_else(|| env::var("TARGET").ok())
        .or_else(|| env::var("TARGET_PLATFORM").ok())
        .ok_or(GenerationError::EnvVarNotFound(
            "TARGET, TARGET_PLATFORM".to_string(),
        ))
}

/* The From impls make it easier to write code in the test templates. */

impl<'a> From<&'a Const> for MapInput<'a> {
    fn from(c: &'a Const) -> Self {
        MapInput::Const(c)
    }
}

impl<'a> From<&'a crate::Fn> for MapInput<'a> {
    fn from(f: &'a crate::Fn) -> Self {
        MapInput::Fn(f)
    }
}

impl<'a> From<&'a Type> for MapInput<'a> {
    fn from(a: &'a Type) -> Self {
        MapInput::Alias(a)
    }
}

impl<'a> From<&'a Static> for MapInput<'a> {
    fn from(s: &'a Static) -> Self {
        MapInput::Static(s)
    }
}

impl<'a> From<&'a Struct> for MapInput<'a> {
    fn from(s: &'a Struct) -> Self {
        MapInput::Struct(s)
    }
}

impl<'a> From<&'a Union> for MapInput<'a> {
    fn from(u: &'a Union) -> Self {
        MapInput::Union(u)
    }
}
