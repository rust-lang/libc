//! Simple script to verify the coding style of this library.
//!
//! ## How to run
//!
//! The first argument to this script is the directory to run on, so running
//! this script should be as simple as:
//!
//! ```notrust
//! cargo test --test style
//! ```

pub mod style_lib;

use std::env;
use std::path::Path;

use style_lib::{Result, StyleChecker};

/// Relative to `src/`.
const SKIP_PREFIXES: &[&str] = &[
    // Don't run the style checker on the reorganized portion of the crate while we figure
    // out what style we want.
    "new/", "types.rs",
];

#[test]
fn check_style() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src");
    walk(&src_root).unwrap();
    eprintln!("good style!");
}

fn walk(src_root: &Path) -> Result<()> {
    let mut style_checker = StyleChecker::new();

    for entry in glob::glob(&format!(
        "{}/**/*.rs",
        src_root.to_str().expect("dir should be valid UTF-8")
    ))? {
        let entry = entry?;
        let relpath = entry.strip_prefix(src_root).expect("known path");
        if SKIP_PREFIXES.iter().any(|pfx| relpath.starts_with(pfx)) {
            continue;
        }

        let name = entry
            .file_name()
            .expect("file name should not end in ..")
            .to_str()
            .expect("file name should be valid UTF-8");
        if let "lib.rs" | "macros.rs" = &name[..] {
            continue;
        }

        let path = entry.as_path();
        style_checker.check_file(path)?;
        style_checker.reset_state();
    }

    style_checker.finalize()
}
