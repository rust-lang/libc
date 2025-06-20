use std::env;

fn main() {
    println!(
        "cargo:rustc-env={}={}",
        "HOST_PLATFORM",
        env::var("HOST").unwrap()
    );
    println!(
        "cargo:rustc-env={}={}",
        "TARGET_PLATFORM",
        env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env={}={}",
        "LINKER",
        env::var("CC").unwrap_or("".to_string())
    );
    println!("cargo:rerun-if-changed-env=TARGET")
}
