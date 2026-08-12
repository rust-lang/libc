# Contributing to `libc`

Welcome! If you are reading this document, it means you are interested in
contributing to the `libc` crate.

## v1.0 Roadmap

`libc` has two active branches: `main` and `libc-0.2`. `main` is for active
development of the upcoming v1.0 release, and should be the target of all pull
requests. `libc-0.2` is for updates to the currently released version.

If a pull request to `main` is a good candidate for inclusion in an `0.2.x`
release, include `@rustbot label stable-nominated` in a comment to propose this.
Almost everything should be included in stable, unless it involves breaking
changes.

Once a `stable-nominated` PR targeting `main` has merged, it can be cherry
picked to the `libc-0.2` branch. A maintainer will likely do these cherry picks
in a batch before a release, so there is no need for any action as a PR author.

When cherry picking, run the following after branching from  `libc-0.2`:
`git cherry-pick -xe commit-sha-on-main` (or `git cherry-pick -xe
start-sha^..end-sha` if a range of commits is needed). `git` will automatically
add the "cherry picked from commit" note, but try to add a backport note so the
original PR gets crosslinked:

```
# ... original commit message ...

(backport <https://github.com/rust-lang/libc/pull/1234>)             # add manually
(cherry picked from commit 104b6a4ae31c726814c36318dc718470cc96e167) # added by git
```

Cherry pick PRs can then target `libc-0.2`.

See the [tracking issue](https://github.com/rust-lang/libc/issues/3248) for
details.

## Adding an API

Want to use an API which currently isn't bound in `libc`? It's quite easy to add
one!

If you are adding API for a header that doesn't already have some bindings,
create a new file in the appropriate location in `src/new`. These modules should
approximately match source trees: try to follow the patterns that are there but
don't hesitate to ask maintainers if guidance is needed.

If some bindings for the relevant header has already been added outside of
`src/new`, it is fine to extend what already exists. Everything outside of
`src/new` uses a hierarchial structure: a single path from root to a leaf node
represents a single platform, and adding API at a given level will make it
available on all children.

We are currently in a reorganization process, moving from the hierarchial
structure to the source-mapped structure in `src/new`.

New items (i.e. functions, constants etc.) should also be added to the symbol
list(s) found in the `libc-test/semver` directory. These lists  track of what
API is public in the libc crate and ensures they remain available between
changes to the crate. If the new symbol(s) are available on all supported Unixes
it should be added to `unix.txt` list[^1], otherwise they should be added to the
OS-specific list(s).

With that in mind, the steps for adding a new API are:

1. Determine where to add your API.
2. Add the API, including adding new symbol(s) to the semver lists.
3. Send a PR to this repo.
4. Wait for CI to pass, fixing errors.
5. Wait for a merge!

[^1]: Note that this list has nothing to do with any Unix or Posix standard,
      it's just a list shared among all OSs that declare `#[cfg(unix)]`.

## Test before you commit

There are a few relevant tests in `libc`:

1. `libc-test` is the main test suite. This can be run locally with `cargo test
   --workspace`.

   If needed, the `skip_*()` functions in `libc-test/build.rs` can be used to
   skip testing specific API.
2. Style checker and formatter, located at [`./ci/style.py`][style-py]. Running
   this also invokes the code formatter. (This also formats code within macros,
   which `cargo fmt` won't do.)
3. `ci/verify-build.py`, which checks the build on a wide range of targets.
   Typically there is no need to run this locally.

[style-py]: https://github.com/rust-lang/libc/blob/main/ci/style.py

## Submitting a Pull Request

When you submit a PR, please follow these guidelines to give your PR the best
chance of being accepted:

1. All new API should be added to `libc-test/semver`, which makes sure it
   doesn't get removed in the future.
2. All changes *must* have permalink to headers in commit messages. See
   [Source links](#source-links) below for more details.
3. For any constants that are expected to change, e.g. `*LAST` or `*MAX`, try to
   add the following doc comment:

   ```rust
   /// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
   /// for details.
   ```
5. Run tests locally, following the directions at
   [Test before you commit](test-before-you-commit). This is especially relevant
   for platforms that aren't tested on CI.

### Source links

Please include permalinks to headers in commit messages for all API changes;
the maintainers need to compare changes against these sources, and we also want
to keep a record of exactly what the API looked like when bindings were written.
Common sources include:

* Apple XNU: <https://github.com/apple-oss-distributions/xnu>,
  libc <https://github.com/apple-oss-distributions/Libc/tree/main/include>
* Android: <https://cs.android.com/android/platform/superproject/main>
* Cygwin: <https://github.com/cygwin/cygwin> (official mirror),
  <https://cygwin.com/cgit/newlib-cygwin/tree> (original)
* DragonFlyBSD: <https://github.com/DragonFlyBSD/DragonFlyBSD> (official mirror),
  <https://gitweb.dragonflybsd.org/?p=dragonfly.git;a=tree> (original)
* Emscripten: <https://github.com/emscripten-core/emscripten>
* FreeBSD: <https://github.com/freebsd/freebsd-src> (official mirror),
  <https://cgit.freebsd.org/src/tree> (original)
* Fuchsia: <https://cs.opensource.google/fuchsia/fuchsia/+/main:zircon/>
* Glibc: <https://github.com/sailfishos-mirror/glibc> (unofficial mirror),
  <https://sourceware.org/git/?p=glibc.git;a=tree> (original)
* Illumos: <https://github.com/illumos/illumos-gate> (official mirror),
  <https://code.illumos.org/plugins/gitiles/illumos-gate/+/refs/heads/master> (original)
* Linux uapi: <https://github.com/torvalds/linux/tree/master/include/uapi> (official mirror),
  <https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/> (original)
* Musl: <https://github.com/kraj/musl> (unofficial mirror),
  <https://git.musl-libc.org/cgit/musl/tree> (original)
* NetBSD: <https://github.com/NetBSD> (official mirror),
  <https://cvsweb.netbsd.org/bsdweb.cgi> (original)
* OpenBSD: <https://github.com/openbsd> (official mirror),
  <https://cvsweb.openbsd.org/src> (original)
* RedoxOS: <https://gitlab.redox-os.org/redox-os/relibc>
* Windows GNU: <https://github.com/mingw-w64/mingw-w64> (unofficial mirror),
  <https://sourceforge.net/p/mingw-w64/mingw-w64/ci/master/tree/> (original)
* Windows MSVC: <https://github.com/microsoft/win32metadata>

After navigating to the relevant file and selecting relevant lines, get a
permalink:

* On GitHub, click the triple dots and selecting "copy permalink".
* On the Android and Fuchsia source viewers, type l-r (links->commit).
* Cgit is trickier. Locate the latest commit by clicking "summary", clicking
  the commit message tagged with "master" (or other name as applicable), and
  copying the sha from that commit. Next, insert `?id=abcd1234...` in the URL,
  (before line number indicators like `#123`) and copy the result. This should
  look something like
  <https://git.musl-libc.org/cgit/musl/tree/include/sys/stat.h?id=f21a96538f78fa8e2040831b4209b35f2fb581da#n80>.
* For CVSweb, navigate to the file of interest and click on the "annotate" link
  for the desired revision. Example:
  <https://cvsweb.netbsd.org/bsdweb.cgi/src/sys/sys/stat.h?annotate=1.70>.

If sources are not public, link to documentation or paste relevant C
declarations.

(Including this information in the PR description is fine too, commit messages
are preferred because they become part of history.)

### Manual pages

Including links to manpages is not required but can also be very helpful to
include. Some platforms also publish manpages but not sources.

Common web manuals:

* AIX: <https://www.ibm.com/docs/en/aix/7.3.0>
* Apple: <https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man5/manpages.5.html>
  is the only known official site but it is severly outdated. <https://manp.gs/mac/>
  or <https://ss64.com/mac/> are better options.
* DragonFlyBSD: <https://www.dragonflybsd.org/cgi/web-man>
* FreeBSD: <https://man.freebsd.org/cgi/man.cgi>
* Illumos: <https://illumos.org/man/>
* NetBSD: <https://man.netbsd.org/>
* OpenBSD: <https://man.openbsd.org/>
* Solaris: <https://docs.oracle.com/cd/E88353_01/>
* Windows MSVC: <https://learn.microsoft.com/en-us/cpp/c-runtime-library/c-run-time-library-reference?view=msvc-180>

## Breaking change policy

See `src/lib.rs` for details.

## Supported target policy

When Rust removes a support for a target, the libc crate also may remove the
support at any time. MSRV may be different for different targets.

## Releasing change to crates.io

This repository uses [release-plz] to handle releases. Once your pull request
has been merged, a maintainer needs to create the changelog and merge a PR
bumping the version. This will automatically publish to crates.io!

[release-plz]: https://github.com/MarcoIeni/release-plz
