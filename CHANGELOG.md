# Changelog

## [Unreleased]

## [0.2.156](https://github.com/rust-lang/libc/compare/v0.2.155...v0.2.156) - 2024-08-15

### Added
- Apple: add `F_ALLOCATEPERSIST`
- Apple: add `os_sync_wait_on_address` and related definitions
- BSD: generalise `IPV6_DONTFRAG` to all BSD targets
- FreeBSD/DragonFly: add `IP_RECVTTL`/`IPV6_RECVHOPLIMIT`
- Hurd: add `XATTR_CREATE`, `XATTR_REPLACE`
- Linux GNU: `confstr` API and `_CS_*`
- Linux musl: add `preadv2` and `pwritev2` (1.2.5 min.)
- VxWorks: add the constant `SOMAXCONN`
- VxWorks: add a few errnoLib related constants

### Fixed
- Solaris/illumos: Change `ifa_flags` type to u64 for 
- QNX 7.0: Disable `libregex`

### Changed
- QNX NTO: update platform support
- `addr_of!(EXTERN_STATIC)` is now considered safe

### Removed
- Apple: remove `rmx_state`

### Other
- Update or remove CI tests that have been failing
