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

pub mod style;

use std::env;
use std::path::Path;

use style::{Result, StyleChecker};

#[test]
fn check_style() {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../src");
    walk(&root_dir).unwrap();
    eprintln!("good style!");
}

fn walk(root_dir: &Path) -> Result<()> {
    let mut style_checker = StyleChecker::new();

    for entry in glob::glob(&format!(
        "{}/**/*.rs",
        root_dir.to_str().expect("dir should be valid UTF-8")
    ))? {
        let entry = entry?;

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
