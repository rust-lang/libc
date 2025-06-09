use crate::{
    ir::{Constant, Field, Function, Struct},
    symbol_table::SymbolTable,
};

/// `TranslationOverrides` defines customization hooks for how identifiers
/// are translated during code generation.
///
/// Implementors can override name generation for fields, types, constants,
/// and functions.
pub trait TranslationOverrides {
    fn field_name(&self, structure: &Struct, field: &Field) -> String {
        field.ident().clone().unwrap().to_string()
    }
    fn type_name(&self, name: &str, table: &SymbolTable) -> String {
        if table.contains_struct(name) {
            format!("struct {}", name)
        } else if table.contains_union(name) {
            format!("union {}", name)
        } else {
            name.to_string()
        }
    }
    fn const_name(&self, constant: Constant) -> String {
        constant.ident().clone().to_string()
    }
    fn func_name(&self, function: Function, link_name: String) -> String {
        function.ident().to_string()
    }
}

/// The default implementation of `TranslationOverrides`,
/// which performs no renaming and uses the source identifiers directly.
pub struct DefaultOverrides;

impl TranslationOverrides for DefaultOverrides {}
