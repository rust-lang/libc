#!/bin/bash
# Download a baseline crate to run semver checks against

set -euxo pipefail

# Retrieve the index for libc
index=$(curl -L https://index.crates.io/li/bc/libc)

# Regex for versions matching what we want to check against. Note we check only
# a suffix since in the merge queue `base_ref` is set to something like
# `refs/heads/main` rather than only the branch name.
if [[ "${TARGET_REF:-}" = *"libc-0.2" ]]; then
    pat="^0.2"
elif [[ "${TARGET_REF:-}" = *"main" ]]; then
    pat="^1.0"
else
    echo "TARGET_REF must be set and end with either 'libc-0.2' or 'main'"
    exit 1
fi

# Find the most recent version matching a pattern.
version=$(
    echo "$index" |
        jq -er --slurp --arg pat "$pat" '
        map(select(.vers | test($pat)))
        | last
        | debug("version:", .)
        | .vers
        '
)

libc_cache="${XDG_CACHE_DIR:-$HOME/.cache}/libc-ci/"
mkdir -p "$libc_cache"

curl -L "https://static.crates.io/crates/libc/libc-$version.crate" | tar xzf - -C "$libc_cache"
crate_dir="$libc_cache/libc-$version"

echo "BASELINE_CRATE_DIR=$crate_dir" >> "$GITHUB_ENV"
