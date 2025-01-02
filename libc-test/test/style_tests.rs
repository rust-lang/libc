//! Verifies the implementation of the style checker in [style]

use style::StyleChecker;

pub mod style;

#[test]
fn correct_module_layout() {
    let contents = r#"
use core::mem::size_of;
pub type foo_t = u32;
struct Foo {}
pub const FOO: u32 = 0x20000;
f! {}
extern "C" {}
mod foolib;
pub use self::foolib::*;
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn incorrect_module_layout() {
    let contents = r#"
use core::mem::size_of;
pub type foo_t = u32;
struct Foo {}
pub const FOO: u32 = 0x20000;
extern "C" {}
f! {}
mod foolib;
pub use self::foolib::*;
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn incorrect_cfg_if_layout() {
    let contents = r#"
cfg_if! {
    if #[cfg(foo)] {
        pub type foo_t = u32;
        use core::mem::size_of;
    }
}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn cfg_if_branch_resets_state() {
    let contents = r#"
cfg_if! {
    if #[cfg(foo)] {
        use core::mem::size_of;
        pub type foo_t = u32;
    } else {
        use core::mem::align_of;
    }
}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn multiple_f_macros() {
    let contents = r#"
f! {}
f! {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn cfg_if_over_cfg() {
    let contents = r#"
#[cfg(foo)]
pub struct Foo {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn cfg_if_ignore_target_arch() {
    let contents = r#"
#[cfg(target_arch = "x86")]
pub struct Foo {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn cfg_if_ignore_target_endian_nested() {
    let contents = r#"
#[cfg(all(target_endian = "little"))]
pub struct Foo {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn manual_copy() {
    let contents = r#"
#[derive(Copy)]
pub struct Foo {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn manual_clone() {
    let contents = r#"
#[derive(Clone)]
pub struct Foo {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}
