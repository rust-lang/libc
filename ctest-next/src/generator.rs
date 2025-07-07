use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use askama::Template;
use syn::visit::Visit;
use thiserror::Error;

use crate::{
    expand,
    ffi_items::FfiItems,
    template::{CTestTemplate, RustTestTemplate},
};

#[derive(Debug, Error)]
pub enum GenerationError {
    #[error("unable to expand crate {0}: {1}")]
    MacroExpansion(PathBuf, String),
    #[error("unable to parse expanded crate {0}: {1}")]
    RustSyntax(String, String),
    #[error("unable to render {0} template: {1}")]
    TemplateRender(String, String),
    #[error("unable to create or write template file: {0}")]
    OsError(std::io::Error),
}

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
    ) -> Result<PathBuf, GenerationError> {
        let expanded = expand(&crate_path).map_err(|e| {
            GenerationError::MacroExpansion(crate_path.as_ref().to_path_buf(), e.to_string())
        })?;
        let ast = syn::parse_file(&expanded)
            .map_err(|e| GenerationError::RustSyntax(expanded, e.to_string()))?;

        let mut ffi_items = FfiItems::new();
        ffi_items.visit_file(&ast);

        let output_directory = self
            .out_dir
            .clone()
            .unwrap_or_else(|| env::var("OUT_DIR").unwrap().into());
        let output_file_path = output_directory.join(output_file_path);

        // Generate the Rust side of the tests.
        File::create(output_file_path.with_extension("rs"))
            .map_err(GenerationError::OsError)?
            .write_all(
                RustTestTemplate::new(&ffi_items)
                    .render()
                    .map_err(|e| {
                        GenerationError::TemplateRender("Rust".to_string(), e.to_string())
                    })?
                    .as_bytes(),
            )
            .map_err(GenerationError::OsError)?;

        // Generate the C side of the tests.
        // FIXME(ctest): Cpp not supported yet.
        let c_output_path = output_file_path.with_extension("c");
        let headers = self.headers.iter().map(|h| h.as_str()).collect();
        File::create(&c_output_path)
            .map_err(GenerationError::OsError)?
            .write_all(
                CTestTemplate::new(headers, &ffi_items)
                    .render()
                    .map_err(|e| GenerationError::TemplateRender("C".to_string(), e.to_string()))?
                    .as_bytes(),
            )
            .map_err(GenerationError::OsError)?;

        Ok(output_file_path)
    }
}
