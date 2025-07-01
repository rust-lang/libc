use std::{env, fs::canonicalize, path::Path, process::Command};

use crate::Result;

/// Use rustc to expand all macros and pretty print the crate into a single file.
pub fn expand<P: AsRef<Path>>(crate_path: P) -> Result<String> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));

    let output = Command::new(rustc)
        .env("RUSTC_BOOTSTRAP", "1")
        .arg("-Zunpretty=expanded")
        .arg("--edition")
        .arg("2024") // By default, -Zunpretty=expanded uses 2015 edition.
        .arg(canonicalize(crate_path)?)
        .output()?;

    if !output.status.success() {
        let error = std::str::from_utf8(&output.stderr)?;
        return Err(error.into());
    }

    let expanded = std::str::from_utf8(&output.stdout)?.to_string();

    Ok(expanded)
}
