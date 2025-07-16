//! Translation of Rust types to C for test generation.
//!
//! Simple to semi complex types are supported only.

use std::fmt;
use std::ops::Deref;

use proc_macro2::Span;
use quote::ToTokens;
use syn::spanned::Spanned;
use thiserror::Error;

use crate::ffi_items::FfiItems;

/// An error that occurs during translation, detailing cause and location.
#[derive(Debug, Error)]
pub struct TranslationError {
    #[source]
    kind: TranslationErrorKind,
    source: String,
    #[expect(unused)]
    span: Span,
}

impl TranslationError {
    /// Create a new translation error.
    pub(crate) fn new(kind: TranslationErrorKind, source: &str, span: Span) -> Self {
        Self {
            kind,
            source: source.to_string(),
            span,
        }
    }
}

impl fmt::Display for TranslationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: `{}`",
            self.kind,
            self.source,
            // FIXME(ctest): Not yet stable, see:
            // https://github.com/dtolnay/proc-macro2/issues/503
            // self.span.file(),
            // self.span.start().line,
            // self.span.start().column,
        )
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
    #[error("this type is not guaranteed to have a C compatible layout. See improper_ctypes_definitions lint")]
    NotFfiCompatible,
}

#[derive(Clone, Debug, Default)]
/// A Rust to C/Cxx translator.
pub(crate) struct Translator {}

impl Translator {
    /// Create a new translator.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Translate mutability from Rust to C.
    fn translate_mut(&self, mutability: Option<syn::Token![mut]>) -> String {
        mutability.map(|_| "").unwrap_or("const").to_string()
    }

    /// Translate a Rust type into its equivalent C type.
    pub(crate) fn translate_type(&self, ty: &syn::Type) -> Result<String, TranslationError> {
        match ty {
            syn::Type::Ptr(ptr) => self.translate_ptr(ptr),
            syn::Type::Path(path) => self.translate_path(path),
            syn::Type::Tuple(tuple) if tuple.elems.is_empty() => Ok("void".to_string()),
            syn::Type::Array(array) => self.translate_array(array),
            syn::Type::Reference(reference) => self.translate_reference(reference),
            syn::Type::BareFn(function) => self.translate_bare_fn(function),
            syn::Type::Never(_) => Ok("void".to_string()),
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
    ) -> Result<String, TranslationError> {
        let modifier = self.translate_mut(reference.mutability);

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
                        let base_type = self.translate_primitive_type(&last_segment.ident);
                        Ok(format!("{modifier} {base_type}*").trim().to_string())
                    }
                    _ => Err(TranslationError::new(
                        TranslationErrorKind::NonPrimitiveReference,
                        &ident,
                        path.span(),
                    )),
                }
            }
            syn::Type::Array(arr) => {
                let len = translate_expr(&arr.len);
                let ty = self.translate_type(arr.elem.deref())?;
                let inner_type = format!("{ty} (*) [{len}]");
                Ok(inner_type
                    .replacen("(*)", &format!("(*{modifier})"), 1)
                    .trim()
                    .to_string())
            }
            syn::Type::BareFn(_) => {
                let inner_type = self.translate_type(reference.elem.deref())?;
                Ok(inner_type
                    .replacen("(*)", &format!("(*{modifier})"), 1)
                    .trim()
                    .to_string())
            }
            syn::Type::Reference(_) | syn::Type::Ptr(_) => {
                let inner_type = self.translate_type(reference.elem.deref())?;
                if inner_type.contains("(*)") {
                    Ok(inner_type
                        .replacen("(*)", &format!("(*{modifier})"), 1)
                        .trim()
                        .to_string())
                } else {
                    Ok(format!("{inner_type} {modifier}*").trim().to_string())
                }
            }
            _ => Err(TranslationError::new(
                TranslationErrorKind::UnsupportedType,
                &reference.elem.to_token_stream().to_string(),
                reference.elem.span(),
            )),
        }
    }

    /// Translate a Rust function pointer type to its C equivalent.
    fn translate_bare_fn(&self, function: &syn::TypeBareFn) -> Result<String, TranslationError> {
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

        let mut parameters = function
            .inputs
            .iter()
            .map(|arg| self.translate_type(&arg.ty))
            .collect::<Result<Vec<_>, TranslationError>>()?;

        let return_type = match &function.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, ty) => self.translate_type(ty)?,
        };

        if parameters.is_empty() {
            parameters.push("void".to_string());
        }

        if return_type.contains("(*)") {
            let params = parameters.join(", ");
            Ok(return_type.replacen("(*)", &format!("(*(*)({params}))"), 1))
        } else {
            Ok(format!("{return_type}(*)({})", parameters.join(", ")))
        }
    }

    /// Translate a Rust primitive type into its C equivalent.
    pub(crate) fn translate_primitive_type(&self, ty: &syn::Ident) -> String {
        match ty.to_string().as_str() {
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

    /// Translate a Rust path into its C equivalent.
    fn translate_path(&self, path: &syn::TypePath) -> Result<String, TranslationError> {
        let last = path.path.segments.last().unwrap();
        Ok(self.translate_primitive_type(&last.ident))
    }

    /// Translate a Rust array declaration into its C equivalent.
    fn translate_array(&self, array: &syn::TypeArray) -> Result<String, TranslationError> {
        Ok(format!(
            "{}[{}]",
            self.translate_type(array.elem.deref())?,
            translate_expr(&array.len)
        ))
    }

    /// Translate a Rust pointer into its equivalent C pointer.
    fn translate_ptr(&self, ptr: &syn::TypePtr) -> Result<String, TranslationError> {
        let modifier = self.translate_mut(ptr.mutability);
        let inner = ptr.elem.deref();

        match inner {
            syn::Type::BareFn(_) => {
                let inner_type = self.translate_type(ptr.elem.deref())?;
                Ok(inner_type
                    .replacen("(*)", &format!("(*{modifier})"), 1)
                    .trim()
                    .to_string())
            }
            syn::Type::Array(arr) => {
                let len = translate_expr(&arr.len);
                let ty = self.translate_type(arr.elem.deref())?;
                let inner_type = format!("{ty} (*) [{len}]");
                Ok(inner_type
                    .replacen("(*)", &format!("(*{modifier})"), 1)
                    .trim()
                    .to_string())
            }
            syn::Type::Reference(_) | syn::Type::Ptr(_) => {
                let inner_type = self.translate_type(ptr.elem.deref())?;
                if inner_type.contains("(*)") {
                    Ok(inner_type
                        .replacen("(*)", &format!("(*{modifier} *)"), 1)
                        .trim()
                        .to_string())
                } else {
                    Ok(format!("{inner_type} {modifier}*").trim().to_string())
                }
            }
            _ => {
                let inner_type = self.translate_type(inner)?;
                Ok(format!("{inner_type} {modifier}*"))
            }
        }
    }

    /// Partially translate a Rust bare function type into its equivalent C type.
    ///
    /// It returns the translated return type, translated argument types, and whether
    /// it is variadic as a tuple.
    pub(crate) fn translate_signature_partial(
        &self,
        signature: &syn::TypeBareFn,
    ) -> Result<(String, Vec<String>, bool), TranslationError> {
        let args = signature
            .inputs
            .iter()
            .map(|arg| self.translate_type(&arg.ty))
            .collect::<Result<Vec<_>, TranslationError>>()?;
        let return_type = match &signature.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, ty) => match ty.deref() {
                syn::Type::Never(_) => "void".to_string(),
                syn::Type::Tuple(tuple) if tuple.elems.is_empty() => "void".to_string(),
                _ => self.translate_type(ty.deref())?,
            },
        };
        Ok((return_type, args, signature.variadic.is_some()))
    }

    /// Determine whether a C type is a signed type.
    pub(crate) fn is_signed(&self, ffi_items: &FfiItems, ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Path(path) => {
                let ident = path.path.segments.last().unwrap().ident.clone();
                // The only thing other than a primitive that can be signed is an alias.
                if let Some(aliased) = ffi_items.aliases().iter().find(|a| ident == a.ident()) {
                    return self.is_signed(ffi_items, &aliased.ty);
                }
                match self.translate_primitive_type(&ident).as_str() {
                    "char" | "short" | "int" | "long" | "long long" | "int8_t" | "int16_t"
                    | "int32_t" | "int64_t" | "uint8_t" | "uint16_t" | "uint32_t" | "uint64_t"
                    | "size_t" | "ssize_t" => true,
                    s => s.starts_with("signed ") || s.starts_with("unsigned "),
                }
            }
            _ => false,
        }
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
