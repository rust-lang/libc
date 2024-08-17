# Changelog

## [Unreleased]

## [0.2.157](https://github.com/rust-lang/libc/compare/0.2.156...0.2.157) - 2024-08-17

### Other
- Merge pull request [#3852](https://github.com/rust-lang/libc/pull/3852) from tgross35/backport-onion
- always use freebsd12 when rustc_dep_of_std is set
- freebsd adding execvpe support from 14.1 release
- Merge pull request [#3845](https://github.com/rust-lang/libc/pull/3845) from nathaniel-bennett/rustc-wrapper-fix-0.2
- Add RUSTC_WRAPPER support to build script
- Update CHANGELOG.md for 0.2.156
- Merge pull request [#3835](https://github.com/rust-lang/libc/pull/3835) from tgross35/revert-bsd-getnameinfo-0.2
- Merge pull request [#3833](https://github.com/rust-lang/libc/pull/3833) from tgross35/backport-potato
- add `pthread_equal`
- netbsd adding _lwp_park api.
- Merge pull request [#3829](https://github.com/rust-lang/libc/pull/3829) from rust-lang/tgross35-patch-2
- Merge pull request [#3828](https://github.com/rust-lang/libc/pull/3828) from rust-lang/tgross35-patch-1
- Update CHANGELOG.md

## [0.2.156](https://github.com/rust-lang/libc/compare/v0.2.155...v0.2.156) - 2024-08-15

### Added
- Apple: add `F_ALLOCATEPERSIST` in <https://github.com/rust-lang/libc/pull/3712>
- Apple: add `os_sync_wait_on_address` and related definitions in <https://github.com/rust-lang/libc/pull/3769>
- BSD: generalise `IPV6_DONTFRAG` to all BSD targets in <https://github.com/rust-lang/libc/pull/3716>
- FreeBSD/DragonFly: add `IP_RECVTTL`/`IPV6_RECVHOPLIMIT` in <https://github.com/rust-lang/libc/pull/3751>
- Hurd: add `XATTR_CREATE`, `XATTR_REPLACE` in <https://github.com/rust-lang/libc/pull/3739>
- Linux GNU: `confstr` API and `_CS_*` in <https://github.com/rust-lang/libc/pull/3771>
- Linux musl: add `preadv2` and `pwritev2` (1.2.5 min.) in <https://github.com/rust-lang/libc/pull/3762>
- VxWorks: add the constant `SOMAXCONN` in <https://github.com/rust-lang/libc/pull/3761>
- VxWorks: add a few errnoLib related constants in <https://github.com/rust-lang/libc/pull/3780>

### Fixed
- Solaris/illumos: Change `ifa_flags` type to u64 in <https://github.com/rust-lang/libc/pull/3729>
- QNX 7.0: Disable `libregex` in <https://github.com/rust-lang/libc/pull/3775>

### Changed
- QNX NTO: update platform support in <https://github.com/rust-lang/libc/pull/3815>
- `addr_of!(EXTERN_STATIC)` is now considered safe in <https://github.com/rust-lang/libc/pull/3776>

### Removed
- Apple: remove `rmx_state` in <https://github.com/rust-lang/libc/pull/3776>

### Other
- Update or remove CI tests that have been failing
