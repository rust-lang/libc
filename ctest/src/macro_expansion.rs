use std::env;
use std::fs::canonicalize;
use std::path::Path;
use std::process::Command;

use crate::{EDITION, Result};

/// Use rustc to expand all macros and pretty print the crate into a single file.
pub fn expand<P: AsRef<Path>>(
    crate_path: P,
    cfg: &[(String, Option<String>)],
    target: String,
) -> Result<String> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));

    let mut cmd = Command::new(rustc);
    cmd.env("RUSTC_BOOTSTRAP", "1")
        .arg("-Zunpretty=expanded")
        .arg("--edition")
        .arg(EDITION) // By default, -Zunpretty=expanded uses 2015 edition.
        .arg(canonicalize(crate_path)?);

    if !target.is_empty() {
        cmd.arg("--target").arg(target);
    }

    // `libc` uses non standard cfg flags as well, which have to be manually expanded.
    for (k, v) in cfg {
        match v {
            None => cmd.arg("--cfg").arg(k),
            Some(val) => cmd.arg("--cfg").arg(format!("{k}=\"{val}\"")),
        };
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let stderr = std::str::from_utf8(&output.stderr)?;
        return Err(format!("macro expansion failed with {}: {}", output.status, stderr).into());
    }

    let expanded = std::str::from_utf8(&output.stdout)?.to_string();

    Ok(expanded)
}
