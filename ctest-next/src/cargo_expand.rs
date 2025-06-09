use std::{env, error::Error, path::Path, process::Command};

/// Use cargo expand to expand all macros and pretty print the crate
/// into a single file.
pub fn expand(crate_path: &Path) -> Result<String, Box<dyn Error>> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| String::from("cargo"));

    let output = Command::new(cargo)
        .arg("expand")
        .current_dir(crate_path)
        .output()?;

    let expanded = std::str::from_utf8(&output.stdout)?.to_string();

    Ok(expanded)
}
