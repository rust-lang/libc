use std::{
    env, fs,
    path::{Path, PathBuf},
};

use pretty_assertions::assert_eq;

use ctest_next::{Result, TestGenerator, __compile_test, __run_test, generate_test};

// Headers are found relevative to the include directory, all files are generated
// relative to the output directory.

/// Create a test generator configured to useful settings.
///
/// The files will be generated in a unique temporary directory that gets
/// deleted when it goes out of scope.
fn default_generator(opt_level: u8, header: &str) -> Result<(TestGenerator, tempfile::TempDir)> {
    env::set_var("OPT_LEVEL", opt_level.to_string());
    let temp_dir = tempfile::tempdir()?;
    let mut generator = TestGenerator::new();

    Ok((
        generator
            .out_dir(&temp_dir)
            .include("tests/input")
            .header(header)
            .to_owned(),
        temp_dir,
    ))
}

/// Assert whether the contents of two files match.
///
/// If the contents do not match and LIBC_BLESS is set, overwrite the
/// test file with the content of the generated file.
fn bless_equal(new_file: impl AsRef<Path>, old_file: impl AsRef<Path>) {
    let new_content = fs::read_to_string(&new_file).unwrap().replace("\r", "");
    let old_content = fs::read_to_string(&old_file).unwrap().replace("\r", "");

    let equal = new_content != old_content;
    if env::var("LIBC_BLESS").is_ok() && !equal {
        fs::write(old_file, &new_content).unwrap();
    } else {
        // Use pretty_assertions for easier diffs.
        assert_eq!(new_content, old_content);
    }
}

/// Generate test files for the given header and crate path and compare with pregenerated test files.
///
/// If LIBC_BLESS is set, it will overwrite the pregenerated files with the new ones.
/// Additionally, if this test is not being ran on a cross compiled target, it will compile
/// and run the generated tests as well.
fn check_entrypoint(
    header_name: &str,
    crate_path: impl AsRef<Path>,
    library_path: impl AsRef<Path>,
    include_path: impl AsRef<Path>,
) {
    let (mut gen, out_dir) = default_generator(1, header_name).unwrap();
    let output_file = gen.generate_files(&crate_path, &library_path).unwrap();

    let rs = include_path
        .as_ref()
        .join(library_path.as_ref().with_extension("rs"));
    let c = include_path
        .as_ref()
        .join(library_path.as_ref().with_extension("c"));

    bless_equal(output_file.with_extension("rs"), rs);
    bless_equal(output_file.with_extension("c"), c);

    if env::var("TARGET_PLATFORM") == env::var("HOST_PLATFORM") {
        generate_test(&mut gen, &crate_path, &library_path).unwrap();
        let test_binary = __compile_test(&out_dir, crate_path, library_path).unwrap();
        let result = __run_test(test_binary);
        if let Err(err) = &result {
            eprintln!("Test failed: {err:?}");
        }
        assert!(result.is_ok());
    }
}

#[test]
fn test_entrypoint_hierarchy() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("hierarchy/lib.rs");
    let library_path = "hierarchy.out.a";

    check_entrypoint("hierarchy.h", crate_path, library_path, include_path);
}

#[test]
fn test_entrypoint_simple() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("simple.rs");
    let library_path = "simple.out.a";

    check_entrypoint("simple.h", crate_path, library_path, include_path);
}

#[test]
fn test_entrypoint_macro() {
    let include_path = PathBuf::from("tests/input");
    let crate_path = include_path.join("macro.rs");
    let library_path = "macro.out.a";

    check_entrypoint("macro.h", crate_path, library_path, include_path);
}

#[test]
fn test_entrypoint_invalid_syntax() {
    let crate_path = "tests/input/invalid_syntax.rs";
    let mut gen = TestGenerator::new();

    let fails = generate_test(&mut gen, crate_path, "invalid_syntax.out").is_err();

    assert!(fails)
}
