extern crate ctest;
extern crate gcc;

fn main() {
    gcc::Config::new()
                .include("src")
                .file("src/t1.c")
                .compile("libt1.a");
    gcc::Config::new()
                .file("src/t2.c")
                .compile("libt2.a");
    ctest::TestGenerator::new()
                         .header("t1.h")
                         .include("src")
                         .generate("src/t1.rs", "t1gen.rs");
    ctest::TestGenerator::new()
                         .header("t2.h")
                         .include("src")
                         .generate("src/t2.rs", "t2gen.rs");
}
