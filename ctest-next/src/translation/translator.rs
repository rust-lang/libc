use std::fmt::{self, Write};

use crate::{
    ir::TypeAlias,
    symbol_table::SymbolTable,
    translation::{overrides::DefaultOverrides, TranslationOverrides},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Language {
    C,
    CXX,
    Rust,
}

pub struct RustToCTranslator<O: TranslationOverrides> {
    table: SymbolTable,
    overrides: O,
}

impl RustToCTranslator<DefaultOverrides> {
    pub fn new(table: SymbolTable) -> Self {
        Self {
            table,
            overrides: DefaultOverrides,
        }
    }
}

impl<O: TranslationOverrides> RustToCTranslator<O> {
    pub fn table(&self) -> &SymbolTable {
        &self.table
    }

    pub fn new_with_overrides(table: SymbolTable, overrides: O) -> Self {
        Self { table, overrides }
    }

    pub fn translate_type_alias(&self, type_alias: &TypeAlias) -> Result<String, fmt::Error> {
        let mut s = String::new();
        writeln!(
            s,
            "typedef {} {};",
            // TODO: Add support for more complex types such as crate::c_int.
            self.translate_ptr_type(&type_alias.ty().to_string()),
            type_alias.ident().to_string()
        )?;
        Ok(s)
    }

    pub fn translate_primitive_type(&self, ty: &str) -> String {
        match ty {
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
            s => self.overrides.type_name(s, &self.table),
        }
    }

    pub fn translate_ptr_type(&self, mut ty: &str) -> String {
        if ty == "&str" {
            return "char*".to_string();
        }

        // FIXME: This might fail on some edge case.
        // This had to be slightly modified because the syn roundtrip
        // always adds whitespace between punctuation like *mut => * mut.
        let cty = ty.replace("* mut ", "").replace("* const ", "");
        let mut cty = self.translate_primitive_type(&cty);
        while ty.starts_with("*") {
            if ty.starts_with("* const") {
                cty = format!("const {}*", cty);
                ty = &ty[8..];
            } else {
                cty = format!("{}*", cty);
                ty = &ty[6..];
            }
        }

        cty
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

    #[test]
    fn test_type_alias_complex_type() -> Result<(), Box<dyn Error>> {
        let table = construct_table(quote! {
            pub type ptr = *mut *const i32;
        })?;
        assert_eq!(table.aliases().len(), 1);

        let translator = RustToCTranslator::new(table);
        let alias = &translator.table().aliases()[0];

        assert_eq!(
            translator.translate_type_alias(&alias)?,
            "typedef const int32_t** ptr;\n"
        );

        Ok(())
    }

    #[test]
    fn test_type_name_override() -> Result<(), Box<dyn Error>> {
        let table = construct_table(quote! {
            struct Bob;

            pub type ptr = *mut *const Bob;
        })?;
        assert_eq!(table.aliases().len(), 1);

        struct PrefixTypeWith__;

        impl TranslationOverrides for PrefixTypeWith__ {
            fn type_name(&self, name: &str, _: &SymbolTable) -> String {
                format!("__{}", name)
            }
        }

        let translator = RustToCTranslator::new_with_overrides(table, PrefixTypeWith__);
        let alias = &translator.table().aliases()[0];

        assert_eq!(
            translator.translate_type_alias(&alias)?,
            "typedef const __Bob** ptr;\n"
        );

        Ok(())
    }
}
