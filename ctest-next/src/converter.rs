use std::fmt::{self, Write};

use crate::{ir::TypeAlias, symbol_table::SymbolTable};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Language {
    C,
    CXX,
    Rust,
}

struct RustToCTranslator {
    table: SymbolTable,
}

impl RustToCTranslator {
    fn new(table: SymbolTable) -> Self {
        Self { table }
    }

    fn table(&self) -> &SymbolTable {
        &self.table
    }

    fn translate_type_alias(&self, type_alias: &TypeAlias) -> Result<String, fmt::Error> {
        let mut s = String::new();
        writeln!(
            s,
            "typedef {} {};",
            self.translate_type(&type_alias.ty().to_string()),
            type_alias.ident().to_string()
        )?;
        Ok(s)
    }

    fn translate_type(&self, ty: &str) -> String {
        match ty {
            "usize" => "size_t".to_string(),
            "isize" => "ssize_t".to_string(),
            "u8" => "uint8_t".to_string(),
            "u16" => "uint16_t".to_string(),
            "u32" => "uint32_t".to_string(),
            "u64" => "uint64_t".to_string(),
            "i8" => "iint8_t".to_string(),
            "i16" => "iint16_t".to_string(),
            "i32" => "iint32_t".to_string(),
            "i64" => "iint64_t".to_string(),
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
            s => {
                // Would be replaced with a custom way to translate type later.
                if self.table.contains_struct(s) {
                    format!("struct {}", s)
                } else if self.table.contains_union(s) {
                    format!("union {}", s)
                } else {
                    s.to_string()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::visit::Visit;

    use super::*;

    fn construct_table(code: TokenStream) -> Result<SymbolTable, Box<dyn Error>> {
        let mut table = SymbolTable::new();
        let code: syn::File = syn::parse2(code)?;
        table.visit_file(&code);
        Ok(table)
    }

    #[test]
    fn test_type_alias_primitive_type() -> Result<(), Box<dyn Error>> {
        let table = construct_table(quote! {
            pub type byte = u8;
        })?;
        assert_eq!(table.aliases().len(), 1);

        let translator = RustToCTranslator::new(table);
        let alias = &translator.table().aliases()[0];

        assert_eq!(
            translator.translate_type_alias(&alias)?,
            "typedef uint8_t byte;\n"
        );

        Ok(())
    }

    #[test]
    fn test_type_alias_struct_type() -> Result<(), Box<dyn Error>> {
        let table = construct_table(quote! {
            struct Person {
                name: String,
                age: u8,
            }

            pub type Bob = Person;
        })?;
        assert_eq!(table.aliases().len(), 1);

        let translator = RustToCTranslator::new(table);
        let alias = &translator.table().aliases()[0];

        assert_eq!(
            translator.translate_type_alias(&alias)?,
            "typedef struct Person Bob;\n"
        );

        Ok(())
    }
}
