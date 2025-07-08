// FIXME(ctest): this test doesn't work when cross compiling.
#![cfg(target_arch = "x86_64")]

use std::collections::HashSet;
use std::env;
use std::process::{Command, ExitStatus};

/// Create a command that starts in the `target/debug` or `target/release` directory.
fn cmd(name: &str) -> Command {
    let mut path = env::current_exe().unwrap();
    path.pop();
    if path.file_name().unwrap().to_str() == Some("deps") {
        path.pop();
    }
    path.push(name);
    Command::new(path)
}

/// Executes a command, returning stdout and stderr combined and it's status.
fn output(cmd: &mut Command) -> (String, ExitStatus) {
    eprintln!("command: {cmd:?}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    (stdout + &stderr, output.status)
}

#[test]
fn t1_next() {
    // t1 must run to completion without any errors.
    let (output, status) = output(&mut cmd("t1-next"));
    assert!(status.success(), "output: {output}");
    assert!(!output.contains("bad "), "{output}");
    eprintln!("output: {output}");
}

#[test]
fn t2_next() {
    // t2 must fail to run to completion, and only have the errors we expect it to have.
    let (output, status) = output(&mut cmd("t2-next"));
    assert!(!status.success(), "output: {output}");

    // FIXME(ctest): Errors currently commented out are not tested.
    let errors = [
        "bad T2Foo signed",
        "bad T2TypedefFoo signed",
        "bad T2TypedefInt signed",
        "bad T2Bar size",
        "bad T2Bar align",
        "bad T2Bar signed",
        "bad T2Baz size",
        "bad field offset a of T2Baz",
        "bad field type a of T2Baz",
        "bad field offset b of T2Baz",
        "bad field type b of T2Baz",
        // "bad T2a function pointer",
        "bad T2C value at byte 0",
        "bad T2S string",
        "bad T2Union size",
        "bad field type b of T2Union",
        "bad field offset b of T2Union",
    ];
    let mut errors = errors.iter().cloned().collect::<HashSet<_>>();

    // Extract any errors that are not contained within the known error set.
    let mut bad = false;
    for line in output.lines().filter(|l| l.starts_with("bad ")) {
        let msg = &line[..line.find(":").unwrap()];
        if !errors.remove(&msg) {
            println!("unknown error: {msg:#?}");
            bad = true;
        }
    }

    // If any errors are left over, t2 did not run properly.
    for error in errors {
        println!("didn't find error: {error}");
        bad = true;
    }

    if bad {
        println!("output was:\n\n{output:#?}");
        panic!();
    }
}
