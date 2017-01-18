// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// This is a script to deploy and execute a binary on an x86-64 iOS simulator.
// The primary use of this is to be able to run unit tests on the simulator and retrieve the results.
//
// To do this through Cargo instead, use Dinghy (https://github.com/snipsco/dinghy): 
// cargo dinghy install, then cargo dinghy test.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;

// Step one: Wrap as an app
fn package_as_simulator_app(crate_name: &str, test_binary_path: &Path) {
    println!("Packaging simulator app");
    Command::new("rm").arg("-rf").arg("ios_simulator_app").status().unwrap();
    Command::new("mkdir").arg("ios_simulator_app").check_status();
    Command::new("cp").arg(test_binary_path).arg(["ios_simulator_app/", crate_name].join("")).check_status();
    let mut f = File::create("ios_simulator_app/Info.plist").unwrap();
    f.write_all(&[
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>",
        "<!DOCTYPE plist PUBLIC \"-//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">",
        "<plist version=\"1.0\">",
        "    <dict>",
        "        <key>CFBundleExecutable</key>",
      &["        <string>", crate_name, "</string>"].join(""),
        "        <key>CFBundleIdentifier</key>",
        "        <string>com.rust.unittests</string>",
        "        <key>UIRequiredDeviceCapabilities</key>",
        "        <array>",
        "            <string>x86_64</string>",
        "        </array>",
        "    </dict>",
        "</plist>"].join("\n").into_bytes()).unwrap();
}

// Step two: Start the iOS simulator
fn start_simulator() {
    println!("Looking for iOS simulator");
    let output = Command::new("xcrun").arg("simctl").arg("list").output().unwrap();
    let mut simulator_exists = false;
    let mut simulator_booted = false;
    let mut found_rust_sim = false;
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        if line.contains("rust_ios") {
            if found_rust_sim {
                panic!("Duplicate rust_ios simulators found. Please double-check xcrun simctl list.");
            }
            simulator_exists = true;
            simulator_booted = line.contains("(Booted)");
            found_rust_sim = true;
        }
    }

    if simulator_exists == false {
        println!("Creating iOS simulator");
        Command::new("xcrun").arg("simctl").arg("create").arg("rust_ios")
            .arg("com.apple.CoreSimulator.SimDeviceType.iPhone-SE").arg("com.apple.CoreSimulator.SimRuntime.iOS-10-2").check_status();
    } else if simulator_booted == true {
        println!("Shutting down already-booted simulator");
        Command::new("xcrun").arg("simctl").arg("shutdown").arg("rust_ios").check_status();
    }

    println!("Starting iOS simulator");
    // We can't uninstall the app (if present) as that will hang if the simulator isn't completely booted;just erase the simulator instead.
    Command::new("xcrun").arg("simctl").arg("erase").arg("rust_ios").check_status();
    Command::new("xcrun").arg("simctl").arg("boot").arg("rust_ios").check_status();
}

// Step three: Install the app
fn install_app_to_simulator() {    
    println!("Installing app to simulator");
    Command::new("xcrun").arg("simctl").arg("install").arg("booted").arg("ios_simulator_app/").check_status();
}

// Step four: Run the app
fn run_app_on_simulator() {
    println!("Running app");
    let output = Command::new("xcrun").arg("simctl").arg("launch").arg("--console").arg("booted").arg("com.rust.unittests").output().unwrap();
    let mut test_run_passed = false;
    let stdout = String::from_utf8(output.stdout).unwrap();
    for line in stdout.lines() {
        println!("{}", line);
        
        if test_run_passed == false {
            // Based on all.rs test output
            test_run_passed = line.contains("PASSED") && line.contains("tests");
        }
    }

    println!("Shutting down simulator");
    Command::new("xcrun").arg("simctl").arg("shutdown").arg("rust_ios").check_status();
    assert!(test_run_passed);
}

trait CheckStatus {
    fn check_status(&mut self);
}

impl CheckStatus for Command {
    fn check_status(&mut self) {
        assert!(self.status().unwrap().success());
    }
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {:?} executable", Path::new(&args[0]).file_name().unwrap());
        process::exit(-1);
    }
    
    let test_binary_path = Path::new(&args[1]);
    let crate_name = test_binary_path.file_name().unwrap();

    package_as_simulator_app(crate_name.to_str().unwrap(), test_binary_path);
    start_simulator();
    install_app_to_simulator();
    run_app_on_simulator();
}