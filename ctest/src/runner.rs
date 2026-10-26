//! Generation, compilation, and running of tests.

use std::env;
use std::path::{
    Path,
    PathBuf,
};

use crate::generator::GenerationError;
use crate::{
    Language,
    Result,
    TestGenerator,
    get_build_target,
};

/// Generate all tests for the given crate and output the Rust side to a file.
#[doc(hidden)]
#[deprecated(note = "use `cfg.build_test()` instead")]
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
    cfg.target(&target);

    if target.contains("msvc") {
        cfg.flag("/W3")
            .flag("/Wall")
            .flag("/WX")
            // ignored warnings
            .flag("/wd4746") // volatile use, seems to be enabled by default on aarch64
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
            .flag("-Wno-deprecated-declarations") // allow deprecated items
            // Probe support instead of assuming a compiler family. This keeps
            // the suppression on newer GCC/Clang while avoiding regressions on
            // older toolchains that reject these flags.
            .flag_if_supported("-Wno-address-of-packed-member")
            .flag_if_supported("-Wno-unknown-warning-option");
    }

    for p in &generator.includes {
        cfg.include(p);
    }

    for flag in &generator.flags {
        cfg.flag(flag);
    }

    for flag in &generator.flags_if_supported {
        cfg.flag_if_supported(flag);
    }

    for (k, v) in &generator.global_defines {
        cfg.define(k, v.as_ref().map(|s| &s[..]));
    }

    cfg.cpp(matches!(generator.language, Language::CXX));

    let stem: &str = output_file_path.file_stem().unwrap().to_str().unwrap();
    cfg.out_dir(output_file_path.parent().unwrap())
        .try_compile(stem)
        .map_err(GenerationError::CompileError)?;

    Ok(output_file_path)
}
