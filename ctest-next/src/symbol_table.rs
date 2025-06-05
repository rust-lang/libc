use std::collections::HashMap;

use quote::ToTokens;
use syn::visit::Visit;

use crate::ir::{Constant, Field, Function, Parameter, Static, Struct, TypeAlias, Union};

type Abi = String;

#[derive(Debug)]
pub struct SymbolTable {
    aliases: Vec<TypeAlias>,
    structs: Vec<Struct>,
    unions: Vec<Union>,
    constants: Vec<Constant>,
    foreign_functions: HashMap<Abi, Vec<Function>>,
    foreign_statics: HashMap<Abi, Vec<Static>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
            structs: Vec::new(),
            unions: Vec::new(),
            constants: Vec::new(),
            foreign_functions: HashMap::new(),
            foreign_statics: HashMap::new(),
        }
    }
}

fn collect_fields(fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>) -> Vec<Field> {
    fields
        .iter()
        .map(|field| Field::new(field.ident.clone(), field.ty.to_token_stream()))
        .collect()
}

fn visit_foreign_item_fn(table: &mut SymbolTable, i: &syn::ForeignItemFn, abi: &str) {
    let ident = i.sig.ident.clone();
    let parameters = i
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(arg) => {
                Parameter::new(arg.pat.to_token_stream(), arg.ty.to_token_stream())
            }
            syn::FnArg::Receiver(_) => {
                unreachable!("Foreign functions can't have self/receiver parameters.")
            }
        })
        .collect::<Vec<_>>();
    let return_value = match &i.sig.output {
        syn::ReturnType::Default => "()".to_token_stream(),
        syn::ReturnType::Type(_, ty) => ty.to_token_stream(),
    };

    table
        .foreign_functions
        .entry(abi.to_string())
        .or_default()
        .push(Function::new(ident, parameters, return_value));
}

fn visit_foreign_item_static(table: &mut SymbolTable, i: &syn::ForeignItemStatic, abi: &str) {
    let ident = i.ident.clone();
    let ty = i.ty.to_token_stream();

    table
        .foreign_statics
        .entry(abi.to_string())
        .or_default()
        .push(Static::new(ident, ty));
}

impl<'ast> Visit<'ast> for SymbolTable {
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let ty = i.ty.to_token_stream();
        let ident = i.ident.clone();

        self.aliases.push(TypeAlias::new(ident, ty));
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let ident = i.ident.clone();
        let fields = match &i.fields {
            syn::Fields::Named(fields) => collect_fields(&fields.named),
            syn::Fields::Unnamed(fields) => collect_fields(&fields.unnamed),
            syn::Fields::Unit => Vec::new(),
        };

        self.structs.push(Struct::new(ident, fields));
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let ident = i.ident.clone();
        let fields = collect_fields(&i.fields.named);

        self.unions.push(Union::new(ident, fields));
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        let ident = i.ident.clone();
        let ty = i.ty.to_token_stream();
        let value = i.expr.to_token_stream();

        self.constants.push(Constant::new(ident, ty, value));
    }

    fn visit_item_foreign_mod(&mut self, i: &'ast syn::ItemForeignMod) {
        // Because we require the ABI we can't directly visit the foreign functions/statics.
        let abi = i
            .abi
            .name
            .clone()
            .map(|s| s.value())
            .unwrap_or_else(|| "C".to_string());

        for item in &i.items {
            match item {
                syn::ForeignItem::Fn(function) => visit_foreign_item_fn(self, &function, &abi),
                syn::ForeignItem::Static(static_variable) => {
                    visit_foreign_item_static(self, &static_variable, &abi)
                }
                _ => (),
            }
        }
    }
}
