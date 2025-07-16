use syn::visit::Visit;

use crate::ffi_items::FfiItems;
use crate::translator::Translator;
use crate::{Result, TranslationError};

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

    #[link_name = "calloc"]
    fn malloc(size: usize) -> *mut c_void;
}
"#;

macro_rules! collect_idents {
    ($items:expr) => {
        $items.iter().map(|a| a.ident()).collect::<Vec<_>>()
    };
}

/// Translate a Rust type to C.
fn ty(s: &str) -> Result<String, TranslationError> {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str(s).unwrap();
    translator.translate_type(&ty)
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

    assert_eq!(
        ffi_items.foreign_functions()[0].link_name.as_deref(),
        Some("calloc")
    );
}

#[test]
fn test_translation_type_ptr() {
    assert_eq!(
        ty("*const *mut i32").unwrap(),
        "int32_t * const*".to_string()
    );
    assert_eq!(
        ty("*const [u128; 2 + 3]").unwrap(),
        "unsigned __int128 (*const) [2 + 3]".to_string()
    );
    // FIXME(ctest): While not a valid C type, it will be used to
    // generate a valid test in the future.
    // assert_eq!(
    //     ty("*const *mut [u8; 5]").unwrap(),
    //     "uint8_t (*const *) [5]".to_string()
    // );
}

#[test]
fn test_translation_type_reference() {
    assert_eq!(ty("&u8").unwrap(), "const uint8_t*".to_string());
    assert_eq!(ty("&&u8").unwrap(), "const uint8_t* const*".to_string());
    assert_eq!(ty("*mut &u8").unwrap(), "const uint8_t* *".to_string());
    assert_eq!(ty("& &mut u8").unwrap(), "uint8_t* const*".to_string());
}

#[test]
fn test_translation_type_bare_fn() {
    assert_eq!(
        ty("fn(*mut u8, i16) -> *const char").unwrap(),
        "char const*(*)(uint8_t *, int16_t)".to_string()
    );
    assert_eq!(
        ty("*const fn(*mut u8, &mut [u8; 16]) -> &mut *mut u8").unwrap(),
        "uint8_t * *(*const)(uint8_t *, uint8_t (*) [16])".to_string()
    );
}

#[test]
fn test_translation_type_array() {
    assert_eq!(
        ty("[&u8; 2 + 2]").unwrap(),
        "const uint8_t*[2 + 2]".to_string()
    );
}

#[test]
fn test_translation_fails_for_unsupported() {
    assert!(ty("[&str; 2 + 2]").is_err());
    assert!(ty("fn(*mut [u8], i16) -> *const char").is_err());
}
