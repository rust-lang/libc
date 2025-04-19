use std::process::Command;

fn main() {
    use std::env;
    let opt_level = env::var("OPT_LEVEL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "release" || opt_level >= 2 {
        println!("cargo:rustc-cfg=optimized");
    }
    cc::Build::new()
        .include("src")
        .warnings(false)
        .file("src/t1.c")
        .compile("libt1.a");
    println!("cargo:rerun-if-changed=src/t1.c");
    println!("cargo:rerun-if-changed=src/t1.h");
    cc::Build::new()
        .warnings(false)
        .file("src/t2.c")
        .compile("libt2.a");
    println!("cargo:rerun-if-changed=src/t2.c");
    println!("cargo:rerun-if-changed=src/t2.h");
    ctest::TestGenerator::new()
        .header("t1.h")
        .include("src")
        .fn_cname(|a, b| b.unwrap_or(a).to_string())
        .type_name(move |ty, is_struct, is_union| match ty {
            "T1Union" => ty.to_string(),
            "Transparent" => ty.to_string(),
            t if is_struct => format!("struct {t}"),
            t if is_union => format!("union {t}"),
            t => t.to_string(),
        })
        .volatile_item(t1_volatile)
        .array_arg(t1_arrays)
        .skip_roundtrip(|n| n == "Arr")
        .generate("src/t1.rs", "t1gen.rs")
        .expect("generation failed");
    ctest::TestGenerator::new()
        .header("t2.h")
        .include("src")
        .type_name(move |ty, is_struct, is_union| match ty {
            "T2Union" => ty.to_string(),
            t if is_struct => format!("struct {t}"),
            t if is_union => format!("union {t}"),
            t => t.to_string(),
        })
        .skip_roundtrip(|_| true)
        .generate("src/t2.rs", "t2gen.rs")
        .expect("generation failed");

    // Test error handling for OUT_DIR missing
    let orig_out_dir = std::env::var_os("OUT_DIR");
    std::env::remove_var("OUT_DIR");

    let err1 = ctest::TestGenerator::new()
        .header("t1.h")
        .generate("src/t1.rs", "out_dir_gen.rs");

    // Restore OUT_DIR
    if let Some(dir) = orig_out_dir {
        std::env::set_var("OUT_DIR", dir);
    }

    assert!(err1.is_err(), "Expected error when OUT_DIR is missing");

    // Test error handling for invalid output path
    let err = ctest::TestGenerator::new()
        .header("t1.h")
        .include("src")
        .out_dir("/nonexistent_dir") // Should fail with permission error
        .generate("src/t1.rs", "out_path_gen.rs");

    assert!(err.is_err(), "Expected error with invalid output path");

    // Test parsing error
    // Create a temporary file with invalid Rust syntax
    let temp_dir = env::temp_dir();
    let invalid_file = temp_dir.join("invalid.rs");
    std::fs::write(&invalid_file, "fn invalid_syntax {").unwrap();

    let err = ctest::TestGenerator::new()
        .header("t1.h")
        .include("src")
        .generate(&invalid_file, "parse_gen.rs");

    assert!(err.is_err(), "Expected error when parsing invalid syntax");
    let _ = std::fs::remove_file(invalid_file);

    // Test non-existent header
    let err = ctest::TestGenerator::new()
        .header("nonexistent_header.h")
        .include("src")
        .generate("src/t1.rs", "missing_header_gen.rs");

    assert!(err.is_err(), "Expected error with non-existent header");

    // Test invalid include path
    let err = ctest::TestGenerator::new()
        .header("t1.h")
        .include("nonexistent_directory")
        .generate("src/t1.rs", "invalid_include_gen.rs");

    assert!(err.is_err(), "Expected error with invalid include path");

    println!("cargo::rustc-check-cfg=cfg(has_cxx)");
    if !cfg!(unix) || Command::new("c++").arg("v").output().is_ok() {
        // A C compiler is always available, but these are only run if a C++ compiler is
        // also available.
        println!("cargo::rustc-cfg=has_cxx");

        ctest::TestGenerator::new()
            .header("t1.h")
            .language(ctest::Lang::CXX)
            .include("src")
            .fn_cname(|a, b| b.unwrap_or(a).to_string())
            .type_name(move |ty, is_struct, is_union| match ty {
                "T1Union" => ty.to_string(),
                "Transparent" => ty.to_string(),
                t if is_struct => format!("struct {t}"),
                t if is_union => format!("union {t}"),
                t => t.to_string(),
            })
            .volatile_item(t1_volatile)
            .array_arg(t1_arrays)
            .skip_roundtrip(|n| n == "Arr")
            .generate("src/t1.rs", "t1gen_cxx.rs")
            .expect("generation failed");
        ctest::TestGenerator::new()
            .header("t2.h")
            .language(ctest::Lang::CXX)
            .include("src")
            .type_name(move |ty, is_struct, is_union| match ty {
                "T2Union" => ty.to_string(),
                t if is_struct => format!("struct {t}"),
                t if is_union => format!("union {t}"),
                t => t.to_string(),
            })
            .skip_roundtrip(|_| true)
            .generate("src/t2.rs", "t2gen_cxx.rs")
            .expect("generation failed");
    } else {
        println!("cargo::warning=skipping C++ tests");
    }
}

fn t1_volatile(i: ctest::VolatileItemKind) -> bool {
    use ctest::VolatileItemKind::*;
    match i {
        StructField(ref n, ref f) if n == "V" && f == "v" => true,
        Static(ref n) if n == "vol_ptr" => true,
        FunctionArg(ref n, 0) if n == "T1_vol0" => true,
        FunctionArg(ref n, 1) if n == "T1_vol2" => true,
        FunctionRet(ref n) if n == "T1_vol1" || n == "T1_vol2" => true,
        Static(ref n) if n == "T1_fn_ptr_vol" => true,
        _ => false,
    }
}

fn t1_arrays(n: &str, i: usize) -> bool {
    match n {
        "T1r" | "T1s" | "T1t" | "T1v" if i == 0 => true,
        _ => false,
    }
}
