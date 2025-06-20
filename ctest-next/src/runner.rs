use crate::Result;

use std::env;
use std::fs::{canonicalize, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Compile the given Rust file with the given static library.
/// All arguments must be valid paths.
pub fn compile_test<P: AsRef<Path>>(
    output_dir: P,
    crate_path: P,
    library_file: P,
) -> Result<PathBuf> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());

    let output_dir = output_dir.as_ref();
    let crate_path = crate_path.as_ref();
    let library_file = library_file.as_ref();

    let rust_file = output_dir
        .join(crate_path.file_stem().unwrap())
        .with_extension("rs");
    let binary_path = output_dir.join(rust_file.file_stem().unwrap());

    let mut file = File::create(&rust_file)?;
    writeln!(
        file,
        "include!(r#\"{}\"#);",
        canonicalize(crate_path)?.display()
    )?;
    writeln!(file, "include!(r#\"{}.rs\"#);", library_file.display())?;

    let output = Command::new(rustc)
        .arg(&rust_file)
        .arg(format!("-Lnative={}", output_dir.display()))
        .arg(format!(
            "-lstatic={}",
            library_file.file_stem().unwrap().to_str().unwrap()
        ))
        .arg("-o")
        .arg(&binary_path)
        .arg("-Aunused")
        .output()?;

    if !output.status.success() {
        return Err(std::str::from_utf8(&output.stderr)?.into());
    }

    Ok(binary_path)
}

/// Run the compiled test binary.
pub fn run_test<P: AsRef<Path>>(test_binary: P) -> Result<String> {
    let output = Command::new(test_binary.as_ref()).output()?;

    if !output.status.success() {
        return Err(std::str::from_utf8(&output.stderr)?.into());
    }

    // The template prints to stderr regardless.
    Ok(std::str::from_utf8(&output.stderr)?.to_string())
}
