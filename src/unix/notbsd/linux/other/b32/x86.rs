pub type c_char = i8;
pub type wchar_t = i32;
pub type greg_t = i32;

s! {
    pub struct _libc_fpreg {
        pub significand: [u16; 4],
        pub exponent: u16,
    }

    pub struct _libc_fpstate {
        pub cw: ::c_ulong,
        pub sw: ::c_ulong,
        pub tag: ::c_ulong,
        pub ipoff: ::c_ulong,
        pub cssel: ::c_ulong,
        pub dataoff: ::c_ulong,
        pub datasel: ::c_ulong,
        pub _st: [_libc_fpreg; 8],
        pub status: ::c_ulong,
    }

    pub struct user_fpregs_struct {
        pub cwd: ::c_long,
        pub swd: ::c_long,
        pub twd: ::c_long,
        pub fip: ::c_long,
        pub fcs: ::c_long,
        pub foo: ::c_long,
        pub fos: ::c_long,
        pub st_space: [::c_long; 20],
    }

    pub struct user_fpxregs_struct {
        pub cwd: ::c_ushort,
        pub swd: ::c_ushort,
        pub twd: ::c_ushort,
        pub fop: ::c_ushort,
        pub fip: ::c_long,
        pub fcs: ::c_long,
        pub foo: ::c_long,
        pub fos: ::c_long,
        pub mxcsr: ::c_long,
        __reserved: ::c_long,
        pub st_space: [::c_long; 32],
        pub xmm_space: [::c_long; 32],
        padding: [::c_long; 56],
    }

    pub struct user_regs_struct {
        pub ebx: ::c_long,
        pub ecx: ::c_long,
        pub edx: ::c_long,
        pub esi: ::c_long,
        pub edi: ::c_long,
        pub ebp: ::c_long,
        pub eax: ::c_long,
        pub xds: ::c_long,
        pub xes: ::c_long,
        pub xfs: ::c_long,
        pub xgs: ::c_long,
        pub orig_eax: ::c_long,
        pub eip: ::c_long,
        pub xcs: ::c_long,
        pub eflags: ::c_long,
        pub esp: ::c_long,
        pub xss: ::c_long,
    }

    pub struct user {
        pub regs: user_regs_struct,
        pub u_fpvalid: ::c_int,
        pub i387: user_fpregs_struct,
        pub u_tsize: ::c_ulong,
        pub u_dsize: ::c_ulong,
        pub u_ssize: ::c_ulong,
        pub start_code: ::c_ulong,
        pub start_stack: ::c_ulong,
        pub signal: ::c_long,
        __reserved: ::c_int,
        pub u_ar0: *mut user_regs_struct,
        pub u_fpstate: *mut user_fpregs_struct,
        pub magic: ::c_ulong,
        pub u_comm: [c_char; 32],
        pub u_debugreg: [::c_int; 8],
    }

    pub struct mcontext_t {
        pub gregs: [greg_t; 19],
        pub fpregs: *mut _libc_fpstate,
        pub oldmask: ::c_ulong,
        pub cr2: ::c_ulong,
    }

    pub struct ucontext_t {
        pub uc_flags: ::c_ulong,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        pub uc_mcontext: mcontext_t,
        pub uc_sigmask: ::sigset_t,
        __private: [u8; 112],
    }

    pub struct ipc_perm {
        pub __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::c_ushort,
        __pad1: ::c_ushort,
        pub __seq: ::c_ushort,
        __pad2: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        __pad1: ::c_uint,
        __st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad2: ::c_uint,
        pub st_size: ::off64_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_ino: ::ino64_t,
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_atime: ::time_t,
        __unused1: ::c_ulong,
        pub shm_dtime: ::time_t,
        __unused2: ::c_ulong,
        pub shm_ctime: ::time_t,
        __unused3: ::c_ulong,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __unused4: ::c_ulong,
        __unused5: ::c_ulong
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        pub msg_stime: ::time_t,
        __glibc_reserved1: ::c_ulong,
        pub msg_rtime: ::time_t,
        __glibc_reserved2: ::c_ulong,
        pub msg_ctime: ::time_t,
        __glibc_reserved3: ::c_ulong,
        __msg_cbytes: ::c_ulong,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __glibc_reserved4: ::c_ulong,
        __glibc_reserved5: ::c_ulong,
    }
}

pub const O_DIRECT: ::c_int = 0x4000;
pub const O_DIRECTORY: ::c_int = 0x10000;
pub const O_NOFOLLOW: ::c_int = 0x20000;

pub const MAP_LOCKED: ::c_int = 0x02000;
pub const MAP_NORESERVE: ::c_int = 0x04000;
pub const MAP_32BIT: ::c_int = 0x0040;

pub const EDEADLOCK: ::c_int = 35;

pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_PASSCRED: ::c_int = 16;
pub const SO_PEERCRED: ::c_int = 17;
pub const SO_RCVLOWAT: ::c_int = 18;
pub const SO_SNDLOWAT: ::c_int = 19;
pub const SO_RCVTIMEO: ::c_int = 20;
pub const SO_SNDTIMEO: ::c_int = 21;

pub const FIOCLEX: ::c_ulong = 0x5451;
pub const FIONBIO: ::c_ulong = 0x5421;

pub const SYS_gettid: ::c_long = 224;
pub const SYS_perf_event_open: ::c_long = 336;

pub const PTRACE_GETFPXREGS: ::c_uint = 18;
pub const PTRACE_SETFPXREGS: ::c_uint = 19;

pub const MCL_CURRENT: ::c_int = 0x0001;
pub const MCL_FUTURE: ::c_int = 0x0002;

pub const SIGSTKSZ: ::size_t = 8192;
pub const MINSIGSTKSZ: ::size_t = 2048;
pub const CBAUD: ::tcflag_t = 0o0010017;
pub const TAB1: ::c_int = 0x00000800;
pub const TAB2: ::c_int = 0x00001000;
pub const TAB3: ::c_int = 0x00001800;
pub const CR1: ::c_int  = 0x00000200;
pub const CR2: ::c_int  = 0x00000400;
pub const CR3: ::c_int  = 0x00000600;
pub const FF1: ::c_int  = 0x00008000;
pub const BS1: ::c_int  = 0x00002000;
pub const VT1: ::c_int  = 0x00004000;
pub const VWERASE: usize = 14;
pub const VREPRINT: usize = 12;
pub const VSUSP: usize = 10;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VDISCARD: usize = 13;
pub const VTIME: usize = 5;
pub const IXON: ::tcflag_t = 0x00000400;
pub const IXOFF: ::tcflag_t = 0x00001000;
pub const ONLCR: ::tcflag_t = 0x4;
pub const CSIZE: ::tcflag_t = 0x00000030;
pub const CS6: ::tcflag_t = 0x00000010;
pub const CS7: ::tcflag_t = 0x00000020;
pub const CS8: ::tcflag_t = 0x00000030;
pub const CSTOPB: ::tcflag_t = 0x00000040;
pub const CREAD: ::tcflag_t = 0x00000080;
pub const PARENB: ::tcflag_t = 0x00000100;
pub const PARODD: ::tcflag_t = 0x00000200;
pub const HUPCL: ::tcflag_t = 0x00000400;
pub const CLOCAL: ::tcflag_t = 0x00000800;
pub const ECHOKE: ::tcflag_t = 0x00000800;
pub const ECHOE: ::tcflag_t = 0x00000010;
pub const ECHOK: ::tcflag_t = 0x00000020;
pub const ECHONL: ::tcflag_t = 0x00000040;
pub const ECHOPRT: ::tcflag_t = 0x00000400;
pub const ECHOCTL: ::tcflag_t = 0x00000200;
pub const ISIG: ::tcflag_t = 0x00000001;
pub const ICANON: ::tcflag_t = 0x00000002;
pub const PENDIN: ::tcflag_t = 0x00004000;
pub const NOFLSH: ::tcflag_t = 0x00000080;
pub const CIBAUD: ::tcflag_t = 0o02003600000;
pub const CBAUDEX: ::tcflag_t = 0o010000;
pub const VSWTC: usize = 7;
pub const OLCUC:  ::tcflag_t = 0o000002;
pub const NLDLY:  ::tcflag_t = 0o000400;
pub const CRDLY:  ::tcflag_t = 0o003000;
pub const TABDLY: ::tcflag_t = 0o014000;
pub const BSDLY:  ::tcflag_t = 0o020000;
pub const FFDLY:  ::tcflag_t = 0o100000;
pub const VTDLY:  ::tcflag_t = 0o040000;
pub const XTABS:  ::tcflag_t = 0o014000;

pub const B0: ::speed_t = 0o000000;
pub const B50: ::speed_t = 0o000001;
pub const B75: ::speed_t = 0o000002;
pub const B110: ::speed_t = 0o000003;
pub const B134: ::speed_t = 0o000004;
pub const B150: ::speed_t = 0o000005;
pub const B200: ::speed_t = 0o000006;
pub const B300: ::speed_t = 0o000007;
pub const B600: ::speed_t = 0o000010;
pub const B1200: ::speed_t = 0o000011;
pub const B1800: ::speed_t = 0o000012;
pub const B2400: ::speed_t = 0o000013;
pub const B4800: ::speed_t = 0o000014;
pub const B9600: ::speed_t = 0o000015;
pub const B19200: ::speed_t = 0o000016;
pub const B38400: ::speed_t = 0o000017;
pub const EXTA: ::speed_t = B19200;
pub const EXTB: ::speed_t = B38400;
pub const B57600: ::speed_t = 0o010001;
pub const B115200: ::speed_t = 0o010002;
pub const B230400: ::speed_t = 0o010003;
pub const B460800: ::speed_t = 0o010004;
pub const B500000: ::speed_t = 0o010005;
pub const B576000: ::speed_t = 0o010006;
pub const B921600: ::speed_t = 0o010007;
pub const B1000000: ::speed_t = 0o010010;
pub const B1152000: ::speed_t = 0o010011;
pub const B1500000: ::speed_t = 0o010012;
pub const B2000000: ::speed_t = 0o010013;
pub const B2500000: ::speed_t = 0o010014;
pub const B3000000: ::speed_t = 0o010015;
pub const B3500000: ::speed_t = 0o010016;
pub const B4000000: ::speed_t = 0o010017;

pub const VEOL: usize = 11;
pub const VEOL2: usize = 16;
pub const VMIN: usize = 6;
pub const IEXTEN: ::tcflag_t = 0x00008000;
pub const TOSTOP: ::tcflag_t = 0x00000100;
pub const FLUSHO: ::tcflag_t = 0x00001000;
pub const EXTPROC: ::tcflag_t = 0x00010000;
pub const TCGETS: ::c_ulong = 0x5401;
pub const TCSETS: ::c_ulong = 0x5402;
pub const TCSETSW: ::c_ulong = 0x5403;
pub const TCSETSF: ::c_ulong = 0x5404;
pub const TCGETA: ::c_ulong = 0x5405;
pub const TCSETA: ::c_ulong = 0x5406;
pub const TCSETAW: ::c_ulong = 0x5407;
pub const TCSETAF: ::c_ulong = 0x5408;
pub const TCSBRK: ::c_ulong = 0x5409;
pub const TCXONC: ::c_ulong = 0x540A;
pub const TCFLSH: ::c_ulong = 0x540B;
pub const TIOCINQ: ::c_ulong = 0x541B;
pub const TIOCGPGRP: ::c_ulong = 0x540F;
pub const TIOCSPGRP: ::c_ulong = 0x5410;
pub const TIOCOUTQ: ::c_ulong = 0x5411;
pub const TIOCGWINSZ: ::c_ulong = 0x5413;
pub const TIOCSWINSZ: ::c_ulong = 0x5414;
pub const FIONREAD: ::c_ulong = 0x541B;

extern {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t,
                       func:  extern fn (),
                       argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t,
                       ucp: *const ucontext_t) -> ::c_int;
}

// Syscall table
pub const __NR_restart_syscall: ::c_int = 0;
pub const __NR_exit: ::c_int = 1;
pub const __NR_fork: ::c_int = 2;
pub const __NR_read: ::c_int = 3;
pub const __NR_write: ::c_int = 4;
pub const __NR_open: ::c_int = 5;
pub const __NR_close: ::c_int = 6;
pub const __NR_waitpid: ::c_int = 7;
pub const __NR_creat: ::c_int = 8;
pub const __NR_link: ::c_int = 9;
pub const __NR_unlink: ::c_int = 10;
pub const __NR_execve: ::c_int = 11;
pub const __NR_chdir: ::c_int = 12;
pub const __NR_time: ::c_int = 13;
pub const __NR_mknod: ::c_int = 14;
pub const __NR_chmod: ::c_int = 15;
pub const __NR_lchown: ::c_int = 16;
pub const __NR_break: ::c_int = 17;
pub const __NR_oldstat: ::c_int = 18;
pub const __NR_lseek: ::c_int = 19;
pub const __NR_getpid: ::c_int = 20;
pub const __NR_mount: ::c_int = 21;
pub const __NR_umount: ::c_int = 22;
pub const __NR_setuid: ::c_int = 23;
pub const __NR_getuid: ::c_int = 24;
pub const __NR_stime: ::c_int = 25;
pub const __NR_ptrace: ::c_int = 26;
pub const __NR_alarm: ::c_int = 27;
pub const __NR_oldfstat: ::c_int = 28;
pub const __NR_pause: ::c_int = 29;
pub const __NR_utime: ::c_int = 30;
pub const __NR_stty: ::c_int = 31;
pub const __NR_gtty: ::c_int = 32;
pub const __NR_access: ::c_int = 33;
pub const __NR_nice: ::c_int = 34;
pub const __NR_ftime: ::c_int = 35;
pub const __NR_sync: ::c_int = 36;
pub const __NR_kill: ::c_int = 37;
pub const __NR_rename: ::c_int = 38;
pub const __NR_mkdir: ::c_int = 39;
pub const __NR_rmdir: ::c_int = 40;
pub const __NR_dup: ::c_int = 41;
pub const __NR_pipe: ::c_int = 42;
pub const __NR_times: ::c_int = 43;
pub const __NR_prof: ::c_int = 44;
pub const __NR_brk: ::c_int = 45;
pub const __NR_setgid: ::c_int = 46;
pub const __NR_getgid: ::c_int = 47;
pub const __NR_signal: ::c_int = 48;
pub const __NR_geteuid: ::c_int = 49;
pub const __NR_getegid: ::c_int = 50;
pub const __NR_acct: ::c_int = 51;
pub const __NR_umount2: ::c_int = 52;
pub const __NR_lock: ::c_int = 53;
pub const __NR_ioctl: ::c_int = 54;
pub const __NR_fcntl: ::c_int = 55;
pub const __NR_mpx: ::c_int = 56;
pub const __NR_setpgid: ::c_int = 57;
pub const __NR_ulimit: ::c_int = 58;
pub const __NR_oldolduname: ::c_int = 59;
pub const __NR_umask: ::c_int = 60;
pub const __NR_chroot: ::c_int = 61;
pub const __NR_ustat: ::c_int = 62;
pub const __NR_dup2: ::c_int = 63;
pub const __NR_getppid: ::c_int = 64;
pub const __NR_getpgrp: ::c_int = 65;
pub const __NR_setsid: ::c_int = 66;
pub const __NR_sigaction: ::c_int = 67;
pub const __NR_sgetmask: ::c_int = 68;
pub const __NR_ssetmask: ::c_int = 69;
pub const __NR_setreuid: ::c_int = 70;
pub const __NR_setregid: ::c_int = 71;
pub const __NR_sigsuspend: ::c_int = 72;
pub const __NR_sigpending: ::c_int = 73;
pub const __NR_sethostname: ::c_int = 74;
pub const __NR_setrlimit: ::c_int = 75;
pub const __NR_getrlimit: ::c_int = 76;
pub const __NR_getrusage: ::c_int = 77;
pub const __NR_gettimeofday: ::c_int = 78;
pub const __NR_settimeofday: ::c_int = 79;
pub const __NR_getgroups: ::c_int = 80;
pub const __NR_setgroups: ::c_int = 81;
pub const __NR_select: ::c_int = 82;
pub const __NR_symlink: ::c_int = 83;
pub const __NR_oldlstat: ::c_int = 84;
pub const __NR_readlink: ::c_int = 85;
pub const __NR_uselib: ::c_int = 86;
pub const __NR_swapon: ::c_int = 87;
pub const __NR_reboot: ::c_int = 88;
pub const __NR_readdir: ::c_int = 89;
pub const __NR_mmap: ::c_int = 90;
pub const __NR_munmap: ::c_int = 91;
pub const __NR_truncate: ::c_int = 92;
pub const __NR_ftruncate: ::c_int = 93;
pub const __NR_fchmod: ::c_int = 94;
pub const __NR_fchown: ::c_int = 95;
pub const __NR_getpriority: ::c_int = 96;
pub const __NR_setpriority: ::c_int = 97;
pub const __NR_profil: ::c_int = 98;
pub const __NR_statfs: ::c_int = 99;
pub const __NR_fstatfs: ::c_int = 100;
pub const __NR_ioperm: ::c_int = 101;
pub const __NR_socketcall: ::c_int = 102;
pub const __NR_syslog: ::c_int = 103;
pub const __NR_setitimer: ::c_int = 104;
pub const __NR_getitimer: ::c_int = 105;
pub const __NR_stat: ::c_int = 106;
pub const __NR_lstat: ::c_int = 107;
pub const __NR_fstat: ::c_int = 108;
pub const __NR_olduname: ::c_int = 109;
pub const __NR_iopl: ::c_int = 110;
pub const __NR_vhangup: ::c_int = 111;
pub const __NR_idle: ::c_int = 112;
pub const __NR_vm86old: ::c_int = 113;
pub const __NR_wait4: ::c_int = 114;
pub const __NR_swapoff: ::c_int = 115;
pub const __NR_sysinfo: ::c_int = 116;
pub const __NR_ipc: ::c_int = 117;
pub const __NR_fsync: ::c_int = 118;
pub const __NR_sigreturn: ::c_int = 119;
pub const __NR_clone: ::c_int = 120;
pub const __NR_setdomainname: ::c_int = 121;
pub const __NR_uname: ::c_int = 122;
pub const __NR_modify_ldt: ::c_int = 123;
pub const __NR_adjtimex: ::c_int = 124;
pub const __NR_mprotect: ::c_int = 125;
pub const __NR_sigprocmask: ::c_int = 126;
pub const __NR_create_module: ::c_int = 127;
pub const __NR_init_module: ::c_int = 128;
pub const __NR_delete_module: ::c_int = 129;
pub const __NR_get_kernel_syms: ::c_int = 130;
pub const __NR_quotactl: ::c_int = 131;
pub const __NR_getpgid: ::c_int = 132;
pub const __NR_fchdir: ::c_int = 133;
pub const __NR_bdflush: ::c_int = 134;
pub const __NR_sysfs: ::c_int = 135;
pub const __NR_personality: ::c_int = 136;
pub const __NR_afs_syscall: ::c_int = 137;
pub const __NR_setfsuid: ::c_int = 138;
pub const __NR_setfsgid: ::c_int = 139;
pub const __NR__llseek: ::c_int = 140;
pub const __NR_getdents: ::c_int = 141;
pub const __NR__newselect: ::c_int = 142;
pub const __NR_flock: ::c_int = 143;
pub const __NR_msync: ::c_int = 144;
pub const __NR_readv: ::c_int = 145;
pub const __NR_writev: ::c_int = 146;
pub const __NR_getsid: ::c_int = 147;
pub const __NR_fdatasync: ::c_int = 148;
pub const __NR__sysctl: ::c_int = 149;
pub const __NR_mlock: ::c_int = 150;
pub const __NR_munlock: ::c_int = 151;
pub const __NR_mlockall: ::c_int = 152;
pub const __NR_munlockall: ::c_int = 153;
pub const __NR_sched_setparam: ::c_int = 154;
pub const __NR_sched_getparam: ::c_int = 155;
pub const __NR_sched_setscheduler: ::c_int = 156;
pub const __NR_sched_getscheduler: ::c_int = 157;
pub const __NR_sched_yield: ::c_int = 158;
pub const __NR_sched_get_priority_max: ::c_int = 159;
pub const __NR_sched_get_priority_min: ::c_int = 160;
pub const __NR_sched_rr_get_interval: ::c_int = 161;
pub const __NR_nanosleep: ::c_int = 162;
pub const __NR_mremap: ::c_int = 163;
pub const __NR_setresuid: ::c_int = 164;
pub const __NR_getresuid: ::c_int = 165;
pub const __NR_vm86: ::c_int = 166;
pub const __NR_query_module: ::c_int = 167;
pub const __NR_poll: ::c_int = 168;
pub const __NR_nfsservctl: ::c_int = 169;
pub const __NR_setresgid: ::c_int = 170;
pub const __NR_getresgid: ::c_int = 171;
pub const __NR_prctl: ::c_int = 172;
pub const __NR_rt_sigreturn: ::c_int = 173;
pub const __NR_rt_sigaction: ::c_int = 174;
pub const __NR_rt_sigprocmask: ::c_int = 175;
pub const __NR_rt_sigpending: ::c_int = 176;
pub const __NR_rt_sigtimedwait: ::c_int = 177;
pub const __NR_rt_sigqueueinfo: ::c_int = 178;
pub const __NR_rt_sigsuspend: ::c_int = 179;
pub const __NR_pread64: ::c_int = 180;
pub const __NR_pwrite64: ::c_int = 181;
pub const __NR_chown: ::c_int = 182;
pub const __NR_getcwd: ::c_int = 183;
pub const __NR_capget: ::c_int = 184;
pub const __NR_capset: ::c_int = 185;
pub const __NR_sigaltstack: ::c_int = 186;
pub const __NR_sendfile: ::c_int = 187;
pub const __NR_getpmsg: ::c_int = 188;
pub const __NR_putpmsg: ::c_int = 189;
pub const __NR_vfork: ::c_int = 190;
pub const __NR_ugetrlimit: ::c_int = 191;
pub const __NR_mmap2: ::c_int = 192;
pub const __NR_truncate64: ::c_int = 193;
pub const __NR_ftruncate64: ::c_int = 194;
pub const __NR_stat64: ::c_int = 195;
pub const __NR_lstat64: ::c_int = 196;
pub const __NR_fstat64: ::c_int = 197;
pub const __NR_lchown32: ::c_int = 198;
pub const __NR_getuid32: ::c_int = 199;
pub const __NR_getgid32: ::c_int = 200;
pub const __NR_geteuid32: ::c_int = 201;
pub const __NR_getegid32: ::c_int = 202;
pub const __NR_setreuid32: ::c_int = 203;
pub const __NR_setregid32: ::c_int = 204;
pub const __NR_getgroups32: ::c_int = 205;
pub const __NR_setgroups32: ::c_int = 206;
pub const __NR_fchown32: ::c_int = 207;
pub const __NR_setresuid32: ::c_int = 208;
pub const __NR_getresuid32: ::c_int = 209;
pub const __NR_setresgid32: ::c_int = 210;
pub const __NR_getresgid32: ::c_int = 211;
pub const __NR_chown32: ::c_int = 212;
pub const __NR_setuid32: ::c_int = 213;
pub const __NR_setgid32: ::c_int = 214;
pub const __NR_setfsuid32: ::c_int = 215;
pub const __NR_setfsgid32: ::c_int = 216;
pub const __NR_pivot_root: ::c_int = 217;
pub const __NR_mincore: ::c_int = 218;
pub const __NR_madvise: ::c_int = 219;
pub const __NR_getdents64: ::c_int = 220;
pub const __NR_fcntl64: ::c_int = 221;
pub const __NR_gettid: ::c_int = 224;
pub const __NR_readahead: ::c_int = 225;
pub const __NR_setxattr: ::c_int = 226;
pub const __NR_lsetxattr: ::c_int = 227;
pub const __NR_fsetxattr: ::c_int = 228;
pub const __NR_getxattr: ::c_int = 229;
pub const __NR_lgetxattr: ::c_int = 230;
pub const __NR_fgetxattr: ::c_int = 231;
pub const __NR_listxattr: ::c_int = 232;
pub const __NR_llistxattr: ::c_int = 233;
pub const __NR_flistxattr: ::c_int = 234;
pub const __NR_removexattr: ::c_int = 235;
pub const __NR_lremovexattr: ::c_int = 236;
pub const __NR_fremovexattr: ::c_int = 237;
pub const __NR_tkill: ::c_int = 238;
pub const __NR_sendfile64: ::c_int = 239;
pub const __NR_futex: ::c_int = 240;
pub const __NR_sched_setaffinity: ::c_int = 241;
pub const __NR_sched_getaffinity: ::c_int = 242;
pub const __NR_set_thread_area: ::c_int = 243;
pub const __NR_get_thread_area: ::c_int = 244;
pub const __NR_io_setup: ::c_int = 245;
pub const __NR_io_destroy: ::c_int = 246;
pub const __NR_io_getevents: ::c_int = 247;
pub const __NR_io_submit: ::c_int = 248;
pub const __NR_io_cancel: ::c_int = 249;
pub const __NR_fadvise64: ::c_int = 250;
pub const __NR_exit_group: ::c_int = 252;
pub const __NR_lookup_dcookie: ::c_int = 253;
pub const __NR_epoll_create: ::c_int = 254;
pub const __NR_epoll_ctl: ::c_int = 255;
pub const __NR_epoll_wait: ::c_int = 256;
pub const __NR_remap_file_pages: ::c_int = 257;
pub const __NR_set_tid_address: ::c_int = 258;
pub const __NR_timer_create: ::c_int = 259;
pub const __NR_timer_settime: ::c_int = 260;
pub const __NR_timer_gettime: ::c_int = 261;
pub const __NR_timer_getoverrun: ::c_int = 262;
pub const __NR_timer_delete: ::c_int = 263;
pub const __NR_clock_settime: ::c_int = 264;
pub const __NR_clock_gettime: ::c_int = 265;
pub const __NR_clock_getres: ::c_int = 266;
pub const __NR_clock_nanosleep: ::c_int = 267;
pub const __NR_statfs64: ::c_int = 268;
pub const __NR_fstatfs64: ::c_int = 269;
pub const __NR_tgkill: ::c_int = 270;
pub const __NR_utimes: ::c_int = 271;
pub const __NR_fadvise64_64: ::c_int = 272;
pub const __NR_vserver: ::c_int = 273;
pub const __NR_mbind: ::c_int = 274;
pub const __NR_get_mempolicy: ::c_int = 275;
pub const __NR_set_mempolicy: ::c_int = 276;
pub const __NR_mq_open: ::c_int = 277;
pub const __NR_mq_unlink: ::c_int = 278;
pub const __NR_mq_timedsend: ::c_int = 279;
pub const __NR_mq_timedreceive: ::c_int = 280;
pub const __NR_mq_notify: ::c_int = 281;
pub const __NR_mq_getsetattr: ::c_int = 282;
pub const __NR_kexec_load: ::c_int = 283;
pub const __NR_waitid: ::c_int = 284;
pub const __NR_add_key: ::c_int = 286;
pub const __NR_request_key: ::c_int = 287;
pub const __NR_keyctl: ::c_int = 288;
pub const __NR_ioprio_set: ::c_int = 289;
pub const __NR_ioprio_get: ::c_int = 290;
pub const __NR_inotify_init: ::c_int = 291;
pub const __NR_inotify_add_watch: ::c_int = 292;
pub const __NR_inotify_rm_watch: ::c_int = 293;
pub const __NR_migrate_pages: ::c_int = 294;
pub const __NR_openat: ::c_int = 295;
pub const __NR_mkdirat: ::c_int = 296;
pub const __NR_mknodat: ::c_int = 297;
pub const __NR_fchownat: ::c_int = 298;
pub const __NR_futimesat: ::c_int = 299;
pub const __NR_fstatat64: ::c_int = 300;
pub const __NR_unlinkat: ::c_int = 301;
pub const __NR_renameat: ::c_int = 302;
pub const __NR_linkat: ::c_int = 303;
pub const __NR_symlinkat: ::c_int = 304;
pub const __NR_readlinkat: ::c_int = 305;
pub const __NR_fchmodat: ::c_int = 306;
pub const __NR_faccessat: ::c_int = 307;
pub const __NR_pselect6: ::c_int = 308;
pub const __NR_ppoll: ::c_int = 309;
pub const __NR_unshare: ::c_int = 310;
pub const __NR_set_robust_list: ::c_int = 311;
pub const __NR_get_robust_list: ::c_int = 312;
pub const __NR_splice: ::c_int = 313;
pub const __NR_sync_file_range: ::c_int = 314;
pub const __NR_tee: ::c_int = 315;
pub const __NR_vmsplice: ::c_int = 316;
pub const __NR_move_pages: ::c_int = 317;
pub const __NR_getcpu: ::c_int = 318;
pub const __NR_epoll_pwait: ::c_int = 319;
pub const __NR_utimensat: ::c_int = 320;
pub const __NR_signalfd: ::c_int = 321;
pub const __NR_timerfd_create: ::c_int = 322;
pub const __NR_eventfd: ::c_int = 323;
pub const __NR_fallocate: ::c_int = 324;
pub const __NR_timerfd_settime: ::c_int = 325;
pub const __NR_timerfd_gettime: ::c_int = 326;
pub const __NR_signalfd4: ::c_int = 327;
pub const __NR_eventfd2: ::c_int = 328;
pub const __NR_epoll_create1: ::c_int = 329;
pub const __NR_dup3: ::c_int = 330;
pub const __NR_pipe2: ::c_int = 331;
pub const __NR_inotify_init1: ::c_int = 332;
pub const __NR_preadv: ::c_int = 333;
pub const __NR_pwritev: ::c_int = 334;
pub const __NR_rt_tgsigqueueinfo: ::c_int = 335;
pub const __NR_perf_event_open: ::c_int = 336;
pub const __NR_recvmmsg: ::c_int = 337;
pub const __NR_fanotify_init: ::c_int = 338;
pub const __NR_fanotify_mark: ::c_int = 339;
pub const __NR_prlimit64: ::c_int = 340;
pub const __NR_name_to_handle_at: ::c_int = 341;
pub const __NR_open_by_handle_at: ::c_int = 342;
pub const __NR_clock_adjtime: ::c_int = 343;
pub const __NR_syncfs: ::c_int = 344;
pub const __NR_sendmmsg: ::c_int = 345;
pub const __NR_setns: ::c_int = 346;
pub const __NR_process_vm_readv: ::c_int = 347;
pub const __NR_process_vm_writev: ::c_int = 348;
pub const __NR_kcmp: ::c_int = 349;
pub const __NR_finit_module: ::c_int = 350;
pub const __NR_sched_setattr: ::c_int = 351;
pub const __NR_sched_getattr: ::c_int = 352;
pub const __NR_renameat2: ::c_int = 353;
pub const __NR_seccomp: ::c_int = 354;
pub const __NR_getrandom: ::c_int = 355;
pub const __NR_memfd_create: ::c_int = 356;
pub const __NR_bpf: ::c_int = 357;
pub const __NR_execveat: ::c_int = 358;
pub const __NR_socket: ::c_int = 359;
pub const __NR_socketpair: ::c_int = 360;
pub const __NR_bind: ::c_int = 361;
pub const __NR_connect: ::c_int = 362;
pub const __NR_listen: ::c_int = 363;
pub const __NR_accept4: ::c_int = 364;
pub const __NR_getsockopt: ::c_int = 365;
pub const __NR_setsockopt: ::c_int = 366;
pub const __NR_getsockname: ::c_int = 367;
pub const __NR_getpeername: ::c_int = 368;
pub const __NR_sendto: ::c_int = 369;
pub const __NR_sendmsg: ::c_int = 370;
pub const __NR_recvfrom: ::c_int = 371;
pub const __NR_recvmsg: ::c_int = 372;
pub const __NR_shutdown: ::c_int = 373;
pub const __NR_userfaultfd: ::c_int = 374;
pub const __NR_membarrier: ::c_int = 375;
pub const __NR_mlock2: ::c_int = 376;
pub const __NR_copy_file_range: ::c_int = 377;
pub const __NR_preadv2: ::c_int = 378;
pub const __NR_pwritev2: ::c_int = 379;
pub const __NR_pkey_mprotect: ::c_int = 380;
pub const __NR_pkey_alloc: ::c_int = 381;
pub const __NR_pkey_free: ::c_int = 382;