#!/usr/bin/env python3

import argparse
import os
import platform
import re
import subprocess as sp
import sys
import time
from dataclasses import dataclass, field
from enum import Enum, IntEnum
from typing import Optional


ESC_CYAN = "\033[1;36m"
ESC_END = "\033[0m"


class Os(Enum):
    LINUX = "Linux"
    WINDOWS = "Windows"
    DARWIN = "Darwin"


class Toolchain(IntEnum):
    OTHER = 0  # msrv
    STABLE = 1
    BETA = 2
    NIGHTLY = 3


@dataclass
class Cfg:
    toolchain_name: str
    toolchain: Toolchain = field(init=False)
    host_target: str = field(init=False)
    os_: Os = field(init=False)

    def __post_init__(self):
        rustc_output = check_output(["rustc", f"+{self.toolchain_name}", "-vV"])
        self.host_target = re.findall(r"host: (.*)", rustc_output)[0]
        if "nightly" in self.toolchain_name:
            self.toolchain = Toolchain.NIGHTLY
        elif "beta" in self.toolchain_name:
            self.toolchain = Toolchain.BETA
        elif "stable" in self.toolchain_name:
            self.toolchain = Toolchain.STABLE
        else:
            self.toolchain = Toolchain.OTHER
        self.os_ = Os(platform.system())
        eprint(f"Testing Rust {self.toolchain_name} on {self.os_}")

    def nightly(self) -> bool:
        return self.toolchain == Toolchain.NIGHTLY


@dataclass
class Target:
    name: str
    dist: bool = True
    min_toolchain: Toolchain = Toolchain.OTHER

    def __post_init__(self):
        if not self.dist:
            # We will need to use build-std
            self.min_toolchain = Toolchain.NIGHTLY


FREEBSD_VERSIONS = [11, 12, 13, 14, 15]

# FIXME(ohos): CI fails with warnings
TARGETS = [
    # Tier 1
    Target("aarch64-apple-darwin"),
    Target("aarch64-pc-windows-msvc"),
    Target("aarch64-unknown-linux-gnu"),
    Target("i686-pc-windows-msvc"),
    Target("i686-unknown-linux-gnu"),
    Target("x86_64-pc-windows-gnu"),
    Target("x86_64-pc-windows-msvc"),
    Target("x86_64-unknown-linux-gnu"),
    #
    # Tier 2 with host tools
    Target("aarch64-pc-windows-gnullvm", min_toolchain=Toolchain.STABLE),
    Target("aarch64-unknown-linux-musl"),
    # Target("aarch64-unknown-linux-ohos"),
    Target("arm-unknown-linux-gnueabi"),
    Target("arm-unknown-linux-gnueabihf"),
    Target("armv7-unknown-linux-gnueabihf"),
    # Target("armv7-unknown-linux-ohos"),
    Target("i686-pc-windows-gnu"),
    Target("loongarch64-unknown-linux-gnu", min_toolchain=Toolchain.STABLE),
    Target("loongarch64-unknown-linux-musl", min_toolchain=Toolchain.STABLE),
    Target("powerpc-unknown-linux-gnu"),
    Target("powerpc64-unknown-linux-gnu"),
    Target("powerpc64le-unknown-linux-gnu"),
    Target("powerpc64le-unknown-linux-musl", min_toolchain=Toolchain.STABLE),
    Target("riscv64gc-unknown-linux-gnu"),
    Target("s390x-unknown-linux-gnu"),
    Target("sparcv9-sun-solaris"),
    Target("x86_64-apple-darwin"),
    Target("x86_64-pc-solaris"),
    Target("x86_64-pc-windows-gnullvm", min_toolchain=Toolchain.STABLE),
    Target("x86_64-unknown-freebsd"),
    Target("x86_64-unknown-illumos"),
    Target("x86_64-unknown-linux-musl"),
    # Target("x86_64-unknown-linux-ohos"),
    Target("x86_64-unknown-netbsd"),
    #
    # Tier 2 without host tools
    Target("aarch64-apple-ios"),
    Target("aarch64-linux-android"),
    Target("aarch64-unknown-fuchsia", min_toolchain=Toolchain.STABLE),
    Target("arm-linux-androideabi"),
    Target("arm-unknown-linux-musleabi"),
    Target("arm-unknown-linux-musleabihf"),
    Target("armv5te-unknown-linux-gnueabi"),
    Target("armv5te-unknown-linux-musleabi"),
    Target("armv7-linux-androideabi"),
    Target("armv7-unknown-linux-musleabihf"),
    Target("i586-unknown-linux-gnu"),
    Target("i586-unknown-linux-musl"),
    Target("i686-linux-android"),
    Target("i686-unknown-freebsd"),
    Target("i686-unknown-linux-musl"),
    Target("sparc64-unknown-linux-gnu"),
    Target("wasm32-unknown-emscripten"),
    Target("wasm32-unknown-unknown"),
    Target("wasm32-wasip1", min_toolchain=Toolchain.STABLE),
    Target("wasm32-wasip2", min_toolchain=Toolchain.STABLE),
    Target("x86_64-fortanix-unknown-sgx"),
    Target("x86_64-linux-android"),
    Target("x86_64-unknown-fuchsia", min_toolchain=Toolchain.STABLE),
    Target("x86_64-unknown-linux-gnux32"),
    Target("x86_64-unknown-redox"),
    #
    # Libc has historically checked that a number of tier 3 targets build. Technically
    # there is no need to do this given the target tier policy, but the cost is small
    # and the saved churn from accidental breakage is significant, so we keep it around.
    Target("aarch64-unknown-freebsd", dist=False),
    Target("aarch64-unknown-hermit", dist=False),
    Target("aarch64-unknown-illumos", dist=False),
    Target("aarch64-unknown-netbsd", dist=False),
    Target("aarch64-unknown-openbsd", dist=False),
    Target("aarch64-wrs-vxworks", dist=False),
    Target("armebv7r-none-eabihf", dist=False),
    Target("armv7-wrs-vxworks-eabihf", dist=False),
    Target("armv7r-none-eabihf", dist=False),
    Target("armv7s-apple-ios", dist=False),
    Target("hexagon-unknown-linux-musl", dist=False),
    Target("i386-apple-ios", dist=False),
    Target("i686-apple-darwin", dist=False),
    Target("i686-unknown-haiku", dist=False),
    Target("i686-unknown-hurd-gnu", dist=False),
    Target("i686-unknown-netbsd", dist=False),
    Target("i686-unknown-openbsd", dist=False),
    Target("i686-wrs-vxworks", dist=False),
    Target("mips-unknown-linux-gnu", dist=False),
    Target("mips-unknown-linux-musl", dist=False),
    Target("mips64-unknown-linux-gnuabi64", dist=False),
    Target("mips64-unknown-linux-muslabi64", dist=False),
    Target("mips64el-unknown-linux-gnuabi64", dist=False),
    Target("mips64el-unknown-linux-muslabi64", dist=False),
    Target("mipsel-unknown-linux-gnu", dist=False),
    Target("mipsel-unknown-linux-musl", dist=False),
    Target("nvptx64-nvidia-cuda", dist=False),
    Target("powerpc-unknown-linux-gnuspe", dist=False),
    Target("powerpc-unknown-netbsd", dist=False),
    Target("powerpc-wrs-vxworks", dist=False),
    Target("powerpc-wrs-vxworks-spe", dist=False),
    Target("powerpc64-ibm-aix", dist=False),
    Target("powerpc64-unknown-freebsd", dist=False),
    Target("powerpc64-wrs-vxworks", dist=False),
    Target("riscv32-wrs-vxworks", dist=False),
    Target("riscv32gc-unknown-linux-gnu", dist=False),
    Target("riscv32i-unknown-none-elf", dist=False),
    Target("riscv32imac-unknown-none-elf", dist=False),
    Target("riscv32imc-unknown-none-elf", dist=False),
    Target("riscv64-wrs-vxworks", dist=False),
    Target("riscv64a23-unknown-linux-gnu", dist=False),
    Target("riscv64gc-unknown-freebsd", dist=False),
    Target("riscv64gc-unknown-hermit", dist=False),
    Target("riscv64gc-unknown-linux-musl", dist=False),
    Target("riscv64gc-unknown-none-elf", dist=False),
    Target("riscv64imac-unknown-none-elf", dist=False),
    Target("s390x-unknown-linux-musl", dist=False),
    Target("sparc-unknown-linux-gnu", dist=False),
    Target("sparc64-unknown-netbsd", dist=False),
    Target("thumbv7em-none-eabihf", dist=False),
    Target("thumbv7m-none-eabi", dist=False),
    Target("thumbv7neon-linux-androideabi", dist=False),
    Target("thumbv7neon-unknown-linux-gnueabihf", dist=False),
    Target("thumbv8m.main-none-eabi", dist=False),
    Target("x86_64-unknown-dragonfly", dist=False),
    Target("x86_64-unknown-haiku", dist=False),
    Target("x86_64-unknown-hermit", dist=False),
    Target("x86_64-unknown-l4re-uclibc", dist=False),
    Target("x86_64-unknown-openbsd", dist=False),
    Target("x86_64-wrs-vxworks", dist=False),
]


def eprint(*args, **kw):
    print(*args, file=sys.stderr, **kw)


def xtrace(args: list[str], /, env: Optional[dict[str, str]]):
    """Print commands before running them."""
    astr = " ".join(args)
    if env is None:
        eprint(f"+ {astr}")
    else:
        envdiff = set(env.items()) - set(os.environ.items())
        estr = " ".join(f"{k}='{v}'" for (k, v) in envdiff)
        eprint(f"+ {estr} {astr}")


def check_output(args: list[str], /, env: Optional[dict[str, str]] = None) -> str:
    xtrace(args, env=env)
    return sp.check_output(args, env=env, encoding="utf8")


def run(args: list[str], /, env: Optional[dict[str, str]] = None):
    xtrace(args, env=env)
    sp.run(args, env=env, check=True)


def check_dup_targets():
    all = set()
    duplicates = set()
    for target in TARGETS:
        if target.name in all:
            duplicates.add(target.name)
        all.add(target.name)
    assert len(duplicates) == 0, f"duplicate targets: {duplicates}"


def test_target(cfg: Cfg, target: Target):
    start = time.time()
    env = os.environ.copy()
    env.setdefault("RUSTFLAGS", "")

    tname = target.name
    target_cfg = check_output(["rustc", "--print=cfg", "--target", tname])
    target_env = re.findall(r'target_env="(.*)"', target_cfg)
    target_os = re.findall(r'target_os="(.*)"', target_cfg)
    target_bits = re.findall(r'target_pointer_width="(.*)"', target_cfg)[0]
    assert target_bits in ["32", "64"]
    eprint(f"env {target_env}, os {target_os}, bits {target_bits}")

    # Usually we do a full build to make sure we don't run into any crashes or link
    # problems. If we need to use build-std, though, only do a check to speed
    # things up.
    if target.dist:
        action = "build"
    else:
        action = "check"

    cmd = ["cargo", f"+{cfg.toolchain_name}", action, "--target", tname]

    if not target.dist:
        # If we can't download a `core`, we need to build it
        cmd += ["-Zbuild-std=core"]
        # FIXME: With `build-std` feature, `compiler_builtins` emits a lot of lint warnings.
        env["RUSTFLAGS"] += " -Aimproper_ctypes_definitions"
    else:
        run(["rustup", "target", "add", tname, "--toolchain", cfg.toolchain_name])

    # Test with expected combinations of features
    run(cmd, env=env)
    run(cmd + ["--features=extra_traits"], env=env)

    # Check with different env for 64-bit time_t
    if target_os == "linux" and target_bits == "32":
        # Equivalent of __USE_TIME_BITS64
        run(cmd, env=env | {"RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64": "1"})

    if "gnu" in target_env and target_bits == "32":
        # Equivalent of _FILE_OFFSET_BITS=64
        run(cmd, env=env | {"RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS": "64"})
        # Equivalent of _TIME_BITS=64
        run(cmd, env=env | {"RUST_LIBC_UNSTABLE_GNU_TIME_BITS": "64"})

    # Test again without default features, i.e. without `std`
    run(cmd + ["--no-default-features"])
    run(cmd + ["--no-default-features", "--features=extra_traits"])

    # Ensure the crate will build when used as a dependency of `std`
    if cfg.nightly():
        run(cmd + ["--no-default-features", "--features=rustc-dep-of-std"])

    # For freebsd targets, check with the different versions we support
    # if on nightly or stable
    if "freebsd" in tname and cfg.toolchain >= Toolchain.STABLE:
        for version in FREEBSD_VERSIONS:
            run(cmd, env=env | {"RUST_LIBC_UNSTABLE_FREEBSD_VERSION": str(version)})
            run(
                cmd + ["--no-default-features"],
                env=env | {"RUST_LIBC_UNSTABLE_FREEBSD_VERSION": str(version)},
            )

    is_stable = cfg.toolchain == Toolchain.STABLE
    # FIXME(semver): can't pass `--target` to `cargo-semver-checks` so we restrict to
    # the host target
    is_host = tname == cfg.host_target
    if is_stable and is_host:
        eprint("Running semver checks")
        run(
            [
                "cargo",
                "semver-checks",
                "--only-explicit-features",
                "--features=std,extra_traits",
            ]
        )
    else:
        eprint("Skipping semver checks")

    elapsed = round(time.time() - start, 2)
    eprint(f"Finished checking target {tname} in {elapsed} seconds")


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--toolchain", required=True, help="Rust toolchain")
    p.add_argument("--only", help="only targets matching this regex")
    p.add_argument("--skip", help="skip targets matching this regex")
    p.add_argument(
        "--half",
        type=int,
        choices=[1, 2],
        help="specify 1 or 2 to run half of the targets",
    )
    args = p.parse_args()

    cfg = Cfg(toolchain_name=args.toolchain)
    eprint(f"Config: {cfg}")
    eprint("Python version: ", sys.version)
    check_dup_targets()
    start = time.time()

    if cfg.nightly():
        # Needed for build-std
        run(["rustup", "component", "add", "rust-src"])

    targets = TARGETS
    eprint(f"Total checked targets across platforms: {len(targets)}")

    if not cfg.nightly():
        # Non-dist targets need nightly for build-std
        targets = [t for t in targets if t.dist]

    # Filter to targets supported on the current toolchain
    targets = [t for t in targets if cfg.toolchain >= t.min_toolchain]
    eprint(f"Targets checkable with this toolchain: {len(targets)}")

    # Apply filtering
    if args.only:
        targets = [t for t in targets if re.match(args.only, t.name)]
    if args.skip:
        targets = [t for t in targets if not re.match(args.skip, t.name)]

    # Allow splitting the targets in half for time improvements
    if args.half == 1:
        targets = targets[0::2]
    elif args.half == 2:
        targets = targets[1::2]

    total = len(targets)
    eprint(f"Targets to run: {total}")
    assert total > 0, "some tests should be run"

    for i, target in enumerate(targets):
        at = i + 1
        eprint(f"::group::Target: {target.name} ({at}/{total})")
        eprint(f"{ESC_CYAN}Checking target {target} ({at}/{total}){ESC_END}")
        test_target(cfg, target)
        eprint("::endgroup::")

    elapsed = round(time.time() - start, 2)
    eprint(f"Checked {total} targets in {elapsed} seconds")


main()
