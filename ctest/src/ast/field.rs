use crate::BoxStr;

/// Represents a field in a struct or union defined in Rust.
#[derive(Debug, Clone)]
pub struct Field {
    pub(crate) public: bool,
    pub(crate) ident: BoxStr,
    pub(crate) ty: syn::Type,
}

impl Field {
    /// Return the identifier of the field as a string.
    pub fn ident(&self) -> &str {
        &self.ident
    }

    /// Return the identifier escaped for Rust source output when needed.
    pub fn rust_ident(&self) -> String {
        if is_rust_keyword(&self.ident) {
            format!("r#{}", self.ident)
        } else {
            self.ident.to_string()
        }
    }
}

fn is_rust_keyword(ident: &str) -> bool {
    matches!(
        ident,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}
