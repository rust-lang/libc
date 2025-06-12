use std::path::Path;

use crate::{expand, Result};

/// A builder used to generate a test suite.
#[non_exhaustive]
pub struct TestGenerator {}

impl Default for TestGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl TestGenerator {
    /// Creates a new blank test generator.
    pub fn new() -> Self {
        Self {}
    }

    /// Generate all tests for the given crate and output the Rust side to a file.
    pub fn generate<P: AsRef<Path>>(&self, crate_path: P, _output_file_path: P) -> Result<()> {
        let _expanded = expand(crate_path)?;
        Ok(())
    }
}
