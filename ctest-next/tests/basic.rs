use std::env;

use ctest_next::{compile_test, run_test, Result, TestGenerator};

// TODO: Create a unique directory within the temp_dir where all this is done
// to prevent name collisions.

// Headers are found relevative to the include directory, all files are generated
// relative to the output directory.

/// Create a test generator configured to useful settings.
fn default_generator(opt_level: u8) -> Result<TestGenerator> {
    env::set_var("OPT_LEVEL", opt_level.to_string());
    Ok(TestGenerator::new()
        .out_dir(env::temp_dir())
        .include("tests/input")
        .clone())
}

#[test]
fn test_entrypoint_hierarchy() {
    let crate_path = "tests/input/hierarchy/lib.rs";

    default_generator(1)
        .unwrap()
        .header("hierarchy.h")
        .generate(crate_path, "hierarchy_out.rs")
        .unwrap();

    let test_binary = compile_test(
        env::temp_dir().to_str().unwrap(),
        crate_path,
        "hierarchy_out",
    )
    .unwrap();

    assert!(run_test(test_binary.to_str().unwrap()).is_ok());
}

#[test]
fn test_entrypoint_simple() {
    let crate_path = "tests/input/simple.rs";

    default_generator(1)
        .unwrap()
        .header("simple.h")
        .generate(crate_path, "simple_out.rs")
        .unwrap();

    let test_binary =
        compile_test(env::temp_dir().to_str().unwrap(), crate_path, "simple_out").unwrap();

    assert!(run_test(test_binary.to_str().unwrap()).is_ok());
}

#[test]
fn test_entrypoint_macro() {
    let crate_path = "tests/input/macro.rs";

    default_generator(1)
        .unwrap()
        .header("macro.h")
        .generate(crate_path, "macro_out.rs")
        .unwrap();

    let test_binary =
        compile_test(env::temp_dir().to_str().unwrap(), crate_path, "macro_out").unwrap();

    assert!(run_test(test_binary.to_str().unwrap()).is_ok());
}

#[test]
fn test_entrypoint_invalid_syntax() {
    let mut generator = TestGenerator::new();

    let fails = generator
        .generate("./tests/input/invalid_syntax.rs", "invalid_syntax_out.rs")
        .is_err();

    assert!(fails)
}
