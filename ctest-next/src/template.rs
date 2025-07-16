use askama::Template;
use quote::ToTokens;

use crate::ffi_items::FfiItems;
use crate::translator::Translator;
use crate::{BoxStr, MapInput, Result, TestGenerator, TranslationError};

/// Represents the Rust side of the generated testing suite.
#[derive(Template, Clone)]
#[template(path = "test.rs")]
pub(crate) struct RustTestTemplate {
    pub(crate) template: TestTemplate,
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
    pub(crate) template: TestTemplate,
    pub(crate) headers: Vec<String>,
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
    pub(crate) const_cstr_tests: Vec<TestCstr>,
    pub(crate) const_tests: Vec<TestConst>,
    pub(crate) test_idents: Vec<BoxStr>,
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
            if let syn::Type::Ptr(ptr) = &constant.ty {
                let is_const_c_char_ptr = matches!(
                    &*ptr.elem,
                    syn::Type::Path(path)
                        if path.path.segments.last().unwrap().ident == "c_char"
                        && ptr.mutability.is_none()
                );
                if is_const_c_char_ptr {
                    let item = TestCstr {
                        test_ident: cstr_test_ident(constant.ident()),
                        rust_ident: constant.ident().into(),
                        c_ident: helper.c_ident(constant).into(),
                        c_type: helper.c_type(constant)?.into(),
                    };
                    const_cstr_tests.push(item)
                }
            } else {
                let item = TestConst {
                    test_ident: const_test_ident(constant.ident()),
                    rust_ident: constant.ident.clone(),
                    rust_type: constant.ty.to_token_stream().to_string().into_boxed_str(),
                    c_ident: helper.c_ident(constant).into(),
                    c_type: helper.c_type(constant)?.into(),
                };
                const_tests.push(item)
            }
        }

        let mut test_idents = vec![];
        test_idents.extend(const_cstr_tests.iter().map(|test| test.test_ident.clone()));
        test_idents.extend(const_tests.iter().map(|test| test.test_ident.clone()));

        Ok(Self {
            const_cstr_tests,
            const_tests,
            test_idents,
        })
    }
}

/// Information required to test a constant CStr.
#[derive(Clone, Debug)]
pub(crate) struct TestCstr {
    pub(crate) test_ident: BoxStr,
    pub(crate) rust_ident: BoxStr,
    pub(crate) c_ident: BoxStr,
    pub(crate) c_type: BoxStr,
}

/// Information required to test a constant.
#[derive(Clone, Debug)]
pub(crate) struct TestConst {
    pub(crate) test_ident: BoxStr,
    pub(crate) rust_ident: BoxStr,
    pub(crate) rust_type: BoxStr,
    pub(crate) c_ident: BoxStr,
    pub(crate) c_type: BoxStr,
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
            MapInput::Alias(a) => (a.ident(), self.translator.translate_type(&a.ty)?),
            MapInput::Field(_, f) => (f.ident(), self.translator.translate_type(&f.ty)?),
            MapInput::Static(s) => (s.ident(), self.translator.translate_type(&s.ty)?),
            MapInput::Fn(_) => unimplemented!(),
            MapInput::Struct(_) => unimplemented!(),
            MapInput::StructType(_) => panic!("MapInput::StructType is not allowed!"),
            MapInput::UnionType(_) => panic!("MapInput::UnionType is not allowed!"),
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
