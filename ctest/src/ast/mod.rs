mod constant;
mod field;
mod function;
mod module;
mod parameter;
mod static_variable;
mod structure;
mod type_alias;
mod union;

use std::fmt;

pub use constant::Const;
pub use field::Field;
pub use function::Fn;
pub use module::Module;
pub use parameter::Parameter;
use quote::ToTokens;
pub use static_variable::Static;
pub use structure::Struct;
use syn::punctuated::Punctuated;
pub use type_alias::Type;
pub use union::Union;

use crate::MapInput;

/// Transforms an item's absolute path to use `_` as path separator instead of
/// `::`.
pub(crate) fn escape_item_path<'a>(item: impl Into<MapInput<'a>>) -> String {
    let path = match item.into() {
        MapInput::Struct(s) => s.path(),
        MapInput::Union(u) => u.path(),
        MapInput::Fn(f) => f.path(),
        MapInput::Alias(a) => a.path(),
        MapInput::Const(c) => c.path(),
        MapInput::Static(s) => s.path(),

        _ => unimplemented!("other MapInput data constructors do not represent rust items"),
    };
    path.replace("::", "_")
}

/// Transforms a given item's type into an equivalent type with all inner type
/// paths longer than a single segment trimmed to the last segment.
pub(crate) fn escape_item_type<'a>(item: impl Into<MapInput<'a>>) -> String {
    fn recurrence(base: &syn::Type) -> String {
        match base {
            syn::Type::Array(type_array @ syn::TypeArray { elem, .. }) => {
                let mut out = type_array.clone();
                let inner = recurrence(elem);
                out.elem = syn::parse_quote! { #inner };
                out.into_token_stream().to_string()
            }
            syn::Type::FnPtr(type_fn_ptr @ syn::TypeFnPtr { inputs, .. }) => {
                let mut out = type_fn_ptr.clone();
                let new_inputs = inputs
                    .iter()
                    .map(|syn::NamedArg { ty, .. }| ty)
                    .map(recurrence)
                    .fold(Punctuated::new(), |mut new_elems, elem| {
                        (new_elems.push(syn::parse_quote! { #elem }), new_elems).1
                    });
                out.inputs = new_inputs;
                out.into_token_stream().to_string()
            }
            syn::Type::Ptr(type_ptr @ syn::TypePtr { elem, .. }) => {
                let mut out = type_ptr.clone();
                let inner = recurrence(elem);
                out.elem = syn::parse_quote! { #inner };
                out.into_token_stream().to_string()
            }
            syn::Type::Reference(type_reference @ syn::TypeReference { elem, .. }) => {
                let mut out = type_reference.clone();
                let inner = recurrence(elem);
                out.elem = syn::parse_quote! { #inner };
                out.into_token_stream().to_string()
            }
            syn::Type::Slice(type_slice @ syn::TypeSlice { elem, .. }) => {
                let mut out = type_slice.clone();
                let inner = recurrence(elem);
                out.elem = syn::parse_quote! { #inner };
                out.into_token_stream().to_string()
            }
            syn::Type::Tuple(type_tuple @ syn::TypeTuple { elems, .. }) => {
                let mut out = type_tuple.clone();
                let new_elems =
                    elems
                        .iter()
                        .map(recurrence)
                        .fold(Punctuated::new(), |mut new_elems, elem| {
                            (new_elems.push(syn::parse_quote! { #elem }), new_elems).1
                        });
                out.elems = new_elems;
                out.into_token_stream().to_string()
            }

            syn::Type::Path(syn::TypePath {
                path: syn::Path { segments, .. },
                ..
            }) => segments
                .last()
                .map(ToTokens::to_token_stream)
                .map(|ty| ty.to_string())
                .expect(
                    "parsed types always have at least one segment in their \
                     paths",
                ),

            _ => unimplemented!(
                "other types include either not ffi-safe types like trait \
                 objects or types that simply cannot occur within any one of \
                 type aliases, constants or statics (the above considered \
                 items)"
            ),
        }
    }
    let base = match item.into() {
        MapInput::Alias(a) => &a.ty,
        MapInput::Const(c) => &c.ty,
        MapInput::Static(s) => &s.ty,

        _ => unimplemented!(
            "other MapInput data constructors do not represent instances of \
             rust items and thus have no types for they are themselves types"
        ),
    };
    recurrence(base)
}

/// The ABI as defined by the extern block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Abi {
    /// The C ABI.
    C,
    /// The Rust ABI.
    Rust,
    /// Any other ABI.
    Other(String),
}

impl From<&str> for Abi {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "c" => Abi::C,
            "rust" => Abi::Rust,
            s => Abi::Other(s.to_string()),
        }
    }
}

impl fmt::Display for Abi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Abi::C => write!(f, "C"),
            Abi::Rust => write!(f, "Rust"),
            Abi::Other(s) => write!(f, "{s}"),
        }
    }
}
