//! Source directory: `sysdeps/unix/sysv/linux/bits/types`
//!
//! <https://github.com/sailfishos-mirror/glibc/tree/master/sysdeps/unix/sysv/linux/bits/types>

#[cfg_attr(
    any(target_arch = "mips", target_arch = "mips64"),
    path = "../../mips/bits/siginfo_arch.rs"
)]
#[cfg_attr(
    any(target_arch = "sparc", target_arch = "sparc64"),
    path = "../../sparc/bits/siginfo_arch.rs"
)]
#[cfg_attr(target_arch = "x86", path = "../../x86/bits/siginfo_arch.rs")]
mod siginfo_t;
