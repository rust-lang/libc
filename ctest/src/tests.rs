use syn::spanned::Spanned;
use syn::visit::Visit;

use crate::ffi_items::FfiItems;
use crate::translator::{TranslationErrorKind, Translator};
use crate::{Result, TestGenerator, TranslationError, cdecl};

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

/// Translate a Rust type into a c typedef declaration.
fn r2cdecl(s: &str, name: &str) -> Result<String, TranslationError> {
    let ffi_items = FfiItems::new();
    let generator = TestGenerator::new();
    let translator = Translator::new(&ffi_items, &generator);
    let ty: syn::Type = syn::parse_str(s).unwrap();
    let translated = translator.translate_type(&ty)?;
    cdecl::cdecl(&translated, name.to_string())
        .map_err(|_| TranslationError::new(TranslationErrorKind::InvalidReturn, s, ty.span()))
}

#[track_caller]
fn assert_r2cdecl(rust: &str, expected: &str) {
    assert_eq!(r2cdecl(rust, "foo").unwrap(), expected)
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
    assert_r2cdecl("*const *mut i32", "int32_t *const *foo");
    assert_r2cdecl("*const [u128; 2 + 3]", "unsigned __int128 (*foo)[2 + 3]");
    assert_r2cdecl("*const *mut [u8; 5]", "uint8_t (*const *foo)[5]");
    assert_r2cdecl("*mut *const [u8; 5]", "uint8_t (**foo)[5]");
    assert_r2cdecl("*const *const [u8; 5]", "uint8_t (*const *foo)[5]");
    assert_r2cdecl("*mut *mut [u8; 5]", "uint8_t (**foo)[5]");
}

#[test]
fn test_translation_type_reference() {
    assert_r2cdecl("&u8", "const uint8_t *foo");
    assert_r2cdecl("&&u8", "const uint8_t *const *foo");
    assert_r2cdecl("*mut &u8", "const uint8_t **foo");
    assert_r2cdecl("& &mut u8", "uint8_t *const *foo");
}

#[test]
fn test_translation_type_bare_fn() {
    assert_r2cdecl(
        "fn(*mut u8, i16) -> *const char",
        "const char *(*foo)(uint8_t *, int16_t)",
    );
    assert_r2cdecl(
        "*const fn(*mut u8, &mut [u8; 16]) -> &mut *mut u8",
        "uint8_t **(*const *foo)(uint8_t *, uint8_t (*)[16])",
    );
    assert_r2cdecl(
        "extern \"C\" fn(c_int) -> *const c_void",
        "const void *(*foo)(int)",
    );
    // FIXME(ctest): Reimplement support for ABI in a more robust way.
    // assert_r2cdecl(
    //     "Option<extern \"stdcall\" fn(*const c_char, [u32; 16]) -> u8>",
    //     "uint8_t (__stdcall **foo)(const char *, uint32_t [16])"
    // );
}

#[test]
fn test_translation_type_array() {
    assert_r2cdecl(
        "[&u8; crate::ERRNO as usize + 2]",
        "const uint8_t *foo[(size_t)ERRNO + 2]",
    );
    assert_eq!(
        r2cdecl("[u8; 10usize]", "foo").unwrap(),
        "uint8_t foo[(size_t)10]"
    );
    assert_r2cdecl("[u8; 10]", "uint8_t foo[10]");
    assert_r2cdecl("[[u8; 64]; 32]", "uint8_t foo[32][64]");
}

#[test]
fn test_translation_fails_for_unsupported() {
    assert!(r2cdecl("[&str; 2 + 2]", "").is_err());
    assert!(r2cdecl("fn(*mut [u8], i16) -> *const char", "").is_err());
}
