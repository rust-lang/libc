//! Directory source: `libc/sysdeps/linux/common/bits`
//!
//! * Headers: <https://gogs.waldemar-brodkorb.de/oss/uclibc-ng/src/master/libc/sysdeps/linux/common/bits> (official)
//! * Headers: <https://github.com/wbx-github/uclibc-ng/tree/master/libc/sysdeps/linux/common/bits> (mirror)

#[cfg_attr(
    any(target_arch = "mips", target_arch = "mips64"),
    path = "../../mips/bits/siginfo.rs"
)]
pub(crate) mod siginfo;
