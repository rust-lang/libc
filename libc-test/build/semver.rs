use std::fs::File;
use std::io::{
    BufRead,
    BufReader,
    BufWriter,
    Write,
};
use std::path::{
    Path,
    PathBuf,
};
use std::{
    env,
    io,
};

pub(crate) fn do_semver() {
    let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.push("semver.rs");
    let mut output = BufWriter::new(File::create(&out).unwrap());

    let family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    // `libc-test/semver` dir.
    let mut semver_root = PathBuf::from("semver");

    // NOTE: Windows has the same `family` as `os`, no point in including it
    // twice.
    // NOTE: Android doesn't include the unix file (or the Linux file) because
    // there are some many definitions missing it's actually easier just to
    // maintain a file for Android.
    // NOTE: AIX, L4Re and QNX do not include the unix file because there are
    // definitions missing on these systems. It is easier to maintain separate
    // files for them.
    if family != os
        && !matches!(os.as_str(), "android" | "aix" | "l4re" | "nto" | "qnx")
        && os != "vxworks"
    {
        process_semver_file(&mut output, &mut semver_root, &family);
    }
    // We don't do semver for unknown targets.
    if vendor != "unknown" {
        process_semver_file(&mut output, &mut semver_root, &vendor);
    }
    process_semver_file(&mut output, &mut semver_root, &os);
    let os_arch = format!("{os}-{arch}");
    process_semver_file(&mut output, &mut semver_root, &os_arch);
    if !target_env.is_empty() {
        let os_env = format!("{os}-{target_env}");
        process_semver_file(&mut output, &mut semver_root, &os_env);

        let os_env_arch = format!("{os}-{target_env}-{arch}");
        process_semver_file(&mut output, &mut semver_root, &os_env_arch);
    }
}

fn process_semver_file<W: Write, P: AsRef<Path>>(output: &mut W, path: &mut PathBuf, file: P) {
    // NOTE: `path` is reused between calls, so always remove the file again.
    path.push(file);
    path.set_extension("txt");

    println!("cargo:rerun-if-changed={}", path.display());
    let input_file = match File::open(&*path) {
        Ok(file) => file,
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
            path.pop();
            return;
        }
        Err(err) => panic!("unexpected error opening file: {err}"),
    };
    let input = BufReader::new(input_file);

    writeln!(output, "// Source: {}.", path.display()).unwrap();
    output.write_all(b"use libc::{\n").unwrap();
    for line in input.lines() {
        let line = line.unwrap().into_bytes();
        match line.first() {
            // Ignore comments and empty lines.
            Some(b'#') | None => continue,
            _ => {
                output.write_all(b"    ").unwrap();
                output.write_all(&line).unwrap();
                output.write_all(b",\n").unwrap();
            }
        }
    }
    output.write_all(b"};\n\n").unwrap();
    path.pop();
}
