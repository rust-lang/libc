//! Generation, compilation, and running of tests.

use std::env;
use std::fs::{File, canonicalize};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::generator::GenerationError;
use crate::{EDITION, Language, Result, TestGenerator, get_build_target};

/// Generate all tests for the given crate and output the Rust side to a file.
#[doc(hidden)]
pub fn generate_test(
    generator: &mut TestGenerator,
    crate_path: impl AsRef<Path>,
    output_file_path: impl AsRef<Path>,
) -> Result<PathBuf, GenerationError> {
    let output_file_path = generator.generate_files(crate_path, output_file_path)?;

    let target = get_build_target(generator)?;
    let host = env::var("HOST")
        .or_else(|_| env::var("HOST_PLATFORM"))
        .map_err(|_| GenerationError::EnvVarNotFound("HOST, HOST_PLATFORM".to_string()))?;

    let mut cfg = cc::Build::new();
    cfg.file(output_file_path.with_extension(generator.language.extension()));
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

    for p in &generator.includes {
        cfg.include(p);
    }

    for flag in &generator.flags {
        cfg.flag(flag);
    }

    for (k, v) in &generator.global_defines {
        cfg.define(k, v.as_ref().map(|s| &s[..]));
    }

    cfg.cpp(matches!(generator.language, Language::CXX));

    let stem: &str = output_file_path.file_stem().unwrap().to_str().unwrap();
    cfg.target(&target)
        .out_dir(output_file_path.parent().unwrap())
        .compile(stem);

    Ok(output_file_path)
}

/// Compiles a Rust source file and links it against a static library.
///
/// Returns the path to the generated binary.
#[doc(hidden)]
pub fn __compile_test(
    output_dir: impl AsRef<Path>,
    crate_path: impl AsRef<Path>,
    library_file: impl AsRef<Path>,
) -> Result<PathBuf> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let output_dir = output_dir.as_ref();
    let crate_path = crate_path.as_ref();
    let library_file = library_file.as_ref().file_stem().unwrap();

    let rust_file = output_dir
        .join(crate_path.file_stem().unwrap())
        .with_extension("rs");
    let binary_path = output_dir.join(rust_file.file_stem().unwrap());

    // Create a file that contains the Rust 'bindings' as well as the generated test code.
    File::create(&rust_file)?.write_all(
        format!(
            "include!(r#\"{}\"#);\ninclude!(r#\"{}.rs\"#);",
            canonicalize(crate_path)?.display(),
            library_file.to_str().unwrap()
        )
        .as_bytes(),
    )?;

    // Compile the test file with the compiled C library file found in `output_dir`
    // into a binary file, ignoring all warnings about unused items. (not all items
    // are currently tested)

    let mut cmd = Command::new(rustc);
    cmd.arg(&rust_file)
        .arg(format!("-Lnative={}", output_dir.display()))
        .arg(format!("-lstatic={}", library_file.to_str().unwrap()))
        .arg("--edition")
        .arg(EDITION) // Defaults to 2015.
        .arg("-o")
        .arg(&binary_path);

    // Pass in a different target, linker or flags if set, useful for cross compilation.

    let target = env::var("TARGET_PLATFORM").unwrap_or_default();
    if !target.is_empty() {
        cmd.arg("--target").arg(target);
    }

    let linker = env::var("LINKER").unwrap_or_default();
    if !linker.is_empty() {
        cmd.arg(format!("-Clinker={linker}"));
    }

    let flags = env::var("FLAGS").unwrap_or_default();
    if !flags.is_empty() {
        cmd.args(flags.split_whitespace());
    }

    let output = cmd.output()?;
    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)?;
        return Err(format!("compile test failed with {}: {}", output.status, stderr).into());
    }

    Ok(binary_path)
}

/// Executes the compiled test binary and returns its output.
///
/// If a RUNNER environment variable is present, it will use that to run the binary.
#[doc(hidden)]
pub fn __run_test<P: AsRef<Path>>(test_binary: P) -> Result<String> {
    let runner = env::var("RUNNER").unwrap_or_default();
    let mut cmd;
    if runner.is_empty() {
        cmd = Command::new(test_binary.as_ref());
    } else {
        let mut args = runner.split_whitespace();
        cmd = Command::new(args.next().unwrap());
        cmd.args(args);
    };

    cmd.arg(test_binary.as_ref());
    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)?;
        return Err(format!("run test failed with {}: {}", output.status, stderr).into());
    }

    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}
