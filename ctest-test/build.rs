use std::env;

use ctest_next::{generate_test, TestGenerator};

fn main() {
    let opt_level = env::var("OPT_LEVEL")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let profile = env::var("PROFILE").unwrap_or_default();
    if profile == "release" || opt_level >= 2 {
        println!("cargo:rustc-cfg=optimized");
    }

    // FIXME(ctest): The .c files are ignored right now, I'm not sure if they
    // were used or how they were used before.
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

    let mut t1gen = TestGenerator::new();
    t1gen
        .header("t1.h")
        .include("src")
        .skip_private(true)
        .rename_fn(|f| f.link_name().unwrap_or(f.ident()).to_string().into())
        .rename_union_ty(|ty| (ty == "T1Union").then_some(ty.to_string()))
        .rename_struct_ty(|ty| (ty == "Transparent").then_some(ty.to_string()))
        .volatile_field(|s, f| s.ident() == "V" && f.ident() == "v")
        .volatile_static(|s| s.ident() == "vol_ptr")
        .volatile_static(|s| s.ident() == "T1_fn_ptr_vol")
        .volatile_fn_arg(|f, p| f.ident() == "T1_vol0" && p.ident() == "arg0")
        .volatile_fn_arg(|f, p| f.ident() == "T1_vol2" && p.ident() == "arg1")
        .volatile_fn_return_type(|f| f.ident() == "T1_vol1")
        .volatile_fn_return_type(|f| f.ident() == "T1_vol2")
        // The parameter `a` of the functions `T1r`, `T1s`, `T1t`, `T1v` is an array.
        .array_arg(|f, p| matches!(f.ident(), "T1r" | "T1s" | "T1t" | "T1v") && p.ident() == "a")
        .skip_roundtrip(|n| n == "Arr");
    generate_test(&mut t1gen, "src/t1.rs", "t1gen.rs").unwrap();

    let mut t2gen = TestGenerator::new();
    t2gen
        .header("t2.h")
        .include("src")
        // public C typedefs have to manually be specified because they are identical to normal
        // structs on the Rust side.
        .rename_union_ty(|ty| (ty == "T2Union").then_some(ty.to_string()))
        .skip_roundtrip(|_| true);
    generate_test(&mut t2gen, "src/t2.rs", "t2gen.rs").unwrap();
}
