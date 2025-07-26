use syn::visit::Visit;

use crate::ffi_items::FfiItems;
use crate::template::TranslateHelper;
use crate::translator::Translator;
use crate::{Result, TestGenerator, TranslationError};

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

/// Translate a Rust type to C.
fn ty(s: &str) -> Result<String, TranslationError> {
    let translator = Translator {};
    let ty: syn::Type = syn::parse_str(s).unwrap();
    translator.translate_type(&ty)
}

/// Translate a Rust type into a c typedef declaration.
fn cdecl(s: &str) -> Result<String, TranslationError> {
    let ty: syn::Type = syn::parse_str(s).unwrap();
    let ffi_items = FfiItems::new();
    let generator = TestGenerator::new();
    let helper = TranslateHelper::new(&ffi_items, &generator);

    helper.make_cdecl("test_make_cdecl", &ty)
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

#[test]
fn test_translate_helper_function_pointer() {
    assert_eq!(
        cdecl("extern \"C\" fn(c_int) -> *const c_void").unwrap(),
        "void const* (**test_make_cdecl)(int)"
    );
    assert_eq!(
        cdecl("Option<extern \"stdcall\" fn(*const c_char, [u32; 16]) -> u8>").unwrap(),
        "uint8_t (__stdcall **test_make_cdecl)(char const*, uint32_t[16])"
    );
}

#[test]
fn test_translate_helper_array_1d_2d() {
    assert_eq!(cdecl("[u8; 10]").unwrap(), "uint8_t (*test_make_cdecl)[10]");
    assert_eq!(
        cdecl("[[u8; 64]; 32]").unwrap(),
        "uint8_t (*test_make_cdecl)[32][64]"
    );
}
