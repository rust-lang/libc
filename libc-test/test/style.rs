//! Simple script to verify the coding style of this library
//!
//! ## How to run
//!
//! The first argument to this script is the directory to run on, so running
//! this script should be as simple as:
//!
//! ```notrust
//! cargo test --test style
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

use std::fmt::Display;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fs};

use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::Token;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

#[test]
fn check_style() {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src");
    walk(&root_dir).unwrap();
    eprintln!("good style!");
}

fn walk(root_dir: &Path) -> Result<()> {
    let mut style_checker = StyleChecker::new();

    for entry in glob::glob(&format!(
        "{}/**/*.rs",
        root_dir.to_str().expect("dir should be valid UTF-8")
    ))? {
        let entry = entry?;

        let name = entry
            .file_name()
            .expect("file name should not end in ..")
            .to_str()
            .expect("file name should be valid UTF-8");
        if let "lib.rs" | "macros.rs" = &name[..] {
            continue;
        }

        let path = entry.as_path();
        style_checker.check_file(path)?;
        style_checker.reset_state();
    }

    style_checker.finalize()
}

#[derive(Default)]
struct StyleChecker {
    state: State,
    // FIXME: see StyleChecker::set_state
    _s_macros: usize,
    f_macros: usize,
    errors: Vec<FileError>,
    /// Path of the currently active file
    path: PathBuf,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    #[default]
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
struct ExprCfgIf {
    _attr: syn::Attribute,
    then_branch: Vec<syn::Item>,
    else_branch: Option<Box<ExprCfgElse>>,
}

enum ExprCfgElse {
    Block(Vec<syn::Item>),
    If(ExprCfgIf),
}

impl StyleChecker {
    fn new() -> Self {
        Self::default()
    }

    /// Reads and parses the file at the given path and checks
    /// for any style violations.
    fn check_file(&mut self, path: &Path) -> Result<()> {
        let contents = fs::read_to_string(path)?;
        let file = syn::parse_file(&contents)?;

        self.path = PathBuf::from(path);
        self.visit_file(&file);

        Ok(())
    }

    /// Resets the state of the [StyleChecker].
    fn reset_state(&mut self) {
        *self = Self {
            errors: std::mem::take(&mut self.errors),
            ..Self::default()
        };
    }

    /// Collect all errors into a single error, reporting them if any.
    fn finalize(self) -> Result<()> {
        if self.errors.is_empty() {
            return Ok(());
        }

        for error in self.errors {
            eprintln!("{error}");
        }

        Err("some tests failed".into())
    }

    fn set_state(&mut self, new_state: State, line: usize) {
        if self.state > new_state {
            self.error(
                line,
                format!(
                    "{} found after {} when it belongs before",
                    new_state.desc(),
                    self.state.desc()
                ),
            );
        }

        if self.f_macros == 2 {
            self.f_macros += 1;
            self.error(line, "multiple f! macros in one module".to_string());
        }

        // FIXME(#4109): multiple should be allowed if at least one is `cfg(not) within `cfg_if`.
        // For now just disable this and check by hand.
        // if self.s_macros == 2 {
        //     self.s_macros += 1;
        //     (self.on_err)(line, "multiple s! macros in one module");
        // }

        self.state = new_state;
    }

    /// Visit the items inside the [ExprCfgIf], restoring the state after
    /// each branch.
    fn visit_cfg_expr_if(&mut self, cfg_expr_if: &ExprCfgIf) {
        let initial_state = self.state;

        for item in &cfg_expr_if.then_branch {
            self.visit_item(item);
        }
        self.state = initial_state;

        if let Some(else_branch) = &cfg_expr_if.else_branch {
            match else_branch.deref() {
                ExprCfgElse::Block(items) => {
                    for item in items {
                        self.visit_item(item);
                    }
                }
                ExprCfgElse::If(cfg_expr_if) => self.visit_cfg_expr_if(&cfg_expr_if),
            }
        }
        self.state = initial_state;
    }

    fn error(&mut self, line: usize, msg: String) {
        self.errors.push(FileError {
            path: self.path.clone(),
            line,
            msg,
        });
    }
}

impl<'ast> Visit<'ast> for StyleChecker {
    fn visit_meta_list(&mut self, meta_list: &'ast syn::MetaList) {
        let line = meta_list.span().start().line;
        let meta_str = meta_list.tokens.to_string();
        if meta_list.path.is_ident("cfg")
            && !(meta_str.contains("target_endian") || meta_str.contains("target_arch"))
            && self.state != State::Structs
        {
            self.error(
                line,
                "use cfg_if! and submodules instead of #[cfg]".to_string(),
            );
        } else if meta_list.path.is_ident("derive")
            && (meta_str.contains("Copy") || meta_str.contains("Clone"))
        {
            self.error(line, "impl Copy and Clone manually".to_string());
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
            let cfg_expr_if: ExprCfgIf = mac
                .parse_body()
                .expect("cfg_if! should be parsed since it compiled");

            self.visit_cfg_expr_if(&cfg_expr_if);
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

impl Parse for ExprCfgIf {
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
                else_branch = Some(Box::new(ExprCfgElse::If(input.parse()?)));
            } else {
                let content;
                syn::braced!(content in input);
                let mut items = Vec::new();
                while !content.is_empty() {
                    items.push(content.parse()?);
                }
                else_branch = Some(Box::new(ExprCfgElse::Block(items)));
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

#[derive(Debug)]
struct FileError {
    path: PathBuf,
    line: usize,
    msg: String,
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}: {}", self.path.display(), self.line, self.msg)
    }
}

impl std::error::Error for FileError {}
