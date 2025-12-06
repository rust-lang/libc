//! Verifies the implementation of the style checker in [style].

use style_lib::StyleChecker;

pub mod style_lib;

#[test]
fn check_style_accept_correct_module_layout() {
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
fn check_style_reject_incorrect_module_layout() {
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
fn check_style_reject_incorrect_cfg_if_layout() {
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
fn check_style_accept_cfg_if_branch_resets_state() {
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
fn check_style_reject_multiple_f_macros() {
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
fn check_style_accept_cfg_ignore_target_endian_nested() {
    let contents = r#"
pub struct Foo {
    #[cfg(target_endian = "little")]
    pub id: __u16,
}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn check_style_reject_manual_copy() {
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
fn check_style_reject_manual_clone() {
    let contents = r#"
#[derive(Clone)]
pub struct Foo {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn check_style_accept_multiple_s_macros_with_disjoint_cfg() {
    let contents = r#"
// Main `s!`
s! {}

// These are not supported on a single arch. It doesn't make sense to
// duplicate `foo` into every single file except one, so allow this here.
#[cfg(not(target_arch = "foo"))]
s! { pub struct foo { /* ... */ } }

// Similar to the above, no problems here
#[cfg(not(target_os = "illumos"))]
s! { pub struct bar { /* ... */ } }
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn check_style_reject_duplicated_s_macro_cfg() {
    let contents = r#"
#[cfg(not(target_arch = "foo"))]
s! {}

#[cfg(not(target_arch = "foo"))]
s! {}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn check_style_reject_single_positive_s_macro_cfg() {
    let contents = r#"
// A positive (no `not`) config: reject because this should go into
// the relevant file.
#[cfg(target_arch = "foo")]
s! { pub struct foo { /* ... */ } }
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn check_style_reject_single_positive_s_macro_cfg_target_os() {
    let contents = r#"
// A positive (no `not`) config: reject because this should go into
// the relevant file.
#[cfg(target_os = "foo")]
s! { pub struct foo { /* ... */ } }
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}

#[test]
fn check_style_accept_positive_s_macro_any() {
    let contents = r#"
// It's nicer to accept this so that we don't have to duplicate the same struct 3 times.
#[cfg(any(target_arch = "foo", target_arch = "bar", target_arch = "baz"))]
s! { pub struct foo { /* ... */ } }
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn check_style_accept_positive_s_macro_all() {
    let contents = r#"
#[cfg(all(target_arch = "foo", target_arch = "bar", target_arch = "baz"))]
s! { pub struct foo { /* ... */ } }
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    checker.finalize().unwrap();
}

#[test]
fn check_style_reject_duplicated_cfg_and_cfg_if() {
    let contents = r#"
#[cfg(not(target_arch = "foo"))]
s! { pub struct foo { /* ... */ } }

cfg_if! {
    if #[cfg(not(target_arch = "foo"))] {
        s!{ pub struct bar {} }
    }
}
"#
    .to_string();

    let mut checker = StyleChecker::new();
    checker.check_string(contents).unwrap();
    assert!(checker.finalize().is_err());
}
