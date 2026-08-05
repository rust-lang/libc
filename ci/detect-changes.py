#!/usr/bin/env python3
"""Emit the test matrices for the CI workflow as GitHub Actions output.

Each tier is printed on its own line as `tierN=<json>` so the workflow can
feed it straight into a `matrix: include` block. Merge queues, schedules
and manual runs always get every target; there is no file detection yet.
"""

import argparse
import json
import sys
from dataclasses import dataclass, field
from enum import IntEnum


class Tier(IntEnum):
    """Roughly ordered by how much we care about the target staying green."""

    T1 = 1
    T2 = 2
    T3 = 3  # tier 2 that only runs inside a VM


@dataclass(frozen=True)
class TestTarget:
    """One row of the test matrix.

    The fields map straight to matrix variables in ci.yaml; a missing `os`
    means the default ubuntu-26.04 runner.
    """

    name: str  # rust target triple
    #: runner OS, fall back to ubuntu-26.04 when unset
    os: str | None = None
    tier: Tier = Tier.T1
    vm: bool = False
    release: str | None = None  # OS version for the VM jobs
    env: dict[str, str | int] = field(default_factory=dict)
    artifact_tag: str | None = None


# the full list of matrix rows, grouped by tier for `tier_rows()`
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
        env={"TEST_MUSL_V1_2_3": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("arm-linux-androideabi", tier=Tier.T2),
    TestTarget("arm-unknown-linux-gnueabihf", tier=Tier.T2),
    TestTarget("arm-unknown-linux-musleabihf", tier=Tier.T2),
    TestTarget(
        "arm-unknown-linux-musleabihf",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2_3": 1},
        artifact_tag="new-musl",
    ),
    # FIXME(#4297): spurious test failures, keep disabled
    # TestTarget("i686-linux-android", tier=Tier.T2),
    TestTarget("i686-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "i686-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2_3": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("loongarch64-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("loongarch64-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "loongarch64-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2_3": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("powerpc64-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("powerpc64-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "powerpc64-unknown-linux-musl",
        tier=Tier.T2,
        env={"RUST_LIBC_UNSTABLE_MUSL_V1_2_3": 1},
        artifact_tag="new-musl",
    ),
    TestTarget("powerpc64le-unknown-linux-gnu", tier=Tier.T2),
    TestTarget("powerpc64le-unknown-linux-musl", tier=Tier.T2),
    TestTarget(
        "powerpc64le-unknown-linux-musl",
        tier=Tier.T2,
        env={"TEST_MUSL_V1_2_3": 1},
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
        env={"TEST_MUSL_V1_2_3": 1},
        artifact_tag="new-musl",
    ),
    # FIXME: some items in `src/unix/mod.rs` aren't defined on redox yet
    # TestTarget("x86_64-unknown-redox", tier=Tier.T2),
    # FIXME(ppc): SIGILL running tests, see rust-lang/libc#4254
    # TestTarget("powerpc-unknown-linux-gnu", tier=Tier.T2),
    # tier 2, VM only
    TestTarget("i686-unknown-freebsd", tier=Tier.T3, vm=True, release="15.0"),
    TestTarget("x86_64-unknown-freebsd", tier=Tier.T3, vm=True, release="14.4"),
    TestTarget("x86_64-unknown-freebsd", tier=Tier.T3, vm=True, release="15.0"),
    TestTarget("x86_64-pc-solaris", tier=Tier.T3, vm=True),
    TestTarget("x86_64-unknown-netbsd", tier=Tier.T3, vm=True),
    TestTarget("x86_64-unknown-illumos", tier=Tier.T3, vm=True),
]

#: tier value -> output variable name
TIER_OUTPUT_NAMES = {
    Tier.T1: "tier1",
    Tier.T2: "tier2",
    Tier.T3: "tier2_vm",
}


def to_matrix_row(target: TestTarget) -> dict[str, str | int]:
    """Convert a target into the dict a matrix `include` row expects.

    None fields are dropped so the JSON stays identical to the old
    hardcoded `include:` blocks and the workflow's `matrix.os` fallbacks
    keep working.
    """
    row: dict[str, str | int] = {"target": target.name}
    if target.os is not None:
        row["os"] = target.os
    if target.env:
        row["env"] = dict(target.env)
    if target.artifact_tag is not None:
        row["artifact-tag"] = target.artifact_tag
    if target.release is not None:
        row["release"] = target.release
    return row


def tier_rows() -> dict[str, list[dict[str, str | int]]]:
    """All rows, grouped by tier so the workflow gets one JSON per job."""
    out: dict[str, list[dict[str, str | int]]] = {
        name: [] for name in TIER_OUTPUT_NAMES.values()
    }
    for target in TARGETS:
        out[TIER_OUTPUT_NAMES[target.tier]].append(to_matrix_row(target))
    return out


def sanity() -> None:
    """Fail loudly if the matrices drift from the full current set."""
    counts = {name: len(rows) for name, rows in tier_rows().items()}
    assert counts == {"tier1": 9, "tier2": 28, "tier2_vm": 6}, counts
    for target in TARGETS:
        assert target.vm == (target.tier == Tier.T3)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--sanity", action="store_true", help="check the matrix is complete"
    )
    args = parser.parse_args()
    if args.sanity:
        sanity()
        sys.exit(0)
    for name, rows in tier_rows().items():
        print(f"{name}={json.dumps(rows)}")


if __name__ == "__main__":
    main()
