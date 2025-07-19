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
    pub signededness_tests: Vec<TestSignededness>,
    pub size_align_tests: Vec<TestSizeAlign>,
    pub const_cstr_tests: Vec<TestCStr>,
    pub const_tests: Vec<TestConst>,
    pub test_idents: Vec<BoxStr>,
}

impl TestTemplate {
    /// Populate all tests for all items depending on the configuration provided.
    pub(crate) fn new(
        ffi_items: &FfiItems,
        generator: &TestGenerator,
    ) -> Result<Self, TranslationError> {
        let helper = TranslateHelper {
            ffi_items,
            generator,
            translator: Translator::new(),
        };

        let mut template = Self::default();
        template.populate_const_and_cstr_tests(&helper)?;
        template.populate_size_align_tests(&helper)?;
        template.populate_signededness_tests(&helper)?;

        Ok(template)
    }

    /// Populates tests for constants and C-str constants, keeping track of the names of each test.
    fn populate_const_and_cstr_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        for constant in helper.ffi_items.constants() {
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
                self.const_cstr_tests.push(item.clone());
                self.test_idents.push(item.test_name);
            } else {
                let item = TestConst {
                    id: constant.ident().into(),
                    test_name: const_test_ident(constant.ident()),
                    rust_val: constant.ident().into(),
                    rust_ty: constant.ty.to_token_stream().to_string().into_boxed_str(),
                    c_val: helper.c_ident(constant).into(),
                    c_ty: helper.c_type(constant)?.into(),
                };
                self.const_tests.push(item.clone());
                self.test_idents.push(item.test_name);
            }
        }

        Ok(())
    }

    /// Populates size and alignment tests for aliases, structs, and unions.
    ///
    /// It also keeps track of the names of each test.
    fn populate_size_align_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        for alias in helper.ffi_items.aliases() {
            let item = TestSizeAlign {
                test_name: size_align_test_ident(alias.ident()),
                id: alias.ident().into(),
                rust_ty: alias.ident().into(),
                c_ty: helper.c_type(alias)?.into(),
            };
            self.size_align_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }
        for struct_ in helper.ffi_items.structs() {
            let item = TestSizeAlign {
                test_name: size_align_test_ident(struct_.ident()),
                id: struct_.ident().into(),
                rust_ty: struct_.ident().into(),
                c_ty: helper.c_type(struct_)?.into(),
            };
            self.size_align_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }
        for union_ in helper.ffi_items.unions() {
            let item = TestSizeAlign {
                test_name: size_align_test_ident(union_.ident()),
                id: union_.ident().into(),
                rust_ty: union_.ident().into(),
                c_ty: helper.c_type(union_)?.into(),
            };
            self.size_align_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }

        Ok(())
    }

    /// Populates signededness tests for aliases.
    ///
    /// It also keeps track of the names of each test.
    fn populate_signededness_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        for alias in helper.ffi_items.aliases() {
            let should_skip_signededness_test = helper
                .generator
                .skip_signededness
                .as_ref()
                .is_some_and(|skip| skip(alias.ident()));

            if !helper.translator.is_signed(helper.ffi_items, &alias.ty)
                || should_skip_signededness_test
            {
                continue;
            }
            let item = TestSignededness {
                test_name: signededness_test_ident(alias.ident()),
                id: alias.ident().into(),
                c_ty: helper.c_type(alias)?.into(),
            };
            self.signededness_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }

        Ok(())
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

#[derive(Clone, Debug)]
pub(crate) struct TestSignededness {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub c_ty: BoxStr,
}

#[derive(Clone, Debug)]
pub(crate) struct TestSizeAlign {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub rust_ty: BoxStr,
    pub c_ty: BoxStr,
}

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

fn signededness_test_ident(ident: &str) -> BoxStr {
    format!("ctest_signededness_{ident}").into()
}

fn size_align_test_ident(ident: &str) -> BoxStr {
    format!("ctest_size_align_{ident}").into()
}

fn cstr_test_ident(ident: &str) -> BoxStr {
    format!("ctest_const_cstr_{ident}").into()
}

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
