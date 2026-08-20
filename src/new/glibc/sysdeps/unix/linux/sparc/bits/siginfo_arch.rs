//! Header source: `sysdeps/unix/sysv/linux/sparc/bits/siginfo-arch.h`
//!
//! <https://github.com/sailfishos-mirror/glibc/blob/4a07bb292f921c10e71fbf48c4a7f44391feb06c/sysdeps/unix/sysv/linux/sparc/bits/siginfo-arch.h>

use crate::prelude::*;

const __SI_MAX_SIZE: usize = 128;
const __SI_PAD_SIZE: usize = if cfg!(target_pointer_width = "64") {
    __SI_MAX_SIZE / size_of::<c_int>() - 4
} else {
    __SI_MAX_SIZE / size_of::<c_int>() - 3
};

s_no_extra_traits! {
    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_errno: c_int,
        pub si_code: c_int,
        #[cfg(target_pointer_width = "64")]
        __pad0: Padding<c_int>,
        _sifields: __c_anonymous_siginfo_t__si_fields,
    }

    union __c_anonymous_siginfo_t__si_fields {
        _pad: [c_int; __SI_PAD_SIZE],
        _kill: __c_anonymous__si_fields__kill,
        _timer: __c_anonymous__si_fields__timer,
        _rt: __c_anonymous__si_fields__rt,
        _sigchld: __c_anonymous__si_fields__sigchld,
        _sigfault: __c_anonymous__si_fields__sigfault,
        _sigpoll: __c_anonymous__si_fields__sigpoll,
        _sigsys: __c_anonymous__si_fields__sigsys,
    }

    struct __c_anonymous__si_fields__kill {
        si_pid: crate::pid_t,
        si_uid: crate::uid_t,
    }

    struct __c_anonymous__si_fields__timer {
        si_tid: c_int,
        si_overrun: c_int,
        si_sigval: crate::sigval,
    }

    struct __c_anonymous__si_fields__rt {
        si_pid: crate::pid_t,
        si_uid: crate::uid_t,
        si_sigval: crate::sigval,
    }

    struct __c_anonymous__si_fields__sigchld {
        si_pid: crate::pid_t,
        si_uid: crate::uid_t,
        si_status: c_int,
        si_utime: crate::clock_t,
        si_stime: crate::clock_t,
    }

    struct __c_anonymous__si_fields__sigfault {
        si_addr: *mut c_void,
        _si_trapno: c_int,
        si_addr_lsb: c_short,
        _bounds: __c_anonymous__sigfault__bounds,
    }

    struct __c_anonymous__si_fields__sigpoll {
        #[cfg(target_pointer_width = "64")]
        si_band: c_int,
        #[cfg(not(target_pointer_width = "64"))]
        si_band: c_long,
        si_fd: c_int,
    }

    struct __c_anonymous__si_fields__sigsys {
        _call_addr: *mut c_void,
        _syscall: c_int,
        _arch: c_uint,
    }

    union __c_anonymous__sigfault__bounds {
        _addr_bnd: __c_anonymous__bounds__addr_bnd,
        _pkey: u32,
    }

    struct __c_anonymous__bounds__addr_bnd {
        _lower: *mut c_void,
        _upper: *mut c_void,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for siginfo_t {
            fn eq(&self, other: &Self) -> bool {
                (self.si_signo, self.si_errno, self.si_code)
                    == (other.si_signo, other.si_errno, other.si_code)
            }
        }

        impl Eq for siginfo_t {}

        impl core::hash::Hash for siginfo_t {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.si_signo.hash(state);
                self.si_errno.hash(state);
                self.si_code.hash(state);
            }
        }
    }
}

impl siginfo_t {
    #[inline]
    pub unsafe fn si_pid(&self) -> crate::pid_t {
        unsafe { self._sifields._kill.si_pid }
    }

    #[inline]
    pub unsafe fn si_uid(&self) -> crate::uid_t {
        unsafe { self._sifields._kill.si_uid }
    }

    #[inline]
    pub unsafe fn si_timerid(&self) -> c_int {
        unsafe { self._sifields._timer.si_tid }
    }

    #[inline]
    pub unsafe fn si_overrun(&self) -> c_int {
        unsafe { self._sifields._timer.si_overrun }
    }

    #[inline]
    pub unsafe fn si_status(&self) -> c_int {
        unsafe { self._sifields._sigchld.si_status }
    }

    #[inline]
    pub unsafe fn si_utime(&self) -> crate::clock_t {
        unsafe { self._sifields._sigchld.si_utime }
    }

    #[inline]
    pub unsafe fn si_stime(&self) -> crate::clock_t {
        unsafe { self._sifields._sigchld.si_stime }
    }

    #[inline]
    pub unsafe fn si_value(&self) -> crate::sigval {
        unsafe { self._sifields._rt.si_sigval }
    }

    #[inline]
    pub unsafe fn si_int(&self) -> c_int {
        unsafe { self._sifields._rt.si_sigval.sival_int }
    }

    #[inline]
    pub unsafe fn si_ptr(&self) -> *mut c_void {
        unsafe { self._sifields._rt.si_sigval.sival_ptr }
    }

    #[inline]
    pub unsafe fn si_addr(&self) -> *mut c_void {
        unsafe { self._sifields._sigfault.si_addr }
    }

    #[inline]
    pub unsafe fn si_trapno(&self) -> c_int {
        unsafe { self._sifields._sigfault._si_trapno }
    }

    #[inline]
    pub unsafe fn si_addr_lsb(&self) -> c_short {
        unsafe { self._sifields._sigfault.si_addr_lsb }
    }

    #[inline]
    pub unsafe fn si_lower(&self) -> *mut c_void {
        unsafe { self._sifields._sigfault._bounds._addr_bnd._lower }
    }

    #[inline]
    pub unsafe fn si_upper(&self) -> *mut c_void {
        unsafe { self._sifields._sigfault._bounds._addr_bnd._upper }
    }

    #[inline]
    pub unsafe fn si_pkey(&self) -> u32 {
        unsafe { self._sifields._sigfault._bounds._pkey }
    }

    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub unsafe fn si_band(&self) -> c_int {
        unsafe { self._sifields._sigpoll.si_band }
    }

    #[cfg(not(target_pointer_width = "64"))]
    #[inline]
    pub unsafe fn si_band(&self) -> c_long {
        unsafe { self._sifields._sigpoll.si_band }
    }

    #[inline]
    pub unsafe fn si_fd(&self) -> c_int {
        unsafe { self._sifields._sigpoll.si_fd }
    }

    #[inline]
    pub unsafe fn si_call_addr(&self) -> *mut c_void {
        unsafe { self._sifields._sigsys._call_addr }
    }

    #[inline]
    pub unsafe fn si_syscall(&self) -> c_int {
        unsafe { self._sifields._sigsys._syscall }
    }

    #[inline]
    pub unsafe fn si_arch(&self) -> c_uint {
        unsafe { self._sifields._sigsys._arch }
    }
}
