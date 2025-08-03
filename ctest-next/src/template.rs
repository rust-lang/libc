use askama::Template;
use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;

use crate::cdecl::Constness;
use crate::ffi_items::FfiItems;
use crate::translator::{TranslationErrorKind, Translator};
use crate::{
    BoxStr, Field, MapInput, Result, TestGenerator, TranslationError, VolatileItemKind, cdecl,
};

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
    pub field_ptr_tests: Vec<TestFieldPtr>,
    pub field_size_offset_tests: Vec<TestFieldSizeOffset>,
    pub roundtrip_tests: Vec<TestRoundtrip>,
    pub foreign_fn_tests: Vec<TestForeignFn>,
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
        let helper = TranslateHelper::new(ffi_items, generator);

        let mut template = Self::default();
        template.populate_const_and_cstr_tests(&helper)?;
        template.populate_size_align_tests(&helper)?;
        template.populate_signededness_tests(&helper)?;
        template.populate_field_size_offset_tests(&helper)?;
        template.populate_field_ptr_tests(&helper)?;
        template.populate_roundtrip_tests(&helper)?;
        template.populate_foreign_fn_tests(&helper)?;

        Ok(template)
    }

    /// Populates tests for constants and C-str constants, keeping track of the names of each test.
    fn populate_const_and_cstr_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        for constant in helper.filtered_ffi_items.constants() {
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
        for alias in helper.filtered_ffi_items.aliases() {
            let item = TestSizeAlign {
                test_name: size_align_test_ident(alias.ident()),
                id: alias.ident().into(),
                rust_ty: alias.ident().into(),
                c_ty: helper.c_type(alias)?.into(),
            };
            self.size_align_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }
        for struct_ in helper.filtered_ffi_items.structs() {
            let item = TestSizeAlign {
                test_name: size_align_test_ident(struct_.ident()),
                id: struct_.ident().into(),
                rust_ty: struct_.ident().into(),
                c_ty: helper.c_type(struct_)?.into(),
            };
            self.size_align_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }
        for union_ in helper.filtered_ffi_items.unions() {
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
        for alias in helper.filtered_ffi_items.aliases() {
            let should_skip_signededness_test = helper
                .generator
                .skip_signededness
                .as_ref()
                .is_some_and(|skip| skip(alias.ident()));

            if !helper.translator.is_signed(&alias.ty) || should_skip_signededness_test {
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

    /// Populates field size and offset tests for structs/unions.
    ///
    /// It also keeps track of the names of each test.
    fn populate_field_size_offset_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        let should_skip = |map_input| helper.generator.skips.iter().any(|f| f(&map_input));

        let struct_fields = helper
            .filtered_ffi_items
            .structs()
            .iter()
            .flat_map(|struct_| struct_.fields.iter().map(move |field| (struct_, field)))
            .filter(|(struct_, field)| {
                !should_skip(MapInput::StructField(struct_, field)) && field.public
            })
            .map(|(struct_, field)| {
                (
                    struct_.ident(),
                    field,
                    helper.c_type(struct_),
                    helper.c_ident(MapInput::StructField(struct_, field)),
                )
            });
        let union_fields = helper
            .filtered_ffi_items
            .unions()
            .iter()
            .flat_map(|union_| union_.fields.iter().map(move |field| (union_, field)))
            .filter(|(union_, field)| {
                !should_skip(MapInput::UnionField(union_, field)) && field.public
            })
            .map(|(union_, field)| {
                (
                    union_.ident(),
                    field,
                    helper.c_type(union_),
                    helper.c_ident(MapInput::UnionField(union_, field)),
                )
            });

        for (id, field, c_ty, c_field) in struct_fields.chain(union_fields) {
            let item = TestFieldSizeOffset {
                test_name: field_size_offset_test_ident(id, field.ident()),
                id: id.into(),
                c_ty: c_ty?.into(),
                field: field.clone(),
                c_field: c_field.into_boxed_str(),
            };
            self.field_size_offset_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }

        Ok(())
    }

    /// Populates roundtrip tests for aliases/structs/unions.
    ///
    /// It also keeps track of the names of each test.
    fn populate_roundtrip_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        for alias in helper.filtered_ffi_items.aliases() {
            let c_ty = helper.c_type(alias)?;
            self.add_roundtrip_test(helper, alias.ident(), &[], &c_ty, true);
        }
        for struct_ in helper.filtered_ffi_items.structs() {
            let c_ty = helper.c_type(struct_)?;
            self.add_roundtrip_test(helper, struct_.ident(), &struct_.fields, &c_ty, false);
        }
        for union_ in helper.filtered_ffi_items.unions() {
            let c_ty = helper.c_type(union_)?;
            self.add_roundtrip_test(helper, union_.ident(), &union_.fields, &c_ty, false);
        }

        Ok(())
    }

    fn add_roundtrip_test(
        &mut self,
        helper: &TranslateHelper,
        ident: &str,
        fields: &[Field],
        c_ty: &str,
        is_alias: bool,
    ) {
        let should_skip_roundtrip_test = helper
            .generator
            .skip_roundtrip
            .as_ref()
            .is_some_and(|skip| skip(ident));
        if !should_skip_roundtrip_test {
            let item = TestRoundtrip {
                test_name: roundtrip_test_ident(ident),
                id: ident.into(),
                fields: fields.iter().filter(|f| f.public).cloned().collect(),
                c_ty: c_ty.into(),
                is_alias,
            };
            self.roundtrip_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }
    }

    /// Populates field tests for structs/unions.
    ///
    /// It also keeps track of the names of each test.
    fn populate_field_ptr_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        let should_skip = |map_input| helper.generator.skips.iter().any(|f| f(&map_input));

        let struct_fields = helper
            .filtered_ffi_items
            .structs()
            .iter()
            .flat_map(|s| s.fields.iter().map(move |f| (s, f)))
            .filter(|(s, f)| {
                !(should_skip(MapInput::StructField(s, f))
                    || should_skip(MapInput::StructFieldType(s, f))
                    || !f.public)
            })
            .map(|(s, f)| {
                (
                    s.ident(),
                    f,
                    helper.c_type(s),
                    helper.c_ident(MapInput::StructField(s, f)),
                    if !helper.generator.volatile_items.is_empty()
                        && helper
                            .generator
                            .volatile_items
                            .iter()
                            .any(|vf| vf(VolatileItemKind::StructField(s.clone(), f.clone())))
                    {
                        "volatile "
                    } else {
                        ""
                    },
                )
            });
        let union_fields = helper
            .filtered_ffi_items
            .unions()
            .iter()
            .flat_map(|u| u.fields.iter().map(move |f| (u, f)))
            .filter(|(u, f)| {
                !(should_skip(MapInput::UnionField(u, f))
                    || should_skip(MapInput::UnionFieldType(u, f))
                    || !f.public)
            })
            .map(|(u, f)| {
                (
                    u.ident(),
                    f,
                    helper.c_type(u),
                    helper.c_ident(MapInput::UnionField(u, f)),
                    "",
                )
            });

        for (id, field, c_ty, c_field, volatile_keyword) in struct_fields.chain(union_fields) {
            let field_return_type = cdecl::cdecl(
                &cdecl::ptr(
                    helper.translator.translate_type(&field.ty)?,
                    cdecl::Constness::Mut,
                ),
                format!("ctest_field_ty__{}__{}", id, field.ident()),
            )
            .map_err(|_| {
                TranslationError::new(
                    TranslationErrorKind::InvalidReturn,
                    &field.ty.to_token_stream().to_string(),
                    field.ty.span(),
                )
            })?
            .into_boxed_str();
            let item = TestFieldPtr {
                test_name: field_ptr_test_ident(id, field.ident()),
                id: id.into(),
                c_ty: c_ty?.into(),
                field: field.clone(),
                c_field: c_field.into_boxed_str(),
                volatile_keyword: volatile_keyword.into(),
                field_return_type,
            };
            self.field_ptr_tests.push(item.clone());
            self.test_idents.push(item.test_name);
        }

        Ok(())
    }

    /// Populates tests for extern functions.
    ///
    /// It also keeps track of the names of each test.
    fn populate_foreign_fn_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        let should_skip_fn_test = |ident| {
            helper
                .generator
                .skip_fn_ptrcheck
                .as_ref()
                .is_some_and(|skip| skip(ident))
        };
        for func in helper.ffi_items.foreign_functions() {
            if should_skip_fn_test(func.ident()) {
                continue;
            }

            let item = TestForeignFn {
                test_name: foreign_fn_test_ident(func.ident()),
                id: func.ident().into(),
                c_val: helper.c_ident(func).into_boxed_str(),
            };

            self.foreign_fn_tests.push(item.clone());
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

#[derive(Clone, Debug)]
pub(crate) struct TestFieldPtr {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub field: Field,
    pub c_field: BoxStr,
    pub c_ty: BoxStr,
    pub volatile_keyword: BoxStr,
    pub field_return_type: BoxStr,
}

#[derive(Clone, Debug)]
pub(crate) struct TestFieldSizeOffset {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub field: Field,
    pub c_field: BoxStr,
    pub c_ty: BoxStr,
}

#[derive(Clone, Debug)]
pub(crate) struct TestRoundtrip {
    pub test_name: BoxStr,
    pub id: BoxStr,
    pub fields: Vec<Field>,
    pub c_ty: BoxStr,
    pub is_alias: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct TestForeignFn {
    pub test_name: BoxStr,
    pub c_val: BoxStr,
    pub id: BoxStr,
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

fn field_ptr_test_ident(ident: &str, field_ident: &str) -> BoxStr {
    format!("ctest_field_ptr_{ident}_{field_ident}").into()
}

fn field_size_offset_test_ident(ident: &str, field_ident: &str) -> BoxStr {
    format!("ctest_field_size_offset_{ident}_{field_ident}").into()
}

fn roundtrip_test_ident(ident: &str) -> BoxStr {
    format!("ctest_roundtrip_{ident}").into()
}

fn foreign_fn_test_ident(ident: &str) -> BoxStr {
    format!("ctest_foreign_fn_{ident}").into()
}

/// Wrap methods that depend on both ffi items and the generator.
pub(crate) struct TranslateHelper<'a> {
    filtered_ffi_items: FfiItems,
    ffi_items: &'a FfiItems,
    generator: &'a TestGenerator,
    translator: Translator<'a>,
}

impl<'a> TranslateHelper<'a> {
    /// Create a new translation helper.
    pub(crate) fn new(ffi_items: &'a FfiItems, generator: &'a TestGenerator) -> Self {
        let filtered_ffi_items = ffi_items.clone();
        let mut helper = Self {
            filtered_ffi_items,
            ffi_items,
            generator,
            translator: Translator::new(ffi_items, generator),
        };
        helper.filter_ffi_items();

        helper
    }

    /// Skips entire items such as structs, constants, and aliases from being tested.
    ///
    /// Does not skip specific tests or specific fields. If `skip_private` is true,
    /// it will skip tests for all private items.
    fn filter_ffi_items(&mut self) {
        let verbose = self.generator.verbose_skip;

        macro_rules! filter {
            ($field:ident, $variant:ident, $label:literal) => {{
                let skipped = self.filtered_ffi_items.$field.extract_if(.., |item| {
                    (self.generator.skip_private && !item.public)
                        || self
                            .generator
                            .skips
                            .iter()
                            .any(|f| f(&MapInput::$variant(item)))
                });
                for item in skipped {
                    if verbose {
                        eprintln!("Skipping {} \"{}\"", $label, item.ident())
                    }
                }
            }};
        }

        filter!(aliases, Alias, "alias");
        filter!(constants, Const, "const");
        filter!(structs, Struct, "struct");
        filter!(unions, Union, "union");
        filter!(foreign_functions, Fn, "fn");
        filter!(foreign_statics, Static, "static");
    }

    /// Returns the equivalent C/Cpp identifier of the Rust item.
    pub(crate) fn c_ident(&self, item: impl Into<MapInput<'a>>) -> String {
        self.generator.rty_to_cty(item)
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
            MapInput::Alias(a) => (a.ident(), cdecl::named(a.ident(), Constness::Mut)),
            MapInput::Struct(s) => (s.ident(), cdecl::named(s.ident(), Constness::Mut)),
            MapInput::Union(u) => (u.ident(), cdecl::named(u.ident(), Constness::Mut)),

            MapInput::StructType(_) => panic!("MapInput::StructType is not allowed!"),
            MapInput::UnionType(_) => panic!("MapInput::UnionType is not allowed!"),
            MapInput::StructFieldType(_, _) => panic!("MapInput::StructFieldType is not allowed!"),
            MapInput::UnionFieldType(_, _) => panic!("MapInput::UnionFieldType is not allowed!"),
            MapInput::Type(_) => panic!("MapInput::Type is not allowed!"),
        };

        let ty = cdecl::cdecl(&ty, "".to_string()).map_err(|_| {
            TranslationError::new(
                TranslationErrorKind::InvalidReturn,
                ident,
                Span::call_site(),
            )
        })?;

        let item = if self.ffi_items.contains_struct(&ty) {
            MapInput::StructType(&ty)
        } else if self.ffi_items.contains_union(ident) {
            MapInput::UnionType(&ty)
        } else {
            MapInput::Type(&ty)
        };

        Ok(self.generator.rty_to_cty(item))
    }
}
