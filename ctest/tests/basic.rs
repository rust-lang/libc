use std::fs::File;
use std::io::Write;
use std::path::{
    Path,
    PathBuf,
};
use std::process::Command;
use std::{
    env,
    fs,
};

use ctest::{
    Result,
    TestGenerator,
};
use pretty_assertions::assert_eq;

// Headers are found relevative to the include directory, all files are generated
// relative to the output directory.

/// Create a test generator configured to useful settings.
///
/// The files will be generated in a unique temporary directory that gets
/// deleted when it goes out of scope.
fn default_generator(
    opt_level: u8,
    header: Option<&str>,
) -> Result<(TestGenerator, tempfile::TempDir)> {
    // FIXME(mbyx): Remove this in favor of not-unsafe alternatives.
    unsafe { env::set_var("OPT_LEVEL", opt_level.to_string()) };
    let temp_dir = tempfile::tempdir()?;
    let mut generator = TestGenerator::new();
    generator.out_dir(&temp_dir).include("tests/input");
    if let Some(header) = header {
        generator.header(header);
    }

    Ok((generator, temp_dir))
}

/// Assert whether the contents of two files match.
///
/// If the contents do not match and LIBC_BLESS is set, overwrite the
/// test file with the content of the generated file.
fn bless_equal(new_file: impl AsRef<Path>, old_file: impl AsRef<Path>) {
    let new_content = fs::read_to_string(&new_file).unwrap().replace("\r", "");
    if env::var("LIBC_BLESS").is_ok() {
        fs::write(&old_file, &new_content).unwrap();
        return;
    }
    let old_content = fs::read_to_string(&old_file).unwrap().replace("\r", "");

    assert_eq!(
        new_content, old_content,
        "the template file has changed. Please run the tests with `LIBCBLESS=1`."
    );
}

/// Generate test files for the given header and crate path and compare with pregenerated test files.
///
/// If LIBC_BLESS is set, it will overwrite the pregenerated files with the new ones.
/// Additionally, if this test is not being ran on a cross compiled target, it will compile
/// and run the generated tests as well.
fn check_entrypoint(
    gen_: &mut TestGenerator,
    out_dir: tempfile::TempDir,
    crate_path: impl AsRef<Path>,
    test_name: &str,
    include_path: impl AsRef<Path>,
) {
    let output_file = gen_.generate_files(&crate_path, test_name).unwrap();

    let rs = include_path
        .as_ref()
        .join(Path::new(test_name).with_extension("rs"));
    let c = include_path
        .as_ref()
        .join(Path::new(test_name).with_extension("c"));

    bless_equal(output_file.with_extension("rs"), rs);
    bless_equal(output_file.with_extension("c"), c);

    if env::var("TARGET_PLATFORM") == env::var("HOST_PLATFORM") {
        gen_.build_test(&crate_path, test_name);
        let test_binary = compile_test(&out_dir, crate_path, test_name).unwrap();
        let result = run_test(test_binary);
        if let Err(err) = &result {
            eprintln!("Test failed: {err:?}");
        }
        assert!(result.is_ok());
    }
}

/// Test if a hierarchy of modules generates tests properly.
#[test]
fn test_entrypoint_hierarchy() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("hierarchy/lib.rs");
    let library_path = "hierarchy.out.a";

    let (mut gen_, out_dir) = default_generator(1, Some("hierarchy.h")).unwrap();
    check_entrypoint(&mut gen_, out_dir, crate_path, library_path, include_path);
}

/// Test if every type can be skipped.
#[test]
fn test_skip_simple() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("simple.rs");
    let library_path = "simple.out.with-skips.a";

    let (mut gen_, out_dir) = default_generator(1, Some("simple.h")).unwrap();
    gen_.skip_const(|c| c.ident() == "B" || c.ident() == "A")
        .skip_c_enum(|e| e == "Color")
        .skip_alias(|a| a.ident() == "Byte" || a.ident() == "gregset_t")
        .skip_struct(|s| s.ident() == "Person")
        .skip_union(|u| u.ident() == "Word")
        .skip_fn(|f| f.ident() == "calloc")
        .skip_static(|s| s.ident() == "byte");

    check_entrypoint(&mut gen_, out_dir, crate_path, library_path, include_path);
}

/// Test if a type can be renamed.
#[test]
fn test_map_simple() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("simple.rs");
    let library_path = "simple.out.with-renames.a";

    let (mut gen_, out_dir) = default_generator(1, Some("simple.h")).unwrap();
    gen_.rename_constant(|c| (c.ident() == "B").then(|| "C_B".to_string()))
        .alias_is_c_enum(|e| e == "Color")
        .skip_signededness(|ty| ty == "Color");

    check_entrypoint(&mut gen_, out_dir, crate_path, library_path, include_path);
}

/// Test if macros are expanded properly.
#[test]
fn test_entrypoint_macro() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("macro.rs");
    let library_path = "macro.out.a";

    let (mut gen_, out_dir) = default_generator(1, None).unwrap();
    gen_.header_with_defines("macro.h", vec!["SUPPRESS_ERROR"]);

    check_entrypoint(&mut gen_, out_dir, crate_path, library_path, include_path);
}

/// Test if generated code for macro.rs passes requirements for edition 2024.
#[test]
fn test_edition_2024_macro() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("macro.rs");
    let library_path = "macro.out.edition-2024.a";

    let (mut gen_, out_dir) = default_generator(1, None).unwrap();
    gen_.edition(2024)
        .header_with_defines("macro.h", vec!["SUPPRESS_ERROR"]);

    check_entrypoint(&mut gen_, out_dir, crate_path, library_path, include_path);
}

/// Test if a file with invalid syntax fails to generate tests.
#[test]
fn test_entrypoint_invalid_syntax() {
    let crate_path = "tests/input/invalid_syntax.rs";
    let mut gen_ = TestGenerator::new();

    let fails = gen_
        .try_build_test(crate_path, "invalid_syntax.rs")
        .is_err();

    assert!(fails)
}

/// Test if a field with a raw identifier successfully has tests generated.
#[test]
fn test_raw_identifier_field() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("raw_ident.rs");
    let library_path = "raw_ident.out.rs";

    let (mut gen_, out_dir) = default_generator(1, Some("raw_ident.h")).unwrap();
    gen_.rename_struct_ty(|ty| Some(ty.to_string()));
    let output_file = gen_.generate_files(&crate_path, library_path).unwrap();

    let rust_output = fs::read_to_string(output_file.with_extension("rs")).unwrap();
    let c_output = fs::read_to_string(output_file.with_extension("c")).unwrap();

    assert!(rust_output.contains("(*uninit_ty).r#type"));
    assert!(rust_output.contains("offset_of!(RawIdent, r#type)"));
    assert!(rust_output.contains("ctest_offset_of__RawIdent__type"));
    assert!(rust_output.contains("ctest_field_ptr__RawIdent__type"));

    assert!(c_output.contains(", type)"));
    assert!(c_output.contains("->type"));
    assert!(!c_output.contains("r#type"));
    assert!(c_output.contains("ctest_offset_of__RawIdent__type"));
    assert!(c_output.contains("ctest_field_ptr__RawIdent__type"));

    if env::var("TARGET_PLATFORM") == env::var("HOST_PLATFORM") {
        gen_.build_test(&crate_path, library_path);
        let test_binary = compile_test(&out_dir, crate_path, library_path).unwrap();
        let result = run_test(test_binary);
        if let Err(err) = &result {
            eprintln!("Test failed: {err:?}");
        }
        assert!(result.is_ok());
    }
}

#[test]
fn test_mismatched_union_field_ty() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("mismatched_union_field_ty.rs");
    let library_path = "mismatched_union_field_ty.out.rs";

    let (mut gen_, _out_dir) = default_generator(1, Some("mismatched_union_field_ty.h")).unwrap();

    if env::var("TARGET_PLATFORM") == env::var("HOST_PLATFORM") {
        let result = gen_.try_build_test(&crate_path, library_path);
        assert!(result.is_err());
        // As cc prints its warning/error messages to stderr, we cannot access them from cc::Error,
        // and so cannot assert that the error was actually due to -Wincompatible-pointer-types.
    }
}

#[test]
fn test_mismatched_struct_field_ty() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("mismatched_struct_field_ty.rs");
    let library_path = "mismatched_struct_field_ty.out.rs";

    let (mut gen_, _out_dir) = default_generator(1, Some("mismatched_struct_field_ty.h")).unwrap();

    if env::var("TARGET_PLATFORM") == env::var("HOST_PLATFORM") {
        let result = gen_.try_build_test(&crate_path, library_path);
        assert!(result.is_err());
        // FIXME(ctest): As cc prints its warning/error messages to stderr, we cannot access them from cc::Error,
        // and so cannot assert that the error was actually due to -Wincompatible-pointer-types.
    }
}

#[test]
fn test_function_signatures() {
    check_function_signatures(ctest::Language::C);
}

#[test]
fn test_function_signatures_cpp() {
    check_function_signatures(ctest::Language::CXX);
}

fn check_function_signatures(language: ctest::Language) {
    if env::var("TARGET_PLATFORM") != env::var("HOST_PLATFORM") {
        return;
    }

    for (index, (header, signature, should_compile)) in [
        (
            "int signature(int);",
            "pub fn signature(arg: i32) -> i32;",
            true,
        ),
        ("int signature(int);", "pub fn signature();", false),
        (
            "int signature(int);",
            "pub fn signature(arg: f64) -> i32;",
            false,
        ),
        (
            "int signature(int);",
            "pub fn signature(arg: i32) -> f64;",
            false,
        ),
        (
            "int signature(int);",
            "pub fn signature(arg: i32, ...) -> i32;",
            false,
        ),
        (
            "int signature(int, ...);",
            "pub fn signature(arg: i32, ...) -> i32;",
            true,
        ),
        (
            "int signature(int, ...);",
            "pub fn signature(arg: i32) -> i32;",
            false,
        ),
        ("void signature(void);", "pub fn signature();", true),
        (
            "void signature(const int *);",
            "pub fn signature(arg: *const i32);",
            true,
        ),
        (
            "void signature(const int *);",
            "pub fn signature(arg: *mut i32);",
            false,
        ),
        (
            "void signature(int (*)(int));",
            "pub fn signature(arg: unsafe extern \"C\" fn(i32) -> i32);",
            true,
        ),
        (
            "void signature(int (*)(int));",
            "pub fn signature(arg: unsafe extern \"C\" fn(f64) -> i32);",
            false,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let (mut gen_, out_dir) = default_generator(1, Some("signature.h")).unwrap();
        gen_.include(out_dir.path()).language(language.clone());
        fs::write(out_dir.path().join("signature.h"), header).unwrap();
        let crate_path = out_dir.path().join(format!("signature_{index}.rs"));
        fs::write(&crate_path, format!("extern \"C\" {{ {signature} }}")).unwrap();

        let result = gen_.try_build_test(&crate_path, "signature.out.rs");
        assert_eq!(result.is_ok(), should_compile, "{signature}: {result:?}");
    }
}

/// Compiles a Rust source file and links it against a static library.
///
/// Returns the path to the generated binary.
#[doc(hidden)]
pub fn compile_test(
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
            fs::canonicalize(crate_path)?.display(),
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
        .arg("2021")
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
pub fn run_test<P: AsRef<Path>>(test_binary: P) -> Result<String> {
    let runner = env::var("RUNNER").unwrap_or_default();
    let mut cmd;
    if runner.is_empty() {
        cmd = Command::new(test_binary.as_ref());
    } else {
        let mut args = runner.split_whitespace();
        cmd = Command::new(args.next().unwrap());
        cmd.args(args);
    }

    cmd.arg(test_binary.as_ref());
    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)?;
        return Err(format!("run test failed with {}: {}", output.status, stderr).into());
    }

    Ok(std::str::from_utf8(&output.stdout)?.to_string())
}
