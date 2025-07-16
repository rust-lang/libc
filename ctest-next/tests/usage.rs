use std::env;

use ctest_next::{generate_test, Result, TestGenerator};

/// Create a test generator configured to useful settings for this test.
fn default_generator(opt_level: u8, header: &str) -> Result<TestGenerator> {
    env::set_var("OPT_LEVEL", opt_level.to_string());
    let mut generator = TestGenerator::new();
    generator.header(header);

    Ok(generator)
}

#[test]
fn test_missing_out_dir() {
    env::remove_var("OUT_DIR");

    let mut gen = default_generator(1, "macro.h").unwrap();
    gen.include("tests/input");

    let result = generate_test(&mut gen, "src/t1.rs", "out_dir_gen.rs");
    assert!(result.is_err(), "Expected error when OUT_DIR is missing");
}

#[test]
fn test_invalid_out_dir() {
    let mut gen = default_generator(1, "macro.h").unwrap();
    gen.out_dir("/nonexistent_dir").include("tests/input");

    assert!(
        generate_test(&mut gen, "tests/input/macro.rs", "out_path_gen.rs").is_err(),
        "Expected error with invalid output path"
    );
}

#[test]
fn test_non_existent_header() {
    let mut gen = default_generator(1, "macro.h").unwrap();
    let out_dir = tempfile::tempdir().unwrap();
    gen.out_dir(out_dir)
        .include("tests/input")
        .header("nonexistent_header.h");

    assert!(
        generate_test(&mut gen, "tests/input/macro.rs", "missing_header_gen.rs").is_err(),
        "Expected error with non-existent header"
    );
}

#[test]
fn test_invalid_include_path() {
    let mut gen = default_generator(1, "macro.h").unwrap();
    let out_dir = tempfile::tempdir().unwrap();
    gen.out_dir(out_dir).include("nonexistent_directory");

    assert!(
        generate_test(&mut gen, "tests/input/macro.rs", "invalid_include_gen.rs").is_err(),
        "Expected error with invalid include path"
    );
}
