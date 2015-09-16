use std::process::Command;
use std::env;

fn cmd(name: &str) -> Command {
    let mut p = env::current_exe().unwrap();
    p.pop();
    p.push(name);
    Command::new(p)
}

#[test]
fn t1() {
    let mut c = cmd("t1");
    let output = c.output().unwrap();
    assert!(output.status.success());
}

#[test]
fn t2() {
    let mut c = cmd("t2");
    let output = c.output().unwrap();
    assert!(!output.status.success());
}
