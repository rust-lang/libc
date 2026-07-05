#!/usr/bin/env python3
"""Helper utilities for common libc tasks."""

import argparse
import copy
import datetime as dt
import functools
import json
import os
import pprint
import re
import subprocess as sp
import sys
from dataclasses import dataclass
from inspect import cleandoc
from multiprocessing import Pool
from pathlib import Path

REPO_OWNER = "rust-lang"
REPO = "libc"


def main() -> None:
    p = argparse.ArgumentParser(
        description="Utilities for helping with libc development"
    )
    sub = p.add_subparsers(required=True)

    p_cat = sub.add_parser(
        "check-all-targets",
        aliases=["cat"],
        help="run `cargo check` on some or all targets (see subcommand help for more)",
        description=CheckAllTargets.__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    p_cat.add_argument("package", help="specify the package to build")
    p_cat.add_argument("-o", "--only", help="filter tested targets by this regex")
    p_cat.add_argument("-s", "--skip", help="skip targets matching this regex")
    p_cat.add_argument(
        "cargo_args",
        nargs="*",
        metavar="cargo-args",
        help="extra arguments for `cargo check`",
    )
    p_cat.set_defaults(
        func=lambda args: CheckAllTargets.prepare().check_all_targets(
            package=args.package,
            only=args.only,
            skip=args.skip,
            cargo_args=args.cargo_args,
        )
    )

    p_rel = sub.add_parser(
        "relabel",
        help="replace the stable-nominated label with stable-applied",
        description=Relabel.__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    p_rel.add_argument(
        "pr_number", metavar="pr-number", help="pull request for the backport"
    )
    p_rel.set_defaults(func=lambda args: Relabel(pr_number=args.pr_number).execute())

    p_log = sub.add_parser(
        "make-changelog",
        help="collect changelog entries",
        description=MakeChangelog.__doc__,
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    p_log.add_argument(
        "--old-tag",
        metavar="old-tag",
        help="tag of the previous revision to serve as the changelog base",
        required=True,
    )
    p_log.add_argument(
        "--new-ref",
        metavar="new-ref",
        help="ref to compare to the old tag",
        default="libc-0.2",
    )
    p_log.set_defaults(
        func=lambda args: MakeChangelog(
            old_tag=args.old_tag, new_ref=args.new_ref
        ).execute()
    )

    args = p.parse_args()
    args.func(args)


@dataclass(kw_only=True)
class Relabel:
    """
    Replace the stable-nominated label with stable-applied, given the number of
    a PR listing backported PRs.

    The backport PR should have a list of items like `* http://.../libc/pull/1234`.
    """

    pr_number: int

    def execute(self) -> None:
        num = self.pr_number
        j = check_output(
            [
                "gh",
                "pr",
                "view",
                f"https://github.com/{REPO_OWNER}/{REPO}/pull/{num}",
                "--json=baseRefName,body,state,title",
            ]
        )
        d = json.loads(j)
        base: str = d["baseRefName"]
        body: str = d["body"]
        state: str = d["state"]
        title: str = d["title"]

        if state != "MERGED":
            print(f'expected MERGED state; got {state} for "{title}" (#{num})')
            exit(1)
        if base != "libc-0.2":
            print(f'expected libc-0.2 base ref; got {base} for "{title}" (#{num})')
            exit(1)

        print(f'Relabling PRs listed in {num} "{title}"')

        to_relabel = []
        for line in body.splitlines():
            re.match(r"^[-*]\s*http", line)
            if not re.match(r"^[-*]\s*http", line):
                continue

            match = re.match(
                r"^[-*]\s*https?://github.com/rust-lang/libc/pull/(\d+)", line
            )
            if match is None:
                print(
                    f"{E.YEL}Line `{line}` does not match expected link pattern{E.RST}"
                )
                continue

            num = int(match.group(1))
            to_relabel.append(num)

        # `gh` requests can be pretty slow, parallelize to help with large lists
        with Pool() as p:
            p.map(Relabel.do_relabel_inner, to_relabel)

        print("Finished relabeling")

    @staticmethod
    def do_relabel_inner(num: int):
        j = check_output(["gh", "pr", "view", pr_url(num), "--json=state,title,labels"])
        d = json.loads(j)
        state: str = d["state"]
        title: str = d["title"]
        labels: list[str] = [i["name"] for i in d["labels"]]

        if state != "MERGED":
            print(
                f'{E.YEL}expected MERGED state; got {state} for "{title}" (#{num}){E.RST}'
            )
            return
        if "stable-nominated" not in labels:
            print(
                f'{E.YEL}`stable-nominated` not in labels for "{title}" (#{num}){E.RST}'
            )

        # Use check_output to eat the stdout since otherwise `gh` draws a spinner that
        # messes up interleaved stdout.
        check_output(
            [
                "gh",
                "pr",
                "edit",
                "--remove-label=stable-nominated",
                "--add-label=stable-applied",
                pr_url(num),
            ]
        )


@dataclass(kw_only=True)
class MakeChangelog:
    """
    Prepare a template with commits merged between `old-tag` and `new-ref`, for
    appending to the changelog.
    """

    old_tag: str
    new_ref: str

    def execute(self) -> None:
        old_tag: str = self.old_tag
        new_ref: str = self.new_ref
        date = dt.datetime.now(dt.UTC).strftime("%Y-%m-%d")
        split = old_tag.split(".")
        split[2] = str(int(split[2]) + 1)  # increment patch version
        next_tag = ".".join(split)

        ret = cleandoc(f"""
            ## [{next_tag}](https://github.com/rust-lang/libc/compare/{old_tag}...{next_tag}) - {date}

            ### Support
            ### Added
            ### Deprecated
            ### Fixed
            ### Changed
            ### Removed
            ### Other
        """)
        ret += "\n\n"

        changes = check_output(
            [
                "git",
                "log",
                f"{old_tag}..{new_ref}",
                "--no-merges",
                "--oneline",
                "--reverse",
            ],
        ).splitlines()

        print(
            f"Generating changelog for {old_tag}..{new_ref}. Make sure {new_ref} "
            "is up to date!"
        )

        for logline in changes:
            sha = logline.split()[0]
            summary = check_output(["git", "log", sha, "--format=%s", "-1"], quiet=True)
            body = check_output(["git", "log", sha, "--format=%b", "-1"], quiet=True)
            summary = summary.strip()
            body = body.strip()

            link = None

            # Extract expected trailers
            orig_sha_opt = re.search(r"\(cherry picked from commit (\w+)\)", body, re.M)
            url_opt = re.search(r"^\(backport <(.*)>\)", body, re.M)

            # Prefer a PR URL if available
            if url_opt is not None:
                pr_url = url_opt[1]
                pr_opt = re.match(
                    rf"https://github.com/{REPO_OWNER}/{REPO}/pull/(\d+)", pr_url
                )
                if pr_opt is not None:
                    pr_num = pr_opt[1]
                    link = f"[#{pr_num}]({pr_url})"

            # If there is no PR URL, link the backported commit
            if link is None and orig_sha_opt is not None:
                orig_sha = orig_sha_opt[1]
                short = orig_sha[:12]
                link = f"[{short}](https://github.com/rust-lang/libc/commit/{orig_sha})"

            # If there is no PR URL or cherry pick commit, link the commit itself
            if link is None:
                patch_sha = check_output(["git", "rev-parse", sha], quiet=True).strip()
                short = patch_sha[:12]
                link = (
                    f"[{short}](https://github.com/rust-lang/libc/commit/{patch_sha})"
                )

            line = f"- {summary} ({link})"

            ret += "\n"
            ret += line

        print(
            "Copy the below to CHANGELOG.md, then sort log messages into the "
            "relevant categories:\n"
        )
        print(ret)


@dataclass(kw_only=True)
class CheckAllTargets:
    """
    Run `cargo check` on all targets, possibly with filtering.

    Query `rustc` for a list of supported targets, then run `cargo check` on each.
    `-Zbuild-std` is used if the toolchain is not installed.

    This uses a pinned toolchain and a separate target directory in
    `~/.cache/libc-build`. This is done to avoid accidental cache deletion with `cargo
    clean` or invalidation by changing toolchain, since the initial build takes so long
    that it can be worth keeping the cache around.

    The pinned toolchain can be overridden by setting RUSTC_CACHE_TOOLCHAIN.
    """

    toolchain: str
    targets: list["RustcTarget"]
    checks: list["CheckInvocation"]
    failure_limit: int

    FREEBSD_VERSIONS = [13, 14, 15]

    # Targets that don't pass for one reason or another
    BROKEN_TARGETS = [
        # libc problems
        ("aarch64-unknown-nto-qnx800", "libc error, unsupported arch"),
        ("aarch64.*-gnu_ilp32.*time_bits=64", "libc error, time64 mismatches"),
        ("armv6k-nintendo-3ds", "libc error, stat missing"),
        ("armv7-sony-vita-newlibeabihf", "libc error, stat missing"),
        ("csky.*gnu.*time_bits=64", "libc error, time64 mismatches"),
        ("i686-pc-nto-qnx700", "libc error, unsupported arch"),
        ("managarm-mlibc", "libc error, unresolved import"),
        ("x86_64-lynx-lynxos178", "libc error, unresolved import"),
        ("x86_64-pc-nto-qnx800", "libc error, unsupported arch"),
        ("x86_64-unknown-linux-none", "libc error, unresolved import"),
        # rustc problems
        ("xtensa-esp32.*", "target string mismatch in rustc"),
        ("amdgcn-amd-amdhsa", "unsupported instructions with some CPUs"),
        # llvm problems
        ("m68k-unknown-.*", "llvm crash building core"),
        ("mipsisa32r6(el)?-.*", "llvm crash building core"),
    ]

    # Flags that always need to be passed to specific targets
    EXTRA_TARGET_FLAGS = {
        # Target CPU must be specified
        "avr-none": ["-Ctarget-cpu=atmega328p"],
        # Emits a lot of warnings
        "hexagon-unknown-qurt": ["-Aunused_imports"],
    }

    @staticmethod
    def prepare() -> "CheckAllTargets":
        """Build a list of checks."""
        toolchain = CheckAllTargets.get_cache_toolchain()
        target_dir = cache_dir() / "libc-build" / toolchain
        all_targets = RustcTarget.fetch_all(toolchain)
        installed_targets = check_output(
            [
                "rustup",
                "target",
                "list",
                "--installed",
                f"--toolchain={toolchain}",
            ]
        ).splitlines()
        checks: list[CheckInvocation] = []

        # Add a check for each target and, if applicable, a variant for any cfg options
        # that also make sense to check.
        for t in all_targets:
            base = CheckInvocation(
                target=t.triple,
                name="",
                target_dir=target_dir,
                attributes={},
                installed=t.triple in installed_targets,
                skip=[],
                extra_rustflags=[],
            )

            new_checks = [base]

            # When doing variants, use a separate target directory because otherwise
            # running twice with different `RUSTFLAGS` is a cache miss.

            if t.os_ == "freebsd":
                new_checks.clear()
                for vers in CheckAllTargets.FREEBSD_VERSIONS:
                    new = copy.deepcopy(base)
                    new.attributes = base.attributes | {"freebsd": str(vers)}
                    new.target_dir = base.target_dir / f"freebsd-{vers}"
                    new.extra_rustflags = base.extra_rustflags + [
                        f'--cfg=libc_unstable_freebsd_version="{vers}"'
                    ]

                    new_checks.append(new)

            if t.env == "gnu" and t.bits == 32:
                new = copy.deepcopy(base)
                new.attributes = base.attributes | {"time_bits": "64"}
                new.target_dir = base.target_dir / "time64"
                new.extra_rustflags = base.extra_rustflags + [
                    '--cfg=libc_unstable_gnu_time_bits="64"'
                ]
                new_checks.append(new)

            if t.env == "musl" and t.bits == 32:
                new = copy.deepcopy(base)
                new.attributes = base.attributes | {"time_bits": "64"}
                new.target_dir = base.target_dir / "time64"
                new.extra_rustflags = base.extra_rustflags + [
                    "--cfg=libc_unstable_musl_v1_2_3"
                ]
                new_checks.append(new)

            # Update the name field and check whether there are any targets that we
            # always need to skip, or that need flags.
            for check in new_checks:
                if len(check.attributes) == 0:
                    check.name = f"{check.target} (default)"
                else:
                    attrs = ",".join(f"{k}={v}" for (k, v) in check.attributes.items())
                    check.name = f"{check.target} ({attrs})"

                for pat, reason in CheckAllTargets.BROKEN_TARGETS:
                    if check.pattern_matches(pat):
                        check.skip.append(reason)

                for pat, flags in CheckAllTargets.EXTRA_TARGET_FLAGS.items():
                    if check.pattern_matches(pat):
                        check.extra_rustflags.extend(flags)

            checks.extend(new_checks)

        return CheckAllTargets(
            toolchain=toolchain,
            targets=all_targets,
            checks=checks,
            failure_limit=5,
        )

    def check_all_targets(
        self,
        *,
        package: str,
        only: str | None = None,
        skip: str | None = None,
        cargo_args: list[str] = [],
    ) -> None:
        """Run checks from the populated list."""
        checks = self.checks
        ran = 0
        passed = 0
        skipped = 0
        failures = []
        matched_only_already_skipped = []

        if only is not None:
            for t in checks:
                if t.pattern_matches(only):
                    if t.skip:
                        matched_only_already_skipped.append(t.name)
                    continue
                t.skip.append("does not match --only pattern")

        if skip is not None:
            for t in checks:
                if not t.pattern_matches(skip):
                    continue
                t.skip.append("matches --skip pattern")

        # `sum(1 for _ in ...)` seems to be the best way to get an iterator's count
        total = sum(1 for _ in (t for t in checks if not t.skip))

        # List skips first so the interesting output is together at the bottom
        checks.sort(key=lambda t: len(t.skip) == 0)

        env = os.environ.copy()
        env_rustflags = env.get("RUSTFLAGS")

        for t in checks:
            fulldesc = f"{t.name} ({ran + 1} / {total})"

            if len(t.skip) > 0:
                print(f"{E.YEL}Skipping {fulldesc} ({", ".join(t.skip)}){E.RST}")
                skipped += 1
                continue

            print(f"{E.CY_B}Checking {fulldesc}{E.RST}")

            extra_args = [] if t.installed else ["-Zbuild-std=core"]
            common_args = [
                f"--package={package}",
                f"--target={t.target}",
            ]

            # We aim to be warning-free
            rustflags = ["-Dwarnings"]
            rustflags += t.extra_rustflags

            # Allow forwarding rustflags
            if env_rustflags is not None:
                rustflags += [env_rustflags]

            print(f"    {E.GRN_D}Running build{E.RST}")

            try:
                run(
                    [
                        "cargo",
                        f"+{self.toolchain}",
                        "check",
                        f"--target-dir={t.target_dir}",
                    ]
                    + common_args
                    + extra_args
                    + cargo_args,
                    env=env | {"RUSTFLAGS": " ".join(rustflags)},
                )
                ok = True
            except sp.CalledProcessError:
                ok = False

            ran += 1

            if ok:
                passed += 1
            else:
                print(f"{E.RED_B}failed: {t.target}{E.RST}")
                failures.append(t.name)

            if len(failures) > self.failure_limit:
                break

        print(
            f"finished checking {ran} targets. {passed} passed, "
            f"{len(failures)} failed, {skipped} skipped"
        )
        if len(matched_only_already_skipped) > 0:
            print(
                f"note: {len(matched_only_already_skipped)} targets matched `--only` "
                "but were already skipped"
            )

        if len(failures) > 0:
            print("failures:")
            pprint.pp(failures)

    @staticmethod
    def get_cache_toolchain() -> str:
        # Arbitrary but reasonably recent default if unset.
        return os.environ.get("RUSTC_CACHE_TOOLCHAIN") or "nightly-2026-06-24"


@dataclass(kw_only=True)
class CheckInvocation:
    """Config for a single invocation of `cargo check`."""

    target: str
    name: str  # target plus attributes
    target_dir: Path
    installed: bool
    attributes: dict[str, str]
    # skip reasons, empty if we shouldn't skip
    skip: list[str]
    extra_rustflags: list[str]

    def pattern_matches(self, pat: str) -> bool:
        """Ensure pattern matching is applied consistently"""
        assert self.name != ""
        return re.search(pat, self.name) is not None


@dataclass(kw_only=True)
class RustcTarget:
    """Config queried from rustc."""

    triple: str
    arch: str
    os_: str | None
    env: str | None
    bits: int

    @staticmethod
    @functools.cache
    def get_one(toolchain: str, triple: str) -> "RustcTarget":
        target_cfg = check_output(
            ["rustc", f"+{toolchain}", "--print=cfg", f"--target={triple}"], quiet=True
        )
        return RustcTarget(
            triple=triple,
            arch=re.findall(r'target_arch="(.*)"', target_cfg)[0],
            env=re.findall(r'target_env="(.*)"', target_cfg)[0],
            os_=re.findall(r'target_os="(.*)"', target_cfg)[0],
            bits=int(re.findall(r'target_pointer_width="(.*)"', target_cfg)[0]),
        )

    @staticmethod
    @functools.cache
    def fetch_all(toolchain: str) -> list["RustcTarget"]:
        """Collect all targets for a given toolchain."""
        all_targets = check_output(
            ["rustc", f"+{toolchain}", "--print=target-list"]
        ).splitlines()

        # Iterating targets is really slow, throw some threads at it.
        with Pool() as p:
            ret = p.starmap(RustcTarget.get_one, [(toolchain, t) for t in all_targets])

        ret.sort(key=lambda t: t.triple)
        return ret


class E:
    """ANSI escapes."""

    YEL = "\033[33m"
    GRN = "\033[32m"

    # Bright colors
    CY_B = "\033[1;36m"
    YEL_B = "\033[1;33m"
    RED_B = "\033[1;31m"

    # Dimmed colors
    YEL_D = "\033[2;33m"
    GRN_D = "\033[2;32m"

    RST = "\033[0m"


@functools.cache
def cache_dir() -> Path:
    xdg_cache = os.environ.get("XDG_CACHE_DIR")
    if xdg_cache is not None:
        return Path(xdg_cache)
    return Path.home() / ".cache"


def pr_url(num: int) -> str:
    return f"https://github.com/{REPO_OWNER}/{REPO}/pull/{num}"


def check_output(args: list[str], *, quiet: bool = False, **kw) -> str:
    if not quiet:
        xtrace(args, env=kw.get("env"))
    return sp.check_output(args, encoding="utf8", text=True, **kw)


def run(args: list[str], *, quiet: bool = False, **kw) -> sp.CompletedProcess:
    if not quiet:
        xtrace(args, env=kw.get("env"))
    return sp.run(args, check=True, text=True, **kw)


def xtrace(args: list[str], *, env: dict[str, str] | None) -> None:
    """Print commands before running them."""
    astr = " ".join(str(arg) for arg in args)
    if env is None:
        eprint(f"+ {astr}")
    else:
        envdiff = set(env.items()) - set(os.environ.items())
        estr = " ".join(f"{k}='{v}'" for (k, v) in envdiff)
        eprint(f"+ {estr} {astr}")


def eprint(*args, **kw) -> None:
    print(*args, file=sys.stderr, **kw)


if __name__ == "__main__":
    main()
