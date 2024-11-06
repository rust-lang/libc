#!/bin/env bash
#:
#: name = "x86_64-unknown-illumos"
#: variety = "basic"
#: target = "helios-latest"
#: rust_toolchain = "stable"

exec .github/buildomat/build-and-test.sh illumos
