//! File `Kernel/API/POSIX/signal.h`
//!
//! <https://github.com/SerenityOS/serenity/blob/master/Kernel/API/POSIX/signal.h>
use crate::prelude::*;

pub type sigset_t = u32;

s! {
    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_code: c_int,
        pub si_errno: c_int,
        pub si_pid: crate::pid_t,
        pub si_uid: crate::uid_t,
        pub si_addr: *mut c_void,
        pub si_status: c_int,
        pub si_band: c_long,
        pub si_value: crate::sigval,
    }

    pub struct sigaction {
        pub sa_sigaction: crate::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_flags: c_int,
    }

    pub struct stack_t {
        pub ss_sp: *mut c_void,
        pub ss_size: size_t,
        pub ss_flags: c_int,
    }
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut c_void {
        self.si_addr
    }

    pub unsafe fn si_value(&self) -> crate::sigval {
        self.si_value
    }

    pub unsafe fn si_pid(&self) -> crate::pid_t {
        self.si_pid
    }

    pub unsafe fn si_uid(&self) -> crate::uid_t {
        self.si_uid
    }

    pub unsafe fn si_status(&self) -> c_int {
        self.si_status
    }
}

pub const SS_ONSTACK: c_int = 1;
pub const SS_DISABLE: c_int = 2;

pub const MINSIGSTKSZ: size_t = 4096;
pub const SIGSTKSZ: size_t = 32768;

pub const SA_NOCLDSTOP: c_int = 1;
pub const SA_NOCLDWAIT: c_int = 2;
pub const SA_SIGINFO: c_int = 4;
pub const SA_ONSTACK: c_int = 0x08000000;
pub const SA_RESTART: c_int = 0x10000000;
pub const SA_NODEFER: c_int = 0x40000000;
pub const SA_RESETHAND: c_int = 0x80000000u32.cast_signed();

pub const SA_NOMASK: c_int = SA_NODEFER;
pub const SA_ONESHOT: c_int = SA_RESETHAND;

pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 1;
pub const SIG_SETMASK: c_int = 2;

pub const CLD_EXITED: c_int = 0;
pub const CLD_KILLED: c_int = 1;
pub const CLD_DUMPED: c_int = 2;
pub const CLD_TRAPPED: c_int = 3;
pub const CLD_STOPPED: c_int = 4;
pub const CLD_CONTINUED: c_int = 5;

pub const FPE_INTDIV: c_int = 0;
pub const FPE_INTOVF: c_int = 1;
pub const FPE_FLTDIV: c_int = 2;
pub const FPE_FLTOVF: c_int = 3;
pub const FPE_FLTUND: c_int = 4;
pub const FPE_FLTRES: c_int = 5;
pub const FPE_FLTINV: c_int = 6;
pub const FPE_FLTSUB: c_int = 7;
