#!/bin/sh

set -eux

if [ -n "${CI:-}" ]; then
    rustup toolchain install nightly -c rustfmt --allow-downgrade
    rustup override set nightly

    check="--check"
fi

rustc ci/style.rs && ./style src

command -v rustfmt
rustfmt -V

# Run once to cover everything that isn't in `src/`
cargo fmt

# Save a list of all source files
tmpfile="file-list~" # trailing tilde for gitignore
find src -name '*.rs' > "$tmpfile"

# Before formatting, replace all macro identifiers with a function signature.
# This allows `rustfmt` to format it.
while IFS= read -r file; do
    if [ "$file" = "src/macros.rs" ]; then
        # Too much special syntax in `macros.rs` that we don't want to format
        continue
    fi

    # Turn all braced macro `foo! { /* ... */ }` invocations into
    # `fn foo_fmt_tmp() { /* ... */ }`.
    perl -pi -e 's/(?!macro_rules)\b(\w+)!\s*\{/fn $1_fmt_tmp() {/g' "$file"

    # Replace `if #[cfg(...)]` within `cfg_if` with `if cfg_tmp!([...])` which
    # `rustfmt` will format. We put brackets within the parens so it is easy to
    # match (trying to match parentheses would catch the first closing `)` which
    # wouldn't be correct for something like `all(any(...), ...)`).
    perl -pi -0777 -e 's/if #\[cfg\((.*?)\)\]/if cfg_tmp!([$1])/gms' "$file"

    # We have some instances of `{const}` that make macros happy but aren't
    # valid syntax. Replace this with just the keyword, plus an indicator
    # comment on the preceding line (which is where rustfmt puts it. Also
    # rust-lang/rustfmt#5464).
    perl -pi -e 's/^(\s*)(.*)\{const\}/$1\/\* FMT-CONST \*\/\n$1$2const/g' "$file"

    # Format the file. We need to invoke `rustfmt` directly since `cargo fmt`
    # can't figure out the module tree with the hacks in place.
    failed=false
    rustfmt --config-path rustfmt.toml "$file" ${check:+"$check"} || failed=true

    # Restore all changes to the files.
    perl -pi -e 's/fn (\w+)_fmt_tmp\(\)/$1!/g' "$file"
    perl -pi -0777 -e 's/cfg_tmp!\(\[(.*?)\]\)/#[cfg($1)]/gms' "$file"
    perl -pi -0777 -e 's/\/\* FMT-CONST \*\/(?:\n\s*)?(.*?)const/$1\{const\}/gms' "$file"

    # Defer emitting the failure until after the files get reset
    if [ "$failed" != "false" ]; then
        echo "Formatting failed"
        exit 1
    fi
done < "$tmpfile"

rm "$tmpfile"

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
