use crate::{ffi_items::FfiItems, translator::Translator};

use syn::visit::Visit;

const ALL_ITEMS: &str = r#"
use std::os::raw::c_void;

mod level1 {
    pub type Foo = u8;

    pub const bar: u32 = 512;

    pub union Word {
        word: u16,
        bytes: [u8; 2],
    }
}

pub struct Array {
    ptr: *mut c_void,
    len: usize,
}

extern "C" {
    static baz: u16;

    fn malloc(size: usize) -> *mut c_void;
}
"#;

macro_rules! collect_idents {
    ($items:expr) => {
        $items.iter().map(|a| a.ident()).collect::<Vec<_>>()
    };
}

#[test]
fn test_extraction_ffi_items() {
    let ast = syn::parse_file(ALL_ITEMS).unwrap();

    let mut ffi_items = FfiItems::new();
    ffi_items.visit_file(&ast);

    assert_eq!(collect_idents!(ffi_items.aliases()), ["Foo"]);
    assert_eq!(collect_idents!(ffi_items.constants()), ["bar"]);
    assert_eq!(collect_idents!(ffi_items.foreign_functions()), ["malloc"]);
    assert_eq!(collect_idents!(ffi_items.foreign_statics()), ["baz"]);
    assert_eq!(collect_idents!(ffi_items.structs()), ["Array"]);
    assert_eq!(collect_idents!(ffi_items.unions()), ["Word"]);
}

#[test]
fn test_translation_type_path() {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str("std::option::Option<u8>").unwrap();

    assert_eq!(translator.translate_type(&ty), "uint8_t");
}

#[test]
fn test_translation_type_ptr() {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str("*const *mut i32").unwrap();

    assert_eq!(translator.translate_type(&ty), " int32_t* const*");
}

#[test]
fn test_translation_type_reference() {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str("&u8").unwrap();

    assert_eq!(translator.translate_type(&ty), "uint8_t*");
}

#[test]
fn test_translation_type_bare_fn() {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str("fn(*mut u8, i16) -> &str").unwrap();

    assert_eq!(
        translator.translate_type(&ty),
        "char*(*)( uint8_t*, int16_t)"
    );
}

#[test]
fn test_translation_type_array() {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str("[&u8; 2 + 2]").unwrap();

    assert_eq!(translator.translate_type(&ty), "uint8_t*[2 + 2]");
}
