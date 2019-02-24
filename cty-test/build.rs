#![deny(warnings)]

extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    cfg.flag("-Wno-deprecated-declarations");

    cfg.header("stddef.h");
    cfg.header("stdint.h");
    cfg.header("ctype.h");
    // For POSIX: ssize_t
    cfg.header("sys/types.h");

    /* cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            "ssize_t" => "SSIZE_T".to_string(),
            t if is_union => format!("union {}", t),
            t if t.ends_with("_t") => t.to_string(),
            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),
            t => t.to_string(),
        }
    });*/

    cfg.generate("../cty/src/lib.rs", "main.rs");
}
