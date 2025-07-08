use std::ops::Deref;

use askama::Template;
use either::Either;
use quote::ToTokens;

use crate::ffi_items::FfiItems;
use crate::translator::{translate_expr, Translator};
use crate::{
    Field, MapInput, Result, Struct, TestGenerator, TranslationError, Union, VolatileItemKind,
};

/// Represents the Rust side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.rs")]
pub(crate) struct RustTestTemplate<'a> {
    ffi_items: &'a FfiItems,
    translator: Translator,
    generator: &'a TestGenerator,
}

/// Represents the C side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.c")]
pub(crate) struct CTestTemplate<'a> {
    translator: Translator,
    ffi_items: &'a FfiItems,
    generator: &'a TestGenerator,
}

impl<'a> RustTestTemplate<'a> {
    /// Create a new test template to test the given items.
    pub(crate) fn new(ffi_items: &'a FfiItems, generator: &'a TestGenerator) -> Self {
        Self {
            ffi_items,
            translator: Translator::new(),
            generator,
        }
    }
}

impl<'a> CTestTemplate<'a> {
    /// Create a new test template to test the given items.
    pub(crate) fn new(ffi_items: &'a FfiItems, generator: &'a TestGenerator) -> Self {
        Self {
            ffi_items,
            translator: Translator::new(),
            generator,
        }
    }

    /// Returns the equivalent C/Cpp identifier of the Rust item.
    pub(crate) fn c_ident(&self, item: impl Into<MapInput<'a>>) -> String {
        self.generator.map(item)
    }

    /// Returns the equivalent C/Cpp type of the Rust item.
    pub(crate) fn c_type(&self, item: impl Into<MapInput<'a>>) -> Result<String, TranslationError> {
        let item: MapInput<'a> = item.into();

        let (ident, ty) = match item {
            MapInput::Const(c) => (c.ident(), self.translator.translate_type(&c.ty)?),
            MapInput::Field(_, f) => (f.ident(), self.translator.translate_type(&f.ty)?),
            MapInput::Static(s) => (s.ident(), self.translator.translate_type(&s.ty)?),
            MapInput::Fn(_) => unimplemented!(),
            // For structs/unions/aliases, their type is the same as their identifier.
            MapInput::Alias(a) => (a.ident(), a.ident().to_string()),
            MapInput::Struct(s) => (s.ident(), s.ident().to_string()),
            MapInput::Union(u) => (u.ident(), u.ident().to_string()),

            MapInput::StructType(_) => panic!("MapInput::StructType is not allowed!"),
            MapInput::UnionType(_) => panic!("MapInput::UnionType is not allowed!"),
            MapInput::FieldType(_, _) => panic!("MapInput::FieldType is not allowed!"),
            MapInput::Type(_) => panic!("MapInput::Type is not allowed!"),
        };

        let item = if self.ffi_items.contains_struct(ident) {
            MapInput::StructType(&ty)
        } else if self.ffi_items.contains_union(ident) {
            MapInput::UnionType(&ty)
        } else {
            MapInput::Type(&ty)
        };

        Ok(self.generator.map(item))
    }

    /// Modify a C function `signature` that returns a ptr `ty` to be correctly translated.
    ///
    /// Arrays and Function types in C have different rules for placement, such as array lengths
    /// being placed after the parameter list.
    pub(crate) fn c_signature(
        &self,
        ty: &syn::Type,
        signature: &str,
    ) -> Result<String, TranslationError> {
        let new_signature = match ty {
            syn::Type::BareFn(f) => {
                let (ret, mut args, variadic) = self.translator.translate_signature_partial(f)?;
                let abi = f
                    .abi
                    .clone()
                    .unwrap()
                    .name
                    .map(|s| s.value())
                    .unwrap_or("C".to_string());

                if variadic {
                    args.push("...".to_string());
                } else if args.is_empty() {
                    args.push("void".to_string());
                }

                format!("{}({}**{})({})", ret, abi, signature, args.join(", "))
            }
            // Handles up to 2D arrays.
            syn::Type::Array(outer) => match outer.elem.deref() {
                syn::Type::Array(inner) => format!(
                    "{}(*{})[{}][{}]",
                    self.translator.translate_type(inner.elem.deref())?,
                    signature,
                    translate_expr(&outer.len),
                    translate_expr(&inner.len)
                ),
                _ => format!(
                    "{}(*{})[{}]",
                    self.translator.translate_type(outer.elem.deref())?,
                    signature,
                    translate_expr(&outer.len)
                ),
            },
            _ => {
                let unmapped_c_type = self.translator.translate_type(ty)?;
                let map_input = if self
                    .ffi_items
                    .contains_struct(&ty.to_token_stream().to_string())
                {
                    MapInput::StructType(&unmapped_c_type)
                } else if self
                    .ffi_items
                    .contains_union(&ty.to_token_stream().to_string())
                {
                    MapInput::UnionType(&unmapped_c_type)
                } else {
                    MapInput::Type(&unmapped_c_type)
                };
                format!("{}* {}", self.generator.map(map_input), signature)
            }
        };

        Ok(new_signature)
    }

    /// Returns the volatile keyword if the given item is volatile.
    pub(crate) fn emit_volatile(&self, v: VolatileItemKind) -> &str {
        if !self.generator.volatile_items.is_empty()
            && self.generator.volatile_items.iter().any(|f| f(v.clone()))
        {
            "volatile "
        } else {
            ""
        }
    }
}

/* Helper functions to make the template code readable. */

/// Determine whether a Rust alias/struct/union should have a round trip test.
///
/// By default all alias/struct/unions are roundtripped. Aliases or fields with arrays should
/// not be part of the roundtrip.
pub(crate) fn should_roundtrip(gen: &TestGenerator, ident: &str) -> bool {
    gen.skip_roundtrip.as_ref().is_none_or(|skip| !skip(ident))
}

/// Determine whether a struct field should be skipped for tests.
pub(crate) fn should_skip_field(
    gen: &TestGenerator,
    e: Either<&Struct, &Union>,
    field: &Field,
) -> bool {
    gen.skips.iter().any(|f| f(&MapInput::Field(e, field)))
}

/// Determine whether a struct field type should be skipped for tests.
pub(crate) fn should_skip_field_type(
    gen: &TestGenerator,
    e: Either<&Struct, &Union>,
    field: &Field,
) -> bool {
    gen.skips.iter().any(|f| f(&MapInput::FieldType(e, field)))
}
