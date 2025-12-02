#!/usr/bin/env python3
"""Create a tarball of intermediate output for inspection if tests fail.

This is useful for seeing what exactly `ctest` is running.
"""

import os
import subprocess as sp
import sys

from datetime import datetime, timezone
from glob import glob
from pathlib import Path


def main():
    # Find the most recently touched file named "ctest_output.c" in the target
    # directory. This will be libc-tests's `OUT_DIR`
    marker_files = [Path(p) for p in glob("target/**/ctest_output.c", recursive=True)]
    marker_files.sort(key=lambda path: path.stat().st_mtime)
    build_dir = marker_files[0].parent
    print(f"Located build directory '{build_dir}'")

    # Collect all relevant Rust and C files
    add_files = glob("**/*.rs", recursive=True, root_dir=build_dir)
    add_files += glob("**/*.c", recursive=True, root_dir=build_dir)
    file_list = "\n".join(add_files).encode()

    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H%MZ")
    archive_name = f"archive-{now}"
    archive_path = f"{archive_name}.tar.gz"

    sp.run(
        ["tar", "czvf", archive_path, "-C", build_dir, "-T-"],
        input=file_list,
        check=True,
    )

    # If we are in GHA, set these env vars for future use
    gh_env = os.getenv("GITHUB_ENV")
    if gh_env is not None:
        print("Updating CI environment")
        with open(gh_env, "w+") as f:
            f.write(f"ARCHIVE_NAME={archive_name}\n")
            f.write(f"ARCHIVE_PATH={archive_path}\n")


if __name__ == "__main__":
    print("Starting script...")  # For debugging CI failures
    print("version: ", sys.version)
    # FIXME(ci): If the windows-2025 images update to a minimum python above 3.9,
    # this can be removed.
    #
    # Python <= 3.9 does not support the very helpful `root_dir` argument,
    # and that is the version used by the Windows GHA images. Rather than
    # using setup-python or doing something in the CI script, just find
    # the newer version and relaunch if this happens to be run with an old
    # version.
    try:
        glob("", root_dir="")
    except TypeError as e:
        if os.environ.get("CI") is None:
            raise e  # Just fail if we're not in CI

        # Find the next 3.1x Python version
        dirs = sorted(Path(r"C:\hostedtoolcache\windows\Python").iterdir())
        usepy = next(x for x in dirs if r"\3.1" in str(x))
        py = usepy.joinpath(r"x64\python.exe")
        args = [py, __file__, *sys.argv[1:]]
        print(f"relaunching with {args}")
        sp.run(args, check=True)
        exit()

    main()
else:
    print("not invoked as main, exiting")  # For debugging CI failures
