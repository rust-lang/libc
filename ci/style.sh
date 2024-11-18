#!/bin/sh

set -eux

rustc ci/style.rs && ./style src

rustup toolchain install nightly -c rustfmt --allow-downgrade
rustup override set nightly
command -v rustfmt
rustfmt -V
cargo fmt --all -- --check

if shellcheck --version ; then
    find . -name '*.sh' -print0 | xargs -0 shellcheck
else
    echo "shellcheck not found"
    exit 1
fi

# Ensure that `sort` output is not locale-dependent
export LC_ALL=C

for file in libc-test/semver/*.txt; do
    case "$file" in 
      *TODO*) continue ;;
    esac

    if ! sort -C "$file"; then
        echo "Unsorted semver file $file"
        exit 1
    fi

    duplicates=$(uniq -d "$file")
    if [ -n "$duplicates" ]; then
        echo "Semver file $file contains duplicates:"
        echo "$duplicates"

        exit 1
    fi
done
