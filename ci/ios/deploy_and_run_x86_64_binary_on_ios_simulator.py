#!/usr/bin/env python
#
# Copyright 2017 The Rust Project Developers. See the COPYRIGHT
# file at the top-level directory of this distribution and at
# http://rust-lang.org/COPYRIGHT.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.
#
# This is a script to deploy and execute a binary on an x86-64 iOS simulator.
# The primary use of this is to be able to run unit tests on the simulator and retrieve the results.
#
# To do this through Cargo instead, use Dinghy (https://github.com/snipsco/dinghy): 
# cargo dinghy install, then cargo dinghy test.

import os.path
import subprocess
import sys
import time

# Step one: Wrap as an app
def package_as_simulator_app(crate_name, test_binary_path):
	print 'Packaging simulator app'
	subprocess.call(['rm', '-rf', 'ios_simulator_app'])
	subprocess.check_call(['mkdir', 'ios_simulator_app'])
	subprocess.check_call(['cp', test_binary_path, 'ios_simulator_app/' + crate_name])
	f = open('ios_simulator_app/Info.plist', 'w')
	f.write('<?xml version="1.0" encoding="UTF-8"?>\n'
			'<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">\n'
			'<plist version="1.0">\n'
			'	<dict>\n'
			'		<key>CFBundleExecutable</key>\n'
			'		<string>' + crate_name + '</string>\n'
			'		<key>CFBundleIdentifier</key>\n'
			'		<string>com.rust.unittests</string>\n'
			'		<key>UIRequiredDeviceCapabilities</key>\n'
			'		<array>\n'
			'			<string>x86_64</string>\n'
			'		</array>\n'
			'	</dict>\n'
			'</plist>\n')

# Step two: Start the iOS simulator
def start_simulator():
	print 'Looking for iOS simulator'
	xcrun_list = subprocess.Popen(['xcrun', 'simctl', 'list'], stdout=subprocess.PIPE, bufsize=1, universal_newlines=True)
	simulator_exists = False
	simulator_booted = False
	found_rust_sim = False
	for line in xcrun_list.stdout:
		if "rust_ios" in line:
			if found_rust_sim == True:
				raise Exception("Duplicate rust_ios simulators found. Please double-check xcrun simctl list.")
			simulator_exists = True
			simulator_booted = "(Booted)" in line
			found_rust_sim = True
	
	if simulator_exists == False:
		print 'Creating iOS simulator'
		subprocess.check_output(['xcrun',  'simctl', 'create', 'rust_ios', 'com.apple.CoreSimulator.SimDeviceType.iPhone-SE', 'com.apple.CoreSimulator.SimRuntime.iOS-10-2'])
	elif simulator_booted == True:
		print 'Shutting down already-booted simulator'
		subprocess.call(['xcrun', 'simctl', 'shutdown', 'rust_ios'])
	
	print 'Starting iOS simulator'
	# We can't uninstall the app (if present) as that will hang if the simulator isn't completely booted; just erase the simulator instead.
	subprocess.check_call(['xcrun', 'simctl', 'erase', 'rust_ios'])
	subprocess.check_call(['xcrun', 'simctl', 'boot', 'rust_ios'])

# Step three: Install the app
def install_app_to_simulator():
	print 'Installing app to simulator'	
	subprocess.check_call(['xcrun', 'simctl', 'install', 'booted', 'ios_simulator_app/'])

# Step four: Run the app
def run_app_on_simulator():
	print 'Running app'
	try:
		test_run = subprocess.Popen(['xcrun', 'simctl', 'launch', '--console', 'booted', 'com.rust.unittests'], stdout=subprocess.PIPE, bufsize=1, universal_newlines=True)
		test_run_failed = False
		for line in test_run.stdout:
			sys.stdout.write(line)
			if test_run_failed == False:
				# Based on all.rs test output
				test_run_failed = 'some tests failed' in line
		
		sys.stdout.flush()
		if test_run_failed == True:
			raise Exception('Some tests failed')
	finally:
		print 'Shutting down simulator'
		subprocess.call(['xcrun', 'simctl', 'shutdown', 'rust_ios'])

# Run all steps in sequence
# TODO 1: Use a /tmp place for the app instead of current dir?
def main():
	if len(sys.argv) != 2:
		print 'usage: ' + os.path.basename(sys.argv[0]) + ' executable'
		sys.exit(-1)
	
	crate_name = os.path.basename(sys.argv[1])
	test_binary_path = sys.argv[1]

	package_as_simulator_app(crate_name, test_binary_path)
	start_simulator()
	install_app_to_simulator()
	run_app_on_simulator()

# Entry point
if __name__ == '__main__':
    main()