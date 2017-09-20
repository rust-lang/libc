extern crate ctest;
extern crate cc;

fn main() {
    cc::Build::new()
        .include("src")
        .warnings(false)
        .file("src/t1.c")
        .compile("libt1.a");
    cc::Build::new()
        .warnings(false)
        .file("src/t2.c")
        .compile("libt2.a");
    ctest::TestGenerator::new()
                         .header("t1.h")
                         .include("src")
                         .fn_cname(|a, b| b.unwrap_or(a).to_string())
                         .generate("src/t1.rs", "t1gen.rs");
    ctest::TestGenerator::new()
                         .header("t2.h")
                         .include("src")
                         .generate("src/t2.rs", "t2gen.rs");
}
