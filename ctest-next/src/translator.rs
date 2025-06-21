use std::ops::Deref;

#[derive(Debug, Default)]
/// A Rust to C/Cxx translator.
pub(crate) struct Translator {}

impl Translator {
    /// Create a new translator.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Return whether a type is a Rust primitive type.
    fn is_rust_primitive(&self, ty: &str) -> bool {
        let rustc_types = [
            "usize", "u8", "u16", "u32", "u64", "isize", "i8", "i16", "i32", "i64", "f32", "f64",
        ];
        ty.starts_with("c_") || rustc_types.contains(&ty)
    }

    /// Translate mutability from Rust to C.
    fn translate_mut(&self, mutability: Option<syn::Token![mut]>) -> String {
        mutability.map(|_| "const ").unwrap_or("").to_string()
    }

    /// Translate a Rust type into its equivalent C type.
    pub(crate) fn translate_type(&self, ty: &syn::Type) -> String {
        match ty {
            syn::Type::Ptr(ptr) => self.translate_ptr(ptr),
            syn::Type::Path(path) => self.translate_path(path),
            syn::Type::Tuple(tuple) if tuple.elems.is_empty() => "void".to_string(),
            syn::Type::Array(array) => self.translate_array(array),
            syn::Type::Reference(reference) => self.translate_reference(reference),
            syn::Type::BareFn(function) => self.translate_bare_fn(function),
            syn::Type::Never(_) => "void".to_string(),
            _ => unimplemented!(),
        }
    }

    /// Translate a Rust reference to its C equivalent.
    fn translate_reference(&self, reference: &syn::TypeReference) -> String {
        let path = match reference.elem.deref() {
            syn::Type::Path(path) => path.path.segments.last().unwrap(),
            syn::Type::Array(array) => {
                return format!(
                    "{}{}*",
                    self.translate_mut(reference.mutability),
                    self.translate_type(&array.elem),
                )
            }
            _ => panic!("Unknown type! {:?}", reference.elem),
        };

        let ident = path.ident.to_string();
        match ident.as_str() {
            "str" => {
                if reference.mutability.is_some() {
                    panic!("Unknown type, &mut str");
                }

                "char*".to_string()
            }
            c if self.is_rust_primitive(c) => format!(
                "{}{}*",
                self.translate_mut(reference.mutability),
                self.translate_primitive_type(&path.ident)
            ),
            _ => unimplemented!("References to non primitive types are not implemented."),
        }
    }

    /// Translate a Rust function pointer type to its C equivalent.
    fn translate_bare_fn(&self, function: &syn::TypeBareFn) -> String {
        assert!(function.lifetimes.is_none(), "No lifetimes allowed.");
        assert!(function.variadic.is_none(), "No variadics allowed.");

        let mut parameters = function
            .inputs
            .iter()
            .map(|arg| self.translate_type(&arg.ty))
            .collect::<Vec<_>>();
        let return_type = match &function.output {
            syn::ReturnType::Default => "void".to_string(),
            syn::ReturnType::Type(_, ty) => self.translate_type(ty),
        };

        if parameters.is_empty() {
            parameters.push("void".to_string());
        }

        if return_type.contains("(*)") {
            return_type.replace("(*)", &format!("(*(*)({}))", parameters.join(", ")))
        } else {
            format!("{}(*)({})", return_type, parameters.join(", "))
        }
    }

    /// Translate a Rust primitve type into its C equivalent.
    fn translate_primitive_type(&self, ty: &syn::Ident) -> String {
        let ty = ty.to_string();
        match ty.as_str() {
            "usize" => "size_t".to_string(),
            "isize" => "ssize_t".to_string(),
            "u8" => "uint8_t".to_string(),
            "u16" => "uint16_t".to_string(),
            "u32" => "uint32_t".to_string(),
            "u64" => "uint64_t".to_string(),
            "i8" => "int8_t".to_string(),
            "i16" => "int16_t".to_string(),
            "i32" => "int32_t".to_string(),
            "i64" => "int64_t".to_string(),
            "f32" => "float".to_string(),
            "f64" => "double".to_string(),
            "()" => "void".to_string(),

            "c_longdouble" | "c_long_double" => "long double".to_string(),
            ty if ty.starts_with("c_") => {
                let ty = &ty[2..].replace("long", " long")[..];
                match ty {
                    "short" => "short".to_string(),
                    s if s.starts_with('u') => format!("unsigned {}", &s[1..]),
                    s if s.starts_with('s') => format!("signed {}", &s[1..]),
                    s => s.to_string(),
                }
            }
            // Overriding type names not yet implemented.
            s => s.to_string(),
        }
    }

    /// Translate a Rust path into its C equivalent.
    fn translate_path(&self, path: &syn::TypePath) -> String {
        // Paths should be fully qualified otherwise they won't properly be translated.
        let last = path.path.segments.last().unwrap();
        if last.ident == "Option" {
            if let syn::PathArguments::AngleBracketed(p) = &last.arguments {
                if let syn::GenericArgument::Type(ty) = p.args.first().unwrap() {
                    self.translate_type(ty)
                } else {
                    unimplemented!("Only simple generic types are supported!")
                }
            } else {
                unreachable!("Option<T> cannot have parentheses.")
            }
        } else {
            self.translate_primitive_type(&last.ident)
        }
    }

    /// Translate a Rust array declaration into its C equivalent.
    fn translate_array(&self, array: &syn::TypeArray) -> String {
        format!(
            "{}[{}]",
            self.translate_type(array.elem.deref()),
            self.translate_expr(&array.len)
        )
    }

    /// Translate a Rust pointer into its equivalent C pointer.
    fn translate_ptr(&self, ptr: &syn::TypePtr) -> String {
        let modifier = ptr.mutability.map(|_| "").unwrap_or("const");
        let inner = ptr.elem.deref();
        match inner {
            syn::Type::BareFn(_) => self.translate_type(inner),
            syn::Type::Ptr(_) => format!("{} {}*", self.translate_type(inner), modifier),
            syn::Type::Array(arr) => {
                let len = self.translate_expr(&arr.len);
                let ty = self.translate_type(inner);
                format!("{} {} [{}]", modifier, ty, len)
            }
            _ => format!("{} {}*", modifier, self.translate_type(inner)),
        }
    }

    /// Translate a simple Rust expression to C.
    ///
    /// This method is only used for translating expressions inside of
    /// array brackets, and will fail for expressions not allowed inside of
    /// those brackets.
    #[expect(clippy::only_used_in_recursion)]
    fn translate_expr(&self, expr: &syn::Expr) -> String {
        match expr {
            syn::Expr::Lit(l) => match &l.lit {
                syn::Lit::Int(i) => i.to_string(),
                _ => panic!("Invalid Syntax! Cannot have non integer literal in array expression."),
            },
            syn::Expr::Path(p) => p.path.segments.last().unwrap().ident.to_string(),
            syn::Expr::Cast(c) => self.translate_expr(c.expr.deref()),
            syn::Expr::Binary(b) => {
                let left = self.translate_expr(b.left.deref());
                let right = self.translate_expr(b.right.deref());

                match b.op {
                    syn::BinOp::Add(_) => format!("{} + {}", left, right),
                    syn::BinOp::Sub(_) => format!("{} - {}", left, right),
                    // Some operators have not been implemented, such as
                    // shift left, shift right etc. Some other operators cannot be
                    // placed inside array brackets.
                    _ => unimplemented!("Unknown Operator! {:?}", b.op),
                }
            }
            // Some expressions have not been implemented, such as
            // braces eg: [u8; { expr }], constant functions, etc.
            _ => unimplemented!("Unknown Expression! {:?}", expr),
        }
    }
}
