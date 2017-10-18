pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type clock_t = i32;
pub type time_t = i32;
pub type suseconds_t = i32;
pub type wchar_t = i32;
pub type off_t = i32;
pub type ino_t = u32;
pub type blkcnt_t = i32;
pub type blksize_t = i32;
pub type nlink_t = u32;
pub type __u64 = ::c_ulonglong;

s! {
    pub struct aiocb {
        pub aio_fildes: ::c_int,
        pub aio_lio_opcode: ::c_int,
        pub aio_reqprio: ::c_int,
        pub aio_buf: *mut ::c_void,
        pub aio_nbytes: ::size_t,
        pub aio_sigevent: ::sigevent,
        __next_prio: *mut aiocb,
        __abs_prio: ::c_int,
        __policy: ::c_int,
        __error_code: ::c_int,
        __return_value: ::ssize_t,
        pub aio_offset: off_t,
        __unused1: [::c_char; 4],
        __glibc_reserved: [::c_char; 32]
    }

    pub struct stat {
        pub st_dev: ::c_ulong,
        st_pad1: [::c_long; 3],
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::c_ulong,
        pub st_pad2: [::c_long; 2],
        pub st_size: ::off_t,
        st_pad3: ::c_long,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        st_pad5: [::c_long; 14],
    }

    pub struct stat64 {
        pub st_dev: ::c_ulong,
        st_pad1: [::c_long; 3],
        pub st_ino: ::ino64_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::c_ulong,
        st_pad2: [::c_long; 2],
        pub st_size: ::off64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        st_pad3: ::c_long,
        pub st_blocks: ::blkcnt64_t,
        st_pad5: [::c_long; 14],
    }

    pub struct statfs64 {
        pub f_type: ::c_long,
        pub f_bsize: ::c_long,
        pub f_frsize: ::c_long,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_bavail: u64,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::c_long,
        pub f_flags: ::c_long,
        pub f_spare: [::c_long; 5],
    }

    pub struct statvfs64 {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_favail: u64,
        #[cfg(target_endian = "little")]
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        #[cfg(target_endian = "big")]
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct pthread_attr_t {
        __size: [u32; 9]
    }

    pub struct sigaction {
        pub sa_flags: ::c_int,
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: sigset_t,
        pub sa_restorer: ::dox::Option<extern fn()>,
        _resv: [::c_int; 1],
    }

    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_size: ::size_t,
        pub ss_flags: ::c_int,
    }

    pub struct sigset_t {
        __val: [::c_ulong; 32],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        pub _pad: [::c_int; 29],
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_uint,
        pub __seq: ::c_ushort,
        __pad1: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        #[cfg(target_endian = "big")]
        __glibc_reserved1: ::c_ulong,
        pub msg_stime: ::time_t,
        #[cfg(target_endian = "little")]
        __glibc_reserved1: ::c_ulong,
        #[cfg(target_endian = "big")]
        __glibc_reserved2: ::c_ulong,
        pub msg_rtime: ::time_t,
        #[cfg(target_endian = "little")]
        __glibc_reserved2: ::c_ulong,
        #[cfg(target_endian = "big")]
        __glibc_reserved3: ::c_ulong,
        pub msg_ctime: ::time_t,
        #[cfg(target_endian = "little")]
        __glibc_reserved3: ::c_ulong,
        __msg_cbytes: ::c_ulong,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __glibc_reserved4: ::c_ulong,
        __glibc_reserved5: ::c_ulong,
    }

    pub struct statfs {
        pub f_type: ::c_long,
        pub f_bsize: ::c_long,
        pub f_frsize: ::c_long,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_files: ::fsblkcnt_t,
        pub f_ffree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_fsid: ::fsid_t,

        pub f_namelen: ::c_long,
        f_spare: [::c_long; 6],
    }

    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::size_t,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::size_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::cc_t,
        pub c_cc: [::cc_t; ::NCCS],
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_sysid: ::c_long,
        pub l_pid: ::pid_t,
        pad: [::c_long; 4],
    }

    pub struct sysinfo {
        pub uptime: ::c_long,
        pub loads: [::c_ulong; 3],
        pub totalram: ::c_ulong,
        pub freeram: ::c_ulong,
        pub sharedram: ::c_ulong,
        pub bufferram: ::c_ulong,
        pub totalswap: ::c_ulong,
        pub freeswap: ::c_ulong,
        pub procs: ::c_ushort,
        pub pad: ::c_ushort,
        pub totalhigh: ::c_ulong,
        pub freehigh: ::c_ulong,
        pub mem_unit: ::c_uint,
        pub _f: [::c_char; 8],
    }
}

pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;

pub const O_LARGEFILE: ::c_int = 0x2000;

pub const RLIM_INFINITY: ::rlim_t = 0x7fffffff;

pub const SYS_syscall: ::c_ulong = 4000 + 0;
pub const SYS_exit: ::c_ulong = 4000 + 1;
pub const SYS_fork: ::c_ulong = 4000 + 2;
pub const SYS_read: ::c_ulong = 4000 + 3;
pub const SYS_write: ::c_ulong = 4000 + 4;
pub const SYS_open: ::c_ulong = 4000 + 5;
pub const SYS_close: ::c_ulong = 4000 + 6;
pub const SYS_waitpid: ::c_ulong = 4000 + 7;
pub const SYS_creat: ::c_ulong = 4000 + 8;
pub const SYS_link: ::c_ulong = 4000 + 9;
pub const SYS_unlink: ::c_ulong = 4000 +  10;
pub const SYS_execve: ::c_ulong = 4000 +  11;
pub const SYS_chdir: ::c_ulong = 4000 +  12;
pub const SYS_time: ::c_ulong = 4000 +  13;
pub const SYS_mknod: ::c_ulong = 4000 +  14;
pub const SYS_chmod: ::c_ulong = 4000 +  15;
pub const SYS_lchown: ::c_ulong = 4000 +  16;
pub const SYS_break: ::c_ulong = 4000 +  17;
pub const SYS_unused18: ::c_ulong = 4000 +  18;
pub const SYS_lseek: ::c_ulong = 4000 +  19;
pub const SYS_getpid: ::c_ulong = 4000 +  20;
pub const SYS_mount: ::c_ulong = 4000 +  21;
pub const SYS_umount: ::c_ulong = 4000 +  22;
pub const SYS_setuid: ::c_ulong = 4000 +  23;
pub const SYS_getuid: ::c_ulong = 4000 +  24;
pub const SYS_stime: ::c_ulong = 4000 +  25;
pub const SYS_ptrace: ::c_ulong = 4000 +  26;
pub const SYS_alarm: ::c_ulong = 4000 +  27;
pub const SYS_unused28: ::c_ulong = 4000 +  28;
pub const SYS_pause: ::c_ulong = 4000 +  29;
pub const SYS_utime: ::c_ulong = 4000 +  30;
pub const SYS_stty: ::c_ulong = 4000 +  31;
pub const SYS_gtty: ::c_ulong = 4000 +  32;
pub const SYS_access: ::c_ulong = 4000 +  33;
pub const SYS_nice: ::c_ulong = 4000 +  34;
pub const SYS_ftime: ::c_ulong = 4000 +  35;
pub const SYS_sync: ::c_ulong = 4000 +  36;
pub const SYS_kill: ::c_ulong = 4000 +  37;
pub const SYS_rename: ::c_ulong = 4000 +  38;
pub const SYS_mkdir: ::c_ulong = 4000 +  39;
pub const SYS_rmdir: ::c_ulong = 4000 +  40;
pub const SYS_dup: ::c_ulong = 4000 +  41;
pub const SYS_pipe: ::c_ulong = 4000 +  42;
pub const SYS_times: ::c_ulong = 4000 +  43;
pub const SYS_prof: ::c_ulong = 4000 +  44;
pub const SYS_brk: ::c_ulong = 4000 +  45;
pub const SYS_setgid: ::c_ulong = 4000 +  46;
pub const SYS_getgid: ::c_ulong = 4000 +  47;
pub const SYS_signal: ::c_ulong = 4000 +  48;
pub const SYS_geteuid: ::c_ulong = 4000 +  49;
pub const SYS_getegid: ::c_ulong = 4000 +  50;
pub const SYS_acct: ::c_ulong = 4000 +  51;
pub const SYS_umount2: ::c_ulong = 4000 +  52;
pub const SYS_lock: ::c_ulong = 4000 +  53;
pub const SYS_ioctl: ::c_ulong = 4000 +  54;
pub const SYS_fcntl: ::c_ulong = 4000 +  55;
pub const SYS_mpx: ::c_ulong = 4000 +  56;
pub const SYS_setpgid: ::c_ulong = 4000 +  57;
pub const SYS_ulimit: ::c_ulong = 4000 +  58;
pub const SYS_unused59: ::c_ulong = 4000 +  59;
pub const SYS_umask: ::c_ulong = 4000 +  60;
pub const SYS_chroot: ::c_ulong = 4000 +  61;
pub const SYS_ustat: ::c_ulong = 4000 +  62;
pub const SYS_dup2: ::c_ulong = 4000 +  63;
pub const SYS_getppid: ::c_ulong = 4000 +  64;
pub const SYS_getpgrp: ::c_ulong = 4000 +  65;
pub const SYS_setsid: ::c_ulong = 4000 +  66;
pub const SYS_sigaction: ::c_ulong = 4000 +  67;
pub const SYS_sgetmask: ::c_ulong = 4000 +  68;
pub const SYS_ssetmask: ::c_ulong = 4000 +  69;
pub const SYS_setreuid: ::c_ulong = 4000 +  70;
pub const SYS_setregid: ::c_ulong = 4000 +  71;
pub const SYS_sigsuspend: ::c_ulong = 4000 +  72;
pub const SYS_sigpending: ::c_ulong = 4000 +  73;
pub const SYS_sethostname: ::c_ulong = 4000 +  74;
pub const SYS_setrlimit: ::c_ulong = 4000 +  75;
pub const SYS_getrlimit: ::c_ulong = 4000 +  76;
pub const SYS_getrusage: ::c_ulong = 4000 +  77;
pub const SYS_gettimeofday: ::c_ulong = 4000 +  78;
pub const SYS_settimeofday: ::c_ulong = 4000 +  79;
pub const SYS_getgroups: ::c_ulong = 4000 +  80;
pub const SYS_setgroups: ::c_ulong = 4000 +  81;
pub const SYS_reserved82: ::c_ulong = 4000 +  82;
pub const SYS_symlink: ::c_ulong = 4000 +  83;
pub const SYS_unused84: ::c_ulong = 4000 +  84;
pub const SYS_readlink: ::c_ulong = 4000 +  85;
pub const SYS_uselib: ::c_ulong = 4000 +  86;
pub const SYS_swapon: ::c_ulong = 4000 +  87;
pub const SYS_reboot: ::c_ulong = 4000 +  88;
pub const SYS_readdir: ::c_ulong = 4000 +  89;
pub const SYS_mmap: ::c_ulong = 4000 +  90;
pub const SYS_munmap: ::c_ulong = 4000 +  91;
pub const SYS_truncate: ::c_ulong = 4000 +  92;
pub const SYS_ftruncate: ::c_ulong = 4000 +  93;
pub const SYS_fchmod: ::c_ulong = 4000 +  94;
pub const SYS_fchown: ::c_ulong = 4000 +  95;
pub const SYS_getpriority: ::c_ulong = 4000 +  96;
pub const SYS_setpriority: ::c_ulong = 4000 +  97;
pub const SYS_profil: ::c_ulong = 4000 +  98;
pub const SYS_statfs: ::c_ulong = 4000 +  99;
pub const SYS_fstatfs: ::c_ulong = 4000 + 100;
pub const SYS_ioperm: ::c_ulong = 4000 + 101;
pub const SYS_socketcall: ::c_ulong = 4000 + 102;
pub const SYS_syslog: ::c_ulong = 4000 + 103;
pub const SYS_setitimer: ::c_ulong = 4000 + 104;
pub const SYS_getitimer: ::c_ulong = 4000 + 105;
pub const SYS_stat: ::c_ulong = 4000 + 106;
pub const SYS_lstat: ::c_ulong = 4000 + 107;
pub const SYS_fstat: ::c_ulong = 4000 + 108;
pub const SYS_unused109: ::c_ulong = 4000 + 109;
pub const SYS_iopl: ::c_ulong = 4000 + 110;
pub const SYS_vhangup: ::c_ulong = 4000 + 111;
pub const SYS_idle: ::c_ulong = 4000 + 112;
pub const SYS_vm86: ::c_ulong = 4000 + 113;
pub const SYS_wait4: ::c_ulong = 4000 + 114;
pub const SYS_swapoff: ::c_ulong = 4000 + 115;
pub const SYS_sysinfo: ::c_ulong = 4000 + 116;
pub const SYS_ipc: ::c_ulong = 4000 + 117;
pub const SYS_fsync: ::c_ulong = 4000 + 118;
pub const SYS_sigreturn: ::c_ulong = 4000 + 119;
pub const SYS_clone: ::c_ulong = 4000 + 120;
pub const SYS_setdomainname: ::c_ulong = 4000 + 121;
pub const SYS_uname: ::c_ulong = 4000 + 122;
pub const SYS_modify_ldt: ::c_ulong = 4000 + 123;
pub const SYS_adjtimex: ::c_ulong = 4000 + 124;
pub const SYS_mprotect: ::c_ulong = 4000 + 125;
pub const SYS_sigprocmask: ::c_ulong = 4000 + 126;
pub const SYS_create_module: ::c_ulong = 4000 + 127;
pub const SYS_init_module: ::c_ulong = 4000 + 128;
pub const SYS_delete_module: ::c_ulong = 4000 + 129;
pub const SYS_get_kernel_syms: ::c_ulong = 4000 + 130;
pub const SYS_quotactl: ::c_ulong = 4000 + 131;
pub const SYS_getpgid: ::c_ulong = 4000 + 132;
pub const SYS_fchdir: ::c_ulong = 4000 + 133;
pub const SYS_bdflush: ::c_ulong = 4000 + 134;
pub const SYS_sysfs: ::c_ulong = 4000 + 135;
pub const SYS_personality: ::c_ulong = 4000 + 136;
pub const SYS_afs_syscall: ::c_ulong = 4000 + 137;
pub const SYS_setfsuid: ::c_ulong = 4000 + 138;
pub const SYS_setfsgid: ::c_ulong = 4000 + 139;
pub const SYS__llseek: ::c_ulong = 4000 + 140;
pub const SYS_getdents: ::c_ulong = 4000 + 141;
pub const SYS__newselect: ::c_ulong = 4000 + 142;
pub const SYS_flock: ::c_ulong = 4000 + 143;
pub const SYS_msync: ::c_ulong = 4000 + 144;
pub const SYS_readv: ::c_ulong = 4000 + 145;
pub const SYS_writev: ::c_ulong = 4000 + 146;
pub const SYS_cacheflush: ::c_ulong = 4000 + 147;
pub const SYS_cachectl: ::c_ulong = 4000 + 148;
pub const SYS_sysmips: ::c_ulong = 4000 + 149;
pub const SYS_unused150: ::c_ulong = 4000 + 150;
pub const SYS_getsid: ::c_ulong = 4000 + 151;
pub const SYS_fdatasync: ::c_ulong = 4000 + 152;
pub const SYS__sysctl: ::c_ulong = 4000 + 153;
pub const SYS_mlock: ::c_ulong = 4000 + 154;
pub const SYS_munlock: ::c_ulong = 4000 + 155;
pub const SYS_mlockall: ::c_ulong = 4000 + 156;
pub const SYS_munlockall: ::c_ulong = 4000 + 157;
pub const SYS_sched_setparam: ::c_ulong = 4000 + 158;
pub const SYS_sched_getparam: ::c_ulong = 4000 + 159;
pub const SYS_sched_setscheduler: ::c_ulong = 4000 + 160;
pub const SYS_sched_getscheduler: ::c_ulong = 4000 + 161;
pub const SYS_sched_yield: ::c_ulong = 4000 + 162;
pub const SYS_sched_get_priority_max: ::c_ulong = 4000 + 163;
pub const SYS_sched_get_priority_min: ::c_ulong = 4000 + 164;
pub const SYS_sched_rr_get_interval: ::c_ulong = 4000 + 165;
pub const SYS_nanosleep: ::c_ulong = 4000 + 166;
pub const SYS_mremap: ::c_ulong = 4000 + 167;
pub const SYS_accept: ::c_ulong = 4000 + 168;
pub const SYS_bind: ::c_ulong = 4000 + 169;
pub const SYS_connect: ::c_ulong = 4000 + 170;
pub const SYS_getpeername: ::c_ulong = 4000 + 171;
pub const SYS_getsockname: ::c_ulong = 4000 + 172;
pub const SYS_getsockopt: ::c_ulong = 4000 + 173;
pub const SYS_listen: ::c_ulong = 4000 + 174;
pub const SYS_recv: ::c_ulong = 4000 + 175;
pub const SYS_recvfrom: ::c_ulong = 4000 + 176;
pub const SYS_recvmsg: ::c_ulong = 4000 + 177;
pub const SYS_send: ::c_ulong = 4000 + 178;
pub const SYS_sendmsg: ::c_ulong = 4000 + 179;
pub const SYS_sendto: ::c_ulong = 4000 + 180;
pub const SYS_setsockopt: ::c_ulong = 4000 + 181;
pub const SYS_shutdown: ::c_ulong = 4000 + 182;
pub const SYS_socket: ::c_ulong = 4000 + 183;
pub const SYS_socketpair: ::c_ulong = 4000 + 184;
pub const SYS_setresuid: ::c_ulong = 4000 + 185;
pub const SYS_getresuid: ::c_ulong = 4000 + 186;
pub const SYS_query_module: ::c_ulong = 4000 + 187;
pub const SYS_poll: ::c_ulong = 4000 + 188;
pub const SYS_nfsservctl: ::c_ulong = 4000 + 189;
pub const SYS_setresgid: ::c_ulong = 4000 + 190;
pub const SYS_getresgid: ::c_ulong = 4000 + 191;
pub const SYS_prctl: ::c_ulong = 4000 + 192;
pub const SYS_rt_sigreturn: ::c_ulong = 4000 + 193;
pub const SYS_rt_sigaction: ::c_ulong = 4000 + 194;
pub const SYS_rt_sigprocmask: ::c_ulong = 4000 + 195;
pub const SYS_rt_sigpending: ::c_ulong = 4000 + 196;
pub const SYS_rt_sigtimedwait: ::c_ulong = 4000 + 197;
pub const SYS_rt_sigqueueinfo: ::c_ulong = 4000 + 198;
pub const SYS_rt_sigsuspend: ::c_ulong = 4000 + 199;
pub const SYS_pread64: ::c_ulong = 4000 + 200;
pub const SYS_pwrite64: ::c_ulong = 4000 + 201;
pub const SYS_chown: ::c_ulong = 4000 + 202;
pub const SYS_getcwd: ::c_ulong = 4000 + 203;
pub const SYS_capget: ::c_ulong = 4000 + 204;
pub const SYS_capset: ::c_ulong = 4000 + 205;
pub const SYS_sigaltstack: ::c_ulong = 4000 + 206;
pub const SYS_sendfile: ::c_ulong = 4000 + 207;
pub const SYS_getpmsg: ::c_ulong = 4000 + 208;
pub const SYS_putpmsg: ::c_ulong = 4000 + 209;
pub const SYS_mmap2: ::c_ulong = 4000 + 210;
pub const SYS_truncate64: ::c_ulong = 4000 + 211;
pub const SYS_ftruncate64: ::c_ulong = 4000 + 212;
pub const SYS_stat64: ::c_ulong = 4000 + 213;
pub const SYS_lstat64: ::c_ulong = 4000 + 214;
pub const SYS_fstat64: ::c_ulong = 4000 + 215;
pub const SYS_pivot_root: ::c_ulong = 4000 + 216;
pub const SYS_mincore: ::c_ulong = 4000 + 217;
pub const SYS_madvise: ::c_ulong = 4000 + 218;
pub const SYS_getdents64: ::c_ulong = 4000 + 219;
pub const SYS_fcntl64: ::c_ulong = 4000 + 220;
pub const SYS_reserved221: ::c_ulong = 4000 + 221;
pub const SYS_gettid: ::c_ulong = 4000 + 222;
pub const SYS_readahead: ::c_ulong = 4000 + 223;
pub const SYS_setxattr: ::c_ulong = 4000 + 224;
pub const SYS_lsetxattr: ::c_ulong = 4000 + 225;
pub const SYS_fsetxattr: ::c_ulong = 4000 + 226;
pub const SYS_getxattr: ::c_ulong = 4000 + 227;
pub const SYS_lgetxattr: ::c_ulong = 4000 + 228;
pub const SYS_fgetxattr: ::c_ulong = 4000 + 229;
pub const SYS_listxattr: ::c_ulong = 4000 + 230;
pub const SYS_llistxattr: ::c_ulong = 4000 + 231;
pub const SYS_flistxattr: ::c_ulong = 4000 + 232;
pub const SYS_removexattr: ::c_ulong = 4000 + 233;
pub const SYS_lremovexattr: ::c_ulong = 4000 + 234;
pub const SYS_fremovexattr: ::c_ulong = 4000 + 235;
pub const SYS_tkill: ::c_ulong = 4000 + 236;
pub const SYS_sendfile64: ::c_ulong = 4000 + 237;
pub const SYS_futex: ::c_ulong = 4000 + 238;
pub const SYS_sched_setaffinity: ::c_ulong = 4000 + 239;
pub const SYS_sched_getaffinity: ::c_ulong = 4000 + 240;
pub const SYS_io_setup: ::c_ulong = 4000 + 241;
pub const SYS_io_destroy: ::c_ulong = 4000 + 242;
pub const SYS_io_getevents: ::c_ulong = 4000 + 243;
pub const SYS_io_submit: ::c_ulong = 4000 + 244;
pub const SYS_io_cancel: ::c_ulong = 4000 + 245;
pub const SYS_exit_group: ::c_ulong = 4000 + 246;
pub const SYS_lookup_dcookie: ::c_ulong = 4000 + 247;
pub const SYS_epoll_create: ::c_ulong = 4000 + 248;
pub const SYS_epoll_ctl: ::c_ulong = 4000 + 249;
pub const SYS_epoll_wait: ::c_ulong = 4000 + 250;
pub const SYS_remap_file_pages: ::c_ulong = 4000 + 251;
pub const SYS_set_tid_address: ::c_ulong = 4000 + 252;
pub const SYS_restart_syscall: ::c_ulong = 4000 + 253;
pub const SYS_fadvise64: ::c_ulong = 4000 + 254;
pub const SYS_statfs64: ::c_ulong = 4000 + 255;
pub const SYS_fstatfs64: ::c_ulong = 4000 + 256;
pub const SYS_timer_create: ::c_ulong = 4000 + 257;
pub const SYS_timer_settime: ::c_ulong = 4000 + 258;
pub const SYS_timer_gettime: ::c_ulong = 4000 + 259;
pub const SYS_timer_getoverrun: ::c_ulong = 4000 + 260;
pub const SYS_timer_delete: ::c_ulong = 4000 + 261;
pub const SYS_clock_settime: ::c_ulong = 4000 + 262;
pub const SYS_clock_gettime: ::c_ulong = 4000 + 263;
pub const SYS_clock_getres: ::c_ulong = 4000 + 264;
pub const SYS_clock_nanosleep: ::c_ulong = 4000 + 265;
pub const SYS_tgkill: ::c_ulong = 4000 + 266;
pub const SYS_utimes: ::c_ulong = 4000 + 267;
pub const SYS_mbind: ::c_ulong = 4000 + 268;
pub const SYS_get_mempolicy: ::c_ulong = 4000 + 269;
pub const SYS_set_mempolicy: ::c_ulong = 4000 + 270;
pub const SYS_mq_open: ::c_ulong = 4000 + 271;
pub const SYS_mq_unlink: ::c_ulong = 4000 + 272;
pub const SYS_mq_timedsend: ::c_ulong = 4000 + 273;
pub const SYS_mq_timedreceive: ::c_ulong = 4000 + 274;
pub const SYS_mq_notify: ::c_ulong = 4000 + 275;
pub const SYS_mq_getsetattr: ::c_ulong = 4000 + 276;
pub const SYS_vserver: ::c_ulong = 4000 + 277;
pub const SYS_waitid: ::c_ulong = 4000 + 278;
/* pub const SYS_sys_setaltroot: ::c_ulong = 4000 + 279; */
pub const SYS_add_key: ::c_ulong = 4000 + 280;
pub const SYS_request_key: ::c_ulong = 4000 + 281;
pub const SYS_keyctl: ::c_ulong = 4000 + 282;
pub const SYS_set_thread_area: ::c_ulong = 4000 + 283;
pub const SYS_inotify_init: ::c_ulong = 4000 + 284;
pub const SYS_inotify_add_watch: ::c_ulong = 4000 + 285;
pub const SYS_inotify_rm_watch: ::c_ulong = 4000 + 286;
pub const SYS_migrate_pages: ::c_ulong = 4000 + 287;
pub const SYS_openat: ::c_ulong = 4000 + 288;
pub const SYS_mkdirat: ::c_ulong = 4000 + 289;
pub const SYS_mknodat: ::c_ulong = 4000 + 290;
pub const SYS_fchownat: ::c_ulong = 4000 + 291;
pub const SYS_futimesat: ::c_ulong = 4000 + 292;
pub const SYS_fstatat64: ::c_ulong = 4000 + 293;
pub const SYS_unlinkat: ::c_ulong = 4000 + 294;
pub const SYS_renameat: ::c_ulong = 4000 + 295;
pub const SYS_linkat: ::c_ulong = 4000 + 296;
pub const SYS_symlinkat: ::c_ulong = 4000 + 297;
pub const SYS_readlinkat: ::c_ulong = 4000 + 298;
pub const SYS_fchmodat: ::c_ulong = 4000 + 299;
pub const SYS_faccessat: ::c_ulong = 4000 + 300;
pub const SYS_pselect6: ::c_ulong = 4000 + 301;
pub const SYS_ppoll: ::c_ulong = 4000 + 302;
pub const SYS_unshare: ::c_ulong = 4000 + 303;
pub const SYS_splice: ::c_ulong = 4000 + 304;
pub const SYS_sync_file_range: ::c_ulong = 4000 + 305;
pub const SYS_tee: ::c_ulong = 4000 + 306;
pub const SYS_vmsplice: ::c_ulong = 4000 + 307;
pub const SYS_move_pages: ::c_ulong = 4000 + 308;
pub const SYS_set_robust_list: ::c_ulong = 4000 + 309;
pub const SYS_get_robust_list: ::c_ulong = 4000 + 310;
pub const SYS_kexec_load: ::c_ulong = 4000 + 311;
pub const SYS_getcpu: ::c_ulong = 4000 + 312;
pub const SYS_epoll_pwait: ::c_ulong = 4000 + 313;
pub const SYS_ioprio_set: ::c_ulong = 4000 + 314;
pub const SYS_ioprio_get: ::c_ulong = 4000 + 315;
pub const SYS_utimensat: ::c_ulong = 4000 + 316;
pub const SYS_signalfd: ::c_ulong = 4000 + 317;
pub const SYS_timerfd: ::c_ulong = 4000 + 318;
pub const SYS_eventfd: ::c_ulong = 4000 + 319;
pub const SYS_fallocate: ::c_ulong = 4000 + 320;
pub const SYS_timerfd_create: ::c_ulong = 4000 + 321;
pub const SYS_timerfd_gettime: ::c_ulong = 4000 + 322;
pub const SYS_timerfd_settime: ::c_ulong = 4000 + 323;
pub const SYS_signalfd4: ::c_ulong = 4000 + 324;
pub const SYS_eventfd2: ::c_ulong = 4000 + 325;
pub const SYS_epoll_create1: ::c_ulong = 4000 + 326;
pub const SYS_dup3: ::c_ulong = 4000 + 327;
pub const SYS_pipe2: ::c_ulong = 4000 + 328;
pub const SYS_inotify_init1: ::c_ulong = 4000 + 329;
pub const SYS_preadv: ::c_ulong = 4000 + 330;
pub const SYS_pwritev: ::c_ulong = 4000 + 331;
pub const SYS_rt_tgsigqueueinfo: ::c_ulong = 4000 + 332;
pub const SYS_perf_event_open: ::c_ulong = 4000 + 333;
pub const SYS_accept4: ::c_ulong = 4000 + 334;
pub const SYS_recvmmsg: ::c_ulong = 4000 + 335;
pub const SYS_fanotify_init: ::c_ulong = 4000 + 336;
pub const SYS_fanotify_mark: ::c_ulong = 4000 + 337;
pub const SYS_prlimit64: ::c_ulong = 4000 + 338;
pub const SYS_name_to_handle_at: ::c_ulong = 4000 + 339;
pub const SYS_open_by_handle_at: ::c_ulong = 4000 + 340;
pub const SYS_clock_adjtime: ::c_ulong = 4000 + 341;
pub const SYS_syncfs: ::c_ulong = 4000 + 342;
pub const SYS_sendmmsg: ::c_ulong = 4000 + 343;
pub const SYS_setns: ::c_ulong = 4000 + 344;
pub const SYS_process_vm_readv: ::c_ulong = 4000 + 345;
pub const SYS_process_vm_writev: ::c_ulong = 4000 + 346;
pub const SYS_kcmp: ::c_ulong = 4000 + 347;
pub const SYS_finit_module: ::c_ulong = 4000 + 348;
pub const SYS_sched_setattr: ::c_ulong = 4000 + 349;
pub const SYS_sched_getattr: ::c_ulong = 4000 + 350;
pub const SYS_renameat2: ::c_ulong = 4000 + 351;
pub const SYS_seccomp: ::c_ulong = 4000 + 352;
pub const SYS_getrandom: ::c_ulong = 4000 + 353;
pub const SYS_memfd_create: ::c_ulong = 4000 + 354;
pub const SYS_bpf: ::c_ulong = 4000 + 355;
pub const SYS_execveat: ::c_ulong = 4000 + 356;
pub const SYS_userfaultfd: ::c_ulong = 4000 + 357;
pub const SYS_membarrier: ::c_ulong = 4000 + 358;
pub const SYS_mlock2: ::c_ulong = 4000 + 359;
pub const SYS_copy_file_range: ::c_ulong = 4000 + 360;
pub const SYS_preadv2: ::c_ulong = 4000 + 361;
pub const SYS_pwritev2: ::c_ulong = 4000 + 362;
pub const SYS_pkey_mprotect: ::c_ulong = 4000 + 363;
pub const SYS_pkey_alloc: ::c_ulong = 4000 + 364;
pub const SYS_pkey_free: ::c_ulong = 4000 + 365;
