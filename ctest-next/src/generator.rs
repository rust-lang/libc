use std::path::Path;

use syn::visit::Visit;

use crate::{expand, ffi_items::FfiItems, Result};

/// A builder used to generate a test suite.
#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub struct TestGenerator {}

impl TestGenerator {
    /// Creates a new blank test generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate all tests for the given crate and output the Rust side to a file.
    pub fn generate<P: AsRef<Path>>(&mut self, crate_path: P, _output_file_path: P) -> Result<()> {
        let expanded = expand(crate_path)?;
        let ast = syn::parse_file(&expanded)?;

        let mut ffi_items = FfiItems::new();
        ffi_items.visit_file(&ast);

        Ok(())
    }
}
