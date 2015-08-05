use std::env;

fn main() {
    if let Ok(path) = env::var("LIBC_PATH") {
        println!("cargo:rustc-link-search={}", path);
    }
}
