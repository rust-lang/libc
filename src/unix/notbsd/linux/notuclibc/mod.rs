pub const ABDAY_1: ::nl_item = 0x20000;
pub const ABDAY_2: ::nl_item = 0x20001;
pub const ABDAY_3: ::nl_item = 0x20002;
pub const ABDAY_4: ::nl_item = 0x20003;
pub const ABDAY_5: ::nl_item = 0x20004;
pub const ABDAY_6: ::nl_item = 0x20005;
pub const ABDAY_7: ::nl_item = 0x20006;

pub const DAY_1: ::nl_item = 0x20007;
pub const DAY_2: ::nl_item = 0x20008;
pub const DAY_3: ::nl_item = 0x20009;
pub const DAY_4: ::nl_item = 0x2000A;
pub const DAY_5: ::nl_item = 0x2000B;
pub const DAY_6: ::nl_item = 0x2000C;
pub const DAY_7: ::nl_item = 0x2000D;

pub const ABMON_1: ::nl_item = 0x2000E;
pub const ABMON_2: ::nl_item = 0x2000F;
pub const ABMON_3: ::nl_item = 0x20010;
pub const ABMON_4: ::nl_item = 0x20011;
pub const ABMON_5: ::nl_item = 0x20012;
pub const ABMON_6: ::nl_item = 0x20013;
pub const ABMON_7: ::nl_item = 0x20014;
pub const ABMON_8: ::nl_item = 0x20015;
pub const ABMON_9: ::nl_item = 0x20016;
pub const ABMON_10: ::nl_item = 0x20017;
pub const ABMON_11: ::nl_item = 0x20018;
pub const ABMON_12: ::nl_item = 0x20019;

pub const MON_1: ::nl_item = 0x2001A;
pub const MON_2: ::nl_item = 0x2001B;
pub const MON_3: ::nl_item = 0x2001C;
pub const MON_4: ::nl_item = 0x2001D;
pub const MON_5: ::nl_item = 0x2001E;
pub const MON_6: ::nl_item = 0x2001F;
pub const MON_7: ::nl_item = 0x20020;
pub const MON_8: ::nl_item = 0x20021;
pub const MON_9: ::nl_item = 0x20022;
pub const MON_10: ::nl_item = 0x20023;
pub const MON_11: ::nl_item = 0x20024;
pub const MON_12: ::nl_item = 0x20025;

pub const AM_STR: ::nl_item = 0x20026;
pub const PM_STR: ::nl_item = 0x20027;

pub const D_T_FMT: ::nl_item = 0x20028;
pub const D_FMT: ::nl_item = 0x20029;
pub const T_FMT: ::nl_item = 0x2002A;
pub const T_FMT_AMPM: ::nl_item = 0x2002B;

pub const ERA: ::nl_item = 0x2002C;
pub const ERA_D_FMT: ::nl_item = 0x2002E;
pub const ALT_DIGITS: ::nl_item = 0x2002F;
pub const ERA_D_T_FMT: ::nl_item = 0x20030;
pub const ERA_T_FMT: ::nl_item = 0x20031;

pub const CODESET: ::nl_item = 14;

pub const CRNCYSTR: ::nl_item = 0x4000F;

pub const RADIXCHAR: ::nl_item = 0x10000;
pub const THOUSEP: ::nl_item = 0x10001;

pub const YESEXPR: ::nl_item = 0x50000;
pub const NOEXPR: ::nl_item = 0x50001;
pub const YESSTR: ::nl_item = 0x50002;
pub const NOSTR: ::nl_item = 0x50003;

pub const FILENAME_MAX: ::c_uint = 4096;

pub const MSG_COPY: ::c_int = 0o40000;

pub const AF_MAX: ::c_int = 42;
pub const AF_IB: ::c_int = 27;
pub const AF_MPLS: ::c_int = 28;
pub const AF_NFC: ::c_int = 39;
pub const AF_VSOCK: ::c_int = 40;
#[doc(hidden)]
pub const PF_IB: ::c_int = AF_IB;
pub const PF_MPLS: ::c_int = AF_MPLS;
pub const PF_NFC: ::c_int = AF_NFC;
pub const PF_VSOCK: ::c_int = AF_VSOCK;
#[doc(hidden)]
pub const PF_MAX: ::c_int = AF_MAX;

pub const EPOLLEXCLUSIVE: ::c_int = 0x10000000;

pub const AIO_CANCELED: ::c_int = 0;
pub const AIO_NOTCANCELED: ::c_int = 1;
pub const AIO_ALLDONE: ::c_int = 2;
pub const LIO_READ: ::c_int = 0;
pub const LIO_WRITE: ::c_int = 1;
pub const LIO_NOP: ::c_int = 2;
pub const LIO_WAIT: ::c_int = 0;
pub const LIO_NOWAIT: ::c_int = 1;

pub const PR_MPX_ENABLE_MANAGEMENT: ::c_int = 43;
pub const PR_MPX_DISABLE_MANAGEMENT: ::c_int = 44;

pub const PR_FP_MODE_FR: ::c_int = 1 << 0;
pub const PR_FP_MODE_FRE: ::c_int = 1 << 1;

pub const PR_CAP_AMBIENT: ::c_int = 47;
pub const PR_CAP_AMBIENT_IS_SET: ::c_int = 1;
pub const PR_CAP_AMBIENT_RAISE: ::c_int = 2;
pub const PR_CAP_AMBIENT_LOWER: ::c_int = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: ::c_int = 4;

pub const SHM_EXEC: ::c_int = 0o100000;

pub const RB_SW_SUSPEND: ::c_int = 0xd000fce2u32 as i32;
pub const RB_KEXEC: ::c_int = 0x45584543u32 as i32;

use super::aiocb;
use super::termios;
extern {
    pub fn aio_read(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_write(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_fsync(op: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_error(aiocbp: *const aiocb) -> ::c_int;
    pub fn aio_return(aiocbp: *mut aiocb) -> ::ssize_t;
    pub fn aio_suspend(aiocb_list: *const *const aiocb, nitems: ::c_int,
                       timeout: *const ::timespec) -> ::c_int;
    pub fn aio_cancel(fd: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn lio_listio(mode: ::c_int, aiocb_list: *const *mut aiocb,
                      nitems: ::c_int, sevp: *mut ::sigevent) -> ::c_int;

    pub fn fallocate(fd: ::c_int, mode: ::c_int,
                     offset: ::off_t, len: ::off_t) -> ::c_int;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn pwritev(fd: ::c_int,
                   iov: *const ::iovec,
                   iovcnt: ::c_int,
                   offset: ::off_t) -> ::ssize_t;
    pub fn preadv(fd: ::c_int,
                  iov: *const ::iovec,
                  iovcnt: ::c_int,
                  offset: ::off_t) -> ::ssize_t;

    pub fn dup3(oldfd: ::c_int, newfd: ::c_int, flags: ::c_int) -> ::c_int;

    pub fn futimes(fd: ::c_int, times: *const ::timeval) -> ::c_int;
    pub fn mkstemps(template: *mut ::c_char, suffixlen: ::c_int) -> ::c_int;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char,
                     suffixlen: ::c_int,
                     flags: ::c_int) -> ::c_int;

    pub fn openpty(amaster: *mut ::c_int,
                   aslave: *mut ::c_int,
                   name: *mut ::c_char,
                   termp: *const termios,
                   winp: *const ::winsize) -> ::c_int;
    pub fn forkpty(amaster: *mut ::c_int,
                   name: *mut ::c_char,
                   termp: *const termios,
                   winp: *const ::winsize) -> ::pid_t;

    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int)
                         -> ::c_int;

    pub fn getloadavg(loadavg: *mut ::c_double, nelem: ::c_int) -> ::c_int;
    pub fn process_vm_readv(pid: ::pid_t,
                            local_iov: *const ::iovec,
                            liovcnt: ::c_ulong,
                            remote_iov: *const ::iovec,
                            riovcnt: ::c_ulong,
                            flags: ::c_ulong) -> isize;
    pub fn process_vm_writev(pid: ::pid_t,
                             local_iov: *const ::iovec,
                             liovcnt: ::c_ulong,
                             remote_iov: *const ::iovec,
                             riovcnt: ::c_ulong,
                             flags: ::c_ulong) -> isize;

}
