use std::env;

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let target_key = target.replace('-', "_");

    println!("cargo:rustc-env=HOST_PLATFORM={host}");
    println!("cargo:rustc-env=TARGET_PLATFORM={target}");

    let linker = env::var(format!("CARGO_TARGET_{}_LINKER", target_key.to_uppercase()))
        .or_else(|_| env::var("CC"))
        .or_else(|_| env::var(format!("CC_{}", target_key)))
        .unwrap_or_default();

    let runner =
        env::var(format!("CARGO_TARGET_{}_RUNNER", target_key.to_uppercase())).unwrap_or_default();

    // As we invoke rustc directly this does not get passed to it, although RUSTFLAGS does.
    let flags = env::var(format!(
        "CARGO_TARGET_{}_RUSTFLAGS",
        target_key.to_uppercase()
    ))
    .unwrap_or_default();

    println!("cargo:rustc-env=LINKER={linker}");
    println!("cargo:rustc-env=RUNNER={runner}");
    println!("cargo:rustc-env=FLAGS={flags}");

    println!("cargo:rerun-if-changed-env=TARGET");
}
