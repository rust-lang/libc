#!/usr/bin/env python3
"""Utilities for CI.

Generate the test matrices for the CI workflow as GitHub Actions output.
Each tier is printed on its own line as `test_<tier>_matrix=<json>` so the
workflow can feed it straight into a `matrix: include` block. Merge queues,
schedules and manual runs always get every target; there is no file
detection yet.
"""

import json
import sys
from dataclasses import dataclass, field
from enum import IntEnum, StrEnum


class Tier(IntEnum):
    """Rust tier of the target, as defined in
    <https://doc.rust-lang.org/rustc/platform-support.html>.
    """

    T1 = 1
    T2 = 2


class CiJob(StrEnum):
    """Which CI job the target is tested by."""

    T1 = "tier1"
    T2 = "tier2"
    T2_VM = "tier2_vm"


@dataclass(frozen=True)
class TestTarget:
    """One row of the test matrix.

    The fields map straight to matrix variables in ci.yaml; `os` defaults to
    the ubuntu-26.04 runner.
    """

    name: str  # rust target triple
    #: runner OS
    os: str = "ubuntu-26.04"
    tier: Tier = Tier.T1
    vm: bool = False
    release: str | None = None  # OS version for the VM jobs
    env: dict[str, str | int] = field(default_factory=dict)
    artifact_tag: str | None = None

    def ci_job(self) -> CiJob:
        """The CI job this target runs in, based on its tier and VM-ness."""
        if self.tier is Tier.T1:
            return CiJob.T1
        return CiJob.T2_VM if self.vm else CiJob.T2


# the full list of matrix rows, grouped by tier for readability
TARGETS: list[TestTarget] = [
    # tier 1
    TestTarget("aarch64-apple-darwin", os="macos-26"),
    TestTarget("aarch64-pc-windows-msvc", os="windows-11-arm"),
    TestTarget("aarch64-unknown-linux-gnu", os="ubuntu-26.04-arm"),
    TestTarget("i686-pc-windows-gnu", os="windows-2025"),
    TestTarget("i686-pc-windows-msvc", os="windows-2025"),
    TestTarget("i686-unknown-linux-gnu"),
    TestTarget("x86_64-pc-windows-gnu", os="windows-2025"),
    TestTarget("x86_64-pc-windows-msvc", os="windows-2025"),
    TestTarget("x86_64-unknown-linux-gnu"),
    # tier 2
    TestTarget("aarch64-linux-android", tier=Tier.T2),
    TestTarget("aarch64-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "aarch64-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("arm-linux-androideabi", tier=Tier.T2),
    TestTarget("arm-unknown-linux-gnueabihf", tier=Tier.T2),
    TestTarget("arm-unknown-linux-musleabihf", tier=Tier.T2),
    TestTarget(
        "arm-unknown-linux-musleabihf",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    # FIXME(#4297): spurious test failures, keep disabled
    # TestTarget("i686-linux-android", tier=Tier.T2),
    TestTarget("i686-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "i686-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("loongarch64-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("loongarch64-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "loongarch64-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("powerpc64-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("powerpc64-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "powerpc64-unknown-linux-musl",
        tier=Tier.T2,
        env={"RUST_LIBC_UNSTABLE_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("powerpc64le-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("powerpc64le-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "powerpc64le-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("riscv64gc-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("s390x-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("sparc64-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("wasm32-unknown-emscripten", tier=Tier.T2),
    TestTarget("wasm32-wasip1", tier=Tier.T2),
    TestTarget("wasm32-wasip2", tier=Tier.T2),
    TestTarget("x86_64-apple-darwin", os="macos-26-intel", tier=Tier.T2),
    # keep in sync with the android build pinned in ci/cuttlefish-setup.sh
    TestTarget("x86_64-linux-android", tier=Tier.T2, artifact_tag="android17"),
    # FIXME: fails to run, exec format error (os error 8)
    # TestTarget("x86_64-unknown-linux-gnux32", tier=Tier.T2),
    TestTarget("x86_64-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "x86_64-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2": 1},
        artifact_tag="new-musl",
    ),
    # FIXME: some items in `src/unix/mod.rs` aren't defined on redox yet
    # TestTarget("x86_64-unknown-redox", tier=Tier.T2),
    # FIXME(ppc): SIGILL running tests, see rust-lang/libc#4254
    # TestTarget("powerpc-unknown-linux-gnu", tier=Tier.T2),
    # tier 2, VM only
    TestTarget("i686-unknown-freebsd", tier=Tier.T2, vm=True, release="15.0"),
    TestTarget("x86_64-unknown-freebsd", tier=Tier.T2, vm=True, release="14.4"),
    TestTarget("x86_64-unknown-freebsd", tier=Tier.T2, vm=True, release="15.0"),
    TestTarget("x86_64-pc-solaris", tier=Tier.T2, vm=True),
    TestTarget("x86_64-unknown-netbsd", tier=Tier.T2, vm=True),
    TestTarget("x86_64-unknown-illumos", tier=Tier.T2, vm=True),
]


def emit_workflow_output() -> None:
    """Print the test matrices, one `test_<job>_matrix=<json>` line per job."""
    rows = {job: [] for job in CiJob}
    for target in TARGETS:
        row: dict[str, str | int] = {
            "target": target.name,
            "os": target.os,
            "env": dict(target.env) or None,
            "artifact-tag": target.artifact_tag,
            "release": target.release,
        }
        rows[target.ci_job()].append({k: v for k, v in row.items() if v is not None})
    for job, targets in rows.items():
        print(f"test_{job.value}_matrix={json.dumps(targets)}")


def main() -> None:
    match sys.argv[1:]:
        case ["generate-matrix"]:
            emit_workflow_output()
        case ["--help" | "-h"] | []:
            print(
                """usage: ci/ci-util.py <COMMAND>

COMMAND:
    generate-matrix
        Print the test matrix for each CI job as `test_<job>_matrix=<json>`."""
            )
        case _:
            print(f"error: unknown command {sys.argv[1:]}", file=sys.stderr)
            sys.exit(1)


if __name__ == "__main__":
    main()
