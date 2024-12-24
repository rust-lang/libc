//! Simple script to verify the coding style of this library
//!
//! ## How to run
//!
//! The first argument to this script is the directory to run on, so running
//! this script should be as simple as:
//!
//! ```notrust
//! cargo test --test style -- ../src
//! ```
//!
//! ## Guidelines
//!
//! The current style is:
//!
//! * Specific module layout:
//!     1. use directives
//!     2. typedefs
//!     3. structs
//!     4. constants
//!     5. f! { ... } functions
//!     6. extern functions
//!     7. modules + pub use
//!
//! Things not verified:
//!
//! * alignment
//! * leading colons on paths

use std::io::prelude::*;
use std::path::Path;
use std::{env, fs};

use syn::spanned::Spanned;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

#[test]
fn main() {
    let arg = env::args().skip(1).next().unwrap_or("../src".to_string());

    let mut errors = Errors { errs: false };
    walk(Path::new(&arg), &mut errors);

    if errors.errs {
        panic!("found some lint errors");
    } else {
        println!("good style!");
    }
}

fn walk(path: &Path, err: &mut Errors) {
    for entry in t!(path.read_dir()).map(|e| t!(e)) {
        let path = entry.path();
        if t!(entry.file_type()).is_dir() {
            walk(&path, err);
            continue;
        }

        let name = entry.file_name().into_string().unwrap();
        match &name[..] {
            n if !n.ends_with(".rs") => continue,

            "lib.rs" | "macros.rs" => continue,

            _ => {}
        }

        let mut contents = String::new();
        t!(t!(fs::File::open(&path)).read_to_string(&mut contents));

        let file = t!(syn::parse_file(&contents));
        check_style(file, &path, err);
    }
}

struct Errors {
    errs: bool,
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    Start,
    Imports,
    Typedefs,
    Structs,
    Constants,
    FunctionDefinitions,
    Functions,
    Modules,
}

fn check_style(file: syn::File, path: &Path, err: &mut Errors) {
    let mut state = State::Start;

    // FIXME: see below
    // let mut s_macros = 0;
    let mut f_macros = 0;

    for item in file.items {
        let line = item.span().start().line;

        for attr in attrs_of(&item) {
            if let syn::Meta::List(meta_list) = &attr.meta {
                let meta_str = meta_list.tokens.to_string();
                if meta_list.path.is_ident("cfg")
                    && !(meta_str.contains("target_endian") || meta_str.contains("target_arch"))
                    && state != State::Structs
                {
                    // TODO: not quite the right line number
                    err.error(path, line, "use cfg_if! and submodules instead of #[cfg]");
                } else if meta_list.path.is_ident("derive")
                    && (meta_str.contains("Copy") || meta_str.contains("Clone"))
                {
                    err.error(path, line, "impl Copy and Clone manually");
                }
            }
        }

        let item_state = if let syn::Item::Use(item_use) = item {
            if matches!(item_use.vis, syn::Visibility::Public(_)) {
                State::Modules
            } else {
                State::Imports
            }
        } else if let syn::Item::Const(_) = item {
            State::Constants
        } else if let syn::Item::Type(_) = item {
            State::Typedefs
        } else if let syn::Item::Macro(item_macro) = item {
            if item_macro.mac.path.is_ident("s") {
                // FIXME: see below
                // s_macros += 1;
                State::Structs
            } else if item_macro.mac.path.is_ident("s_no_extra_traits") {
                // multiple macros of this type are allowed
                State::Structs
            } else if item_macro.mac.path.is_ident("s_paren") {
                // multiple macros of this type are allowed
                State::Structs
            } else if item_macro.mac.path.is_ident("f") {
                f_macros += 1;
                State::FunctionDefinitions
            } else {
                continue;
            }
        } else if let syn::Item::ForeignMod(_) = item {
            State::Functions
        } else if let syn::Item::Mod(_) = item {
            State::Modules
        } else {
            continue;
        };

        if state as usize > item_state as usize {
            err.error(
                path,
                line,
                &format!(
                    "{} found after {} when it belongs before",
                    item_state.desc(),
                    state.desc()
                ),
            );
        }

        if f_macros == 2 {
            f_macros += 1;
            err.error(path, line, "multiple f! macros in one module");
        }

        // FIXME(#4109): multiple should be allowed if at least one is `cfg(not) within `cfg_if`.
        // For now just disable this and check by hand.
        // if s_macros == 2 {
        //     s_macros += 1;
        //     err.error(path, i, "multiple s! macros in one module");
        // }

        state = item_state;
    }
}

fn attrs_of(item: &syn::Item) -> Vec<syn::Attribute> {
    match item {
        syn::Item::Const(item_const) => item_const.attrs.clone(),
        syn::Item::Enum(item_enum) => item_enum.attrs.clone(),
        syn::Item::ExternCrate(item_extern_crate) => item_extern_crate.attrs.clone(),
        syn::Item::Fn(item_fn) => item_fn.attrs.clone(),
        syn::Item::ForeignMod(item_foreign_mod) => item_foreign_mod.attrs.clone(),
        syn::Item::Impl(item_impl) => item_impl.attrs.clone(),
        syn::Item::Macro(item_macro) => item_macro.attrs.clone(),
        syn::Item::Mod(item_mod) => item_mod.attrs.clone(),
        syn::Item::Static(item_static) => item_static.attrs.clone(),
        syn::Item::Struct(item_struct) => item_struct.attrs.clone(),
        syn::Item::Trait(item_trait) => item_trait.attrs.clone(),
        syn::Item::TraitAlias(item_trait_alias) => item_trait_alias.attrs.clone(),
        syn::Item::Type(item_type) => item_type.attrs.clone(),
        syn::Item::Union(item_union) => item_union.attrs.clone(),
        syn::Item::Use(item_use) => item_use.attrs.clone(),
        _ => vec![],
    }
}

impl State {
    fn desc(&self) -> &str {
        match *self {
            State::Start => "start",
            State::Imports => "import",
            State::Typedefs => "typedef",
            State::Structs => "struct",
            State::Constants => "constant",
            State::FunctionDefinitions => "function definition",
            State::Functions => "extern function",
            State::Modules => "module",
        }
    }
}

impl Errors {
    fn error(&mut self, path: &Path, line: usize, msg: &str) {
        self.errs = true;
        println!("{}:{}: {}", path.display(), line + 1, msg);
    }
}
