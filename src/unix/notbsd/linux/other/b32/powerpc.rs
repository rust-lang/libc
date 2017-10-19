pub type c_char = u8;
pub type wchar_t = i32;

s! {
    pub struct ipc_perm {
        __key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::mode_t,
        __seq: ::uint32_t,
        __pad1: ::uint32_t,
        __glibc_reserved1: ::uint64_t,
        __glibc_reserved2: ::uint64_t,
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino64_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad2: ::c_ushort,
        pub st_size: ::off64_t,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt64_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __glibc_reserved4: ::c_ulong,
        __glibc_reserved5: ::c_ulong,
    }

    pub struct statfs64 {
        pub f_type: ::__fsword_t,
        pub f_bsize: ::__fsword_t,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: u64,
        pub f_files: u64,
        pub f_ffree: u64,
        pub f_fsid: ::fsid_t,
        pub f_namelen: ::__fsword_t,
        pub f_frsize: ::__fsword_t,
        pub f_flags: ::__fsword_t,
        pub f_spare: [::__fsword_t; 4],
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
        pub f_fsid: ::c_ulong,
        __f_unused: ::c_int,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        __f_spare: [::c_int; 6],
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        __glibc_reserved1: ::c_uint,
        pub shm_atime: ::time_t,
        __glibc_reserved2: ::c_uint,
        pub shm_dtime: ::time_t,
        __glibc_reserved3: ::c_uint,
        pub shm_ctime: ::time_t,
        __glibc_reserved4: ::c_uint,
        pub shm_segsz: ::size_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __glibc_reserved5: ::c_ulong,
        __glibc_reserved6: ::c_ulong,
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        __glibc_reserved1: ::c_uint,
        pub msg_stime: ::time_t,
        __glibc_reserved2: ::c_uint,
        pub msg_rtime: ::time_t,
        __glibc_reserved3: ::c_uint,
        pub msg_ctime: ::time_t,
        __msg_cbytes: ::c_ulong,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        __glibc_reserved4: ::c_ulong,
        __glibc_reserved5: ::c_ulong,
    }
}

pub const O_DIRECT: ::c_int = 0x20000;
pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_NOFOLLOW: ::c_int = 0x8000;
pub const O_LARGEFILE: ::c_int = 0o200000;

pub const MAP_LOCKED: ::c_int = 0x00080;
pub const MAP_NORESERVE: ::c_int = 0x00040;

pub const EDEADLOCK: ::c_int = 58;

pub const SO_SNDBUFFORCE: ::c_int = 32;
pub const SO_RCVBUFFORCE: ::c_int = 33;
pub const SO_NO_CHECK: ::c_int = 11;
pub const SO_RCVLOWAT: ::c_int = 16;
pub const SO_SNDLOWAT: ::c_int = 17;
pub const SO_RCVTIMEO: ::c_int = 18;
pub const SO_SNDTIMEO: ::c_int = 19;
pub const SO_PASSCRED: ::c_int = 20;
pub const SO_PEERCRED: ::c_int = 21;

pub const FIOCLEX: ::c_ulong = 0x20006601;
pub const FIONBIO: ::c_ulong = 0x8004667e;

pub const MCL_CURRENT: ::c_int = 0x2000;
pub const MCL_FUTURE: ::c_int = 0x4000;

pub const SIGSTKSZ: ::size_t = 0x4000;
pub const MINSIGSTKSZ: ::size_t = 4096;
pub const CBAUD: ::tcflag_t = 0xff;
pub const TAB1: ::c_int = 0x400;
pub const TAB2: ::c_int = 0x800;
pub const TAB3: ::c_int = 0xc00;
pub const CR1: ::c_int  = 0x1000;
pub const CR2: ::c_int  = 0x2000;
pub const CR3: ::c_int  = 0x3000;
pub const FF1: ::c_int  = 0x4000;
pub const BS1: ::c_int  = 0x8000;
pub const VT1: ::c_int  = 0x10000;
pub const VWERASE: usize = 0xa;
pub const VREPRINT: usize = 0xb;
pub const VSUSP: usize = 0xc;
pub const VSTART: usize = 0xd;
pub const VSTOP: usize = 0xe;
pub const VDISCARD: usize = 0x10;
pub const VTIME: usize = 0x7;
pub const IXON: ::tcflag_t = 0x200;
pub const IXOFF: ::tcflag_t = 0x400;
pub const ONLCR: ::tcflag_t = 0x2;
pub const CSIZE: ::tcflag_t = 0x300;
pub const CS6: ::tcflag_t = 0x100;
pub const CS7: ::tcflag_t = 0x200;
pub const CS8: ::tcflag_t = 0x300;
pub const CSTOPB: ::tcflag_t = 0x400;
pub const CREAD: ::tcflag_t = 0x800;
pub const PARENB: ::tcflag_t = 0x1000;
pub const PARODD: ::tcflag_t = 0x2000;
pub const HUPCL: ::tcflag_t = 0x4000;
pub const CLOCAL: ::tcflag_t = 0x8000;
pub const ECHOKE: ::tcflag_t = 0x1;
pub const ECHOE: ::tcflag_t = 0x2;
pub const ECHOK: ::tcflag_t = 0x4;
pub const ECHONL: ::tcflag_t = 0x10;
pub const ECHOPRT: ::tcflag_t = 0x20;
pub const ECHOCTL: ::tcflag_t = 0x40;
pub const ISIG: ::tcflag_t = 0x80;
pub const ICANON: ::tcflag_t = 0x100;
pub const PENDIN: ::tcflag_t = 0x20000000;
pub const NOFLSH: ::tcflag_t = 0x80000000;
pub const VSWTC: usize = 9;
pub const OLCUC:  ::tcflag_t = 0o000004;
pub const NLDLY:  ::tcflag_t = 0o001400;
pub const CRDLY:  ::tcflag_t = 0o030000;
pub const TABDLY: ::tcflag_t = 0o006000;
pub const BSDLY:  ::tcflag_t = 0o100000;
pub const FFDLY:  ::tcflag_t = 0o040000;
pub const VTDLY:  ::tcflag_t = 0o200000;
pub const XTABS:  ::tcflag_t = 0o006000;

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
pub const CBAUDEX: ::speed_t = 0o000020;
pub const B57600: ::speed_t = 0o0020;
pub const B115200: ::speed_t = 0o0021;
pub const B230400: ::speed_t = 0o0022;
pub const B460800: ::speed_t = 0o0023;
pub const B500000: ::speed_t = 0o0024;
pub const B576000: ::speed_t = 0o0025;
pub const B921600: ::speed_t = 0o0026;
pub const B1000000: ::speed_t = 0o0027;
pub const B1152000: ::speed_t = 0o0030;
pub const B1500000: ::speed_t = 0o0031;
pub const B2000000: ::speed_t = 0o0032;
pub const B2500000: ::speed_t = 0o0033;
pub const B3000000: ::speed_t = 0o0034;
pub const B3500000: ::speed_t = 0o0035;
pub const B4000000: ::speed_t = 0o0036;
pub const BOTHER: ::speed_t = 0o0037;

pub const VEOL: usize = 6;
pub const VEOL2: usize = 8;
pub const VMIN: usize = 5;
pub const IEXTEN: ::tcflag_t = 0x400;
pub const TOSTOP: ::tcflag_t = 0x400000;
pub const FLUSHO: ::tcflag_t = 0x800000;
pub const EXTPROC: ::tcflag_t = 0x10000000;
pub const TCGETS: ::c_ulong = 0x403c7413;
pub const TCSETS: ::c_ulong = 0x803c7414;
pub const TCSETSW: ::c_ulong = 0x803c7415;
pub const TCSETSF: ::c_ulong = 0x803c7416;
pub const TCGETA: ::c_ulong = 0x40147417;
pub const TCSETA: ::c_ulong = 0x80147418;
pub const TCSETAW: ::c_ulong = 0x80147419;
pub const TCSETAF: ::c_ulong = 0x8014741c;
pub const TCSBRK: ::c_ulong = 0x2000741d;
pub const TCXONC: ::c_ulong = 0x2000741e;
pub const TCFLSH: ::c_ulong = 0x2000741f;
pub const TIOCINQ: ::c_ulong = 0x4004667f;
pub const TIOCGPGRP: ::c_ulong = 0x40047477;
pub const TIOCSPGRP: ::c_ulong = 0x80047476;
pub const TIOCOUTQ: ::c_ulong = 0x40047473;
pub const TIOCGWINSZ: ::c_ulong = 0x40087468;
pub const TIOCSWINSZ: ::c_ulong = 0x80087467;
pub const FIONREAD: ::c_ulong = 0x4004667f;

pub const SYS_restart_syscall: ::c_ulong = 0;
pub const SYS_exit: ::c_ulong = 1;
pub const SYS_fork: ::c_ulong = 2;
pub const SYS_read: ::c_ulong = 3;
pub const SYS_write: ::c_ulong = 4;
pub const SYS_open: ::c_ulong = 5;
pub const SYS_close: ::c_ulong = 6;
pub const SYS_waitpid: ::c_ulong = 7;
pub const SYS_creat: ::c_ulong = 8;
pub const SYS_link: ::c_ulong = 9;
pub const SYS_unlink: ::c_ulong = 10;
pub const SYS_execve: ::c_ulong = 11;
pub const SYS_chdir: ::c_ulong = 12;
pub const SYS_time: ::c_ulong = 13;
pub const SYS_mknod: ::c_ulong = 14;
pub const SYS_chmod: ::c_ulong = 15;
pub const SYS_lchown: ::c_ulong = 16;
pub const SYS_break: ::c_ulong = 17;
pub const SYS_oldstat: ::c_ulong = 18;
pub const SYS_lseek: ::c_ulong = 19;
pub const SYS_getpid: ::c_ulong = 20;
pub const SYS_mount: ::c_ulong = 21;
pub const SYS_umount: ::c_ulong = 22;
pub const SYS_setuid: ::c_ulong = 23;
pub const SYS_getuid: ::c_ulong = 24;
pub const SYS_stime: ::c_ulong = 25;
pub const SYS_ptrace: ::c_ulong = 26;
pub const SYS_alarm: ::c_ulong = 27;
pub const SYS_oldfstat: ::c_ulong = 28;
pub const SYS_pause: ::c_ulong = 29;
pub const SYS_utime: ::c_ulong = 30;
pub const SYS_stty: ::c_ulong = 31;
pub const SYS_gtty: ::c_ulong = 32;
pub const SYS_access: ::c_ulong = 33;
pub const SYS_nice: ::c_ulong = 34;
pub const SYS_ftime: ::c_ulong = 35;
pub const SYS_sync: ::c_ulong = 36;
pub const SYS_kill: ::c_ulong = 37;
pub const SYS_rename: ::c_ulong = 38;
pub const SYS_mkdir: ::c_ulong = 39;
pub const SYS_rmdir: ::c_ulong = 40;
pub const SYS_dup: ::c_ulong = 41;
pub const SYS_pipe: ::c_ulong = 42;
pub const SYS_times: ::c_ulong = 43;
pub const SYS_prof: ::c_ulong = 44;
pub const SYS_brk: ::c_ulong = 45;
pub const SYS_setgid: ::c_ulong = 46;
pub const SYS_getgid: ::c_ulong = 47;
pub const SYS_signal: ::c_ulong = 48;
pub const SYS_geteuid: ::c_ulong = 49;
pub const SYS_getegid: ::c_ulong = 50;
pub const SYS_acct: ::c_ulong = 51;
pub const SYS_umount2: ::c_ulong = 52;
pub const SYS_lock: ::c_ulong = 53;
pub const SYS_ioctl: ::c_ulong = 54;
pub const SYS_fcntl: ::c_ulong = 55;
pub const SYS_mpx: ::c_ulong = 56;
pub const SYS_setpgid: ::c_ulong = 57;
pub const SYS_ulimit: ::c_ulong = 58;
pub const SYS_oldolduname: ::c_ulong = 59;
pub const SYS_umask: ::c_ulong = 60;
pub const SYS_chroot: ::c_ulong = 61;
pub const SYS_ustat: ::c_ulong = 62;
pub const SYS_dup2: ::c_ulong = 63;
pub const SYS_getppid: ::c_ulong = 64;
pub const SYS_getpgrp: ::c_ulong = 65;
pub const SYS_setsid: ::c_ulong = 66;
pub const SYS_sigaction: ::c_ulong = 67;
pub const SYS_sgetmask: ::c_ulong = 68;
pub const SYS_ssetmask: ::c_ulong = 69;
pub const SYS_setreuid: ::c_ulong = 70;
pub const SYS_setregid: ::c_ulong = 71;
pub const SYS_sigsuspend: ::c_ulong = 72;
pub const SYS_sigpending: ::c_ulong = 73;
pub const SYS_sethostname: ::c_ulong = 74;
pub const SYS_setrlimit: ::c_ulong = 75;
pub const SYS_getrlimit: ::c_ulong = 76;
pub const SYS_getrusage: ::c_ulong = 77;
pub const SYS_gettimeofday: ::c_ulong = 78;
pub const SYS_settimeofday: ::c_ulong = 79;
pub const SYS_getgroups: ::c_ulong = 80;
pub const SYS_setgroups: ::c_ulong = 81;
pub const SYS_select: ::c_ulong = 82;
pub const SYS_symlink: ::c_ulong = 83;
pub const SYS_oldlstat: ::c_ulong = 84;
pub const SYS_readlink: ::c_ulong = 85;
pub const SYS_uselib: ::c_ulong = 86;
pub const SYS_swapon: ::c_ulong = 87;
pub const SYS_reboot: ::c_ulong = 88;
pub const SYS_readdir: ::c_ulong = 89;
pub const SYS_mmap: ::c_ulong = 90;
pub const SYS_munmap: ::c_ulong = 91;
pub const SYS_truncate: ::c_ulong = 92;
pub const SYS_ftruncate: ::c_ulong = 93;
pub const SYS_fchmod: ::c_ulong = 94;
pub const SYS_fchown: ::c_ulong = 95;
pub const SYS_getpriority: ::c_ulong = 96;
pub const SYS_setpriority: ::c_ulong = 97;
pub const SYS_profil: ::c_ulong = 98;
pub const SYS_statfs: ::c_ulong = 99;
pub const SYS_fstatfs: ::c_ulong = 100;
pub const SYS_ioperm: ::c_ulong = 101;
pub const SYS_socketcall: ::c_ulong = 102;
pub const SYS_syslog: ::c_ulong = 103;
pub const SYS_setitimer: ::c_ulong = 104;
pub const SYS_getitimer: ::c_ulong = 105;
pub const SYS_stat: ::c_ulong = 106;
pub const SYS_lstat: ::c_ulong = 107;
pub const SYS_fstat: ::c_ulong = 108;
pub const SYS_olduname: ::c_ulong = 109;
pub const SYS_iopl: ::c_ulong = 110;
pub const SYS_vhangup: ::c_ulong = 111;
pub const SYS_idle: ::c_ulong = 112;
pub const SYS_vm86: ::c_ulong = 113;
pub const SYS_wait4: ::c_ulong = 114;
pub const SYS_swapoff: ::c_ulong = 115;
pub const SYS_sysinfo: ::c_ulong = 116;
pub const SYS_ipc: ::c_ulong = 117;
pub const SYS_fsync: ::c_ulong = 118;
pub const SYS_sigreturn: ::c_ulong = 119;
pub const SYS_clone: ::c_ulong = 120;
pub const SYS_setdomainname: ::c_ulong = 121;
pub const SYS_uname: ::c_ulong = 122;
pub const SYS_modify_ldt: ::c_ulong = 123;
pub const SYS_adjtimex: ::c_ulong = 124;
pub const SYS_mprotect: ::c_ulong = 125;
pub const SYS_sigprocmask: ::c_ulong = 126;
pub const SYS_create_module: ::c_ulong = 127;
pub const SYS_init_module: ::c_ulong = 128;
pub const SYS_delete_module: ::c_ulong = 129;
pub const SYS_get_kernel_syms: ::c_ulong = 130;
pub const SYS_quotactl: ::c_ulong = 131;
pub const SYS_getpgid: ::c_ulong = 132;
pub const SYS_fchdir: ::c_ulong = 133;
pub const SYS_bdflush: ::c_ulong = 134;
pub const SYS_sysfs: ::c_ulong = 135;
pub const SYS_personality: ::c_ulong = 136;
pub const SYS_afs_syscall: ::c_ulong = 137; /* Syscall for Andrew File System */
pub const SYS_setfsuid: ::c_ulong = 138;
pub const SYS_setfsgid: ::c_ulong = 139;
pub const SYS__llseek: ::c_ulong = 140;
pub const SYS_getdents: ::c_ulong = 141;
pub const SYS__newselect: ::c_ulong = 142;
pub const SYS_flock: ::c_ulong = 143;
pub const SYS_msync: ::c_ulong = 144;
pub const SYS_readv: ::c_ulong = 145;
pub const SYS_writev: ::c_ulong = 146;
pub const SYS_getsid: ::c_ulong = 147;
pub const SYS_fdatasync: ::c_ulong = 148;
pub const SYS__sysctl: ::c_ulong = 149;
pub const SYS_mlock: ::c_ulong = 150;
pub const SYS_munlock: ::c_ulong = 151;
pub const SYS_mlockall: ::c_ulong = 152;
pub const SYS_munlockall: ::c_ulong = 153;
pub const SYS_sched_setparam: ::c_ulong = 154;
pub const SYS_sched_getparam: ::c_ulong = 155;
pub const SYS_sched_setscheduler: ::c_ulong = 156;
pub const SYS_sched_getscheduler: ::c_ulong = 157;
pub const SYS_sched_yield: ::c_ulong = 158;
pub const SYS_sched_get_priority_max: ::c_ulong = 159;
pub const SYS_sched_get_priority_min: ::c_ulong = 160;
pub const SYS_sched_rr_get_interval: ::c_ulong = 161;
pub const SYS_nanosleep: ::c_ulong = 162;
pub const SYS_mremap: ::c_ulong = 163;
pub const SYS_setresuid: ::c_ulong = 164;
pub const SYS_getresuid: ::c_ulong = 165;
pub const SYS_query_module: ::c_ulong = 166;
pub const SYS_poll: ::c_ulong = 167;
pub const SYS_nfsservctl: ::c_ulong = 168;
pub const SYS_setresgid: ::c_ulong = 169;
pub const SYS_getresgid: ::c_ulong = 170;
pub const SYS_prctl: ::c_ulong = 171;
pub const SYS_rt_sigreturn: ::c_ulong = 172;
pub const SYS_rt_sigaction: ::c_ulong = 173;
pub const SYS_rt_sigprocmask: ::c_ulong = 174;
pub const SYS_rt_sigpending: ::c_ulong = 175;
pub const SYS_rt_sigtimedwait: ::c_ulong = 176;
pub const SYS_rt_sigqueueinfo: ::c_ulong = 177;
pub const SYS_rt_sigsuspend: ::c_ulong = 178;
pub const SYS_pread64: ::c_ulong = 179;
pub const SYS_pwrite64: ::c_ulong = 180;
pub const SYS_chown: ::c_ulong = 181;
pub const SYS_getcwd: ::c_ulong = 182;
pub const SYS_capget: ::c_ulong = 183;
pub const SYS_capset: ::c_ulong = 184;
pub const SYS_sigaltstack: ::c_ulong = 185;
pub const SYS_sendfile: ::c_ulong = 186;
pub const SYS_getpmsg: ::c_ulong = 187; /* some people actually want streams */
pub const SYS_putpmsg: ::c_ulong = 188; /* some people actually want streams */
pub const SYS_vfork: ::c_ulong = 189;
pub const SYS_ugetrlimit: ::c_ulong = 190; /* SuS compliant getrlimit */
pub const SYS_readahead: ::c_ulong = 191;
pub const SYS_mmap2: ::c_ulong = 192;
pub const SYS_truncate64: ::c_ulong = 193;
pub const SYS_ftruncate64: ::c_ulong = 194;
pub const SYS_stat64: ::c_ulong = 195;
pub const SYS_lstat64: ::c_ulong = 196;
pub const SYS_fstat64: ::c_ulong = 197;
pub const SYS_pciconfig_read: ::c_ulong = 198;
pub const SYS_pciconfig_write: ::c_ulong = 199;
pub const SYS_pciconfig_iobase: ::c_ulong = 200;
pub const SYS_multiplexer: ::c_ulong = 201;
pub const SYS_getdents64: ::c_ulong = 202;
pub const SYS_pivot_root: ::c_ulong = 203;
pub const SYS_fcntl64: ::c_ulong = 204;
pub const SYS_madvise: ::c_ulong = 205;
pub const SYS_mincore: ::c_ulong = 206;
pub const SYS_gettid: ::c_ulong = 207;
pub const SYS_tkill: ::c_ulong = 208;
pub const SYS_setxattr: ::c_ulong = 209;
pub const SYS_lsetxattr: ::c_ulong = 210;
pub const SYS_fsetxattr: ::c_ulong = 211;
pub const SYS_getxattr: ::c_ulong = 212;
pub const SYS_lgetxattr: ::c_ulong = 213;
pub const SYS_fgetxattr: ::c_ulong = 214;
pub const SYS_listxattr: ::c_ulong = 215;
pub const SYS_llistxattr: ::c_ulong = 216;
pub const SYS_flistxattr: ::c_ulong = 217;
pub const SYS_removexattr: ::c_ulong = 218;
pub const SYS_lremovexattr: ::c_ulong = 219;
pub const SYS_fremovexattr: ::c_ulong = 220;
pub const SYS_futex: ::c_ulong = 221;
pub const SYS_sched_setaffinity: ::c_ulong = 222;
pub const SYS_sched_getaffinity: ::c_ulong = 223;
pub const SYS_tuxcall: ::c_ulong = 225;
pub const SYS_sendfile64: ::c_ulong = 226;
pub const SYS_io_setup: ::c_ulong = 227;
pub const SYS_io_destroy: ::c_ulong = 228;
pub const SYS_io_getevents: ::c_ulong = 229;
pub const SYS_io_submit: ::c_ulong = 230;
pub const SYS_io_cancel: ::c_ulong = 231;
pub const SYS_set_tid_address: ::c_ulong = 232;
pub const SYS_fadvise64: ::c_ulong = 233;
pub const SYS_exit_group: ::c_ulong = 234;
pub const SYS_lookup_dcookie: ::c_ulong = 235;
pub const SYS_epoll_create: ::c_ulong = 236;
pub const SYS_epoll_ctl: ::c_ulong = 237;
pub const SYS_epoll_wait: ::c_ulong = 238;
pub const SYS_remap_file_pages: ::c_ulong = 239;
pub const SYS_timer_create: ::c_ulong = 240;
pub const SYS_timer_settime: ::c_ulong = 241;
pub const SYS_timer_gettime: ::c_ulong = 242;
pub const SYS_timer_getoverrun: ::c_ulong = 243;
pub const SYS_timer_delete: ::c_ulong = 244;
pub const SYS_clock_settime: ::c_ulong = 245;
pub const SYS_clock_gettime: ::c_ulong = 246;
pub const SYS_clock_getres: ::c_ulong = 247;
pub const SYS_clock_nanosleep: ::c_ulong = 248;
pub const SYS_swapcontext: ::c_ulong = 249;
pub const SYS_tgkill: ::c_ulong = 250;
pub const SYS_utimes: ::c_ulong = 251;
pub const SYS_statfs64: ::c_ulong = 252;
pub const SYS_fstatfs64: ::c_ulong = 253;
pub const SYS_fadvise64_64: ::c_ulong = 254;
pub const SYS_rtas: ::c_ulong = 255;
pub const SYS_sys_debug_setcontext: ::c_ulong = 256;
pub const SYS_migrate_pages: ::c_ulong = 258;
pub const SYS_mbind: ::c_ulong = 259;
pub const SYS_get_mempolicy: ::c_ulong = 260;
pub const SYS_set_mempolicy: ::c_ulong = 261;
pub const SYS_mq_open: ::c_ulong = 262;
pub const SYS_mq_unlink: ::c_ulong = 263;
pub const SYS_mq_timedsend: ::c_ulong = 264;
pub const SYS_mq_timedreceive: ::c_ulong = 265;
pub const SYS_mq_notify: ::c_ulong = 266;
pub const SYS_mq_getsetattr: ::c_ulong = 267;
pub const SYS_kexec_load: ::c_ulong = 268;
pub const SYS_add_key: ::c_ulong = 269;
pub const SYS_request_key: ::c_ulong = 270;
pub const SYS_keyctl: ::c_ulong = 271;
pub const SYS_waitid: ::c_ulong = 272;
pub const SYS_ioprio_set: ::c_ulong = 273;
pub const SYS_ioprio_get: ::c_ulong = 274;
pub const SYS_inotify_init: ::c_ulong = 275;
pub const SYS_inotify_add_watch: ::c_ulong = 276;
pub const SYS_inotify_rm_watch: ::c_ulong = 277;
pub const SYS_spu_run: ::c_ulong = 278;
pub const SYS_spu_create: ::c_ulong = 279;
pub const SYS_pselect6: ::c_ulong = 280;
pub const SYS_ppoll: ::c_ulong = 281;
pub const SYS_unshare: ::c_ulong = 282;
pub const SYS_splice: ::c_ulong = 283;
pub const SYS_tee: ::c_ulong = 284;
pub const SYS_vmsplice: ::c_ulong = 285;
pub const SYS_openat: ::c_ulong = 286;
pub const SYS_mkdirat: ::c_ulong = 287;
pub const SYS_mknodat: ::c_ulong = 288;
pub const SYS_fchownat: ::c_ulong = 289;
pub const SYS_futimesat: ::c_ulong = 290;
pub const SYS_fstatat64: ::c_ulong = 291;
pub const SYS_unlinkat: ::c_ulong = 292;
pub const SYS_renameat: ::c_ulong = 293;
pub const SYS_linkat: ::c_ulong = 294;
pub const SYS_symlinkat: ::c_ulong = 295;
pub const SYS_readlinkat: ::c_ulong = 296;
pub const SYS_fchmodat: ::c_ulong = 297;
pub const SYS_faccessat: ::c_ulong = 298;
pub const SYS_get_robust_list: ::c_ulong = 299;
pub const SYS_set_robust_list: ::c_ulong = 300;
pub const SYS_move_pages: ::c_ulong = 301;
pub const SYS_getcpu: ::c_ulong = 302;
pub const SYS_epoll_pwait: ::c_ulong = 303;
pub const SYS_utimensat: ::c_ulong = 304;
pub const SYS_signalfd: ::c_ulong = 305;
pub const SYS_timerfd_create: ::c_ulong = 306;
pub const SYS_eventfd: ::c_ulong = 307;
pub const SYS_sync_file_range2: ::c_ulong = 308;
pub const SYS_fallocate: ::c_ulong = 309;
pub const SYS_subpage_prot: ::c_ulong = 310;
pub const SYS_timerfd_settime: ::c_ulong = 311;
pub const SYS_timerfd_gettime: ::c_ulong = 312;
pub const SYS_signalfd4: ::c_ulong = 313;
pub const SYS_eventfd2: ::c_ulong = 314;
pub const SYS_epoll_create1: ::c_ulong = 315;
pub const SYS_dup3: ::c_ulong = 316;
pub const SYS_pipe2: ::c_ulong = 317;
pub const SYS_inotify_init1: ::c_ulong = 318;
pub const SYS_perf_event_open: ::c_ulong = 319;
pub const SYS_preadv: ::c_ulong = 320;
pub const SYS_pwritev: ::c_ulong = 321;
pub const SYS_rt_tgsigqueueinfo: ::c_ulong = 322;
pub const SYS_fanotify_init: ::c_ulong = 323;
pub const SYS_fanotify_mark: ::c_ulong = 324;
pub const SYS_prlimit64: ::c_ulong = 325;
pub const SYS_socket: ::c_ulong = 326;
pub const SYS_bind: ::c_ulong = 327;
pub const SYS_connect: ::c_ulong = 328;
pub const SYS_listen: ::c_ulong = 329;
pub const SYS_accept: ::c_ulong = 330;
pub const SYS_getsockname: ::c_ulong = 331;
pub const SYS_getpeername: ::c_ulong = 332;
pub const SYS_socketpair: ::c_ulong = 333;
pub const SYS_send: ::c_ulong = 334;
pub const SYS_sendto: ::c_ulong = 335;
pub const SYS_recv: ::c_ulong = 336;
pub const SYS_recvfrom: ::c_ulong = 337;
pub const SYS_shutdown: ::c_ulong = 338;
pub const SYS_setsockopt: ::c_ulong = 339;
pub const SYS_getsockopt: ::c_ulong = 340;
pub const SYS_sendmsg: ::c_ulong = 341;
pub const SYS_recvmsg: ::c_ulong = 342;
pub const SYS_recvmmsg: ::c_ulong = 343;
pub const SYS_accept4: ::c_ulong = 344;
pub const SYS_name_to_handle_at: ::c_ulong = 345;
pub const SYS_open_by_handle_at: ::c_ulong = 346;
pub const SYS_clock_adjtime: ::c_ulong = 347;
pub const SYS_syncfs: ::c_ulong = 348;
pub const SYS_sendmmsg: ::c_ulong = 349;
pub const SYS_setns: ::c_ulong = 350;
pub const SYS_process_vm_readv: ::c_ulong = 351;
pub const SYS_process_vm_writev: ::c_ulong = 352;
pub const SYS_finit_module: ::c_ulong = 353;
pub const SYS_kcmp: ::c_ulong = 354;
pub const SYS_sched_setattr: ::c_ulong = 355;
pub const SYS_sched_getattr: ::c_ulong = 356;
pub const SYS_renameat2: ::c_ulong = 357;
pub const SYS_seccomp: ::c_ulong = 358;
pub const SYS_getrandom: ::c_ulong = 359;
pub const SYS_memfd_create: ::c_ulong = 360;
pub const SYS_bpf: ::c_ulong = 361;
pub const SYS_execveat: ::c_ulong = 362;
pub const SYS_switch_endian: ::c_ulong = 363;
pub const SYS_userfaultfd: ::c_ulong = 364;
pub const SYS_membarrier: ::c_ulong = 365;
pub const SYS_mlock2: ::c_ulong = 378;
pub const SYS_copy_file_range: ::c_ulong = 379;
pub const SYS_preadv2: ::c_ulong = 380;
pub const SYS_pwritev2: ::c_ulong = 381;
pub const SYS_kexec_file_load: ::c_ulong = 382;
