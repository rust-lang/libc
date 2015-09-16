extern crate ctest;
extern crate gcc;

fn main() {
    ctest::TestGenerator::new()
                         .header("t1.h")
                         .include("src")
                         .generate("src/t1.rs", "t1gen.rs");
    ctest::TestGenerator::new()
                         .header("t2.h")
                         .include("src")
                         .generate("src/t2.rs", "t2gen.rs");
}
