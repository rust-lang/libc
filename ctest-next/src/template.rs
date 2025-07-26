use std::ops::Deref;

use askama::Template;
use quote::ToTokens;
use syn::spanned::Spanned;

use crate::ffi_items::FfiItems;
use crate::translator::{TranslationErrorKind, Translator, translate_abi, translate_expr};
use crate::{BoxStr, Field, MapInput, Result, TestGenerator, TranslationError, VolatileItemKind};

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
        template.populate_field_size_offset_tests(&helper)?;
        template.populate_field_ptr_tests(&helper)?;
        template.populate_roundtrip_tests(&helper)?;

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

    /// Populates field size and offset tests for structs/unions.
    ///
    /// It also keeps track of the names of each test.
    fn populate_field_size_offset_tests(
        &mut self,
        helper: &TranslateHelper,
    ) -> Result<(), TranslationError> {
        let should_skip = |map_input| helper.generator.skips.iter().any(|f| f(&map_input));

        let struct_fields = helper
            .ffi_items
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
            .ffi_items
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
        for alias in helper.ffi_items.aliases() {
            let c_ty = helper.c_type(alias)?;
            self.add_roundtrip_test(helper, alias.ident(), &[], &c_ty, true);
        }
        for struct_ in helper.ffi_items.structs() {
            let c_ty = helper.c_type(struct_)?;
            self.add_roundtrip_test(helper, struct_.ident(), &struct_.fields, &c_ty, false);
        }
        for union_ in helper.ffi_items.unions() {
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
            .ffi_items
            .structs()
            .iter()
            .flat_map(|s| s.fields.iter().map(move |f| (s, f)))
            .filter(|(s, f)| {
                !should_skip(MapInput::StructField(s, f))
                    && !should_skip(MapInput::StructFieldType(s, f))
                    && f.public
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
            .ffi_items
            .unions()
            .iter()
            .flat_map(|u| u.fields.iter().map(move |f| (u, f)))
            .filter(|(u, f)| {
                !should_skip(MapInput::UnionField(u, f))
                    && !should_skip(MapInput::UnionFieldType(u, f))
                    && f.public
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
            let field_return_type = helper
                .make_cdecl(
                    &format!("ctest_field_ty__{}__{}", id, field.ident()),
                    &field.ty,
                )?
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

/// Wrap methods that depend on both ffi items and the generator.
pub(crate) struct TranslateHelper<'a> {
    ffi_items: &'a FfiItems,
    generator: &'a TestGenerator,
    translator: Translator,
}

impl<'a> TranslateHelper<'a> {
    /// Create a new translation helper.
    #[cfg_attr(not(test), expect(unused))]
    pub(crate) fn new(ffi_items: &'a FfiItems, generator: &'a TestGenerator) -> Self {
        Self {
            ffi_items,
            generator,
            translator: Translator::new(),
        }
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

        Ok(self.generator.rty_to_cty(item))
    }

    /// Get the properly mapped type for some `syn::Type`, recursing for pointer types as needed.
    ///
    /// This method is meant to only be used to translate simpler types, such as primitives or
    /// pointers/references to primitives. It will also add struct/union keywords as needed.
    fn basic_c_type(&self, ty: &syn::Type) -> Result<String, TranslationError> {
        let type_name = match ty {
            syn::Type::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
            syn::Type::Ptr(p) => self.basic_c_type(&p.elem)?,
            syn::Type::Reference(r) => self.basic_c_type(&r.elem)?,
            _ => ty.to_token_stream().to_string(),
        };

        let unmapped_c_type = self.translator.translate_type(ty)?;
        let item = if self.ffi_items.contains_struct(&type_name) {
            MapInput::StructType(&unmapped_c_type)
        } else if self.ffi_items.contains_union(&type_name) {
            MapInput::UnionType(&unmapped_c_type)
        } else {
            MapInput::Type(&unmapped_c_type)
        };

        Ok(self.generator.rty_to_cty(item))
    }

    /// Partially translate a Rust bare function type into its equivalent C type.
    ///
    /// It returns the translated return type, translated argument types, and whether
    /// it is variadic as a tuple.
    fn translate_signature_partial(
        &self,
        signature: &syn::TypeBareFn,
    ) -> Result<(String, Vec<String>, bool), TranslationError> {
        let args = signature
            .inputs
            .iter()
            .map(|arg| self.basic_c_type(&arg.ty))
            .collect::<Result<Vec<_>, TranslationError>>()?;
        let return_type = match &signature.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, ty) => match ty.deref() {
                syn::Type::Never(_) => "void".to_string(),
                syn::Type::Tuple(tuple) if tuple.elems.is_empty() => "void".to_string(),
                _ => self.basic_c_type(ty.deref())?,
            },
        };
        Ok((return_type, args, signature.variadic.is_some()))
    }

    /// Modify function signatures to properly return pointer types in C.
    ///
    /// In C, function pointers and arrays have a different syntax to return them,
    /// and this translation is done by this method.
    pub(crate) fn make_cdecl(
        &self,
        name: &str,
        ty: &syn::Type,
    ) -> Result<String, TranslationError> {
        match ty {
            syn::Type::Path(p) => {
                let last = p.path.segments.last().unwrap();
                let ident = last.ident.to_string();
                if ident != "Option" {
                    let mapped_type = self.basic_c_type(ty)?;
                    return Ok(format!("{mapped_type}* {name}"));
                }
                if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
                    if let syn::GenericArgument::Type(inner_ty) = args.args.first().unwrap() {
                        // Option<T> is ONLY ffi-safe if it contains a function pointer, or a reference.
                        match inner_ty {
                            syn::Type::Reference(_) | syn::Type::BareFn(_) => {
                                return self.make_cdecl(name, inner_ty);
                            }
                            _ => {
                                return Err(TranslationError::new(
                                    TranslationErrorKind::NotFfiCompatible,
                                    &p.to_token_stream().to_string(),
                                    inner_ty.span(),
                                ));
                            }
                        }
                    }
                }
            }
            syn::Type::BareFn(f) => {
                let (ret, mut args, variadic) = self.translate_signature_partial(f)?;
                let abi = if let Some(abi) = &f.abi {
                    let target = self
                        .generator
                        .target
                        .clone()
                        .or_else(|| std::env::var("TARGET").ok())
                        .or_else(|| std::env::var("TARGET_PLATFORM").ok())
                        .unwrap();
                    translate_abi(abi, &target)
                } else {
                    ""
                };

                if variadic {
                    args.push("...".to_string());
                } else if args.is_empty() {
                    args.push("void".to_string());
                }

                return Ok(format!("{} ({}**{})({})", ret, abi, name, args.join(", ")));
            }
            // Arrays are supported only up to 2D arrays.
            syn::Type::Array(outer) => {
                let elem = outer.elem.deref();
                let len_outer = translate_expr(&outer.len);

                if let syn::Type::Array(inner) = elem {
                    let inner_type = self.basic_c_type(inner.elem.deref())?;
                    let len_inner = translate_expr(&inner.len);
                    return Ok(format!("{inner_type} (*{name})[{len_outer}][{len_inner}]",));
                } else {
                    let elem_type = self.basic_c_type(elem)?;
                    return Ok(format!("{elem_type} (*{name})[{len_outer}]"));
                }
            }
            _ => {
                let elem_type = self.basic_c_type(ty)?;
                return Ok(format!("{elem_type} *{name}"));
            }
        }

        let mapped_type = self.basic_c_type(ty)?;
        Ok(format!("{mapped_type}* {name}"))
    }
}
