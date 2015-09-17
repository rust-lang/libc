#!/bin/sh

set -e

rm -rf target/doc
mkdir -p target/doc

doc() {
  local _target=$1
  echo documenting $_target
  rustdoc -o target/doc/$_target --target $_target src/lib.rs --cfg dox
}

doc x86_64-unknown-linux-gnu
doc i686-unknown-linux-gnu
doc x86_64-apple-darwin
doc i686-apple-darwin
doc x86_64-pc-windows-gnu
doc x86_64-pc-windows-msvc
doc i686-pc-windows-gnu
doc i686-pc-windows-msvc

doc arm-unknown-linux-gnueabihf
doc mips-unknown-linux-gnu
doc arm-linux-androideabi
doc x86_64-unknown-linux-musl

cp ci/landing-page.html target/doc/index.html

if [ "$TRAVIS_PULL_REQUEST" = "false" ] && [ "$TRAVIS_BRANCH" = "autotest" ]; then
  pip install ghp-import --user $USER
  $HOME/.local/bin/ghp-import -n target/doc
  git push -qf https://${TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
fi
