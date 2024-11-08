#!/bin/env bash
#:
#: name = "x86_64-unknown-illumos"
#: variety = "basic"
#: target = "helios-latest"

exec .github/buildomat/build-and-test.sh x86_64-unknown-illumos
