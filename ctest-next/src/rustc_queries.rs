use std::{env, fmt::Display, num::ParseIntError, process::Command};

use crate::Result;

/// Represents the current version of the rustc compiler globally in use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustcVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl RustcVersion {
    /// Define a rustc version with the given major.minor.patch.
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Display for RustcVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RustcVersion({}, {}, {})",
            self.major, self.minor, self.patch
        )
    }
}

/// Return the global rustc version.
pub fn rustc_version() -> Result<RustcVersion> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));

    let output = Command::new(rustc).arg("--version").output()?;

    if !output.status.success() {
        let error = std::str::from_utf8(&output.stderr)?;
        return Err(error.into());
    }

    // eg: rustc 1.87.0-(optionally nightly) (17067e9ac 2025-05-09)
    // Assume the format does not change.
    let [major, minor, patch] = std::str::from_utf8(&output.stdout)?
        .split_whitespace()
        .nth(1)
        .unwrap()
        .split('.')
        .take(3)
        .map(|s| {
            s.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .trim()
                .parse::<u8>()
        })
        .collect::<Result<Vec<u8>, ParseIntError>>()?
        .try_into()
        .unwrap();

    Ok(RustcVersion::new(major, minor, patch))
}

/// Return the host triple.
pub fn rustc_host() -> Result<String> {
    let rustc = env::var("RUSTC").unwrap_or_else(|_| String::from("rustc"));

    let output = Command::new(rustc)
        .arg("--version")
        .arg("--verbose")
        .output()?;

    if !output.status.success() {
        let error = std::str::from_utf8(&output.stderr)?;
        return Err(error.into());
    }

    // eg: rustc 1.87.0-(optionally nightly) (17067e9ac 2025-05-09)
    // binary: rustc
    // commit-hash: 17067e9ac6d7ecb70e50f92c1944e545188d2359
    // commit-date: 2025-05-09
    // host: x86_64-unknown-linux-gnu
    // release: 1.87.0
    // LLVM version: 20.1.1
    // Assume the format does not change.
    let host = std::str::from_utf8(&output.stdout)?
        .lines()
        .nth(4)
        .unwrap()
        .split(':')
        .next_back()
        .map(|s| s.trim())
        .unwrap();

    Ok(host.to_string())
}
