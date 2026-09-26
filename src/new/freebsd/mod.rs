//! FreeBSD libc.
//!
//! * Headers: <https://github.com/freebsd/freebsd-src/blob/main/sys/riscv/include/ucontext.h>
//! * Symbol map: <https://github.com/freebsd/freebsd-src/blob/main/lib/libc/gen/Symbol.map>

pub(crate) mod net;
pub(crate) mod netinet6;

// [NOTE]: this module identifier uses an additional underscore because that it's
// more convenient to reexport in `new`'s top-level module file. The `netlink`
// interfaces need some special casing to avoid collisions with the `if_mib`
// interfaces, so we handle their reexports differently.
pub(crate) mod netlink_;

pub(crate) mod sys;
pub(crate) mod unistd;
