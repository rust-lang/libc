use crate::ffi_items::FfiItems;

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

#[test]
fn test_extraction_ffi_items() {
    let ast = syn::parse_file(ALL_ITEMS).unwrap();

    let mut ffi_items = FfiItems::new();
    ffi_items.visit_file(&ast);

    assert_eq!(
        ffi_items
            .aliases()
            .iter()
            .map(|a| a.ident())
            .collect::<Vec<_>>(),
        ["Foo"]
    );

    assert_eq!(
        ffi_items
            .constants()
            .iter()
            .map(|a| a.ident())
            .collect::<Vec<_>>(),
        ["bar"]
    );

    assert_eq!(
        ffi_items
            .foreign_functions()
            .iter()
            .map(|a| a.ident())
            .collect::<Vec<_>>(),
        ["malloc"]
    );

    assert_eq!(
        ffi_items
            .foreign_statics()
            .iter()
            .map(|a| a.ident())
            .collect::<Vec<_>>(),
        ["baz"]
    );

    assert_eq!(
        ffi_items
            .structs()
            .iter()
            .map(|a| a.ident())
            .collect::<Vec<_>>(),
        ["Array"]
    );

    assert_eq!(
        ffi_items
            .unions()
            .iter()
            .map(|a| a.ident())
            .collect::<Vec<_>>(),
        ["Word"]
    );
}
