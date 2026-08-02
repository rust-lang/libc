#!/usr/bin/env python3
"""decides which ci groups a pull request touches.
tier1 jobs read the result and skip platforms a diff doesnt affect"""

import argparse
import fnmatch
import json
import os
import subprocess as sp
import sys
from pathlib import Path
from typing import List, Optional, Sequence

# everything under these runs on all platforms
CORE_PATHS = [
    "src/lib.rs", "src/types.rs", "src/macros.rs", "src/primitives.rs", "src/unix/mod.rs",
    "build.rs", "Cargo.toml", "Cargo.lock",
    "ci/**", ".github/**",
    "ctest/**", "ctest-test/**", "libc-test/**", "semver/**", "etc/**",
]

#platform groups, matched against the changed files
GROUPS = {
    "core": CORE_PATHS,
    "apple": ["src/unix/bsd/apple/**", "src/unix/bsd/freebsdlike/**", "src/unix/bsd/mod.rs"],
    "linux_gnu": ["src/unix/linux_like/**"],
    "windows_msvc": ["src/windows/msvc/**", "src/windows/mod.rs"],
    "windows_gnu": ["src/windows/gnu/**", "src/windows/mod.rs"],
}

# docs and metadata files dont need a test run
DOCS_ONLY = ["README.md", "CHANGELOG.md", "CONTRIBUTING.md", "LICENSE*", "triagebot.toml"]


def classify(paths: Sequence[str]) -> List[str]:
    if all(any(p.startswith(pat[:-3]) if pat.endswith("/**") else fnmatch.fnmatch(p, pat) for pat in DOCS_ONLY) for p in paths):
        return []
    hit = set()
    for p in paths:
        for g, pats in GROUPS.items():
            if any(p.startswith(pat[:-3]) if pat.endswith("/**") else fnmatch.fnmatch(p, pat) for pat in pats):
                hit.add(g)
    # nothing matched, run all rather than skip something that matters
    if not hit:
        return sorted(GROUPS)
    # print(hit)  # debug
    return sorted(hit)


def diff_names(base: str, head: str) -> List[str]:
    try:
        out = sp.run(["git", "diff", "--name-only", f"{base}...{head}"], capture_output=True, text=True, check=True)
    except sp.CalledProcessError as err:
        sys.exit(f"git diff failed for {base}..{head}, {err}")
    return out.stdout.splitlines()


def changed_paths() -> Optional[List[str]]:
    #local runs have no event file, nothing to classify
    event_path = os.environ.get("GITHUB_EVENT_PATH")
    if not event_path:
        return None
    try:
        ev = json.loads(Path(event_path).read_text())
    except (OSError, ValueError) as err:
        sys.exit(f"cannot read event file {event_path}, {err}")
    pr = ev.get("pull_request")
    mq = ev.get("merge_group")
    if pr:
        return diff_names(pr["base"]["sha"], "HEAD")
    if mq:
        return diff_names(mq["base_sha"], mq["head_sha"])
    #schedule and dispatch carry no diff, run everything
    return None


def sanity() -> None:
    cases = [
        (["src/unix/bsd/apple/x.rs"], ["apple"]),
        (["src/unix/bsd/freebsdlike/x.rs"], ["apple"]),
        (["src/unix/bsd/mod.rs"], ["apple"]),
        (["src/unix/linux_like/linux/gnu/x.rs"], ["linux_gnu"]),
        (["src/windows/msvc/x.rs"], ['windows_msvc']),
        (["src/windows/gnu/x.rs"], ["windows_gnu"]),
        (["src/windows/mod.rs"], ["windows_gnu", "windows_msvc"]),
        (["Cargo.toml"], ["core"]),
        (["ci/run.sh"], ["core"]),
        (["src/types.rs"], ["core"]),
        (["README.md"], []),
        #newlib has no tier1 target, so the fail-safe runs everything
        (["src/newlib/mod.rs"], sorted(GROUPS)),
        (["README.md", "src/newlib/mod.rs"], sorted(GROUPS)),
    ]
    for paths, want in cases:
        got = classify(paths)
        assert got == want, f"expected {want} got {got} for {paths}"
    print("all good")


def main() -> None:
    p = argparse.ArgumentParser()
    p.add_argument("--files", nargs="+", help="git paths to classify")
    p.add_argument("--sanity", action="store_true")
    args = p.parse_args()
    if args.sanity:
        sanity()
        return
    paths = args.files if args.files else changed_paths()
    groups = list(GROUPS) if paths is None else classify(paths)
    print(f"changed files {json.dumps(paths)}")
    print(f"groups {json.dumps(groups)}")
    out_path = os.environ.get("GITHUB_OUTPUT")
    if out_path:
        with open(out_path, "a") as out:
            out.write(f"changes={json.dumps(groups)}\n")


if __name__ == "__main__":
    main()
