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
        .generate("src/t1.rs", "t1gen.rs");
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
        .generate("src/t2.rs", "t2gen.rs");

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
            .generate("src/t1.rs", "t1gen_cxx.rs");
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
            .generate("src/t2.rs", "t2gen_cxx.rs");
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
    i == 0 && matches!(n, "T1r" | "T1s" | "T1t" | "T1v")
}
