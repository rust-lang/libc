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

    /// Whether this field is a zero-length array, `[T; 0]`.
    ///
    /// This is how a C flexible array member (`T field[]`) is written in Rust. C rejects
    /// `sizeof` on these, so their size test is skipped.
    pub fn is_flexible_array(&self) -> bool {
        let syn::Type::Array(array) = &self.ty else {
            return false;
        };
        matches!(
            &array.len,
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(len), .. })
                if len.base10_parse::<usize>().is_ok_and(|len| len == 0)
        )
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
