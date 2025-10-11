use std::path::{Path, PathBuf};
use std::{env, fs};

use ctest::{__compile_test, __run_test, Result, TestGenerator, generate_test};
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
    library_path: impl AsRef<Path>,
    include_path: impl AsRef<Path>,
) {
    let output_file = gen_.generate_files(&crate_path, &library_path).unwrap();

    let rs = include_path
        .as_ref()
        .join(library_path.as_ref().with_extension("rs"));
    let c = include_path
        .as_ref()
        .join(library_path.as_ref().with_extension("c"));

    bless_equal(output_file.with_extension("rs"), rs);
    bless_equal(output_file.with_extension("c"), c);

    if env::var("TARGET_PLATFORM") == env::var("HOST_PLATFORM") {
        generate_test(gen_, &crate_path, &library_path).unwrap();
        let test_binary = __compile_test(&out_dir, crate_path, library_path).unwrap();
        let result = __run_test(test_binary);
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

/// Test if a file with invalid syntax fails to generate tests.
#[test]
fn test_entrypoint_invalid_syntax() {
    let crate_path = "tests/input/invalid_syntax.rs";
    let mut gen_ = TestGenerator::new();

    let fails = generate_test(&mut gen_, crate_path, "invalid_syntax.out").is_err();

    assert!(fails)
}
