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
use std::ops::Deref;
use std::path::Path;
use std::{env, fs};

use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::Token;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

#[test]
fn check_style() {
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
        StyleChecker::new(|line, msg| err.error(&path, line, msg)).visit_file(&file);
    }
}

struct Errors {
    errs: bool,
}

struct StyleChecker<F>
where
    F: FnMut(usize, &str),
{
    state: State,
    // FIXME: see StyleChecker::set_state
    _s_macros: usize,
    f_macros: usize,
    on_err: F,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

/// Similar to [syn::ExprIf] except with [syn::Attribute]
/// as the condition instead of [syn::Expr].
struct CfgExprIf {
    _attr: syn::Attribute,
    then_branch: Vec<syn::Item>,
    else_branch: Option<Box<CfgExprElse>>,
}

enum CfgExprElse {
    Block(Vec<syn::Item>),
    If(CfgExprIf),
}

impl<F> StyleChecker<F>
where
    F: FnMut(usize, &str),
{
    fn new(on_err: F) -> Self {
        Self {
            state: State::Start,
            _s_macros: 0,
            f_macros: 0,
            on_err,
        }
    }

    fn set_state(&mut self, new_state: State, line: usize) {
        if self.state > new_state {
            (self.on_err)(
                line,
                &format!(
                    "{} found after {} when it belongs before",
                    new_state.desc(),
                    self.state.desc()
                ),
            );
        }

        if self.f_macros == 2 {
            self.f_macros += 1;
            (self.on_err)(line, "multiple f! macros in one module");
        }

        // FIXME(#4109): multiple should be allowed if at least one is `cfg(not) within `cfg_if`.
        // For now just disable this and check by hand.
        // if self.s_macros == 2 {
        //     self.s_macros += 1;
        //     (self.on_err)(line, "multiple s! macros in one module");
        // }

        self.state = new_state;
    }

    /// Visit the items inside the [CfgExprIf] and return the final
    /// state after, which is conservatively the minimum of all branches
    /// since the branches could have diverged in state.
    fn visit_cfg_expr_if(&mut self, cfg_expr_if: &CfgExprIf) -> State {
        let initial_state = self.state;

        for item in &cfg_expr_if.then_branch {
            self.visit_item(item);
        }
        let then_end_state = self.state;
        self.state = initial_state;

        match &cfg_expr_if.else_branch {
            Some(else_branch) => match else_branch.deref() {
                CfgExprElse::Block(items) => {
                    for item in items {
                        self.visit_item(item);
                    }
                    then_end_state.min(self.state)
                }
                CfgExprElse::If(cfg_expr_if) => self.visit_cfg_expr_if(&cfg_expr_if),
            },
            None => then_end_state,
        }
    }
}

impl<'ast, F> Visit<'ast> for StyleChecker<F>
where
    F: FnMut(usize, &str),
{
    fn visit_meta_list(&mut self, meta_list: &'ast syn::MetaList) {
        let line = meta_list.span().start().line;
        let meta_str = meta_list.tokens.to_string();
        if meta_list.path.is_ident("cfg")
            && !(meta_str.contains("target_endian") || meta_str.contains("target_arch"))
            && self.state != State::Structs
        {
            (self.on_err)(line, "use cfg_if! and submodules instead of #[cfg]");
        } else if meta_list.path.is_ident("derive")
            && (meta_str.contains("Copy") || meta_str.contains("Clone"))
        {
            (self.on_err)(line, "impl Copy and Clone manually");
        }

        visit::visit_meta_list(self, meta_list);
    }

    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        let line = item_use.span().start().line;
        let new_state = if matches!(item_use.vis, syn::Visibility::Public(_)) {
            State::Modules
        } else {
            State::Imports
        };
        self.set_state(new_state, line);

        visit::visit_item_use(self, item_use);
    }

    fn visit_item_const(&mut self, item_const: &'ast syn::ItemConst) {
        let line = item_const.span().start().line;
        self.set_state(State::Constants, line);

        visit::visit_item_const(self, item_const);
    }

    fn visit_item_type(&mut self, item_type: &'ast syn::ItemType) {
        let line = item_type.span().start().line;
        self.set_state(State::Typedefs, line);

        visit::visit_item_type(self, item_type);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let line = mac.span().start().line;
        if mac.path.is_ident("cfg_if") {
            let cfg_expr_if: CfgExprIf = mac
                .parse_body()
                .expect("cfg_if! should be parsed since it compiled");

            let end_state = self.visit_cfg_expr_if(&cfg_expr_if);
            self.state = end_state;
        } else {
            let new_state = if mac.path.is_ident("s") {
                // FIXME: see StyleChecker::set_state
                // self.s_macros += 1;
                State::Structs
            } else if mac.path.is_ident("s_no_extra_traits") {
                // multiple macros of this type are allowed
                State::Structs
            } else if mac.path.is_ident("s_paren") {
                // multiple macros of this type are allowed
                State::Structs
            } else if mac.path.is_ident("f") {
                self.f_macros += 1;
                State::FunctionDefinitions
            } else {
                self.state
            };
            self.set_state(new_state, line);
        }

        visit::visit_macro(self, mac);
    }

    fn visit_item_foreign_mod(&mut self, item_foreign_mod: &'ast syn::ItemForeignMod) {
        let line = item_foreign_mod.span().start().line;
        self.set_state(State::Functions, line);

        visit::visit_item_foreign_mod(self, item_foreign_mod);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let line = item_mod.span().start().line;
        self.set_state(State::Modules, line);

        visit::visit_item_mod(self, item_mod);
    }
}

impl Parse for CfgExprIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let attr = input
            .call(syn::Attribute::parse_outer)?
            .into_iter()
            .next()
            .expect("an attribute should be present since it compiled");

        let content;
        syn::braced!(content in input);
        let then_branch: Vec<syn::Item> = {
            let mut items = Vec::new();
            while !content.is_empty() {
                items.push(content.parse()?);
            }
            items
        };

        let mut else_branch = None;
        if input.peek(Token![else]) {
            input.parse::<Token![else]>()?;

            if input.peek(Token![if]) {
                else_branch = Some(Box::new(CfgExprElse::If(input.parse()?)));
            } else {
                let content;
                syn::braced!(content in input);
                let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }
                else_branch = Some(Box::new(CfgExprElse::Block(items)));
            }
        }
        Ok(Self {
            _attr: attr,
            then_branch,
            else_branch,
        })
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
        println!("{}:{}: {}", path.display(), line, msg);
    }
}
