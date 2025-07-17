use askama::Template;
use quote::ToTokens;

use crate::ffi_items::FfiItems;
use crate::translator::Translator;
use crate::{BoxStr, MapInput, Result, TestGenerator, TranslationError};

/// Represents the Rust side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.rs")]
pub(crate) struct RustTestTemplate {
    pub template: TestTemplate,
}

impl RustTestTemplate {
    pub(crate) fn new(
        ffi_items: &FfiItems,
        generator: &TestGenerator,
    ) -> Result<Self, TranslationError> {
        Ok(Self {
            template: TestTemplate::new(ffi_items, generator)?,
        })
    }
}

/// Represents the C side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.c")]
pub(crate) struct CTestTemplate {
    pub template: TestTemplate,
    pub headers: Vec<String>,
}

impl CTestTemplate {
    pub(crate) fn new(
        ffi_items: &FfiItems,
        generator: &TestGenerator,
    ) -> Result<Self, TranslationError> {
        Ok(Self {
            template: TestTemplate::new(ffi_items, generator)?,
            headers: generator.headers.clone(),
        })
    }
}

/// Stores all information necessary for generation of tests for all items.
#[derive(Clone, Debug, Default)]
pub(crate) struct TestTemplate {
    pub const_cstr_tests: Vec<TestCStr>,
    pub const_tests: Vec<TestConst>,
    pub test_idents: Vec<BoxStr>,
}

impl TestTemplate {
    pub(crate) fn new(
        ffi_items: &FfiItems,
        generator: &TestGenerator,
    ) -> Result<Self, TranslationError> {
        let helper = TranslateHelper {
            ffi_items,
            generator,
            translator: Translator::new(),
        };

        /* Figure out which tests are to be generated. */
        // FIXME(ctest): Populate more test information, maybe extract into separate methods.
        // The workflow would be to create a struct that stores information for the new test,
        // and populating that struct here, so that the also things that have to be added to
        // the test templates are the new tests parameterized by that struct.

        let mut const_tests = vec![];
        let mut const_cstr_tests = vec![];
        for constant in ffi_items.constants() {
            if let syn::Type::Ptr(ptr) = &constant.ty
                && let syn::Type::Path(path) = &*ptr.elem
                && path.path.segments.last().unwrap().ident == "c_char"
                && ptr.mutability.is_none()
            {
                let item = TestCStr {
                    id: constant.ident().into(),
                    test_name: cstr_test_ident(constant.ident()),
                    rust_val: constant.ident().into(),
                    c_val: helper.c_ident(constant).into(),
                };
                const_cstr_tests.push(item)
            } else {
                let item = TestConst {
                    id: constant.ident().into(),
                    test_name: const_test_ident(constant.ident()),
                    rust_val: constant.ident.clone(),
                    rust_ty: constant.ty.to_token_stream().to_string().into_boxed_str(),
                    c_val: helper.c_ident(constant).into(),
                    c_ty: helper.c_type(constant)?.into(),
                };
                const_tests.push(item)
            }
        }

        let mut test_idents = vec![];
        test_idents.extend(const_cstr_tests.iter().map(|test| test.test_name.clone()));
        test_idents.extend(const_tests.iter().map(|test| test.test_name.clone()));

        Ok(Self {
            const_cstr_tests,
            const_tests,
            test_idents,
        })
    }
}

/* Many test structures have the following fields:
 *
 * - `test_name`: The function name.
 * - `id`: An identifier that can be used to create functions related to this type without conflict,
 *    usually also part of `test_name`.
 * - `rust_val`: Identifier for a Rust value, with path qualifications if needed.
 * - `rust_ty`: The Rust type of the relevant item, with path qualifications if needed.
 * - `c_val`: Identifier for a C value (e.g. `#define`)
 * - `c_ty`: The C type of the constant, qualified with `struct` or `union` if needed.
 */

/// Information required to test a constant CStr.
#[derive(Clone, Debug)]
pub(crate) struct TestCStr {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub rust_val: BoxStr,
    pub c_val: BoxStr,
}

/// Information required to test a constant.
#[derive(Clone, Debug)]
pub(crate) struct TestConst {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub rust_val: BoxStr,
    pub c_val: BoxStr,
    pub rust_ty: BoxStr,
    pub c_ty: BoxStr,
}

/// The Rust name of the cstr test.
///
/// The C name of this same test is the same with `__` prepended.
fn cstr_test_ident(ident: &str) -> BoxStr {
    format!("ctest_const_cstr_{ident}").into()
}

/// The Rust name of the const test.
///
/// The C name of this test is the same with `__` prepended.
fn const_test_ident(ident: &str) -> BoxStr {
    format!("ctest_const_{ident}").into()
}

/// Wrap methods that depend on both ffi items and the generator.
struct TranslateHelper<'a> {
    ffi_items: &'a FfiItems,
    generator: &'a TestGenerator,
    translator: Translator,
}

impl<'a> TranslateHelper<'a> {
    /// Returns the equivalent C/Cpp identifier of the Rust item.
    pub(crate) fn c_ident(&self, item: impl Into<MapInput<'a>>) -> String {
        self.generator.map(item)
    }

    /// Returns the equivalent C/Cpp type of the Rust item.
    pub(crate) fn c_type(&self, item: impl Into<MapInput<'a>>) -> Result<String, TranslationError> {
        let item: MapInput = item.into();

        let (ident, ty) = match item {
            MapInput::Const(c) => (c.ident(), self.translator.translate_type(&c.ty)?),
            MapInput::StructField(_, f) => (f.ident(), self.translator.translate_type(&f.ty)?),
            MapInput::UnionField(_, f) => (f.ident(), self.translator.translate_type(&f.ty)?),
            MapInput::Static(s) => (s.ident(), self.translator.translate_type(&s.ty)?),
            // For functions, their type would be a bare fn signature, which would need to be saved
            // inside of `Fn` when parsed.
            MapInput::Fn(_) => unimplemented!(),
            // For structs/unions/aliases, their type is the same as their identifier.
            MapInput::Alias(a) => (a.ident(), a.ident().to_string()),
            MapInput::Struct(s) => (s.ident(), s.ident().to_string()),
            MapInput::Union(u) => (u.ident(), u.ident().to_string()),

            MapInput::StructType(_) => panic!("MapInput::StructType is not allowed!"),
            MapInput::UnionType(_) => panic!("MapInput::UnionType is not allowed!"),
            MapInput::StructFieldType(_, _) => panic!("MapInput::StructFieldType is not allowed!"),
            MapInput::UnionFieldType(_, _) => panic!("MapInput::UnionFieldType is not allowed!"),
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
}
