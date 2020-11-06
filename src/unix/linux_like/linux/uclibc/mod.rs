pub type sa_family_t = u16;
pub type pthread_key_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type tcflag_t = ::c_uint;
pub type loff_t = ::c_longlong;
pub type clockid_t = ::c_int;
pub type key_t = ::c_int;
pub type id_t = ::c_uint;
pub type useconds_t = u32;
pub type dev_t = u64;
pub type socklen_t = u32;
pub type mode_t = u32;
pub type ino64_t = u64;
pub type off64_t = i64;
pub type blkcnt64_t = i64;
pub type rlim64_t = u64;
pub type shmatt_t = ::c_ulong;
pub type mqd_t = ::c_int;
pub type msgqnum_t = ::c_ulong;
pub type msglen_t = ::c_ulong;
pub type nfds_t = ::c_ulong;
pub type nl_item = ::c_int;
pub type idtype_t = ::c_uint;
pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;
pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;

pub type regoff_t = ::c_int;

#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum fpos64_t {} // FIXME: fill this out with a struct
impl ::Copy for fpos64_t {}
impl ::Clone for fpos64_t {
    fn clone(&self) -> fpos64_t {
        *self
    }
}

#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum timezone {}
impl ::Copy for timezone {}
impl ::Clone for timezone {
    fn clone(&self) -> timezone {
        *self
    }
}

s! {
    

    

    

    

    

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,

        pub ai_addr: *mut ::sockaddr,

        pub ai_canonname: *mut c_char,

        pub ai_next: *mut addrinfo,
    }

    

    

    pub struct sched_param {
        pub sched_priority: ::c_int,
    }

    

    

    

    

    

    pub struct pthread_rwlockattr_t {
        __lockkind: ::c_int,
        __pshared: ::c_int,
    }

    

    

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        #[cfg(target_endian = "little")]
        pub f_fsid: ::c_ulong,
        #[cfg(target_pointer_width = "32")]
        __f_unused: ::c_int,
        #[cfg(target_endian = "big")]
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct dqblk {
        pub dqb_bhardlimit: u32,
        pub dqb_bsoftlimit: u32,
        pub dqb_curblocks: u32,
        pub dqb_ihardlimit: u32,
        pub dqb_isoftlimit: u32,
        pub dqb_curinodes: u32,
        pub dqb_btime: ::time_t,
        pub dqb_itime: ::time_t,
    }

    pub struct cpu_set_t {
        #[cfg(target_pointer_width = "32")]
        bits: [u32; 32],
        #[cfg(target_pointer_width = "64")]
        bits: [u64; 16],
    }
}

s_no_extra_traits! {
    pub struct mq_attr {
        pub mq_flags: ::c_long,
        pub mq_maxmsg: ::c_long,
        pub mq_msgsize: ::c_long,
        pub mq_curmsgs: ::c_long,
        pad: [::c_long; 4]
    }    
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for mq_attr {
            fn eq(&self, other: &mq_attr) -> bool {
                self.mq_flags == other.mq_flags &&
                self.mq_maxmsg == other.mq_maxmsg &&
                self.mq_msgsize == other.mq_msgsize &&
                self.mq_curmsgs == other.mq_curmsgs
            }
        }
        impl Eq for mq_attr {}
        impl ::fmt::Debug for mq_attr {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("mq_attr")
                    .field("mq_flags", &self.mq_flags)
                    .field("mq_maxmsg", &self.mq_maxmsg)
                    .field("mq_msgsize", &self.mq_msgsize)
                    .field("mq_curmsgs", &self.mq_curmsgs)
                    .finish()
            }
        }
        impl ::hash::Hash for mq_attr {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.mq_flags.hash(state);
                self.mq_maxmsg.hash(state);
                self.mq_msgsize.hash(state);
                self.mq_curmsgs.hash(state);
            }
        }

        impl PartialEq for sockaddr_nl {
            fn eq(&self, other: &sockaddr_nl) -> bool {
                self.nl_family == other.nl_family &&
                self.nl_pid == other.nl_pid &&
                self.nl_groups == other.nl_groups
            }
        }
        impl Eq for sockaddr_nl {}
        impl ::fmt::Debug for sockaddr_nl {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sockaddr_nl")
                    .field("nl_family", &self.nl_family)
                    .field("nl_pid", &self.nl_pid)
                    .field("nl_groups", &self.nl_groups)
                    .finish()
            }
        }
        impl ::hash::Hash for sockaddr_nl {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.nl_family.hash(state);
                self.nl_pid.hash(state);
                self.nl_groups.hash(state);
            }
        }

        impl PartialEq for sigevent {
            fn eq(&self, other: &sigevent) -> bool {
                self.sigev_value == other.sigev_value
                    && self.sigev_signo == other.sigev_signo
                    && self.sigev_notify == other.sigev_notify
                    && self.sigev_notify_thread_id
                        == other.sigev_notify_thread_id
            }
        }
        impl Eq for sigevent {}
        impl ::fmt::Debug for sigevent {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("sigevent")
                    .field("sigev_value", &self.sigev_value)
                    .field("sigev_signo", &self.sigev_signo)
                    .field("sigev_notify", &self.sigev_notify)
                    .field("sigev_notify_thread_id",
                           &self.sigev_notify_thread_id)
                    .finish()
            }
        }
        impl ::hash::Hash for sigevent {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.sigev_value.hash(state);
                self.sigev_signo.hash(state);
                self.sigev_notify.hash(state);
                self.sigev_notify_thread_id.hash(state);
            }
        }
    }
}

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

pub const RLIMIT_CPU: ::c_int = 0;
pub const RLIMIT_FSIZE: ::c_int = 1;
pub const RLIMIT_DATA: ::c_int = 2;
pub const RLIMIT_STACK: ::c_int = 3;
pub const RLIMIT_CORE: ::c_int = 4;
pub const RLIMIT_LOCKS: ::c_int = 10;
pub const RLIMIT_SIGPENDING: ::c_int = 11;
pub const RLIMIT_MSGQUEUE: ::c_int = 12;
pub const RLIMIT_NICE: ::c_int = 13;
pub const RLIMIT_RTPRIO: ::c_int = 14;

// These are different than GNU!
pub const LC_MONETARY: ::c_int = 2;
pub const LC_TIME: ::c_int = 3;
pub const LC_COLLATE: ::c_int = 4;
// end different section

pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

// MS_ flags for mount(2)
pub const MS_NOUSER: ::c_ulong = 0x80000000;
pub const MS_RMT_MASK: ::c_ulong = 0x800051;

pub const ENOTSUP: ::c_int = EOPNOTSUPP;

pub const IPV6_JOIN_GROUP: ::c_int = 20;
pub const IPV6_LEAVE_GROUP: ::c_int = 21;

// These actually are different from GNU
pub const EAI_NODATA: ::c_int = -5;
pub const ABDAY_1: ::nl_item = 0x300;
pub const ABDAY_2: ::nl_item = 0x301;
pub const ABDAY_3: ::nl_item = 0x302;
pub const ABDAY_4: ::nl_item = 0x303;
pub const ABDAY_5: ::nl_item = 0x304;
pub const ABDAY_6: ::nl_item = 0x305;
pub const ABDAY_7: ::nl_item = 0x306;
pub const DAY_1: ::nl_item = 0x307;
pub const DAY_2: ::nl_item = 0x308;
pub const DAY_3: ::nl_item = 0x309;
pub const DAY_4: ::nl_item = 0x30A;
pub const DAY_5: ::nl_item = 0x30B;
pub const DAY_6: ::nl_item = 0x30C;
pub const DAY_7: ::nl_item = 0x30D;
pub const ABMON_1: ::nl_item = 0x30E;
pub const ABMON_2: ::nl_item = 0x30F;
pub const ABMON_3: ::nl_item = 0x310;
pub const ABMON_4: ::nl_item = 0x311;
pub const ABMON_5: ::nl_item = 0x312;
pub const ABMON_6: ::nl_item = 0x313;
pub const ABMON_7: ::nl_item = 0x314;
pub const ABMON_8: ::nl_item = 0x315;
pub const ABMON_9: ::nl_item = 0x316;
pub const ABMON_10: ::nl_item = 0x317;
pub const ABMON_11: ::nl_item = 0x318;
pub const ABMON_12: ::nl_item = 0x319;
pub const MON_1: ::nl_item = 0x31A;
pub const MON_2: ::nl_item = 0x31B;
pub const MON_3: ::nl_item = 0x31C;
pub const MON_4: ::nl_item = 0x31D;
pub const MON_5: ::nl_item = 0x31E;
pub const MON_6: ::nl_item = 0x31F;
pub const MON_7: ::nl_item = 0x320;
pub const MON_8: ::nl_item = 0x321;
pub const MON_9: ::nl_item = 0x322;
pub const MON_10: ::nl_item = 0x323;
pub const MON_11: ::nl_item = 0x324;
pub const MON_12: ::nl_item = 0x325;
pub const AM_STR: ::nl_item = 0x326;
pub const PM_STR: ::nl_item = 0x327;
pub const D_T_FMT: ::nl_item = 0x328;
pub const D_FMT: ::nl_item = 0x329;
pub const T_FMT: ::nl_item = 0x32A;
pub const T_FMT_AMPM: ::nl_item = 0x32B;
pub const ERA: ::nl_item = 0x32C;
pub const ERA_D_FMT: ::nl_item = 0x32E;
pub const ALT_DIGITS: ::nl_item = 0x32F;
pub const ERA_D_T_FMT: ::nl_item = 0x330;
pub const ERA_T_FMT: ::nl_item = 0x331;
pub const CODESET: ::nl_item = 10;
pub const CRNCYSTR: ::nl_item = 0x215;
pub const RADIXCHAR: ::nl_item = 0x100;
pub const THOUSEP: ::nl_item = 0x101;
pub const NOEXPR: ::nl_item = 0x501;
pub const YESSTR: ::nl_item = 0x502;
pub const NOSTR: ::nl_item = 0x503;

// Different than Gnu.
pub const FILENAME_MAX: ::c_uint = 4095;

pub const PRIO_PROCESS: ::c_int = 0;
pub const PRIO_PGRP: ::c_int = 1;
pub const PRIO_USER: ::c_int = 2;

extern "C" {
    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::timezone) -> ::c_int;

    pub fn pthread_rwlockattr_getkind_np(
        attr: *const pthread_rwlockattr_t,
        val: *mut ::c_int,
    ) -> ::c_int;
    pub fn pthread_rwlockattr_setkind_np(
        attr: *mut pthread_rwlockattr_t,
        val: ::c_int,
    ) -> ::c_int;

    pub fn prlimit(
        pid: ::pid_t,
        resource: ::c_int,
        new_limit: *const ::rlimit,
        old_limit: *mut ::rlimit,
    ) -> ::c_int;
    pub fn prlimit64(
        pid: ::pid_t,
        resource: ::c_int,
        new_limit: *const ::rlimit64,
        old_limit: *mut ::rlimit64,
    ) -> ::c_int;
<<<<<<< HEAD
    pub fn reboot(how_to: ::c_int) -> ::c_int;
    pub fn setfsgid(gid: ::gid_t) -> ::c_int;
    pub fn setfsuid(uid: ::uid_t) -> ::c_int;
    pub fn setresgid(rgid: ::gid_t, egid: ::gid_t, sgid: ::gid_t) -> ::c_int;
    pub fn setresuid(ruid: ::uid_t, euid: ::uid_t, suid: ::uid_t) -> ::c_int;

    // Not available now on Android
    pub fn mkfifoat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::mode_t,
    ) -> ::c_int;
    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);
    pub fn freeifaddrs(ifa: *mut ::ifaddrs);

    pub fn mremap(
        addr: *mut ::c_void,
        len: ::size_t,
        new_len: ::size_t,
        flags: ::c_int,
        ...
    ) -> *mut ::c_void;

    pub fn glob(
        pattern: *const c_char,
        flags: ::c_int,
        errfunc: ::Option<
            extern "C" fn(epath: *const c_char, errno: ::c_int) -> ::c_int,
        >,
        pglob: *mut ::glob_t,
    ) -> ::c_int;
    pub fn globfree(pglob: *mut ::glob_t);

    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);

    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;

    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;
    pub fn madvise(
        addr: *mut ::c_void,
        len: ::size_t,
        advice: ::c_int,
    ) -> ::c_int;

    pub fn msync(
        addr: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
    ) -> ::c_int;

    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::ssize_t;
    pub fn nl_langinfo(item: ::nl_item) -> *mut ::c_char;

    pub fn bind(
        socket: ::c_int,
        address: *const ::sockaddr,
        address_len: ::socklen_t,
    ) -> ::c_int;

    pub fn writev(
        fd: ::c_int,
        iov: *const ::iovec,
        iovcnt: ::c_int,
    ) -> ::ssize_t;
    pub fn readv(
        fd: ::c_int,
        iov: *const ::iovec,
        iovcnt: ::c_int,
    ) -> ::ssize_t;

    pub fn sendmsg(
        fd: ::c_int,
        msg: *const ::msghdr,
        flags: ::c_int,
    ) -> ::ssize_t;
    pub fn recvmsg(
        fd: ::c_int,
        msg: *mut ::msghdr,
        flags: ::c_int,
    ) -> ::ssize_t;
    pub fn getgrgid_r(
        gid: ::gid_t,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> ::c_int;
    pub fn sem_close(sem: *mut sem_t) -> ::c_int;
    pub fn getdtablesize() -> ::c_int;
    pub fn getgrnam_r(
        name: *const ::c_char,
        grp: *mut ::group,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut ::group,
    ) -> ::c_int;
    pub fn pthread_sigmask(
        how: ::c_int,
        set: *const sigset_t,
        oldset: *mut sigset_t,
    ) -> ::c_int;
    pub fn sem_open(name: *const ::c_char, oflag: ::c_int, ...) -> *mut sem_t;
    pub fn getgrnam(name: *const ::c_char) -> *mut ::group;
    pub fn pthread_kill(thread: ::pthread_t, sig: ::c_int) -> ::c_int;
    pub fn sem_unlink(name: *const ::c_char) -> ::c_int;
    pub fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int;
    pub fn getpwnam_r(
        name: *const ::c_char,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn sigwait(set: *const sigset_t, sig: *mut ::c_int) -> ::c_int;
    pub fn pthread_atfork(
        prepare: ::Option<unsafe extern "C" fn()>,
        parent: ::Option<unsafe extern "C" fn()>,
        child: ::Option<unsafe extern "C" fn()>,
    ) -> ::c_int;
    pub fn pthread_create(
        native: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;
    pub fn dl_iterate_phdr(
        callback: ::Option<
            unsafe extern "C" fn(
                info: *mut ::dl_phdr_info,
                size: ::size_t,
                data: *mut ::c_void,
            ) -> ::c_int,
        >,
        data: *mut ::c_void,
    ) -> ::c_int;
    pub fn getgrgid(gid: ::gid_t) -> *mut ::group;
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut ::FILE;
    pub fn uname(buf: *mut ::utsname) -> ::c_int;
    pub fn getnameinfo(
        sa: *const ::sockaddr,
        salen: ::socklen_t,
        host: *mut ::c_char,
        hostlen: ::socklen_t,
        serv: *mut ::c_char,
        sevlen: ::socklen_t,
        flags: ::c_int,
    ) -> ::c_int;
=======
>>>>>>> 9735c92c7... mostly mechanical removal of redundant symbols.
}

cfg_if! {
    if #[cfg(any(target_arch = "mips", target_arch = "mips64"))] {
        mod mips;
        pub use self::mips::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(target_arch = "arm")] {
        mod arm;
        pub use self::arm::*;
    } else {
        pub use unsupported_target;
    }
}

cfg_if! {
    if #[cfg(libc_align)] {
        #[macro_use]
        mod align;
    } else {
        #[macro_use]
        mod no_align;
    }
}

expand_align!();
