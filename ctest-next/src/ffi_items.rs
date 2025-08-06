use std::ops::Deref;

use syn::punctuated::Punctuated;
use syn::visit::Visit;

use crate::{Abi, BoxStr, Const, Field, Fn, Parameter, Static, Struct, Type, Union};

/// Represents a collected set of top-level Rust items relevant to FFI generation or analysis.
///
/// Includes foreign functions/statics, type aliases, structs, unions, and constants.
#[derive(Default, Clone, Debug)]
pub(crate) struct FfiItems {
    pub(crate) aliases: Vec<Type>,
    pub(crate) structs: Vec<Struct>,
    pub(crate) unions: Vec<Union>,
    pub(crate) constants: Vec<Const>,
    pub(crate) foreign_functions: Vec<Fn>,
    pub(crate) foreign_statics: Vec<Static>,
}

impl FfiItems {
    /// Creates a new blank FfiItems.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return whether the type has parsed a struct with the given identifier.
    pub(crate) fn contains_struct(&self, ident: &str) -> bool {
        self.structs()
            .iter()
            .any(|structure| structure.ident() == ident)
    }

    /// Return whether the type has parsed a union with the given identifier.
    pub(crate) fn contains_union(&self, ident: &str) -> bool {
        self.unions().iter().any(|union| union.ident() == ident)
    }

    /// Return a list of all type aliases found.
    pub(crate) fn aliases(&self) -> &Vec<Type> {
        &self.aliases
    }

    /// Return a list of all structs found.
    pub(crate) fn structs(&self) -> &Vec<Struct> {
        &self.structs
    }

    /// Return a list of all unions found.
    pub(crate) fn unions(&self) -> &Vec<Union> {
        &self.unions
    }

    /// Return a list of all constants found.
    pub(crate) fn constants(&self) -> &Vec<Const> {
        &self.constants
    }

    /// Return a list of all foreign functions found mapped by their ABI.
    pub(crate) fn foreign_functions(&self) -> &Vec<Fn> {
        &self.foreign_functions
    }

    /// Return a list of all foreign statics found mapped by their ABI.
    #[cfg_attr(not(test), expect(unused))]
    pub(crate) fn foreign_statics(&self) -> &Vec<Static> {
        &self.foreign_statics
    }
}

/// Determine whether an item is visible to other crates.
///
/// This function assumes that if the visibility is restricted then it is not
/// meant to be accessed.
fn is_visible(vis: &syn::Visibility) -> bool {
    match vis {
        syn::Visibility::Public(_) => true,
        syn::Visibility::Inherited | syn::Visibility::Restricted(_) => false,
    }
}

/// Collect fields in a syn grammar into ctest's equivalent structure.
fn collect_fields(fields: &Punctuated<syn::Field, syn::Token![,]>) -> Vec<Field> {
    fields
        .iter()
        .filter_map(|field| {
            field.ident.as_ref().map(|ident| Field {
                public: is_visible(&field.vis),
                ident: ident.to_string().into_boxed_str(),
                ty: field.ty.clone(),
            })
        })
        .collect()
}

fn extract_single_link_name(attrs: &[syn::Attribute]) -> Option<BoxStr> {
    let mut link_name_iter = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("link_name"));

    let link_name = link_name_iter.next()?;
    if let Some(attr) = link_name_iter.next() {
        panic!("multiple `#[link_name = ...]` attributes found: {attr:?}");
    }

    if let syn::Meta::NameValue(nv) = &link_name.meta
        && let syn::Expr::Lit(expr_lit) = &nv.value
        && let syn::Lit::Str(lit_str) = &expr_lit.lit
    {
        return Some(lit_str.value().into_boxed_str());
    }

    panic!("unrecognized `link_name` syntax: {link_name:?}");
}

fn visit_foreign_item_fn(table: &mut FfiItems, i: &syn::ForeignItemFn, abi: &Abi) {
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let ident = i.sig.ident.to_string().into_boxed_str();
    let parameters = i
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(arg) => Parameter {
                ident: match arg.pat.deref() {
                    syn::Pat::Ident(i) => i.ident.to_string().into_boxed_str(),
                    _ => {
                        unimplemented!("Foreign functions are unlikely to have any other pattern.")
                    }
                },
                ty: arg.ty.deref().clone(),
            },
            syn::FnArg::Receiver(_) => {
                unreachable!("Foreign functions can't have self/receiver parameters.")
            }
        })
        .collect::<Vec<_>>();
    let return_type = match &i.sig.output {
        syn::ReturnType::Default => None,
        syn::ReturnType::Type(_, ty) => Some(ty.deref().clone()),
    };
    let link_name = extract_single_link_name(&i.attrs);

    table.foreign_functions.push(Fn {
        public,
        abi,
        ident,
        link_name,
        parameters,
        return_type,
    });
}

fn visit_foreign_item_static(table: &mut FfiItems, i: &syn::ForeignItemStatic, abi: &Abi) {
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let ident = i.ident.to_string().into_boxed_str();
    let ty = i.ty.deref().clone();
    let link_name = extract_single_link_name(&i.attrs);

    table.foreign_statics.push(Static {
        public,
        abi,
        ident,
        link_name,
        ty,
    });
}

impl<'ast> Visit<'ast> for FfiItems {
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let public = is_visible(&i.vis);
        let ty = i.ty.deref().clone();
        let ident = i.ident.to_string().into_boxed_str();

        self.aliases.push(Type { public, ident, ty });
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let public = is_visible(&i.vis);
        let ident = i.ident.to_string().into_boxed_str();
        let fields = match &i.fields {
            syn::Fields::Named(fields) => collect_fields(&fields.named),
            syn::Fields::Unnamed(fields) => collect_fields(&fields.unnamed),
            syn::Fields::Unit => Vec::new(),
        };

        self.structs.push(Struct {
            public,
            ident,
            fields,
        });
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let public = is_visible(&i.vis);
        let ident = i.ident.to_string().into_boxed_str();
        let fields = collect_fields(&i.fields.named);

        self.unions.push(Union {
            public,
            ident,
            fields,
        });
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        let public = is_visible(&i.vis);
        let ident = i.ident.to_string().into_boxed_str();
        let ty = i.ty.deref().clone();
        let expr = i.expr.deref().clone();

        self.constants.push(Const {
            public,
            ident,
            ty,
            expr,
        });
    }

    fn visit_item_foreign_mod(&mut self, i: &'ast syn::ItemForeignMod) {
        // Because we need to store the ABI we can't directly visit the foreign
        // functions/statics.

        // Since this is an extern block, assume extern "C" by default.
        let abi = i
            .abi
            .name
            .clone()
            .map(|s| Abi::from(s.value().as_str()))
            .unwrap_or_else(|| Abi::C);

        for item in &i.items {
            match item {
                syn::ForeignItem::Fn(function) => visit_foreign_item_fn(self, function, &abi),
                syn::ForeignItem::Static(static_variable) => {
                    visit_foreign_item_static(self, static_variable, &abi)
                }
                _ => (),
            }
        }
    }
}
