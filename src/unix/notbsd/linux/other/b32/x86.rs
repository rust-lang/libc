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

// Syscall table
pub const SYS_restart_syscall: ::c_int = 0;
pub const SYS_exit: ::c_int = 1;
pub const SYS_fork: ::c_int = 2;
pub const SYS_read: ::c_int = 3;
pub const SYS_write: ::c_int = 4;
pub const SYS_open: ::c_int = 5;
pub const SYS_close: ::c_int = 6;
pub const SYS_waitpid: ::c_int = 7;
pub const SYS_creat: ::c_int = 8;
pub const SYS_link: ::c_int = 9;
pub const SYS_unlink: ::c_int = 10;
pub const SYS_execve: ::c_int = 11;
pub const SYS_chdir: ::c_int = 12;
pub const SYS_time: ::c_int = 13;
pub const SYS_mknod: ::c_int = 14;
pub const SYS_chmod: ::c_int = 15;
pub const SYS_lchown: ::c_int = 16;
pub const SYS_break: ::c_int = 17;
pub const SYS_oldstat: ::c_int = 18;
pub const SYS_lseek: ::c_int = 19;
pub const SYS_getpid: ::c_int = 20;
pub const SYS_mount: ::c_int = 21;
pub const SYS_umount: ::c_int = 22;
pub const SYS_setuid: ::c_int = 23;
pub const SYS_getuid: ::c_int = 24;
pub const SYS_stime: ::c_int = 25;
pub const SYS_ptrace: ::c_int = 26;
pub const SYS_alarm: ::c_int = 27;
pub const SYS_oldfstat: ::c_int = 28;
pub const SYS_pause: ::c_int = 29;
pub const SYS_utime: ::c_int = 30;
pub const SYS_stty: ::c_int = 31;
pub const SYS_gtty: ::c_int = 32;
pub const SYS_access: ::c_int = 33;
pub const SYS_nice: ::c_int = 34;
pub const SYS_ftime: ::c_int = 35;
pub const SYS_sync: ::c_int = 36;
pub const SYS_kill: ::c_int = 37;
pub const SYS_rename: ::c_int = 38;
pub const SYS_mkdir: ::c_int = 39;
pub const SYS_rmdir: ::c_int = 40;
pub const SYS_dup: ::c_int = 41;
pub const SYS_pipe: ::c_int = 42;
pub const SYS_times: ::c_int = 43;
pub const SYS_prof: ::c_int = 44;
pub const SYS_brk: ::c_int = 45;
pub const SYS_setgid: ::c_int = 46;
pub const SYS_getgid: ::c_int = 47;
pub const SYS_signal: ::c_int = 48;
pub const SYS_geteuid: ::c_int = 49;
pub const SYS_getegid: ::c_int = 50;
pub const SYS_acct: ::c_int = 51;
pub const SYS_umount2: ::c_int = 52;
pub const SYS_lock: ::c_int = 53;
pub const SYS_ioctl: ::c_int = 54;
pub const SYS_fcntl: ::c_int = 55;
pub const SYS_mpx: ::c_int = 56;
pub const SYS_setpgid: ::c_int = 57;
pub const SYS_ulimit: ::c_int = 58;
pub const SYS_oldolduname: ::c_int = 59;
pub const SYS_umask: ::c_int = 60;
pub const SYS_chroot: ::c_int = 61;
pub const SYS_ustat: ::c_int = 62;
pub const SYS_dup2: ::c_int = 63;
pub const SYS_getppid: ::c_int = 64;
pub const SYS_getpgrp: ::c_int = 65;
pub const SYS_setsid: ::c_int = 66;
pub const SYS_sigaction: ::c_int = 67;
pub const SYS_sgetmask: ::c_int = 68;
pub const SYS_ssetmask: ::c_int = 69;
pub const SYS_setreuid: ::c_int = 70;
pub const SYS_setregid: ::c_int = 71;
pub const SYS_sigsuspend: ::c_int = 72;
pub const SYS_sigpending: ::c_int = 73;
pub const SYS_sethostname: ::c_int = 74;
pub const SYS_setrlimit: ::c_int = 75;
pub const SYS_getrlimit: ::c_int = 76;
pub const SYS_getrusage: ::c_int = 77;
pub const SYS_gettimeofday: ::c_int = 78;
pub const SYS_settimeofday: ::c_int = 79;
pub const SYS_getgroups: ::c_int = 80;
pub const SYS_setgroups: ::c_int = 81;
pub const SYS_select: ::c_int = 82;
pub const SYS_symlink: ::c_int = 83;
pub const SYS_oldlstat: ::c_int = 84;
pub const SYS_readlink: ::c_int = 85;
pub const SYS_uselib: ::c_int = 86;
pub const SYS_swapon: ::c_int = 87;
pub const SYS_reboot: ::c_int = 88;
pub const SYS_readdir: ::c_int = 89;
pub const SYS_mmap: ::c_int = 90;
pub const SYS_munmap: ::c_int = 91;
pub const SYS_truncate: ::c_int = 92;
pub const SYS_ftruncate: ::c_int = 93;
pub const SYS_fchmod: ::c_int = 94;
pub const SYS_fchown: ::c_int = 95;
pub const SYS_getpriority: ::c_int = 96;
pub const SYS_setpriority: ::c_int = 97;
pub const SYS_profil: ::c_int = 98;
pub const SYS_statfs: ::c_int = 99;
pub const SYS_fstatfs: ::c_int = 100;
pub const SYS_ioperm: ::c_int = 101;
pub const SYS_socketcall: ::c_int = 102;
pub const SYS_syslog: ::c_int = 103;
pub const SYS_setitimer: ::c_int = 104;
pub const SYS_getitimer: ::c_int = 105;
pub const SYS_stat: ::c_int = 106;
pub const SYS_lstat: ::c_int = 107;
pub const SYS_fstat: ::c_int = 108;
pub const SYS_olduname: ::c_int = 109;
pub const SYS_iopl: ::c_int = 110;
pub const SYS_vhangup: ::c_int = 111;
pub const SYS_idle: ::c_int = 112;
pub const SYS_vm86old: ::c_int = 113;
pub const SYS_wait4: ::c_int = 114;
pub const SYS_swapoff: ::c_int = 115;
pub const SYS_sysinfo: ::c_int = 116;
pub const SYS_ipc: ::c_int = 117;
pub const SYS_fsync: ::c_int = 118;
pub const SYS_sigreturn: ::c_int = 119;
pub const SYS_clone: ::c_int = 120;
pub const SYS_setdomainname: ::c_int = 121;
pub const SYS_uname: ::c_int = 122;
pub const SYS_modify_ldt: ::c_int = 123;
pub const SYS_adjtimex: ::c_int = 124;
pub const SYS_mprotect: ::c_int = 125;
pub const SYS_sigprocmask: ::c_int = 126;
pub const SYS_create_module: ::c_int = 127;
pub const SYS_init_module: ::c_int = 128;
pub const SYS_delete_module: ::c_int = 129;
pub const SYS_get_kernel_syms: ::c_int = 130;
pub const SYS_quotactl: ::c_int = 131;
pub const SYS_getpgid: ::c_int = 132;
pub const SYS_fchdir: ::c_int = 133;
pub const SYS_bdflush: ::c_int = 134;
pub const SYS_sysfs: ::c_int = 135;
pub const SYS_personality: ::c_int = 136;
pub const SYS_afs_syscall: ::c_int = 137;
pub const SYS_setfsuid: ::c_int = 138;
pub const SYS_setfsgid: ::c_int = 139;
pub const SYS__llseek: ::c_int = 140;
pub const SYS_getdents: ::c_int = 141;
pub const SYS__newselect: ::c_int = 142;
pub const SYS_flock: ::c_int = 143;
pub const SYS_msync: ::c_int = 144;
pub const SYS_readv: ::c_int = 145;
pub const SYS_writev: ::c_int = 146;
pub const SYS_getsid: ::c_int = 147;
pub const SYS_fdatasync: ::c_int = 148;
pub const SYS__sysctl: ::c_int = 149;
pub const SYS_mlock: ::c_int = 150;
pub const SYS_munlock: ::c_int = 151;
pub const SYS_mlockall: ::c_int = 152;
pub const SYS_munlockall: ::c_int = 153;
pub const SYS_sched_setparam: ::c_int = 154;
pub const SYS_sched_getparam: ::c_int = 155;
pub const SYS_sched_setscheduler: ::c_int = 156;
pub const SYS_sched_getscheduler: ::c_int = 157;
pub const SYS_sched_yield: ::c_int = 158;
pub const SYS_sched_get_priority_max: ::c_int = 159;
pub const SYS_sched_get_priority_min: ::c_int = 160;
pub const SYS_sched_rr_get_interval: ::c_int = 161;
pub const SYS_nanosleep: ::c_int = 162;
pub const SYS_mremap: ::c_int = 163;
pub const SYS_setresuid: ::c_int = 164;
pub const SYS_getresuid: ::c_int = 165;
pub const SYS_vm86: ::c_int = 166;
pub const SYS_query_module: ::c_int = 167;
pub const SYS_poll: ::c_int = 168;
pub const SYS_nfsservctl: ::c_int = 169;
pub const SYS_setresgid: ::c_int = 170;
pub const SYS_getresgid: ::c_int = 171;
pub const SYS_prctl: ::c_int = 172;
pub const SYS_rt_sigreturn: ::c_int = 173;
pub const SYS_rt_sigaction: ::c_int = 174;
pub const SYS_rt_sigprocmask: ::c_int = 175;
pub const SYS_rt_sigpending: ::c_int = 176;
pub const SYS_rt_sigtimedwait: ::c_int = 177;
pub const SYS_rt_sigqueueinfo: ::c_int = 178;
pub const SYS_rt_sigsuspend: ::c_int = 179;
pub const SYS_pread64: ::c_int = 180;
pub const SYS_pwrite64: ::c_int = 181;
pub const SYS_chown: ::c_int = 182;
pub const SYS_getcwd: ::c_int = 183;
pub const SYS_capget: ::c_int = 184;
pub const SYS_capset: ::c_int = 185;
pub const SYS_sigaltstack: ::c_int = 186;
pub const SYS_sendfile: ::c_int = 187;
pub const SYS_getpmsg: ::c_int = 188;
pub const SYS_putpmsg: ::c_int = 189;
pub const SYS_vfork: ::c_int = 190;
pub const SYS_ugetrlimit: ::c_int = 191;
pub const SYS_mmap2: ::c_int = 192;
pub const SYS_truncate64: ::c_int = 193;
pub const SYS_ftruncate64: ::c_int = 194;
pub const SYS_stat64: ::c_int = 195;
pub const SYS_lstat64: ::c_int = 196;
pub const SYS_fstat64: ::c_int = 197;
pub const SYS_lchown32: ::c_int = 198;
pub const SYS_getuid32: ::c_int = 199;
pub const SYS_getgid32: ::c_int = 200;
pub const SYS_geteuid32: ::c_int = 201;
pub const SYS_getegid32: ::c_int = 202;
pub const SYS_setreuid32: ::c_int = 203;
pub const SYS_setregid32: ::c_int = 204;
pub const SYS_getgroups32: ::c_int = 205;
pub const SYS_setgroups32: ::c_int = 206;
pub const SYS_fchown32: ::c_int = 207;
pub const SYS_setresuid32: ::c_int = 208;
pub const SYS_getresuid32: ::c_int = 209;
pub const SYS_setresgid32: ::c_int = 210;
pub const SYS_getresgid32: ::c_int = 211;
pub const SYS_chown32: ::c_int = 212;
pub const SYS_setuid32: ::c_int = 213;
pub const SYS_setgid32: ::c_int = 214;
pub const SYS_setfsuid32: ::c_int = 215;
pub const SYS_setfsgid32: ::c_int = 216;
pub const SYS_pivot_root: ::c_int = 217;
pub const SYS_mincore: ::c_int = 218;
pub const SYS_madvise: ::c_int = 219;
pub const SYS_getdents64: ::c_int = 220;
pub const SYS_fcntl64: ::c_int = 221;
pub const SYS_gettid: ::c_int = 224;
pub const SYS_readahead: ::c_int = 225;
pub const SYS_setxattr: ::c_int = 226;
pub const SYS_lsetxattr: ::c_int = 227;
pub const SYS_fsetxattr: ::c_int = 228;
pub const SYS_getxattr: ::c_int = 229;
pub const SYS_lgetxattr: ::c_int = 230;
pub const SYS_fgetxattr: ::c_int = 231;
pub const SYS_listxattr: ::c_int = 232;
pub const SYS_llistxattr: ::c_int = 233;
pub const SYS_flistxattr: ::c_int = 234;
pub const SYS_removexattr: ::c_int = 235;
pub const SYS_lremovexattr: ::c_int = 236;
pub const SYS_fremovexattr: ::c_int = 237;
pub const SYS_tkill: ::c_int = 238;
pub const SYS_sendfile64: ::c_int = 239;
pub const SYS_futex: ::c_int = 240;
pub const SYS_sched_setaffinity: ::c_int = 241;
pub const SYS_sched_getaffinity: ::c_int = 242;
pub const SYS_set_thread_area: ::c_int = 243;
pub const SYS_get_thread_area: ::c_int = 244;
pub const SYS_io_setup: ::c_int = 245;
pub const SYS_io_destroy: ::c_int = 246;
pub const SYS_io_getevents: ::c_int = 247;
pub const SYS_io_submit: ::c_int = 248;
pub const SYS_io_cancel: ::c_int = 249;
pub const SYS_fadvise64: ::c_int = 250;
pub const SYS_exit_group: ::c_int = 252;
pub const SYS_lookup_dcookie: ::c_int = 253;
pub const SYS_epoll_create: ::c_int = 254;
pub const SYS_epoll_ctl: ::c_int = 255;
pub const SYS_epoll_wait: ::c_int = 256;
pub const SYS_remap_file_pages: ::c_int = 257;
pub const SYS_set_tid_address: ::c_int = 258;
pub const SYS_timer_create: ::c_int = 259;
pub const SYS_timer_settime: ::c_int = 260;
pub const SYS_timer_gettime: ::c_int = 261;
pub const SYS_timer_getoverrun: ::c_int = 262;
pub const SYS_timer_delete: ::c_int = 263;
pub const SYS_clock_settime: ::c_int = 264;
pub const SYS_clock_gettime: ::c_int = 265;
pub const SYS_clock_getres: ::c_int = 266;
pub const SYS_clock_nanosleep: ::c_int = 267;
pub const SYS_statfs64: ::c_int = 268;
pub const SYS_fstatfs64: ::c_int = 269;
pub const SYS_tgkill: ::c_int = 270;
pub const SYS_utimes: ::c_int = 271;
pub const SYS_fadvise64_64: ::c_int = 272;
pub const SYS_vserver: ::c_int = 273;
pub const SYS_mbind: ::c_int = 274;
pub const SYS_get_mempolicy: ::c_int = 275;
pub const SYS_set_mempolicy: ::c_int = 276;
pub const SYS_mq_open: ::c_int = 277;
pub const SYS_mq_unlink: ::c_int = 278;
pub const SYS_mq_timedsend: ::c_int = 279;
pub const SYS_mq_timedreceive: ::c_int = 280;
pub const SYS_mq_notify: ::c_int = 281;
pub const SYS_mq_getsetattr: ::c_int = 282;
pub const SYS_kexec_load: ::c_int = 283;
pub const SYS_waitid: ::c_int = 284;
pub const SYS_add_key: ::c_int = 286;
pub const SYS_request_key: ::c_int = 287;
pub const SYS_keyctl: ::c_int = 288;
pub const SYS_ioprio_set: ::c_int = 289;
pub const SYS_ioprio_get: ::c_int = 290;
pub const SYS_inotify_init: ::c_int = 291;
pub const SYS_inotify_add_watch: ::c_int = 292;
pub const SYS_inotify_rm_watch: ::c_int = 293;
pub const SYS_migrate_pages: ::c_int = 294;
pub const SYS_openat: ::c_int = 295;
pub const SYS_mkdirat: ::c_int = 296;
pub const SYS_mknodat: ::c_int = 297;
pub const SYS_fchownat: ::c_int = 298;
pub const SYS_futimesat: ::c_int = 299;
pub const SYS_fstatat64: ::c_int = 300;
pub const SYS_unlinkat: ::c_int = 301;
pub const SYS_renameat: ::c_int = 302;
pub const SYS_linkat: ::c_int = 303;
pub const SYS_symlinkat: ::c_int = 304;
pub const SYS_readlinkat: ::c_int = 305;
pub const SYS_fchmodat: ::c_int = 306;
pub const SYS_faccessat: ::c_int = 307;
pub const SYS_pselect6: ::c_int = 308;
pub const SYS_ppoll: ::c_int = 309;
pub const SYS_unshare: ::c_int = 310;
pub const SYS_set_robust_list: ::c_int = 311;
pub const SYS_get_robust_list: ::c_int = 312;
pub const SYS_splice: ::c_int = 313;
pub const SYS_sync_file_range: ::c_int = 314;
pub const SYS_tee: ::c_int = 315;
pub const SYS_vmsplice: ::c_int = 316;
pub const SYS_move_pages: ::c_int = 317;
pub const SYS_getcpu: ::c_int = 318;
pub const SYS_epoll_pwait: ::c_int = 319;
pub const SYS_utimensat: ::c_int = 320;
pub const SYS_signalfd: ::c_int = 321;
pub const SYS_timerfd_create: ::c_int = 322;
pub const SYS_eventfd: ::c_int = 323;
pub const SYS_fallocate: ::c_int = 324;
pub const SYS_timerfd_settime: ::c_int = 325;
pub const SYS_timerfd_gettime: ::c_int = 326;
pub const SYS_signalfd4: ::c_int = 327;
pub const SYS_eventfd2: ::c_int = 328;
pub const SYS_epoll_create1: ::c_int = 329;
pub const SYS_dup3: ::c_int = 330;
pub const SYS_pipe2: ::c_int = 331;
pub const SYS_inotify_init1: ::c_int = 332;
pub const SYS_preadv: ::c_int = 333;
pub const SYS_pwritev: ::c_int = 334;
pub const SYS_rt_tgsigqueueinfo: ::c_int = 335;
pub const SYS_perf_event_open: ::c_int = 336;
pub const SYS_recvmmsg: ::c_int = 337;
pub const SYS_fanotify_init: ::c_int = 338;
pub const SYS_fanotify_mark: ::c_int = 339;
pub const SYS_prlimit64: ::c_int = 340;
pub const SYS_name_to_handle_at: ::c_int = 341;
pub const SYS_open_by_handle_at: ::c_int = 342;
pub const SYS_clock_adjtime: ::c_int = 343;
pub const SYS_syncfs: ::c_int = 344;
pub const SYS_sendmmsg: ::c_int = 345;
pub const SYS_setns: ::c_int = 346;
pub const SYS_process_vm_readv: ::c_int = 347;
pub const SYS_process_vm_writev: ::c_int = 348;
pub const SYS_kcmp: ::c_int = 349;
pub const SYS_finit_module: ::c_int = 350;
pub const SYS_sched_setattr: ::c_int = 351;
pub const SYS_sched_getattr: ::c_int = 352;
pub const SYS_renameat2: ::c_int = 353;
pub const SYS_seccomp: ::c_int = 354;
pub const SYS_getrandom: ::c_int = 355;
pub const SYS_memfd_create: ::c_int = 356;
pub const SYS_bpf: ::c_int = 357;
pub const SYS_execveat: ::c_int = 358;
pub const SYS_socket: ::c_int = 359;
pub const SYS_socketpair: ::c_int = 360;
pub const SYS_bind: ::c_int = 361;
pub const SYS_connect: ::c_int = 362;
pub const SYS_listen: ::c_int = 363;
pub const SYS_accept4: ::c_int = 364;
pub const SYS_getsockopt: ::c_int = 365;
pub const SYS_setsockopt: ::c_int = 366;
pub const SYS_getsockname: ::c_int = 367;
pub const SYS_getpeername: ::c_int = 368;
pub const SYS_sendto: ::c_int = 369;
pub const SYS_sendmsg: ::c_int = 370;
pub const SYS_recvfrom: ::c_int = 371;
pub const SYS_recvmsg: ::c_int = 372;
pub const SYS_shutdown: ::c_int = 373;
pub const SYS_userfaultfd: ::c_int = 374;
pub const SYS_membarrier: ::c_int = 375;
pub const SYS_mlock2: ::c_int = 376;
pub const SYS_copy_file_range: ::c_int = 377;
pub const SYS_preadv2: ::c_int = 378;
pub const SYS_pwritev2: ::c_int = 379;
pub const SYS_pkey_mprotect: ::c_int = 380;
pub const SYS_pkey_alloc: ::c_int = 381;
pub const SYS_pkey_free: ::c_int = 382;

extern {
    pub fn getcontext(ucp: *mut ucontext_t) -> ::c_int;
    pub fn setcontext(ucp: *const ucontext_t) -> ::c_int;
    pub fn makecontext(ucp: *mut ucontext_t,
                       func:  extern fn (),
                       argc: ::c_int, ...);
    pub fn swapcontext(uocp: *mut ucontext_t,
                       ucp: *const ucontext_t) -> ::c_int;
}
