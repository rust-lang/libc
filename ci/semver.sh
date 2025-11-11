#!/bin/bash
# Download a baseline crate to run semver checks against

set -euxo pipefail

# https://crates.io/data-access#api

# Need to include versions to get the default and stable versions
meta=$(curl -L https://index.crates.io/li/bc/libc)

# Versions to check against
if [ "${SERIES:-}" = "0.2" ]; then
    pat="^0.2"
elif [ "${SERIES:-}" = "1.0" ]; then
    pat="^1.0"
else
    echo "SERIES must be set to either '0.2' or '1.0'"
    exit 1
fi

# Find the most recent version matching a pattern.
release=$(
    echo "$meta" |
    jq -er --slurp --arg pat "$pat" 'map(select(.vers | test($pat))) | last'
)
version=$(echo "$release" | jq -r '.vers')

crate_dir="libc-$version"
curl -L "https://static.crates.io/crates/libc/libc-$version.crate" | tar xzf -

# Need to convince Cargo it's not part of our workspace
echo '[workspace]' >> "$crate_dir/Cargo.toml"

echo "BASELINE_CRATE_DIR=$(realpath "$crate_dir")" >> "$GITHUB_ENV"
