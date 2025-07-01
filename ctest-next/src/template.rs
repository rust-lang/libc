use askama::Template;
use quote::ToTokens;

use crate::{ffi_items::FfiItems, translator::Translator};

/// Represents the Rust side of the generated testing suite.
#[derive(Template, Debug, Clone)]
#[template(path = "test.rs")]
pub(crate) struct RustTestTemplate<'a> {
    ffi_items: &'a FfiItems,
}

/// Represents the C side of the generated testing suite.
#[derive(Template, Debug, Clone)]
#[template(path = "test.c")]
pub(crate) struct CTestTemplate<'a> {
    translator: Translator,
    headers: Vec<&'a str>,
    ffi_items: &'a FfiItems,
}

impl<'a> RustTestTemplate<'a> {
    /// Create a new test template to test the given items.
    pub(crate) fn new(ffi_items: &'a FfiItems) -> Self {
        Self { ffi_items }
    }
}

impl<'a> CTestTemplate<'a> {
    /// Create a new test template to test the given items.
    pub(crate) fn new(headers: Vec<&'a str>, ffi_items: &'a FfiItems) -> Self {
        Self {
            headers,
            ffi_items,
            translator: Translator::new(),
        }
    }
}
