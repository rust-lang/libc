use std::env;
use std::fs::canonicalize;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::{
    EDITION,
    Result,
};

/// Use rustc to expand all macros and pretty print the crate into a single file.
pub fn expand<P: AsRef<Path>>(
    crate_path: P,
    cfg: &[(String, Option<String>)],
    target: String,
) -> Result<String> {
    let dir = tempfile::tempdir()?;

    // make a Cargo.toml pointing to the crate path
    let cargo_toml_path = dir.path().join("Cargo.toml");
    let mut cargo_toml = std::fs::File::create(&cargo_toml_path)?;
    writeln!(
        &mut cargo_toml,
        r#"
[package]
name = "libc"
edition = "{EDITION}"
[lib]
path = '{}'
[lints.rust]
unexpected_cfgs = "allow"
"#,
        canonicalize(crate_path)?.display()
    )?;
    drop(cargo_toml);

    let mut cmd = Command::new("cargo");

    cmd.env("RUSTC_BOOTSTRAP", "1")
        .env("RUSTFLAGS", "") // ignore RUSTFLAGS so we don't get -Dwarnings
        .current_dir(dir.path())
        .arg("rustc")
        .arg("--lib")
        .arg("--profile")
        .arg("check");

    // set an independent target dir so we don't deadlock on the cargo build lock.
    cmd.arg("--target-dir").arg(dir.path().join("target"));

    if env::var("LIBC_CI_ZBUILD_STD").is_ok() {
        cmd.arg("-Zbuild-std=core,std");
    }

    if !target.is_empty() {
        cmd.arg("--target").arg(target);
    }

    cmd.arg("--");
    cmd.arg("-Zunpretty=expanded");

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
        return Err(format!(
            "macro expansion failed with {}: {}, {:?}",
            output.status, stderr, cmd
        )
        .into());
    }

    let expanded = std::str::from_utf8(&output.stdout)?.to_string();

    Ok(expanded)
}
