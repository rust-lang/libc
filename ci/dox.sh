#!/usr/bin/env sh

# Builds documentation for all target triples that we have a registered URL for
# in liblibc. This scrapes the list of triples to document from `src/lib.rs`
# which has a bunch of `html_root_url` directives we pick up.

set -ex

TARGET_DOC_DIR=target/doc
README=README.md
PLATFORM_SUPPORT=platform-support.md

rm -rf $TARGET_DOC_DIR
mkdir -p $TARGET_DOC_DIR

# List all targets that do currently build successfully:
# shellcheck disable=SC1003
grep '[\d|\w|-]* \\' ci/build.sh > targets
sed -i.bak 's/ \\//g' targets
grep '^[_a-zA-Z0-9-]*$' targets > tmp && mv tmp targets

# Create a markdown list of supported platforms in $PLATFORM_SUPPORT
rm $PLATFORM_SUPPORT || true

printf '### Platform-specific documentation\n' >> $PLATFORM_SUPPORT

while read -r target; do
  echo "documenting ${target}"

  #rustdoc -o "$TARGET_DOC_DIR/${target}" --target "${target}" src/lib.rs --cfg cross_platform_docs \
  #  --crate-name libc

  echo "* [${target}](${target}/libc/index.html)" >> $PLATFORM_SUPPORT
done < targets

# Replace <div class="platform_support"></div> with the contents of $PLATFORM_SUPPORT
cp $README $TARGET_DOC_DIR
line=$(grep -n '<div class="platform_docs"></div>' $README | cut -d ":" -f 1)

set +x
{ head -n "$((line-1))" $README; cat $PLATFORM_SUPPORT; tail -n "+$((line+1))" $README; } > $TARGET_DOC_DIR/$README
set -x

# If we're on travis, not a PR, and on the right branch, publish!
if [ "$TRAVIS_PULL_REQUEST" = "false" ] && [ "$TRAVIS_BRANCH" = "master" ]; then
  pip install ghp_import --install-option="--prefix=$HOME/.local"
  "${HOME}/.local/bin/ghp-import" -n $TARGET_DOC_DIR
  git push -qf "https://${GH_TOKEN}@github.com/${TRAVIS_REPO_SLUG}.git" gh-pages
fi
