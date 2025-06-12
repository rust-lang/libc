use ctest_next::TestGenerator;

#[test]
fn test_entrypoint_hierarchy() {
    let generator = TestGenerator::new();

    generator
        .generate("./tests/input/hierarchy/lib.rs", "hierarchy_out.rs")
        .unwrap();
}

#[test]
fn test_entrypoint_simple() {
    let generator = TestGenerator::new();

    generator
        .generate("./tests/input/simple.rs", "simple_out.rs")
        .unwrap();
}

#[test]
fn test_entrypoint_macro() {
    let generator = TestGenerator::new();

    generator
        .generate("./tests/input/macro.rs", "macro_out.rs")
        .unwrap();
}

#[test]
fn test_entrypoint_invalid_syntax() {
    let generator = TestGenerator::new();

    let fails = generator
        .generate("./tests/input/invalid_syntax.rs", "invalid_syntax_out.rs")
        .is_err();

    assert!(fails)
}
