pub type c_char = u8;
pub type __u64 = ::c_ulonglong;
pub type wchar_t = u32;
pub type nlink_t = u32;
pub type blksize_t = ::c_int;

s! {
    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad0: ::c_ulong,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        __pad1: ::c_int,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_uint; 2],
    }

    pub struct stat64 {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        __pad0: ::c_ulong,
        pub st_size: ::off_t,
        pub st_blksize: ::blksize_t,
        __pad1: ::c_int,
        pub st_blocks: ::blkcnt_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        __unused: [::c_uint; 2],
    }

    pub struct ipc_perm {
        pub __ipc_perm_key: ::key_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub mode: ::mode_t,
        pub __seq: ::c_ushort,
        __unused1: ::c_ulong,
        __unused2: ::c_ulong,
    }
}

pub const O_DIRECT: ::c_int = 0x10000;
pub const O_DIRECTORY: ::c_int = 0x4000;
pub const O_LARGEFILE: ::c_int = 0x20000;
pub const O_NOFOLLOW: ::c_int = 0x8000;

pub const MINSIGSTKSZ: ::size_t = 6144;
pub const SIGSTKSZ: ::size_t = 12288;

#[doc(hidden)]
pub const PF_MAX: ::c_int = 43;
#[doc(hidden)]
pub const AF_MAX: ::c_int = PF_MAX;

pub const SYS_io_setup: ::c_ulong = 0;
pub const SYS_io_destroy: ::c_ulong = 1;
pub const SYS_io_submit: ::c_ulong = 2;
pub const SYS_io_cancel: ::c_ulong = 3;
pub const SYS_io_getevents: ::c_ulong = 4;
pub const SYS_setxattr: ::c_ulong = 5;
pub const SYS_lsetxattr: ::c_ulong = 6;
pub const SYS_fsetxattr: ::c_ulong = 7;
pub const SYS_getxattr: ::c_ulong = 8;
pub const SYS_lgetxattr: ::c_ulong = 9;
pub const SYS_fgetxattr: ::c_ulong = 10;
pub const SYS_listxattr: ::c_ulong = 11;
pub const SYS_llistxattr: ::c_ulong = 12;
pub const SYS_flistxattr: ::c_ulong = 13;
pub const SYS_removexattr: ::c_ulong = 14;
pub const SYS_lremovexattr: ::c_ulong = 15;
pub const SYS_fremovexattr: ::c_ulong = 16;
pub const SYS_getcwd: ::c_ulong = 17;
pub const SYS_lookup_dcookie: ::c_ulong = 18;
pub const SYS_eventfd2: ::c_ulong = 19;
pub const SYS_epoll_create1: ::c_ulong = 20;
pub const SYS_epoll_ctl: ::c_ulong = 21;
pub const SYS_epoll_pwait: ::c_ulong = 22;
pub const SYS_dup: ::c_ulong = 23;
pub const SYS_dup3: ::c_ulong = 24;
pub const SYS_inotify_init1: ::c_ulong = 26;
pub const SYS_inotify_add_watch: ::c_ulong = 27;
pub const SYS_inotify_rm_watch: ::c_ulong = 28;
pub const SYS_ioctl: ::c_ulong = 29;
pub const SYS_ioprio_set: ::c_ulong = 30;
pub const SYS_ioprio_get: ::c_ulong = 31;
pub const SYS_flock: ::c_ulong = 32;
pub const SYS_mknodat: ::c_ulong = 33;
pub const SYS_mkdirat: ::c_ulong = 34;
pub const SYS_unlinkat: ::c_ulong = 35;
pub const SYS_symlinkat: ::c_ulong = 36;
pub const SYS_linkat: ::c_ulong = 37;
pub const SYS_renameat: ::c_ulong = 38;
pub const SYS_umount2: ::c_ulong = 39;
pub const SYS_mount: ::c_ulong = 40;
pub const SYS_pivot_root: ::c_ulong = 41;
pub const SYS_nfsservctl: ::c_ulong = 42;
pub const SYS_fallocate: ::c_ulong = 47;
pub const SYS_faccessat: ::c_ulong = 48;
pub const SYS_chdir: ::c_ulong = 49;
pub const SYS_fchdir: ::c_ulong = 50;
pub const SYS_chroot: ::c_ulong = 51;
pub const SYS_fchmod: ::c_ulong = 52;
pub const SYS_fchmodat: ::c_ulong = 53;
pub const SYS_fchownat: ::c_ulong = 54;
pub const SYS_fchown: ::c_ulong = 55;
pub const SYS_openat: ::c_ulong = 56;
pub const SYS_close: ::c_ulong = 57;
pub const SYS_vhangup: ::c_ulong = 58;
pub const SYS_pipe2: ::c_ulong = 59;
pub const SYS_quotactl: ::c_ulong = 60;
pub const SYS_getdents64: ::c_ulong = 61;
pub const SYS_read: ::c_ulong = 63;
pub const SYS_write: ::c_ulong = 64;
pub const SYS_readv: ::c_ulong = 65;
pub const SYS_writev: ::c_ulong = 66;
pub const SYS_pread64: ::c_ulong = 67;
pub const SYS_pwrite64: ::c_ulong = 68;
pub const SYS_preadv: ::c_ulong = 69;
pub const SYS_pwritev: ::c_ulong = 70;
pub const SYS_pselect6: ::c_ulong = 72;
pub const SYS_ppoll: ::c_ulong = 73;
pub const SYS_signalfd4: ::c_ulong = 74;
pub const SYS_vmsplice: ::c_ulong = 75;
pub const SYS_splice: ::c_ulong = 76;
pub const SYS_tee: ::c_ulong = 77;
pub const SYS_readlinkat: ::c_ulong = 78;
pub const SYS_sync: ::c_ulong = 81;
pub const SYS_fsync: ::c_ulong = 82;
pub const SYS_fdatasync: ::c_ulong = 83;
pub const SYS_sync_file_range: ::c_ulong = 84;
pub const SYS_timerfd_create: ::c_ulong = 85;
pub const SYS_timerfd_settime: ::c_ulong = 86;
pub const SYS_timerfd_gettime: ::c_ulong = 87;
pub const SYS_utimensat: ::c_ulong = 88;
pub const SYS_acct: ::c_ulong = 89;
pub const SYS_capget: ::c_ulong = 90;
pub const SYS_capset: ::c_ulong = 91;
pub const SYS_personality: ::c_ulong = 92;
pub const SYS_exit: ::c_ulong = 93;
pub const SYS_exit_group: ::c_ulong = 94;
pub const SYS_waitid: ::c_ulong = 95;
pub const SYS_set_tid_address: ::c_ulong = 96;
pub const SYS_unshare: ::c_ulong = 97;
pub const SYS_futex: ::c_ulong = 98;
pub const SYS_set_robust_list: ::c_ulong = 99;
pub const SYS_get_robust_list: ::c_ulong = 100;
pub const SYS_nanosleep: ::c_ulong = 101;
pub const SYS_getitimer: ::c_ulong = 102;
pub const SYS_setitimer: ::c_ulong = 103;
pub const SYS_kexec_load: ::c_ulong = 104;
pub const SYS_init_module: ::c_ulong = 105;
pub const SYS_delete_module: ::c_ulong = 106;
pub const SYS_timer_create: ::c_ulong = 107;
pub const SYS_timer_gettime: ::c_ulong = 108;
pub const SYS_timer_getoverrun: ::c_ulong = 109;
pub const SYS_timer_settime: ::c_ulong = 110;
pub const SYS_timer_delete: ::c_ulong = 111;
pub const SYS_clock_settime: ::c_ulong = 112;
pub const SYS_clock_gettime: ::c_ulong = 113;
pub const SYS_clock_getres: ::c_ulong = 114;
pub const SYS_clock_nanosleep: ::c_ulong = 115;
pub const SYS_syslog: ::c_ulong = 116;
pub const SYS_ptrace: ::c_ulong = 117;
pub const SYS_sched_setparam: ::c_ulong = 118;
pub const SYS_sched_setscheduler: ::c_ulong = 119;
pub const SYS_sched_getscheduler: ::c_ulong = 120;
pub const SYS_sched_getparam: ::c_ulong = 121;
pub const SYS_sched_setaffinity: ::c_ulong = 122;
pub const SYS_sched_getaffinity: ::c_ulong = 123;
pub const SYS_sched_yield: ::c_ulong = 124;
pub const SYS_sched_get_priority_max: ::c_ulong = 125;
pub const SYS_sched_get_priority_min: ::c_ulong = 126;
pub const SYS_sched_rr_get_interval: ::c_ulong = 127;
pub const SYS_restart_syscall: ::c_ulong = 128;
pub const SYS_kill: ::c_ulong = 129;
pub const SYS_tkill: ::c_ulong = 130;
pub const SYS_tgkill: ::c_ulong = 131;
pub const SYS_sigaltstack: ::c_ulong = 132;
pub const SYS_rt_sigsuspend: ::c_ulong = 133;
pub const SYS_rt_sigaction: ::c_ulong = 134;
pub const SYS_rt_sigprocmask: ::c_ulong = 135;
pub const SYS_rt_sigpending: ::c_ulong = 136;
pub const SYS_rt_sigtimedwait: ::c_ulong = 137;
pub const SYS_rt_sigqueueinfo: ::c_ulong = 138;
pub const SYS_rt_sigreturn: ::c_ulong = 139;
pub const SYS_setpriority: ::c_ulong = 140;
pub const SYS_getpriority: ::c_ulong = 141;
pub const SYS_reboot: ::c_ulong = 142;
pub const SYS_setregid: ::c_ulong = 143;
pub const SYS_setgid: ::c_ulong = 144;
pub const SYS_setreuid: ::c_ulong = 145;
pub const SYS_setuid: ::c_ulong = 146;
pub const SYS_setresuid: ::c_ulong = 147;
pub const SYS_getresuid: ::c_ulong = 148;
pub const SYS_setresgid: ::c_ulong = 149;
pub const SYS_getresgid: ::c_ulong = 150;
pub const SYS_setfsuid: ::c_ulong = 151;
pub const SYS_setfsgid: ::c_ulong = 152;
pub const SYS_times: ::c_ulong = 153;
pub const SYS_setpgid: ::c_ulong = 154;
pub const SYS_getpgid: ::c_ulong = 155;
pub const SYS_getsid: ::c_ulong = 156;
pub const SYS_setsid: ::c_ulong = 157;
pub const SYS_getgroups: ::c_ulong = 158;
pub const SYS_setgroups: ::c_ulong = 159;
pub const SYS_uname: ::c_ulong = 160;
pub const SYS_sethostname: ::c_ulong = 161;
pub const SYS_setdomainname: ::c_ulong = 162;
pub const SYS_getrlimit: ::c_ulong = 163;
pub const SYS_setrlimit: ::c_ulong = 164;
pub const SYS_getrusage: ::c_ulong = 165;
pub const SYS_umask: ::c_ulong = 166;
pub const SYS_prctl: ::c_ulong = 167;
pub const SYS_getcpu: ::c_ulong = 168;
pub const SYS_gettimeofday: ::c_ulong = 169;
pub const SYS_settimeofday: ::c_ulong = 170;
pub const SYS_adjtimex: ::c_ulong = 171;
pub const SYS_getpid: ::c_ulong = 172;
pub const SYS_getppid: ::c_ulong = 173;
pub const SYS_getuid: ::c_ulong = 174;
pub const SYS_geteuid: ::c_ulong = 175;
pub const SYS_getgid: ::c_ulong = 176;
pub const SYS_getegid: ::c_ulong = 177;
pub const SYS_gettid: ::c_ulong = 178;
pub const SYS_sysinfo: ::c_ulong = 179;
pub const SYS_mq_open: ::c_ulong = 180;
pub const SYS_mq_unlink: ::c_ulong = 181;
pub const SYS_mq_timedsend: ::c_ulong = 182;
pub const SYS_mq_timedreceive: ::c_ulong = 183;
pub const SYS_mq_notify: ::c_ulong = 184;
pub const SYS_mq_getsetattr: ::c_ulong = 185;
pub const SYS_msgget: ::c_ulong = 186;
pub const SYS_msgctl: ::c_ulong = 187;
pub const SYS_msgrcv: ::c_ulong = 188;
pub const SYS_msgsnd: ::c_ulong = 189;
pub const SYS_semget: ::c_ulong = 190;
pub const SYS_semctl: ::c_ulong = 191;
pub const SYS_semtimedop: ::c_ulong = 192;
pub const SYS_semop: ::c_ulong = 193;
pub const SYS_shmget: ::c_ulong = 194;
pub const SYS_shmctl: ::c_ulong = 195;
pub const SYS_shmat: ::c_ulong = 196;
pub const SYS_shmdt: ::c_ulong = 197;
pub const SYS_socket: ::c_ulong = 198;
pub const SYS_socketpair: ::c_ulong = 199;
pub const SYS_bind: ::c_ulong = 200;
pub const SYS_listen: ::c_ulong = 201;
pub const SYS_accept: ::c_ulong = 202;
pub const SYS_connect: ::c_ulong = 203;
pub const SYS_getsockname: ::c_ulong = 204;
pub const SYS_getpeername: ::c_ulong = 205;
pub const SYS_sendto: ::c_ulong = 206;
pub const SYS_recvfrom: ::c_ulong = 207;
pub const SYS_setsockopt: ::c_ulong = 208;
pub const SYS_getsockopt: ::c_ulong = 209;
pub const SYS_shutdown: ::c_ulong = 210;
pub const SYS_sendmsg: ::c_ulong = 211;
pub const SYS_recvmsg: ::c_ulong = 212;
pub const SYS_readahead: ::c_ulong = 213;
pub const SYS_brk: ::c_ulong = 214;
pub const SYS_munmap: ::c_ulong = 215;
pub const SYS_mremap: ::c_ulong = 216;
pub const SYS_add_key: ::c_ulong = 217;
pub const SYS_request_key: ::c_ulong = 218;
pub const SYS_keyctl: ::c_ulong = 219;
pub const SYS_clone: ::c_ulong = 220;
pub const SYS_execve: ::c_ulong = 221;
pub const SYS_swapon: ::c_ulong = 224;
pub const SYS_swapoff: ::c_ulong = 225;
pub const SYS_mprotect: ::c_ulong = 226;
pub const SYS_msync: ::c_ulong = 227;
pub const SYS_mlock: ::c_ulong = 228;
pub const SYS_munlock: ::c_ulong = 229;
pub const SYS_mlockall: ::c_ulong = 230;
pub const SYS_munlockall: ::c_ulong = 231;
pub const SYS_mincore: ::c_ulong = 232;
pub const SYS_madvise: ::c_ulong = 233;
pub const SYS_remap_file_pages: ::c_ulong = 234;
pub const SYS_mbind: ::c_ulong = 235;
pub const SYS_get_mempolicy: ::c_ulong = 236;
pub const SYS_set_mempolicy: ::c_ulong = 237;
pub const SYS_migrate_pages: ::c_ulong = 238;
pub const SYS_move_pages: ::c_ulong = 239;
pub const SYS_rt_tgsigqueueinfo: ::c_ulong = 240;
pub const SYS_perf_event_open: ::c_ulong = 241;
pub const SYS_accept4: ::c_ulong = 242;
pub const SYS_recvmmsg: ::c_ulong = 243;
pub const SYS_wait4: ::c_ulong = 260;
pub const SYS_prlimit64: ::c_ulong = 261;
pub const SYS_fanotify_init: ::c_ulong = 262;
pub const SYS_fanotify_mark: ::c_ulong = 263;
pub const SYS_name_to_handle_at: ::c_ulong = 264;
pub const SYS_open_by_handle_at: ::c_ulong = 265;
pub const SYS_clock_adjtime: ::c_ulong = 266;
pub const SYS_syncfs: ::c_ulong = 267;
pub const SYS_setns: ::c_ulong = 268;
pub const SYS_sendmmsg: ::c_ulong = 269;
pub const SYS_process_vm_readv: ::c_ulong = 270;
pub const SYS_process_vm_writev: ::c_ulong = 271;
pub const SYS_kcmp: ::c_ulong = 272;
pub const SYS_finit_module: ::c_ulong = 273;
pub const SYS_sched_setattr: ::c_ulong = 274;
pub const SYS_sched_getattr: ::c_ulong = 275;
pub const SYS_renameat2: ::c_ulong = 276;
pub const SYS_seccomp: ::c_ulong = 277;
pub const SYS_getrandom: ::c_ulong = 278;
pub const SYS_memfd_create: ::c_ulong = 279;
pub const SYS_bpf: ::c_ulong = 280;
pub const SYS_execveat: ::c_ulong = 281;
pub const SYS_userfaultfd: ::c_ulong = 282;
pub const SYS_membarrier: ::c_ulong = 283;
pub const SYS_mlock2: ::c_ulong = 284;
pub const SYS_copy_file_range: ::c_ulong = 285;
pub const SYS_preadv2: ::c_ulong = 286;
pub const SYS_pwritev2: ::c_ulong = 287;
pub const SYS_pkey_mprotect: ::c_ulong = 288;
pub const SYS_pkey_alloc: ::c_ulong = 289;
pub const SYS_pkey_free: ::c_ulong = 290;
