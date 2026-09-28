//! Conversion of Rust code to a simplified abstract syntax tree.

use std::borrow::Borrow;
use std::ops::{self, Deref};
use std::iter;
use std::ops::ControlFlow;

use quote::ToTokens;
use syn::Visibility;
use syn::punctuated::Punctuated;
use syn::UseTree;
use syn::visit::{
    self,
    Visit,
};

use crate::{
    Abi,
    BoxStr,
    Const,
    Field,
    Fn,
    Module,
    Parameter,
    Static,
    Struct,
    Type,
    Union,
};

enum GenericItem {
    Type(Type),
    Struct(Struct),
    Union(Union),
    Const(Const),
    Fn(Fn),
    Static(Static),
    Module(Module),
}

/// Represents a collected set of top-level Rust items relevant to FFI
/// generation or analysis.
///
/// Includes foreign functions/statics, type aliases, structs, unions, and
/// constants. Modules are collected as recursive `FfiItems`, and currently used
/// to narrow down tested items to those in a specific module.
#[derive(Clone, Debug)]
pub(crate) struct FfiItems {
    pub(crate) aliases: Vec<Type>,
    pub(crate) structs: Vec<Struct>,
    pub(crate) unions: Vec<Union>,
    pub(crate) constants: Vec<Const>,
    pub(crate) foreign_functions: Vec<Fn>,
    pub(crate) foreign_statics: Vec<Static>,
    pub(crate) uses: Vec<RefinedUse>,
    pub(crate) modules: Vec<Module>,

    /// This is used while recursing through parsed modules to gather absolute
    /// paths to them as identifiers for both the modules and the items within
    /// them.
    pub(crate) current_module: syn::Path,
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
    pub(crate) fn foreign_statics(&self) -> &Vec<Static> {
        &self.foreign_statics
    }

    /// Entry point to parse a [`syn::File`].
    ///
    /// This will also resolve `use`-trees such that the resulting `FfiItems`
    /// instance is always left with no reexports that are not sourced from
    /// third-party crates.
    pub(crate) fn visit_file(&mut self, file: &syn::File) {
        let syn::File { attrs, items: mod_items, .. } = file;
        let file_mod = syn::ItemMod {
            attrs: attrs.clone(),
            vis: syn::parse_quote! { pub },
            unsafety: None,
            mod_token: syn::token::Mod::default(),
            ident: syn::Ident::new("__ctest_root_mod", proc_macro2::Span::call_site()),
            content: Some((syn::token::Brace::default(), mod_items.clone())),
            semi: None,
        };
        self.visit_item_mod(&file_mod);
        let root_module = Module {
            public: bool::default(),
            ident: String::new().into_boxed_str(),
            path: syn::Path {
                leading_colon: None,
                segments: Punctuated::new(),
            },
            items: self.clone(),
        };
        *self = resolve_use_trees(root_module).items;
    }

    /// Searches for an item in the container, and returns an owned instance of
    /// that item.
    ///
    /// The generic item may be pattern-matched to find out specifically which
    /// item was found.
    fn search(&self, ident: syn::Ident) -> Option<GenericItem> {
        let aliases = self.aliases.clone().into_iter().map(GenericItem::Type);
        let structs = self.structs.clone().into_iter().map(GenericItem::Struct);
        let unions = self.unions.clone().into_iter().map(GenericItem::Union);
        let constants = self.constants.clone().into_iter().map(GenericItem::Const);
        let foreign_functions = self.foreign_functions.clone().into_iter().map(GenericItem::Fn);
        let foreign_statics = self.foreign_statics.clone().into_iter().map(GenericItem::Static);
        let mut items: Vec<_> = self.modules
            .clone()
            .into_iter()
            .map(GenericItem::Module)
            .chain(foreign_statics)
            .chain(foreign_functions)
            .chain(constants)
            .chain(unions)
            .chain(structs)
            .chain(aliases)
            .collect();
        let key_fn = |it: &GenericItem| match it {
            GenericItem::Type(t) => t.ident(),
            GenericItem::Struct(s) => s.ident(),
            GenericItem::Union(u) => u.ident(),
            GenericItem::Const(c) => c.ident(),
            GenericItem::Fn(f) => f.ident(),
            GenericItem::Static(s) => s.ident(),
            GenericItem::Module(m) => m.ident(),
        };
        items.sort_unstable_by_key(key_fn);
        let Ok(idx) = items.binary_search_by_key(&ident.to_string(), key_fn) else {
            return None;
        };
        items.swap_remove(idx).into()
    }
}

impl Default for FfiItems {
    fn default() -> Self {
        Self {
            aliases: Default::default(),
            structs: Default::default(),
            unions: Default::default(),
            constants: Default::default(),
            foreign_functions: Default::default(),
            foreign_statics: Default::default(),
            modules: Default::default(),
            uses: Default::default(),
            current_module: syn::Path {
                leading_colon: None,
                segments: Default::default(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct RefinedUsePath {
    ident: syn::Ident,
    tree: Box<RefinedUseTree>,
}

#[derive(Clone, Debug)]
pub(crate) enum RefinedUseTree {
    Path(RefinedUsePath),
    Name(syn::UseName),
    Rename(syn::UseRename),
    Glob,
}

#[derive(Clone, Debug)]
pub(crate) struct RefinedUse {
    is_public: bool,
    tree: RefinedUseTree,
}

/// Gets rid of group reexports in a `use` statement by flattenning them into a
/// set of individual reexports.
fn normalize_path(path: syn::UseTree) -> Vec<RefinedUseTree> {
    match path {
        UseTree::Name(n) => vec![RefinedUseTree::Name(n)],
        UseTree::Rename(r) => vec![RefinedUseTree::Rename(r)],
        UseTree::Glob(g) => vec![RefinedUseTree::Glob],

        UseTree::Path(syn::UsePath { ident, tree, .. }) => {
            normalize_path(*tree)
                .into_iter()
                .map(Box::new)
                .map(|tree| RefinedUseTree::Path(RefinedUsePath {
                    ident: ident.clone(),
                    tree
                }))
                .collect()
        }

        UseTree::Group(syn::UseGroup { items, .. }) => {
            items.into_iter().map(normalize_path).flatten().collect()
        }
    }
}

enum Resolution {
    Resolved { original_use: RefinedUse, items: FfiItems },
    Unresolved,
}

fn resolve_use_trees(root: Module) -> Module {
    let resolved_children = root.items.modules.into_iter().map(resolve_use_trees).collect();
    let root = {
        let items = FfiItems {
            modules: resolved_children,
            ..root.items
        };
        Module { items, ..root }
    };
    let (ControlFlow::Continue(root) | ControlFlow::Break(root)) =
        iter::repeat(()).try_fold(root, |root, _| {
            let resolved_uses = resolve_one(root.clone());
            let new_root = merge_module(root.clone(), resolved_uses);
            match root.items.uses.len() == new_root.items.uses.len() {
                true => ControlFlow::Break(new_root),
                false => ControlFlow::Continue(new_root)
            }
        });
    root
}

fn resolve_one(src: Module) -> Vec<Resolution> {
    src.items.uses
        .clone()
        .into_iter()
        .map(|u| (u.clone(), u, src.clone()))
        .map(|(ou, u, m)| resolve_use(ou, u, m))
        .collect()
}

fn resolve_use(original_use: RefinedUse, r#use: RefinedUse, state: Module) -> Resolution {
    match &r#use.tree {
        RefinedUseTree::Name(syn::UseName { ident }) => {
            macro_rules! single_item {
                ($field:ident: $it:ident) => {{
                    let items = FfiItems {
                        $field: vec![$it],
                        ..Default::default()
                    };
                    Resolution::Resolved {
                        original_use,
                        items,
                    }
                }};
            }
            match state.items.search(ident.clone()) {
                Some(GenericItem::Type(t)) => single_item!(aliases: t),
                Some(GenericItem::Struct(s)) => single_item!(structs: s),
                Some(GenericItem::Union(u)) => single_item!(unions: u),
                Some(GenericItem::Const(c)) => single_item!(constants: c),
                Some(GenericItem::Fn(f)) => single_item!(foreign_functions: f),
                Some(GenericItem::Static(s)) => single_item!(foreign_statics: s),
                Some(GenericItem::Module(m)) => single_item!(modules: m),
                None => Resolution::Unresolved,
            }
        }
        RefinedUseTree::Glob => Resolution::Resolved { original_use, items: state.items },
        RefinedUseTree::Rename(syn::UseRename { ident, rename, .. }) => {
            macro_rules! single_item {
                ($field:ident , $it:tt : $ty:tt) => {{
                    let mut new_path = $it.path;
                    new_path.pop();
                    new_path.push(rename);
                    let new_item = $ty {
                        ident: path_to_string(&new_path),
                        path: same_path,
                        ..$id,
                    };
                    Resolution::Resolved {
                        original_use,
                        items: FfiItems { $field: new_item, ..Default::default() },
                    }
                }};
            }
            match state.items.search(ident.clone()) {
                Some(GenericItem::Module(m)) => todo!(),
                Some(GenericItem::Type(t)) => single_item!(aliases, t: Type),
                Some(GenericItem::Struct(s)) => single_item!(structs, s: Struct),
                Some(GenericItem::Union(u)) => single_item!(unions, u: Union),
                Some(GenericItem::Const(c)) => single_item!(constants, c: Const),
                Some(GenericItem::Fn(f)) => single_item!(foreign_functions, f: Fn),
                Some(GenericItem::Static(s)) => single_item!(foreign_statics, s: Static),
                None => Resolution::Unresolved
            }
        }

        RefinedUseTree::Path(RefinedUsePath { ident, tree }) => {
            let new_use = RefinedUse {
                is_public: r#use.is_public,
                tree: *tree.clone(),
            };
            let new_state = match state.items.search(ident.clone()) {
                Some(GenericItem::Module(m)) => m,
                None => return Resolution::Unresolved,
                Some(_) => unreachable!(
                    "paths we parse in ctest include only modules, and not \
                     enum variants; update this in the future if we start \
                     supporting enum variants and thus start having imports \
                     that can span multiple segments of a path without \
                     strictly being nested modules"
                ),
            };
            resolve_use(original_use, new_use, new_state)
        }
    }
}

fn manipulate_path(f: impl ops::Fn(syn::Path) -> syn::Path + Clone, root: Module) -> Module {
    let new_root_path = f(root.path.clone());
    let new_root_cache = path_to_string(&new_root_path);
    macro_rules! map_items {
        ($field:ident: $ty:tt) => {{
            root.items.$field
                .clone()
                .into_iter()
                .map(|it| {
                    let new_path = f(it.path);
                    $ty { path: new_path, ..it }
                })
                .collect::<Vec<_>>()
        }};
    }
    let new_aliases = map_items!(aliases: Type);
    let new_structs = map_items!(structs: Struct);
    let new_unions = map_items!(unions: Union);
    let new_constants = map_items!(constants: Const);
    let new_foreign_functions = map_items!(foreign_functions: Fn);
    let new_foreign_statics = map_items!(foreign_statics: Static);
    let new_modules: Vec<_> = root.items.modules
        .clone()
        .into_iter()
        .map(|m| (f.clone(), m))
        .map(|(f, m)| manipulate_path(f, m))
        .collect();
    let new_items = FfiItems {
        aliases: new_aliases,
        structs: new_structs,
        unions: new_unions,
        constants: new_constants,
        foreign_functions: new_foreign_functions,
        foreign_statics: new_foreign_statics,
        modules: new_modules,
        ..Default::default()
    };
    Module {
        ident: new_root_cache,
        path: new_root_path,
        items: new_items,
        ..root
    }
}

fn merge_module(dst: Module, src: Vec<Resolution>) -> Module {
    todo!();
}

/// Appends a new module-local item to an absolute path that does *not* start
/// with `crate`.
///
/// This is used whenever we need to create paths for either items or modules.
fn append_path(base: impl Borrow<syn::Path>, new: impl Borrow<syn::Ident>) -> syn::Path {
    let base = base.borrow();
    let new = new.borrow();
    if base.segments.is_empty() {
        syn::parse_quote! { #new }
    } else {
        syn::parse_quote! { #base::#new }
    }
}

/// Returns a stringified `syn::Path` that is meant to match one-to-one the path
/// to the item from the crate root.
fn path_to_string(path: impl Borrow<syn::Path>) -> BoxStr {
    path.borrow()
        .into_token_stream()
        .to_string()
        .replace(|c: char| c.is_ascii_whitespace(), "")
        .into_boxed_str()
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
            field.ident.as_ref().map(|ident| {
                let ident = ident.to_string();
                Field {
                    public: is_visible(&field.vis),
                    ident: ident
                        .strip_prefix("r#")
                        .unwrap_or(&ident)
                        .to_string()
                        .into_boxed_str(),
                    ty: field.ty.clone(),
                }
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
    let path = append_path(&table.current_module, &i.sig.ident);
    let ident = path_to_string(&path);
    let parameters = i
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            syn::FnArg::Typed(arg) => Parameter {
                ident: match arg.pat.deref() {
                    syn::Pat::Ident(i) => i.ident.to_string().into_boxed_str(),
                    _ => {
                        unimplemented!(
                            "Foreign functions are unlikely to have any other \
                             pattern."
                        )
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
        path,
        link_name,
        parameters,
        return_type,
    });
}

fn visit_foreign_item_static(table: &mut FfiItems, i: &syn::ForeignItemStatic, abi: &Abi) {
    let public = is_visible(&i.vis);
    let abi = abi.clone();
    let path = append_path(&table.current_module, &i.ident);
    let ident = path_to_string(&path);
    let ty = i.ty.deref().clone();
    let link_name = extract_single_link_name(&i.attrs);

    table.foreign_statics.push(Static {
        public,
        abi,
        ident,
        path,
        link_name,
        ty,
    });
}

impl<'ast> Visit<'ast> for FfiItems {
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let public = is_visible(&i.vis);
        let path = append_path(&self.current_module, &i.ident);
        let ident = path_to_string(&path);
        let ty = i.ty.deref().clone();

        self.aliases.push(Type {
            public,
            ident,
            path,
            ty,
        });
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        let public = is_visible(&i.vis);
        let path = append_path(&self.current_module, &i.ident);
        let ident = path_to_string(&path);
        let fields = match &i.fields {
            syn::Fields::Named(fields) => collect_fields(&fields.named),
            syn::Fields::Unnamed(fields) => collect_fields(&fields.unnamed),
            syn::Fields::Unit => Vec::new(),
        };

        self.structs.push(Struct {
            public,
            ident,
            path,
            fields,
        });
    }

    fn visit_item_union(&mut self, i: &'ast syn::ItemUnion) {
        let public = is_visible(&i.vis);
        let path = append_path(&self.current_module, &i.ident);
        let ident = path_to_string(&path);
        let fields = collect_fields(&i.fields.named);

        self.unions.push(Union {
            public,
            ident,
            path,
            fields,
        });
    }

    fn visit_item_const(&mut self, i: &'ast syn::ItemConst) {
        let public = is_visible(&i.vis);
        let path = append_path(&self.current_module, &i.ident);
        let ident = path_to_string(&path);
        let ty = i.ty.deref().clone();

        self.constants.push(Const {
            public,
            ident,
            path,
            ty,
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
            .map_or(Abi::C, |s| Abi::from(s.value().as_str()));

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

    fn visit_item_mod(&mut self, i: &'ast syn::ItemMod) {
        let syn::ItemMod { vis, ident, content: Some((_, mod_items)), .. } = i else {
            unreachable!("this runs post cargo-expand, which inlines all modules");
        };
        let uses: Vec<_> = mod_items
            .iter()
            .cloned()
            .filter_map(|it| {
                if let syn::Item::Use(syn::ItemUse { vis, tree, .. }) = it {
                    normalize_path(tree)
                        .into_iter()
                        .map(move |tree| RefinedUse {
                            is_public: matches!(vis, syn::Visibility::Public(_)),
                            tree,
                        })
                        .into()
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        // [NOTE]: if the module is known to keep be the virtual module that we
        // create to process the items in the crate root, then we require not
        // creating a new module that will become a child to the current one,
        // but rather make all items in the module become the items of the
        // current module (the crate root.)
        if ident == "__ctest_root_mod" {
            visit::visit_item_mod(self, i);
            self.uses = uses;
        } else {
            let public = matches!(vis, Visibility::Public(_));
            let path = append_path(&self.current_module, ident);
            let ident = path_to_string(&path);
            let mut items = FfiItems::new();
            items.current_module = path.clone();
            visit::visit_item_mod(&mut items, i);
            items.uses = uses;
            self.modules.push(Module {
                public,
                ident,
                path,
                items,
            });
        }
    }
}

#[test]
fn tmp() {
    let source = r#"
use std::*;

mod test { use std::any::*; pub struct Foo; }

fn main() {
    use std::any::*;
}
    "#;
    let mut items = FfiItems::default();
    let file = syn::parse_file(source).unwrap();
    items.visit_file(&file);
    println!("{:#?}", items);
}
