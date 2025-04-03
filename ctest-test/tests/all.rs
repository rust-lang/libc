// FIXME(ctest): this test doesn't work when cross compiling.
#![cfg(target_arch = "x86_64")]

use std::collections::HashSet;
use std::env;
use std::process::{Command, ExitStatus};

fn cmd(name: &str) -> Command {
    let mut p = env::current_exe().unwrap();
    p.pop();
    if p.file_name().unwrap().to_str() == Some("deps") {
        p.pop();
    }
    p.push(name);
    Command::new(p)
}

fn output(cmd: &mut Command) -> (String, ExitStatus) {
    eprintln!("command: {cmd:?}");
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    (stdout + &stderr, output.status)
}

#[test]
fn t1() {
    let (o, status) = output(&mut cmd("t1"));
    assert!(status.success(), "output: {o}");
    assert!(!o.contains("bad "), "{}", o);
    eprintln!("o: {}", o);
}

#[test]
#[cfg(has_cxx)]
fn t1_cxx() {
    let (o, status) = output(&mut cmd("t1_cxx"));
    assert!(status.success(), "output: {o}");
    assert!(!o.contains("bad "), "{}", o);
}

#[test]
fn t2() {
    let (o, status) = output(&mut cmd("t2"));
    assert!(!status.success(), "output: {o}");
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
        "bad T2a function pointer",
        "bad T2C value at byte 0",
        "bad T2S string",
        "bad T2Union size",
        "bad field type b of T2Union",
        "bad field offset b of T2Union",
    ];
    let mut errors = errors.iter().cloned().collect::<HashSet<_>>();

    let mut bad = false;
    for line in o.lines().filter(|l| l.starts_with("bad ")) {
        let msg = &line[..line.find(":").unwrap()];
        if !errors.remove(&msg) {
            println!("unknown error: {}", msg);
            bad = true;
        }
    }

    for error in errors {
        println!("didn't find error: {}", error);
        bad = true;
    }
    if bad {
        println!("output was:\n\n{}", o);
        panic!();
    }
}

#[test]
#[cfg(has_cxx)]
fn t2_cxx() {
    let (o, status) = output(&mut cmd("t2_cxx"));
    assert!(!status.success(), "output: {o}");
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
        "bad T2a function pointer",
        "bad T2C value at byte 0",
        "bad T2S string",
        "bad T2Union size",
        "bad field type b of T2Union",
        "bad field offset b of T2Union",
    ];
    let mut errors = errors.iter().cloned().collect::<HashSet<_>>();

    let mut bad = false;
    for line in o.lines().filter(|l| l.starts_with("bad ")) {
        let msg = &line[..line.find(":").unwrap()];
        if !errors.remove(&msg) {
            println!("unknown error: {}", msg);
            bad = true;
        }
    }

    for error in errors {
        println!("didn't find error: {}", error);
        bad = true;
    }
    if bad {
        println!("output was:\n\n{}", o);
        panic!();
    }
}
