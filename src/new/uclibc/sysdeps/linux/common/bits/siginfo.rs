//! Header source: `libc/sysdeps/linux/common/bits/siginfo.h`
//!
//! * Headers: <https://gogs.waldemar-brodkorb.de/oss/uclibc-ng/src/60d8e8c0cb9be8a241f6f2645daba260c8aec33c/libc/sysdeps/linux/common/bits/siginfo.h> (official)
//! * Headers: <https://github.com/wbx-github/uclibc-ng/blob/60d8e8c0cb9be8a241f6f2645daba260c8aec33c/libc/sysdeps/linux/common/bits/siginfo.h> (mirror)

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
        _sifields: __c_anonymous_siginfo_t__si_fields,
    }

    union __c_anonymous_siginfo_t__si_fields {
        _pad: Padding<[c_int; __SI_PAD_SIZE]>,
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
    }

    struct __c_anonymous__si_fields__sigpoll {
        si_band: c_long,
        si_fd: c_int,
    }

    struct __c_anonymous__si_fields__sigsys {
        _call_addr: *mut c_void,
        _syscall: c_int,
        _arch: c_uint,
    }
}

impl siginfo_t {
    #[inline]
    pub unsafe fn si_pid(&self) -> crate::pid_t {
        self._sifields._kill.si_pid
    }

    #[inline]
    pub unsafe fn si_uid(&self) -> crate::uid_t {
        self._sifields._kill.si_uid
    }

    #[inline]
    pub unsafe fn si_timerid(&self) -> c_int {
        self._sifields._timer.si_tid
    }

    #[inline]
    pub unsafe fn si_overrun(&self) -> c_int {
        self._sifields._timer.si_overrun
    }

    #[inline]
    pub unsafe fn si_status(&self) -> c_int {
        self._sifields._sigchld.si_status
    }

    #[inline]
    pub unsafe fn si_utime(&self) -> crate::clock_t {
        self._sifields._sigchld.si_utime
    }

    #[inline]
    pub unsafe fn si_stime(&self) -> crate::clock_t {
        self._sifields._sigchld.si_stime
    }

    #[inline]
    pub unsafe fn si_value(&self) -> crate::sigval {
        self._sifields._rt.si_sigval
    }

    #[inline]
    pub unsafe fn si_int(&self) -> c_int {
        self._sifields._rt.si_sigval.sival_int
    }

    #[inline]
    pub unsafe fn si_ptr(&self) -> *mut c_void {
        self._sifields._rt.si_sigval.sival_ptr
    }

    #[inline]
    pub unsafe fn si_addr(&self) -> *mut c_void {
        self._sifields._sigfault.si_addr
    }

    #[inline]
    pub unsafe fn si_band(&self) -> c_long {
        self._sifields._sigpoll.si_band
    }

    #[inline]
    pub unsafe fn si_fd(&self) -> c_int {
        self._sifields._sigpoll.si_fd
    }

    #[inline]
    pub unsafe fn si_call_addr(&self) -> *mut c_void {
        self._sifields._sigsys._call_addr
    }

    #[inline]
    pub unsafe fn si_syscall(&self) -> c_int {
        self._sifields._sigsys._syscall
    }

    #[inline]
    pub unsafe fn si_arch(&self) -> c_uint {
        self._sifields._sigsys._arch
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for siginfo_t {
            fn eq(&self, other: &Self) -> bool {
                (self.si_signo, self.si_code, self.si_errno)
                    == (other.si_signo, other.si_code, other.si_errno)
            }
        }

        impl Eq for siginfo_t {}

        impl core::hash::Hash for siginfo_t {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.si_signo.hash(state);
                self.si_code.hash(state);
                self.si_errno.hash(state);
            }
        }
    }
}
