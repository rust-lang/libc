use std::env;

// When we call `cargo test` for a cross compiled target, the following is required:
// - CARGO_TARGET_{}_LINKER: To link the integration tests.
// - CARGO_TARGET_{}_RUNNER: To run the integration tests.
//
// This is already set by the CI for all platforms, so there is no problem up till here.
//
// The integration tests (which are run in qemu, but use host rustc and cc) require the
// following:
// - TARGET_PLATFORM or target set manually. (We forward TARGET in build.rs for this.)
// - HOST_PLATFORM or host set manually. (We forward HOST in build.rs for this.)
// - LINKER: To link the C headers. (We forward CARGO_TARGET_{}_LINKER for this.)
// - FLAGS: Any flags to pass when compiling the test binary for the cross compiled platform.
//   (Forwarded from CARGO_TARGET_{}_RUSTFLAGS)
// - RUNNER: To run the test binary with. (Forward the same runner as CARGO_TARGET_{}_RUNNER)
//
// The TARGET_PLATFORM and HOST_PLATFORM variables are not an issue, cargo will automatically set
// TARGET and PLATFORM and we will forward them.
//
// Similarly FLAGS and RUNNER are also not an issue, if CARGO_TARGET_{}_RUSTFLAGS are present
// they're forwarded. And RUSTFLAGS works by default anyway. Similarly the test binary doesn't
// require any external applications so just the RUNNER is enough to run it.
//
// However since rustc and cc are the host versions, they will only work if we specify the
// correct variables for them. Because we only use them to compile, not run things. For CC we
// MUST specify CC or CC_target otherwise it will fail. (Other flags like AR etc. work without
// forwarding because it is run in the host.) For rustc we MUST specify the correct linker.
// Usually this is the same as CC or CC_target.
//
// In the CI, the CARGO_TARGET_{} variables are always set.

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();
    let target_key = target.replace('-', "_").to_uppercase();

    println!("cargo:rustc-env=HOST_PLATFORM={host}");
    println!("cargo:rerun-if-changed-env=HOST");

    println!("cargo:rustc-env=TARGET_PLATFORM={target}");
    println!("cargo:rerun-if-changed-env=TARGET");

    let link_var = format!("CARGO_TARGET_{target_key}_LINKER");
    println!("cargo:rerun-if-changed-env={link_var}");
    if let Ok(linker) = env::var(link_var) {
        println!("cargo:rustc-env=LINKER={linker}");
    }

    let run_var = format!("CARGO_TARGET_{target_key}_RUNNER");
    println!("cargo:rerun-if-changed-env={run_var}");
    if let Ok(runner) = env::var(run_var) {
        println!("cargo:rustc-env=RUNNER={runner}");
    }

    // As we invoke rustc directly this does not get passed to it, although RUSTFLAGS does.
    let flag_var = format!("CARGO_TARGET_{target_key}_RUSTFLAGS");
    println!("cargo:rerun-if-changed-env={flag_var}");
    if let Ok(flags) = env::var(flag_var) {
        println!("cargo:rustc-env=FLAGS={flags}");
    }

    // Rerun this build script if any of these environment variables change.
    println!("cargo:rerun-if-changed-env=CC");
    println!(
        "cargo:rerun-if-changed-env=CC_{}",
        target_key.to_lowercase()
    );
}
