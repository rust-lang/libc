#!/bin/sh

set -e

rm -rf target/doc
mkdir -p target/doc

cp ci/landing-page-head.html target/doc/index.html

TARGETS=`grep html_root_url src/lib.rs | sed 's/.*".*\/\(.*\)"/\1/'`

for target in $TARGETS; do
  echo documenting $target

  rustdoc -o target/doc/$target --target $target src/lib.rs --cfg dox \
    --crate-name libc

  echo "<li><a href="$target/libc/index.html">$target</a></li>" \
    >> target/doc/index.html
done

cat ci/landing-page-footer.html >> target/doc/index.html

if [ "$TRAVIS_PULL_REQUEST" = "false" ] && [ "$TRAVIS_BRANCH" = "autotest" ]; then
  pip install ghp-import --user $USER
  $HOME/.local/bin/ghp-import -n target/doc
  git push -qf https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git gh-pages
fi
