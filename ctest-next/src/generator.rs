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
#[non_exhaustive]
#[derive(Default, Debug, Clone)]
pub struct TestGenerator {
    headers: Vec<String>,
    target: Option<String>,
    host: Option<String>,
    includes: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
}

impl TestGenerator {
    /// Creates a new blank test generator.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a header to be included as part of the generated C file.
    pub fn header(&mut self, header: &str) -> &mut Self {
        self.headers.push(header.to_string());
        self
    }

    /// Configures the target to compile C code for.
    pub fn target(&mut self, target: &str) -> &mut Self {
        self.target = Some(target.to_string());
        self
    }

    /// Configures the host.
    pub fn host(&mut self, host: &str) -> &mut Self {
        self.host = Some(host.to_string());
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

    /// Generate all tests for the given crate and output the Rust side to a file.
    pub fn generate<P: AsRef<Path>>(&mut self, crate_path: P, output_file_path: P) -> Result<()> {
        let output_file_path = self.generate_files(crate_path, output_file_path)?;

        let target = env::var("TARGET").ok().or(self.target.clone()).unwrap();
        let host = env::var("HOST").ok().or(self.host.clone()).unwrap();

        let mut cfg = cc::Build::new();
        // FIXME: Cpp not supported.
        cfg.file(output_file_path.with_extension("c"));
        cfg.host(&host);
        if target.contains("msvc") {
            cfg.flag("/W3")
                .flag("/Wall")
                .flag("/WX")
                // ignored warnings
                .flag("/wd4820") // warning about adding padding?
                .flag("/wd4100") // unused parameters
                .flag("/wd4996") // deprecated functions
                .flag("/wd4296") // '<' being always false
                .flag("/wd4255") // converting () to (void)
                .flag("/wd4668") // using an undefined thing in preprocessor?
                .flag("/wd4366") // taking ref to packed struct field might be unaligned
                .flag("/wd4189") // local variable initialized but not referenced
                .flag("/wd4710") // function not inlined
                .flag("/wd5045") // compiler will insert Spectre mitigation
                .flag("/wd4514") // unreferenced inline function removed
                .flag("/wd4711"); // function selected for automatic inline
        } else {
            cfg.flag("-Wall")
                .flag("-Wextra")
                .flag("-Werror")
                .flag("-Wno-unused-parameter")
                .flag("-Wno-type-limits")
                // allow taking address of packed struct members:
                .flag("-Wno-address-of-packed-member")
                .flag("-Wno-unknown-warning-option")
                .flag("-Wno-deprecated-declarations"); // allow deprecated items
        }

        for p in &self.includes {
            cfg.include(p);
        }

        let stem: &str = output_file_path.file_stem().unwrap().to_str().unwrap();
        cfg.target(&target)
            .out_dir(output_file_path.parent().unwrap())
            .compile(stem);

        Ok(())
    }

    /// Generate the Rust and C testing files.
    pub(crate) fn generate_files<P: AsRef<Path>>(
        &mut self,
        crate_path: P,
        output_file_path: P,
    ) -> Result<PathBuf> {
        let expanded = expand(crate_path)?;
        let ast = syn::parse_file(&expanded)?;

        let mut ffi_items = FfiItems::new();
        ffi_items.visit_file(&ast);

        let output_directory = self
            .out_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(env::var_os("OUT_DIR").unwrap()));
        let output_file_path = output_directory.join(output_file_path);

        // Generate the Rust side of the tests.
        File::create(&output_file_path)?
            .write_all(RustTestTemplate::new(&ffi_items)?.render()?.as_bytes())?;

        // Generate the C side of the tests.
        // FIXME: Cpp not supported yet.
        let c_output_path = output_file_path.with_extension("c");
        let headers = self.headers.iter().map(|h| h.as_str()).collect();
        File::create(&c_output_path)?
            .write_all(CTestTemplate::new(headers, &ffi_items).render()?.as_bytes())?;

        Ok(output_file_path)
    }
}
