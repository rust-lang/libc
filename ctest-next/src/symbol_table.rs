use quote::ToTokens;
use syn::visit::Visit;

use crate::ir::TypeAlias;

#[derive(Debug)]
pub struct SymbolTable {
    aliases: Vec<TypeAlias>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            aliases: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for SymbolTable {
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        let ty = i.ty.to_token_stream();
        let ident = i.ident.to_owned();

        self.aliases.push(TypeAlias::new(ident, ty));
    }
}

// TODO: Add more tests.

#[cfg(test)]
mod tests {
    use std::error::Error;

    use proc_macro2::TokenStream;
    use quote::quote;

    use super::*;

    fn construct_symbol_table(code: TokenStream) -> Result<SymbolTable, Box<dyn Error>> {
        let ast = syn::parse2(code)?;
        let mut table = SymbolTable::new();
        table.visit_file(&ast);
        Ok(table)
    }

    #[test]
    fn test_alias_to_path_extraction() -> Result<(), Box<dyn Error>> {
        let table = construct_symbol_table(quote! {
            type clong = std::ffi::c_long;
        })?;

        assert_eq!(table.aliases.len(), 1);

        let alias = &table.aliases[0];
        assert_eq!(alias.ident().to_string(), "clong");
        assert_eq!(
            alias.ty().to_string(),
            // syn adds whitespaces between items to guarantee that
            // round-trip works.
            "std :: ffi :: c_long"
        );

        Ok(())
    }

    #[test]
    fn test_alias_to_ptr_extraction() -> Result<(), Box<dyn Error>> {
        let table = construct_symbol_table(quote! {
            type ptr = *mut u32;
        })?;

        assert_eq!(table.aliases.len(), 1);

        let alias = &table.aliases[0];
        assert_eq!(alias.ident().to_string(), "ptr");
        assert_eq!(
            alias.ty().to_string(),
            // syn adds whitespaces between items to guarantee that
            // round-trip works.
            "* mut u32"
        );

        Ok(())
    }
}
