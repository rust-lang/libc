//! Translation of Rust types to C for test generation.
//!
//! Simple to semi complex types are supported only.

use std::fmt;
use std::ops::{Deref, DerefMut};

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use thiserror::Error;

use crate::cdecl::Constness;
use crate::ffi_items::FfiItems;
use crate::{BoxStr, MapInput, TestGenerator, VolatileItemKind, cdecl};

/// An error that occurs during translation, detailing cause and location.
#[derive(Debug, Error)]
pub struct TranslationError {
    #[source]
    kind: TranslationErrorKind,
    source: String,
    span: BoxStr,
}

impl TranslationError {
    /// Create a new translation error.
    pub(crate) fn new(kind: TranslationErrorKind, source: &str, span: Span) -> Self {
        Self {
            kind,
            source: source.to_string(),
            span: format!(
                "{fname}:{line}:{col}",
                // FIXME(ctest): Not yet stable, see:
                // https://github.com/dtolnay/proc-macro2/issues/503
                // fname = span.file(),
                fname = "<unknown file>",
                line = span.start().line,
                col = span.start().column,
            )
            .into(),
        }
    }
}

impl From<TranslationError> for askama::Error {
    fn from(err: TranslationError) -> Self {
        askama::Error::Custom(err.into())
    }
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: `{}` at {}", self.kind, self.source, self.span)
    }
}

/// Errors that can occur during the translation of a type.
#[derive(Debug, Error, PartialEq, Eq)]
pub(crate) enum TranslationErrorKind {
    /// The provided type is unknown or unrecognized.
    #[error("unsupported type")]
    UnsupportedType,

    /// A reference to a non-primitive type was encountered, which is not supported.
    #[error("references to non-primitive types are not allowed")]
    NonPrimitiveReference,

    /// Variadic functions or parameters were found, which cannot be handled.
    #[error("variadics cannot be translated")]
    HasVariadics,

    /// Lifetimes were found in the type or function signature, which are not supported.
    #[error("lifetimes cannot be translated")]
    HasLifetimes,

    /// A type that is not ffi compatible was found.
    #[error(
        "this type is not guaranteed to have a C compatible layout. See improper_ctypes_definitions lint"
    )]
    NotFfiCompatible,

    /// An array or function was attempted to be returned by a function.
    #[error("invalid return type")]
    InvalidReturn,
}

#[derive(Clone)]
/// A Rust to C/Cxx translator.
pub(crate) struct Translator<'a> {
    ffi_items: &'a FfiItems,
    generator: &'a TestGenerator,
}

impl<'a> Translator<'a> {
    /// Create a new translator.
    pub(crate) fn new(ffi_items: &'a FfiItems, generator: &'a TestGenerator) -> Self {
        Self {
            ffi_items,
            generator,
        }
    }

    /// Translate a Rust type into its equivalent C type.
    pub(crate) fn translate_type(&self, ty: &syn::Type) -> Result<cdecl::CTy, TranslationError> {
        match ty {
            syn::Type::Ptr(ptr) => self.translate_ptr(ptr),
            syn::Type::Path(path) => self.translate_path(path),
            syn::Type::Tuple(tuple) if tuple.elems.is_empty() => {
                Ok(cdecl::named("void", Constness::Mut))
            }
            syn::Type::Array(array) => self.translate_array(array),
            syn::Type::Reference(reference) => self.translate_reference(reference),
            syn::Type::BareFn(function) => self.translate_bare_fn(function, ""),
            syn::Type::Never(_) => Ok(cdecl::named("void", Constness::Mut)),
            syn::Type::Slice(slice) => Err(TranslationError::new(
                TranslationErrorKind::NotFfiCompatible,
                &slice.to_token_stream().to_string(),
                slice.span(),
            )),
            syn::Type::Paren(paren) => self.translate_type(&paren.elem),
            syn::Type::Group(group) => self.translate_type(&group.elem),
            ty => Err(TranslationError::new(
                TranslationErrorKind::UnsupportedType,
                &ty.to_token_stream().to_string(),
                ty.span(),
            )),
        }
    }

    /// Translate a Rust reference to its C equivalent.
    fn translate_reference(
        &self,
        reference: &syn::TypeReference,
    ) -> Result<cdecl::CTy, TranslationError> {
        match reference.elem.deref() {
            syn::Type::Path(path) => {
                let last_segment = path.path.segments.last().unwrap();
                let ident = last_segment.ident.to_string();

                match ident.as_str() {
                    "str" => {
                        // &str is not ABI safe and should not be supported.
                        Err(TranslationError::new(
                            TranslationErrorKind::NotFfiCompatible,
                            "&str",
                            path.span(),
                        ))
                    }
                    c if is_rust_primitive(c) => {
                        let type_name = translate_primitive_type(&last_segment.ident.to_string());
                        Ok(ptr_with_inner(
                            cdecl::named(&type_name, Constness::Mut),
                            reference.mutability,
                        ))
                    }
                    _ => Err(TranslationError::new(
                        TranslationErrorKind::NonPrimitiveReference,
                        &ident,
                        path.span(),
                    )),
                }
            }
            syn::Type::Reference(_)
            | syn::Type::Ptr(_)
            | syn::Type::Array(_)
            | syn::Type::BareFn(_) => {
                let ty = self.translate_type(reference.elem.deref())?;
                Ok(ptr_with_inner(ty, reference.mutability))
            }

            _ => Err(TranslationError::new(
                TranslationErrorKind::UnsupportedType,
                &reference.elem.to_token_stream().to_string(),
                reference.elem.span(),
            )),
        }
    }

    /// Translate a Rust function pointer type to its C equivalent.
    pub(crate) fn translate_bare_fn(
        &self,
        function: &syn::TypeBareFn,
        name: &str,
    ) -> Result<cdecl::CTy, TranslationError> {
        if function.lifetimes.is_some() {
            return Err(TranslationError::new(
                TranslationErrorKind::HasLifetimes,
                &function.to_token_stream().to_string(),
                function.span(),
            ));
        }
        if function.variadic.is_some() {
            return Err(TranslationError::new(
                TranslationErrorKind::HasVariadics,
                &function.to_token_stream().to_string(),
                function.span(),
            ));
        }

        // Checking for volatility and array arguments requires knowing the name of the function.
        let func = self
            .ffi_items
            .foreign_functions
            .iter()
            .find(|f| f.ident == name.into());

        let mut parameters = function
            .inputs
            .iter()
            .enumerate()
            .map(|(i, arg)| {
                // Apply array arg transformation.
                let is_array_arg = func
                    .and_then(|func| {
                        self.generator
                            .array_arg
                            .as_ref()
                            .map(|f| f(func.clone(), func.parameters[i].clone()))
                    })
                    .unwrap_or(false);

                // Remove the right most pointer type on the C side to reduce indirection.
                let mut cty = self.translate_type(&arg.ty)?;
                if is_array_arg {
                    if let cdecl::CTy::Ptr { ty, .. } = cty {
                        cty = ty.deref().clone();
                    }
                }

                // Apply volatile parameter transformation.
                let is_volatile = func.is_some_and(|func| {
                    let param = Box::new(func.parameters[i].clone());
                    self.generator
                        .volatile_items
                        .iter()
                        .any(|f| f(VolatileItemKind::FnArgument(func.clone(), param.clone())))
                });

                // The volatile keyword is only applied to the inner most named type.
                let mut current = &mut cty;
                while let cdecl::CTy::Ptr { ty, .. } = current {
                    current = ty;
                }
                if let cdecl::CTy::Named { qual, .. } = current {
                    qual.volatile = is_volatile;
                }

                Ok(cty)
            })
            .collect::<Result<Vec<_>, TranslationError>>()?;

        let mut return_type = match &function.output {
            syn::ReturnType::Default => cdecl::named("void", Constness::Mut),
            syn::ReturnType::Type(_, ty) => self.translate_type(ty)?,
        };

        // Apply volatile return type transformation.
        let is_volatile = func.is_some_and(|func| {
            self.generator
                .volatile_items
                .iter()
                .any(|f| f(VolatileItemKind::FnReturnType(func.clone())))
        });
        // The volatile return type must be applied to the inner most type (left most).
        let mut current = &mut return_type;
        while let cdecl::CTy::Ptr { ty, .. } = current {
            current = ty;
        }
        if let cdecl::CTy::Named { qual, .. } = current {
            qual.volatile = is_volatile;
        }

        if parameters.is_empty() {
            parameters.push(cdecl::named("void", Constness::Mut));
        }

        Ok(cdecl::func_ptr(parameters, return_type))
    }

    /// Translate a Rust path into its C equivalent.
    fn translate_path(&self, path: &syn::TypePath) -> Result<cdecl::CTy, TranslationError> {
        let last = path.path.segments.last().unwrap();
        if let syn::PathArguments::AngleBracketed(args) = &last.arguments {
            if let syn::GenericArgument::Type(inner_ty) = args.args.first().unwrap() {
                // Option<T> is ONLY ffi-safe if it contains a function pointer, or a reference.
                match inner_ty {
                    syn::Type::Reference(_) | syn::Type::BareFn(_) => {
                        return self.translate_type(inner_ty);
                    }
                    _ => {
                        return Err(TranslationError::new(
                            TranslationErrorKind::NotFfiCompatible,
                            &path.to_token_stream().to_string(),
                            inner_ty.span(),
                        ));
                    }
                }
            }
        }
        let name = last.ident.to_string();
        let item = if self.ffi_items.contains_struct(&name) {
            MapInput::StructType(&name)
        } else if self.ffi_items.contains_union(&name) {
            MapInput::UnionType(&name)
        } else {
            MapInput::Type(&name)
        };

        Ok(cdecl::named(
            &self.generator.rty_to_cty(item),
            Constness::Mut,
        ))
    }

    /// Translate a Rust array declaration into its C equivalent.
    fn translate_array(&self, array: &syn::TypeArray) -> Result<cdecl::CTy, TranslationError> {
        Ok(cdecl::array(
            self.translate_type(array.elem.deref())?,
            Some(&translate_expr(&array.len)),
        ))
    }

    /// Translate a Rust pointer into its equivalent C pointer.
    fn translate_ptr(&self, ptr: &syn::TypePtr) -> Result<cdecl::CTy, TranslationError> {
        let modifier = translate_mut(ptr.mutability);

        match ptr.elem.deref() {
            // A pointer to an array in Rust gets translated into an array in C.
            syn::Type::Array(array_ty) => {
                let mut array = self.translate_array(array_ty)?;
                // We assume that *const array means that the const applies to the inner element.
                // Here we try to get a mutable reference to the array element itself (if it can
                // have a qualifier)
                if let cdecl::CTy::Array { ty, .. } = &mut array
                    && let cdecl::CTy::Named { qual, .. } | cdecl::CTy::Ptr { qual, .. } =
                        ty.deref_mut()
                {
                    qual.constness = translate_mut(ptr.mutability);
                }
                Ok(array)
            }

            inner_ty => {
                let ty = self.translate_type(inner_ty)?;
                // Translate a pointer with inner constness as is.
                let mut ptr_ty = ptr_with_inner(ty.clone(), ptr.mutability);

                // FIXME(ctest): Handling of double pointer arrays by ctest is strange.
                // This is probably a bad idea.
                // A C array is made from a ptr to an array, so this actually matches a double ptr
                // to an array.
                if matches!(ty, cdecl::CTy::Array { .. }) {
                    // In this special case, change the constness for no reason at all.
                    if let cdecl::CTy::Ptr { qual, ty } = &mut ptr_ty {
                        qual.constness = modifier;

                        if let cdecl::CTy::Array { ty, .. } = ty.deref_mut()
                            && let cdecl::CTy::Named { qual, .. } | cdecl::CTy::Ptr { qual, .. } =
                                ty.deref_mut()
                        {
                            qual.constness = Constness::Mut;
                        }
                    }
                }

                Ok(ptr_ty)
            }
        }
    }

    /// Determine whether a C type is a signed type.
    ///
    /// For primitive types it checks against a known list of signed types, but for aliases
    /// which are the only thing other than primitives that can be signed, it recursively checks
    /// the underlying type of the alias.
    pub(crate) fn is_signed(&self, ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Path(path) => {
                let ident = path.path.segments.last().unwrap().ident.clone();
                if let Some(aliased) = self.ffi_items.aliases().iter().find(|a| ident == a.ident())
                {
                    return self.is_signed(&aliased.ty);
                }
                match translate_primitive_type(&ident.to_string()).as_str() {
                    "char" | "short" | "long" | "long long" | "size_t" | "ssize_t" => true,
                    s => {
                        s.starts_with("int")
                            || s.starts_with("uint") | s.starts_with("signed ")
                            || s.starts_with("unsigned ")
                    }
                }
            }
            _ => false,
        }
    }
}

/// Translate mutability from Rust to C.
fn translate_mut(mutability: Option<syn::Token![mut]>) -> Constness {
    mutability
        .map(|_| Constness::Mut)
        .unwrap_or(Constness::Const)
}

/// Translate a Rust primitive type into its C equivalent.
pub(crate) fn translate_primitive_type(ty: &str) -> String {
    match ty {
        "usize" => "size_t".to_string(),
        "isize" => "ssize_t".to_string(),
        "u8" => "uint8_t".to_string(),
        "u16" => "uint16_t".to_string(),
        "u32" => "uint32_t".to_string(),
        "u64" => "uint64_t".to_string(),
        "u128" => "unsigned __int128".to_string(),
        "i8" => "int8_t".to_string(),
        "i16" => "int16_t".to_string(),
        "i32" => "int32_t".to_string(),
        "i64" => "int64_t".to_string(),
        "i128" => "__int128".to_string(),
        "f32" => "float".to_string(),
        "f64" => "double".to_string(),
        "()" => "void".to_string(),

        "c_longdouble" | "c_long_double" => "long double".to_string(),
        ty if ty.starts_with("c_") => {
            let ty = &ty[2..].replace("long", " long");
            match ty.as_str() {
                "short" => "short".to_string(),
                s if s.starts_with('u') => format!("unsigned {}", &s[1..]),
                s if s.starts_with('s') => format!("signed {}", &s[1..]),
                s => s.to_string(),
            }
        }
        // Pass typedefs as is.
        s => s.to_string(),
    }
}

/// Construct a CTy and modify the constness of the inner type.
///
/// Basically, `syn` always gives us the `constness` of the inner type of a pointer.
/// However `cdecl::ptr` wants the `constness` of the pointer. So we just modify
/// the way it is built so that `cdecl::ptr` takes the `constness` of the inner type.
pub(crate) fn ptr_with_inner(
    inner: cdecl::CTy,
    mutability: Option<syn::Token![mut]>,
) -> cdecl::CTy {
    let constness = translate_mut(mutability);
    let mut ty = Box::new(inner);
    match ty.deref_mut() {
        cdecl::CTy::Named { name: _, qual } => qual.constness = constness,
        cdecl::CTy::Ptr { ty: _, qual } => qual.constness = constness,
        _ => (),
    }
    cdecl::CTy::Ptr {
        ty,
        qual: cdecl::Qual {
            constness: Constness::Mut,
            volatile: false,
            restrict: false,
        },
    }
}

/// Translate a simple Rust expression to C.
///
/// This function will just pass the expression as is in most cases.
pub(crate) fn translate_expr(expr: &syn::Expr) -> String {
    match expr {
        syn::Expr::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
        syn::Expr::Cast(c) => translate_expr(c.expr.deref()),
        expr => expr.to_token_stream().to_string(),
    }
}

/// Return whether a type is a Rust primitive type.
fn is_rust_primitive(ty: &str) -> bool {
    let rustc_types = [
        "usize", "u8", "u16", "u32", "u64", "u128", "isize", "i8", "i16", "i32", "i64", "i128",
        "f32", "f64",
    ];
    ty.starts_with("c_") || rustc_types.contains(&ty)
}

/// Translate ABI of a rust extern function to its C equivalent.
#[expect(unused)]
pub(crate) fn translate_abi(abi: &syn::Abi, target: &str) -> Option<&'static str> {
    let abi_name = abi.name.as_ref().map(|lit| lit.value());

    match abi_name.as_deref() {
        Some("stdcall") => "__stdcall ".into(),
        Some("system") if target.contains("i686-pc-windows") => "__stdcall ".into(),
        Some("C") | Some("system") | None => None,
        Some(a) => panic!("unknown ABI: {a}"),
    }
}
