use std::process::Command;

#[cfg(not(target_os = "freebsd"))]
fn detect_freebsd_abi_changes() {}

#[cfg(target_os = "freebsd")]
fn detect_freebsd_abi_changes() {
    // We use freebsd-version(1). We could use `uname -k` to get the
    // version of the kernel, but it wouldn't reflect the version of the
    // userland (in case we are in a jail).
    //
    // freebsd-version(1) appeared in FreeBSD 10.0, so it's good enough.

    let _ = Command::new("freebsd-version").arg("-u")
        .output()
        .and_then(
            |output| {
                assert!(output.status.success());

                let full_version_string = String::from_utf8_lossy(&output.stdout);
                let split_version: Vec<&str> = full_version_string
                    .trim()
                    .splitn(2, '.').collect();

                assert!(split_version.len() >= 1);

                let version: u32 = split_version[0]
                    .parse().unwrap();

                // FreeBSD kernel versions are published in the following document:
                // https://www.freebsd.org/doc/en/books/porters-handbook/versions.html

                if version >= 12 { println!("cargo:rustc-cfg=freebsd12_abi"); }

                Ok(())
            });
}

fn main() {
    detect_freebsd_abi_changes();
}
