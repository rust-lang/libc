#!/usr/bin/env python3

import os
import re
import subprocess as sp
import sys

from difflib import unified_diff
from glob import iglob
from pathlib import Path


FMT_DIRS = ["src", "ci"]
IGNORE_FILES = [
    # Too much special syntax that we don't want to format
    "src/macros.rs"
]


def main():
    # if `CI` is set, do a check rather than overwriting
    check_only = os.getenv("CI") is not None
    run(["rustfmt", "-V"])

    fmt_files = []
    for dir in FMT_DIRS:
        fmt_files.extend(iglob(f"{dir}/**/*.rs", recursive=True))

    for file in fmt_files:
        if file in IGNORE_FILES:
            continue
        fmt_one(Path(file), check_only)

    # Run once from workspace root to get everything that wasn't handled as an
    # individual file.
    if check_only:
        run(["cargo", "fmt", "--check"])
    else:
        run(["cargo", "fmt"])

    for file in iglob("libc-test/semver/*.txt"):
        check_semver_file(Path(file))

    # Style tests
    run(
        [
            "cargo",
            "test",
            "--manifest-path=libc-test/Cargo.toml",
            "--test=style",
            "--",
            "--nocapture",
        ]
    )

    try:
        run(["shellcheck", "--version"])
    except sp.CalledProcessError:
        eprint("ERROR: shellcheck not found")
        exit(1)

    for file in iglob("**/*.sh", recursive=True):
        run(["shellcheck", file])


def fmt_one(fpath: Path, check_only: bool):
    eprint(f"Formatting {fpath}")
    text = fpath.read_text()

    # Rustfmt doesn't format the bodies of `{ ... }` macros, which is most of `libc`. To
    # make things usable, we do some hacks to replace macros with some kind of
    # alternative syntax that gets formatted about how we want, then reset the changes
    # after formatting.

    # Turn all braced macro `foo! { /* ... */ }` invocations into
    # `fn foo_fmt_tmp() { /* ... */ }`, since our macro bodies are usually valid in
    # a function context.
    text = re.sub(r"(?!macro_rules)\b(\w+)!\s*\{", r"fn \1_fmt_tmp() {", text)

    # Replace `if #[cfg(...)]` within `cfg_if` with `if cfg_tmp!([...])` which
    # `rustfmt` will format. We put brackets within the parens so it is easy to
    # match (trying to match parentheses would catch the first closing `)` which
    # wouldn't be correct for something like `all(any(...), ...)`).
    text = re.sub(r"if #\[cfg\((.*?)\)\]", r"if cfg_tmp!([\1])", text, flags=re.DOTALL)

    # The `c_enum!` macro allows anonymous enums without names, which isn't valid
    # syntax. Replace it with a dummy name.
    text = re.sub(r"enum #anon\b", r"enum _fmt_anon", text)

    # Invoke rustfmt passing via stdin/stdout so we don't need to write the file. Exits
    # on failure.
    cmd = ["rustfmt", "--config-path=.rustfmt.toml"]
    if check_only:
        res = check_output(cmd + ["--check"], input=text)

        # Unfortunately rustfmt on stdin always completes with 0 exit code even if
        # there are errors, so we need to pick between writing the file to disk or
        # relying on empty stdout to indicate success.
        # <https://github.com/rust-lang/rustfmt/issues/5376>.
        if len(res) == 0:
            return
        eprint(f"ERROR: File {fpath} is not properly formatted")
        print(res)
        exit(1)
    else:
        text = check_output(cmd, input=text)

    # Restore all changes in the formatted text
    text = re.sub(r"fn (\w+)_fmt_tmp\(\)", r"\1!", text)
    text = re.sub(r"cfg_tmp!\(\[(.*?)\]\)", r"#[cfg(\1)]", text, flags=re.DOTALL)
    text = re.sub(r"enum _fmt_anon", r"enum #anon", text)

    # And write the formatted file back
    fpath.write_text(text)


def check_semver_file(fpath: Path):
    if "TODO" in str(fpath):
        eprint(f"Skipping semver file {fpath}")
        return

    eprint(f"Checking semver file {fpath}")

    text = fpath.read_text()
    lines = text.splitlines()
    sort = sorted(lines)
    if lines != sort:
        eprint(f"ERROR: Unsorted semver file {fpath}")
        eprint("\n".join(unified_diff(lines, sort, lineterm="")))
        exit(1)

    duplicates = []
    seen = set()
    for line in lines:
        if line in seen:
            duplicates.append(line)
        seen.add(line)

    if len(duplicates) > 0:
        eprint(f"ERROR: Duplicates in semver file {fpath}")
        eprint(duplicates)
        exit(1)


def check_output(args: list[str], **kw) -> str:
    xtrace(args)
    return sp.check_output(args, encoding="utf8", text=True, **kw)


def run(args: list[str], **kw) -> sp.CompletedProcess:
    xtrace(args)
    return sp.run(args, check=True, text=True, **kw)


def xtrace(args: list[str]):
    astr = " ".join(args)
    eprint(f"+ {astr}")


def eprint(*args, **kw):
    print(*args, file=sys.stderr, **kw)


if __name__ == "__main__":
    main()
