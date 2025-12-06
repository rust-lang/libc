//! Provides the [StyleChecker] visitor to verify the coding style of
//! this library.
//!
//! This is split out so that the implementation itself can be tested
//! separately, see test/check_style.rs for how it's used and
//! test/style_tests.rs for the implementation tests.
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
//! * No manual deriving Copy/Clone
//! * Only one f! per module
//! * Multiple s! macros are allowed as long as there isn't a duplicate cfg,
//!   whether as a standalone attribute (#[cfg]) or in a cfg_if!
//! * s! macros should not just have a positive cfg since they should
//!   just go into the relevant file but combined cfgs with all(...) and
//!   any(...) are allowed

use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::{
    Path,
    PathBuf,
};

use annotate_snippets::{
    Level,
    Renderer,
    Snippet,
};
use proc_macro2::Span;
use syn::parse::{
    Parse,
    ParseStream,
};
use syn::spanned::Spanned;
use syn::visit::{
    self,
    Visit,
};
use syn::Token;

const ALLOWED_REPEATED_MACROS: &[&str] = &["s", "s_no_extra_traits", "s_paren"];
const ALLOWED_POSITIVE_S_CFGS: &[&str] = &[
    "gnu_file_offset_bits64",
    "gnu_time_bits64",
    "musl32_time64",
    "musl_v1_2_3",
];

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct StyleChecker {
    /// The state the style checker is in, used to enforce the module layout.
    state: State,
    /// Span of the first item encountered in this state to use in help
    /// diagnostic text.
    state_span: Option<Span>,
    /// The s! macro cfgs we have seen, whether through #[cfg] attributes
    /// or within the branches of cfg_if! blocks so that we can check for duplicates.
    seen_s_macro_cfgs: HashMap<String, Span>,
    /// Span of the first f! macro seen, used to enforce only one f! macro
    /// per module.
    first_f_macro: Option<Span>,
    /// The errors that the style checker has seen.
    errors: Vec<FileError>,
    /// Path of the currently active file.
    path: PathBuf,
    /// Whether the style checker is currently in an `impl` block.
    in_impl: bool,
}

/// The part of the module layout we are currently checking.
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
    _cond: syn::Attribute,
    /// A `cfg_if!` branch can only contain items.
    then_branch: Vec<syn::Item>,
    else_branch: Option<Box<ExprCfgElse>>,
}

enum ExprCfgElse {
    /// Final block with no condition `else { /* ... */ }`.
    Block(Vec<syn::Item>),
    /// `else if { /* ... */ }` block.
    If(ExprCfgIf),
}

/// Describes an that occurred error when checking the file
/// at the given `path`. Besides the error message, it contains
/// additional span information so that we can print nice error messages.
#[derive(Debug)]
struct FileError {
    path: PathBuf,
    span: Span,
    title: String,
    msg: String,
    help: Option<HelpMsg>,
}

/// Help message with an optional span where the help should point to.
type HelpMsg = (Option<Span>, String);

impl StyleChecker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reads and parses the file at the given path and checks
    /// for any style violations.
    pub fn check_file(&mut self, path: &Path) -> Result<()> {
        let contents = fs::read_to_string(path)?;

        self.path = PathBuf::from(path);
        self.check_string(contents)
    }

    pub fn check_string(&mut self, contents: String) -> Result<()> {
        let file = syn::parse_file(&contents)?;
        self.visit_file(&file);
        Ok(())
    }

    /// Resets the state of the [StyleChecker].
    pub fn reset_state(&mut self) {
        *self = Self {
            errors: std::mem::take(&mut self.errors),
            ..Self::default()
        };
    }

    /// Collect all errors into a single error, reporting them if any.
    pub fn finalize(self) -> Result<()> {
        if self.errors.is_empty() {
            return Ok(());
        }

        let renderer = Renderer::styled();
        for error in self.errors {
            let source = fs::read_to_string(&error.path)?;

            let mut snippet = Snippet::source(&source)
                .origin(error.path.to_str().expect("path to be UTF-8"))
                .fold(true)
                .annotation(Level::Error.span(error.span.byte_range()).label(&error.msg));
            if let Some((help_span, help_msg)) = &error.help {
                if let Some(help_span) = help_span {
                    snippet = snippet
                        .annotation(Level::Help.span(help_span.byte_range()).label(help_msg));
                }
            }

            let mut msg = Level::Error.title(&error.title).snippet(snippet);
            if let Some((help_span, help_msg)) = &error.help {
                if help_span.is_none() {
                    msg = msg.footer(Level::Help.title(help_msg))
                }
            }

            eprintln!("{}", renderer.render(msg));
        }

        Err("some tests failed".into())
    }

    fn set_state(&mut self, new_state: State, span: Span) {
        if self.state > new_state && !self.in_impl {
            let help_span = self
                .state_span
                .expect("state_span should be set since we are on a second state");
            self.error(
                "incorrect module layout".to_string(),
                span,
                format!(
                    "{} found after {} when it belongs before",
                    new_state.desc(),
                    self.state.desc()
                ),
                (
                    Some(help_span),
                    format!(
                        "move the {} to before this {}",
                        new_state.desc(),
                        self.state.desc()
                    ),
                ),
            );
        }

        if self.state != new_state {
            self.state = new_state;
            self.state_span = Some(span);
        }
    }

    /// Visit the items inside the [ExprCfgIf], restoring the state after
    /// each branch.
    fn visit_expr_cfg_if(&mut self, expr_cfg_if: &ExprCfgIf) {
        let initial_state = self.state;

        for item in &expr_cfg_if.then_branch {
            self.visit_item(item);
        }
        self.state = initial_state;

        if let Some(else_branch) = &expr_cfg_if.else_branch {
            match else_branch.deref() {
                ExprCfgElse::Block(items) => {
                    for item in items {
                        self.visit_item(item);
                    }
                }
                ExprCfgElse::If(expr_cfg_if) => self.visit_expr_cfg_if(&expr_cfg_if),
            }
        }
        self.state = initial_state;
    }

    /// If an s! macro has attributes we check for any duplicates as well
    /// as if they are standalone positive cfgs that would be better
    /// in a separate file.
    fn handle_s_macro_with_attrs(&mut self, item_macro: &syn::ItemMacro) {
        for attr in &item_macro.attrs {
            let Ok(meta_list) = attr.meta.require_list() else {
                continue;
            };

            if meta_list.path.is_ident("cfg") {
                let span = meta_list.span();
                let meta_str = meta_list.tokens.to_string();

                match self.seen_s_macro_cfgs.get(&meta_str) {
                    Some(seen_span) => {
                        self.error(
                            "duplicate #[cfg] for s! macro".to_string(),
                            span,
                            "duplicated #[cfg]".to_string(),
                            (Some(*seen_span), "combine the two".to_string()),
                        );
                    }
                    None => {
                        self.seen_s_macro_cfgs.insert(meta_str.clone(), span);
                    }
                }

                if !meta_str.starts_with("not")
                    && !meta_str.starts_with("any")
                    && !meta_str.starts_with("all")
                    && !meta_str.starts_with("target_endian")
                    && !ALLOWED_POSITIVE_S_CFGS.contains(&meta_str.as_str())
                {
                    self.error(
                        "positive #[cfg] for s! macro".to_string(),
                        span,
                        String::new(),
                        (None, "move it to the relevant file".to_string()),
                    );
                }
            }
        }
    }

    fn push_error(&mut self, title: String, span: Span, msg: String, help: Option<HelpMsg>) {
        self.errors.push(FileError {
            path: self.path.clone(),
            title,
            span,
            msg,
            help,
        });
    }

    fn error(&mut self, title: String, span: Span, msg: String, help: HelpMsg) {
        self.push_error(title, span, msg, Some(help));
    }
}

impl<'ast> Visit<'ast> for StyleChecker {
    /// Visit all items; most just update our current state but some also
    /// perform additional checks like for the s! macro.
    fn visit_item_use(&mut self, item_use: &'ast syn::ItemUse) {
        let span = item_use.span();
        let new_state = if matches!(item_use.vis, syn::Visibility::Public(_)) {
            State::Modules
        } else {
            State::Imports
        };
        self.set_state(new_state, span);

        visit::visit_item_use(self, item_use);
    }

    fn visit_item_const(&mut self, item_const: &'ast syn::ItemConst) {
        let span = item_const.span();
        self.set_state(State::Constants, span);

        visit::visit_item_const(self, item_const);
    }

    fn visit_item_impl(&mut self, item_impl: &'ast syn::ItemImpl) {
        self.in_impl = true;
        visit::visit_item_impl(self, item_impl);
        self.in_impl = false;
    }

    fn visit_item_struct(&mut self, item_struct: &'ast syn::ItemStruct) {
        let span = item_struct.span();
        self.set_state(State::Structs, span);

        visit::visit_item_struct(self, item_struct);
    }

    fn visit_item_type(&mut self, item_type: &'ast syn::ItemType) {
        let span = item_type.span();
        self.set_state(State::Typedefs, span);

        visit::visit_item_type(self, item_type);
    }

    /// Checks s! macros for any duplicate cfgs and whether they are
    /// just positive #[cfg(...)] attributes. We need [syn::ItemMacro]
    /// instead of [syn::Macro] because it contains the attributes.
    fn visit_item_macro(&mut self, item_macro: &'ast syn::ItemMacro) {
        if item_macro.mac.path.is_ident("s") {
            if !item_macro.attrs.is_empty() {
                self.handle_s_macro_with_attrs(item_macro);
            }
        }

        visit::visit_item_macro(self, item_macro);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        let span = mac.span();
        if mac.path.is_ident("cfg_if") {
            let expr_cfg_if: ExprCfgIf = mac
                .parse_body()
                .expect("cfg_if! should be parsed since it compiled");

            self.visit_expr_cfg_if(&expr_cfg_if);
        } else {
            let new_state =
                if mac.path.get_ident().is_some_and(|ident| {
                    ALLOWED_REPEATED_MACROS.contains(&ident.to_string().as_str())
                }) {
                    // multiple macros of this type are allowed
                    State::Structs
                } else if mac.path.is_ident("f") {
                    match self.first_f_macro {
                        Some(f_macro_span) => {
                            self.error(
                                "multiple f! macros in one module".to_string(),
                                span,
                                "other f! macro".to_string(),
                                (
                                    Some(f_macro_span),
                                    "combine it with this f! macro".to_string(),
                                ),
                            );
                        }
                        None => {
                            self.first_f_macro = Some(span);
                        }
                    }
                    State::FunctionDefinitions
                } else {
                    self.state
                };
            self.set_state(new_state, span);
        }

        visit::visit_macro(self, mac);
    }

    fn visit_item_foreign_mod(&mut self, item_foreign_mod: &'ast syn::ItemForeignMod) {
        let span = item_foreign_mod.span();
        self.set_state(State::Functions, span);

        visit::visit_item_foreign_mod(self, item_foreign_mod);
    }

    fn visit_item_mod(&mut self, item_mod: &'ast syn::ItemMod) {
        let span = item_mod.span();
        self.set_state(State::Modules, span);

        visit::visit_item_mod(self, item_mod);
    }

    fn visit_meta_list(&mut self, meta_list: &'ast syn::MetaList) {
        let span = meta_list.span();
        let meta_str = meta_list.tokens.to_string();
        if meta_list.path.is_ident("derive")
            && (meta_str.contains("Copy") || meta_str.contains("Clone"))
        {
            self.error(
                "impl Copy and Clone manually".to_string(),
                span,
                "found manual implementation of Copy and/or Clone".to_string(),
                (None, "use one of the s! macros instead".to_string()),
            );
        }

        visit::visit_meta_list(self, meta_list);
    }
}

impl Parse for ExprCfgIf {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let cond = input
            .call(syn::Attribute::parse_outer)?
            .into_iter()
            .next()
            .expect("an attribute should be present since it compiled");

        let content;
        syn::braced!(content in input);
        let mut then_branch = Vec::new();
        while !content.is_empty() {
            let mut value = content.parse()?;
            if let syn::Item::Macro(item_macro) = &mut value {
                item_macro.attrs.push(cond.clone());
            }
            then_branch.push(value);
        }

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
            _cond: cond,
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
