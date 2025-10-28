use std::collections::HashSet;
use std::env;
use std::process::{Command, ExitStatus};

/// Create a command that starts in the `target/debug` or `target/release` directory.
fn cmd(name: &str) -> Command {
    let mut path = env::current_exe().unwrap();
    path.pop();
    if path.file_name().unwrap().to_str() == Some("deps") {
        path.pop();
    }
    path.push(name);
    Command::new(path)
}

/// Executes a command, returning stdout and stderr combined and it's status.
fn output(cmd: &mut Command) -> (String, ExitStatus) {
    eprintln!("command: {cmd:?}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    (stdout + &stderr, output.status)
}

#[test]
fn t1() {
    // t1 must run to completion without any errors.
    let (output, status) = output(&mut cmd("t1"));
    assert!(status.success(), "output: {output}");
    assert!(!output.contains("bad "), "{output}");
    eprintln!("output: {output}");
}

#[test]
fn t2() {
    // t2 must fail to run to completion, and only have the errors we expect it to have.
    let (output, status) = output(&mut cmd("t2"));
    assert!(!status.success(), "output: {output}");
    let errors = [
        "bad T2Foo signed",
        "bad T2TypedefFoo signed",
        "bad T2TypedefInt signed",
        "bad T2Bar size",
        "bad T2Bar align",
        "bad T2Bar signed",
        "bad T2Baz size",
        "bad field offset a of T2Baz",
        "bad field type a of T2Baz",
        "bad field offset b of T2Baz",
        "bad field type b of T2Baz",
        "bad T2a function pointer",
        "bad T2C value at byte 0",
        "bad const T2S string",
        "bad T2Union size",
        "bad field type b of T2Union",
        "bad field offset b of T2Union",
        "bad enum_wrong_signedness signed",
        "bad enum_repr_too_small size",
        "bad enum_repr_too_small align",
    ];
    let mut errors = errors.iter().cloned().collect::<HashSet<_>>();

    // Extract any errors that are not contained within the known error set.
    let mut bad = false;
    for line in output.lines().filter(|l| l.starts_with("bad ")) {
        let msg = &line[..line.find(":").unwrap()];
        if !errors.remove(&msg) {
            println!("unknown error: {msg}");
            bad = true;
        }
    }

    // If any errors are left over, t2 did not run properly.
    for error in errors {
        println!("didn't find error: {error}");
        bad = true;
    }
    if bad {
        println!("output was:\n\n{output}");
        panic!();
    }
}

#[test]
fn test_missing_out_dir() {
    // Save original OUT_DIR
    let orig_out_dir = env::var_os("OUT_DIR");
    env::remove_var("OUT_DIR");

    // Test error handling for OUT_DIR missing
    let result = ctest::TestGenerator::new()
        .header("t1.h")
        .generate_files("src/t1.rs", "out_dir_gen.rs");

    // Restore OUT_DIR
    if let Some(dir) = orig_out_dir {
        env::set_var("OUT_DIR", dir);
    }

    assert!(result.is_err(), "Expected error when OUT_DIR is missing");
}

#[test]
fn test_invalid_output_path() {
    // Test error handling for invalid output path
    let err = ctest::TestGenerator::new()
        .header("t1.h")
        .include("src")
        .out_dir("/nonexistent_dir") // Should fail with permission error
        .generate_files("src/t1.rs", "out_path_gen.rs");

    assert!(err.is_err(), "Expected error with invalid output path");
}

#[test]
fn test_parsing_error() {
    // Test parsing error
    // Create a temporary file with invalid Rust syntax
    let temp_dir = env::temp_dir();
    let invalid_file = temp_dir.join("invalid.rs");
    std::fs::write(&invalid_file, "fn invalid_syntax {").unwrap();

    let err = ctest::TestGenerator::new()
        .header("t1.h")
        .include("src")
        .target("x86_64-unknown-linux-gnu")
        .generate_files(&invalid_file, "parse_gen.rs");

    assert!(err.is_err(), "Expected error when parsing invalid syntax");
    let _ = std::fs::remove_file(invalid_file);
}

#[test]
fn test_non_existent_header() {
    // Test non-existent header
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("nonexistent_header.h").include("src");
    let err = ctest::generate_test(&mut cfg, "src/t1.rs", "missing_header_gen.rs");

    assert!(err.is_err(), "Expected error with non-existent header");
}

#[test]
fn test_invalid_include_path() {
    // Test invalid include path
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("t1.h").include("nonexistent_directory");
    let err = ctest::generate_test(&mut cfg, "src/t1.rs", "invalid_include_gen.rs");

    assert!(err.is_err(), "Expected error with invalid include path");
}
