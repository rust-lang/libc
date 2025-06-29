use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use askama::Template;
use syn::visit::Visit;

use crate::{
    expand,
    ffi_items::FfiItems,
    template::{CTestTemplate, RustTestTemplate},
    Result,
};

/// A builder used to generate a test suite.
#[derive(Default, Debug, Clone)]
pub struct TestGenerator {
    headers: Vec<String>,
    pub(crate) target: Option<String>,
    pub(crate) includes: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
}

impl TestGenerator {
    /// Creates a new blank test generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a header to be included as part of the generated C file.
    ///
    /// The generate C test will be compiled by a C compiler, and this can be
    /// used to ensure that all the necessary header files are included to test
    /// all FFI definitions.
    pub fn header(&mut self, header: &str) -> &mut Self {
        self.headers.push(header.to_string());
        self
    }

    /// Configures the target to compile C code for.
    ///
    /// Note that for Cargo builds this defaults to `$TARGET` and it's not
    /// necessary to call.
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }

    /// Add a path to the C compiler header lookup path.
    ///
    /// This is useful for if the C library is installed to a nonstandard
    /// location to ensure that compiling the C file succeeds.
    pub fn include<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.includes.push(p.as_ref().to_owned());
        self
    }

    /// Configures the output directory of the generated Rust and C code.
    pub fn out_dir<P: AsRef<Path>>(&mut self, p: P) -> &mut Self {
        self.out_dir = Some(p.as_ref().to_owned());
        self
    }

    /// Generate the Rust and C testing files.
    ///
    /// Returns the path to t generated file.
    pub fn generate_files(
        &mut self,
        crate_path: impl AsRef<Path>,
        output_file_path: impl AsRef<Path>,
    ) -> Result<PathBuf> {
        let expanded = expand(crate_path)?;
        let ast = syn::parse_file(&expanded)?;

        let mut ffi_items = FfiItems::new();
        ffi_items.visit_file(&ast);

        let output_directory = self
            .out_dir
            .clone()
            .unwrap_or_else(|| env::var("OUT_DIR").unwrap().into());
        let output_file_path = output_directory.join(output_file_path);

        // Generate the Rust side of the tests.
        File::create(output_file_path.with_extension("rs"))?
            .write_all(RustTestTemplate::new(&ffi_items)?.render()?.as_bytes())?;

        // Generate the C side of the tests.
        // FIXME(ctest): Cpp not supported yet.
        let c_output_path = output_file_path.with_extension("c");
        let headers = self.headers.iter().map(|h| h.as_str()).collect();
        File::create(&c_output_path)?
            .write_all(CTestTemplate::new(headers, &ffi_items).render()?.as_bytes())?;

        Ok(output_file_path)
    }
}
