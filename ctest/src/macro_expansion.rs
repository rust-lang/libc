use std::fs::canonicalize;
use std::path::Path;
use std::process::Command;

use crate::{
    EDITION,
    Result,
};

/// Use rustc to expand all macros and pretty print the crate into a single file.
///
/// `crate_path` is the crate's `Cargo.toml`, or the root file of a crate without dependencies.
pub fn expand<P: AsRef<Path>>(
    crate_path: P,
    cfg: &[(String, Option<String>)],
    target: String,
) -> Result<String> {
    expand_with_args(crate_path, cfg, target, None, &[])
}

/// Extension of `expand` which allows a crate_name override for
/// libc and other clients which require crate_name to match the
/// original crate's name.
pub(crate) fn expand_with_args<P: AsRef<Path>>(
    crate_path: P,
    cfg: &[(String, Option<String>)],
    target: String,
    crate_name: Option<&str>,
    extra_cargo_args: &[String],
) -> Result<String> {
    let crate_path = crate_path.as_ref();
    let dir = tempfile::tempdir()?;

    // cargo runs from the manifest's directory, so the crate's own Cargo configuration applies.
    let manifest_dir = if crate_path.ends_with("Cargo.toml") {
        let mut crate_dir = std::path::absolute(crate_path)?;
        crate_dir.pop();
        crate_dir
    } else {
        // make a Cargo.toml pointing to the crate path
        let crate_name = crate_name.unwrap_or("ctest-expansion-tmp");
        let cargo_toml = format!(
            r#"
[package]
name = '''{crate_name}'''  # Needs to match #![crate_name = "..."] if it exists.
edition = '''{EDITION}'''
[lib]
path = '''{}'''
[lints.rust]
unexpected_cfgs = "allow"
"#,
            canonicalize(crate_path)?.display()
        );
        std::fs::write(dir.path().join("Cargo.toml"), cargo_toml)?;
        dir.path().to_path_buf()
    };

    let mut cmd = Command::new(std::env::var("CARGO").unwrap_or("cargo".into()));
    cmd.env("RUSTC_BOOTSTRAP", "1")
        .current_dir(manifest_dir)
        .arg("rustc")
        .arg("--lib")
        .arg("--profile")
        .arg("check")
        .args(extra_cargo_args);

    // In order to avoid warnings causing failures here when the
    // environment's RUSTFLAGS contains -Dwarnings (or similar),
    // we will clear RUSTFLAGS.
    cmd.env_remove("RUSTFLAGS");

    // set an independent target dir so we don't deadlock on the cargo build lock.
    cmd.arg("--target-dir").arg(dir.path().join("target"));

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
