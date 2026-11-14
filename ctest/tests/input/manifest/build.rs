fn main() {
    println!("cargo::rustc-check-cfg=cfg(manifest_build_script)");
    println!("cargo::rustc-cfg=manifest_build_script");
}
