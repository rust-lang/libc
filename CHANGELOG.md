# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.156](https://github.com/rust-lang/libc/compare/0.2.155...0.2.156) - 2024-08-15

### Added
- F_ALLOCATEPERSIST for apple
- IP_RECVTTL/IPV6_RECVHOPLIMIT for FreeBSD/DragonFly

### Fixed
- Remove macOS 11 usage on CI
- Allow dead_code lint

### Other
- Enable publishing new versions via release-plz
- Add `release-plz` for the 0.2 branch
- Merge pull request [#3821](https://github.com/rust-lang/libc/pull/3821) from tgross35/backport-celery
- Modify QNX NTO platform support
- Adding constant SOMAXCONN to vxworks
- Add XATTR_CREATE, XATTR_REPLACE
- Change ifa_flags type to u64 for Solaris/illumos
- add confstr API and _CS_* to linux-gnu
- Merge pull request [#3814](https://github.com/rust-lang/libc/pull/3814) from tgross35/backport-hermit
- Merge pull request [#3813](https://github.com/rust-lang/libc/pull/3813) from tgross35/backport-futex-constants
- Merge pull request [#3812](https://github.com/rust-lang/libc/pull/3812) from tgross35/backport-fbsd-kinfo_file
- Merge pull request [#3811](https://github.com/rust-lang/libc/pull/3811) from tgross35/backport-riscv64-clone_args
- Merge pull request [#3810](https://github.com/rust-lang/libc/pull/3810) from tgross35/backport-haiku-b_app_image_symbol
- Merge pull request [#3807](https://github.com/rust-lang/libc/pull/3807) from sunshowers/illumos-pthread-pick
- vxWorks adding few errnoLib related constants.
- Disable hexagon-unknown-linux-musl testing for now
- Fix FreeBSD 15 CI
- Remove tier 3 targets from CI
- Tweak comment
- Disable `libregex` for QNX 7.0
- Reference specific MSRV in comment
- `addr_of!(EXTERN_STATIC)` is now considered safe
- `rmx_state` has been removed
- skip API that requires a newer macOS SDK in tests
- add `os_sync_wait_on_address` and related definitions
- adding preadv2/pwritev2 to linux musl (1.2.5 min.)
- Merge pull request [#3716](https://github.com/rust-lang/libc/pull/3716) from devnexen/ipv6_dontfrag
- generalising IPV6_DONTFRAG to all bsd.
- Merge pull request [#3715](https://github.com/rust-lang/libc/pull/3715) from tesuji/ci-verbosity
- Merge pull request [#3700](https://github.com/rust-lang/libc/pull/3700) from asomers/freebsd-capsicum-libc0.2
- Merge pull request [#3738](https://github.com/rust-lang/libc/pull/3738) from operutka/getauxval_uclibc
- Merge pull request [#3737](https://github.com/rust-lang/libc/pull/3737) from operutka/si_pid_uclibc
- Merge pull request [#3747](https://github.com/rust-lang/libc/pull/3747) from djkoloski/libc-0.2
- Merge pull request [#3719](https://github.com/rust-lang/libc/pull/3719) from tesuji/ci-cirrus-cache
- Merge pull request [#3750](https://github.com/rust-lang/libc/pull/3750) from SteveLauC/feat/IP_RECVTTL_FreeBSD
