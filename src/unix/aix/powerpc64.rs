pub type c_long = i64;
pub type c_ulong = u64;

s! {
    pub struct sigset_t {
        pub ss_set: [c_ulong; 4],
    }

    pub struct fd_set {
        pub fds_bits: [c_long; 1024],
    }

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_sysid: ::c_uint,
        pub l_pid: ::pid_t,
        pub l_vfs: ::c_int,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
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
        pub f_fsid: ::c_ulong,
        pub f_basetype: [::c_char; 16],
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
        pub f_fstr: [::c_char; 32],
        pub f_filler: [::c_ulong; 16]
    }

    pub struct pthread_rwlock_t {
        __rw_word: [::c_long; 10],
    }

    pub struct pthread_cond_t {
        __cv_word: [::c_long; 6],
    }

    pub struct pthread_mutex_t {
        __mt_word: [::c_long; 8],
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_flag: ::c_ushort,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_ssize: ::c_int,
        pub st_atime: ::st_timespec,
        pub st_mtime: ::st_timespec,
        pub st_ctime: ::st_timespec,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_vfstype: ::c_int,
        pub st_vfs: ::c_uint,
        pub st_type: ::c_uint,
        pub st_gen: ::c_uint,
        pub st_reserved: [::c_uint; 9],
        pub st_padto_ll: ::c_uint,
        pub st_size: ::off_t,
    }

    pub struct statfs {
        pub f_version: ::c_int,
        pub f_type: ::c_int,
        pub f_bsize: ::c_ulong,
        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_files: ::fsblkcnt_t,
        pub f_ffree: ::fsblkcnt_t,
        pub f_fsid: ::fsid64_t,
        pub f_vfstype: ::c_int,
        pub f_fsize: ::c_ulong,
        pub f_vfsnumber: ::c_int,
        pub f_vfsoff: ::c_int,
        pub f_vfslen: ::c_int,
        pub f_vfsvers: ::c_int,
        pub f_fname: [::c_char; 32],
        pub f_fpack: [::c_char; 32],
        pub f_name_max: ::c_int,
    }

    pub struct aiocb {
        pub aio_lio_opcode: ::c_int,
        pub aio_fildes: ::c_int,
        pub aio_word1: ::c_int,
        pub aio_offset: ::off_t,
        pub aio_buf: *mut ::c_void,
        pub aio_return: ::ssize_t,
        pub aio_errno: ::c_int,
        pub aio_nbytes: ::size_t,
        pub aio_reqprio: ::c_int,
        pub aio_sigevent: ::sigevent,
        pub aio_word2: ::c_int,
        pub aio_fp: ::c_int,
        pub aio_handle: *mut aiocb,
        pub aio_reserved: [::c_uint; 2],
        pub aio_sigev_tid: c_long,
    }

    pub struct ucontext_t {
        pub __sc_onstack: ::c_int,
        pub uc_sigmask: ::sigset_t,
        pub __sc_uerror: ::c_int,
        pub uc_mcontext: ::mcontext_t,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: ::stack_t,
        // Should be pointer to __extctx_t
        pub __extctx: *mut ::c_void,
        pub __extctx_magic: ::c_int,
        pub __pad: [::c_int; 1],
    }

    pub struct mcontext_t {
        pub gpr: [::c_ulonglong; 32],
        pub msr: ::c_ulonglong,
        pub iar: ::c_ulonglong,
        pub lr: ::c_ulonglong,
        pub ctr: ::c_ulonglong,
        pub cr: ::c_uint,
        pub xer: ::c_uint,
        pub fpscr: ::c_uint,
        pub fpscrx: ::c_uint,
        pub except: [::c_ulonglong; 1],
        // Should be array of double type
        pub fpr: [::uint64_t; 32],
        pub fpeu: ::c_char,
        pub fpinfo: ::c_char,
        pub fpscr24_31: ::c_char,
        pub pad: [::c_char; 1],
        pub excp_type: ::c_int,
    }

    pub struct utmpx {
        pub ut_user: [::c_char; 256],
        pub ut_id: [::c_char; 14],
        pub ut_line: [::c_char; 64],
        pub ut_pid: ::pid_t,
        pub ut_type: ::c_short,
        pub ut_tv: ::timeval,
        pub ut_host: [::c_char; 256],
        pub __dbl_word_pad: ::c_int,
        pub __reservedA: [::c_int; 2],
        pub __reservedV: [::c_int; 6],
    }

    pub struct pthread_spinlock_t {
        pub __sp_word: [::c_long; 3],
    }

    pub struct pthread_barrier_t {
        pub __br_word: [::c_long; 5],
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        pub msg_first: ::c_uint,
        pub msg_last: ::c_uint,
        pub msg_cbytes: ::c_uint,
        pub msg_qnum: ::c_uint,
        pub msg_qbytes: ::c_ulong,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        pub msg_stime: ::time_t,
        pub msg_rtime: ::time_t,
        pub msg_ctime: ::time_t,
        pub msg_rwait: ::c_int,
        pub msg_wwait: ::c_int,
        pub msg_reqevents: ::c_ushort,
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_errno: ::c_int,
        pub si_code: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_status: ::c_int,
        pub si_addr: *mut ::c_void,
        pub si_band: ::c_long,
        pub si_value: ::sigval,
        pub __si_flags: ::c_int,
        pub __pad: [::c_int; 3],
    }

    pub union _kernel_simple_lock {
        pub _slock: ::c_long,
        // Should be pointer to 'lock_data_instrumented'
        pub _slockp: *mut ::c_void,
    }

    pub struct fileops_t {
        pub fo_rw: extern fn(file: *mut file, rw: ::uio_rw, io: *mut ::c_void, ext: ::c_long,
                             secattr: *mut ::c_void) -> ::c_int,
        pub fo_ioctl: extern fn(file: *mut file, a: ::c_long, b: ::caddr_t, c: ::c_long,
                                d: ::c_long) -> ::c_int,
        pub fo_select: extern fn(file: *mut file, a: ::c_int, b: *mut ::c_ushort,
                                 c: extern fn()) -> ::c_int,
        pub fo_close: extern fn(file: *mut file) -> ::c_int,
        pub fo_fstat: extern fn(file: *mut file, sstat: *mut ::stat) -> ::c_int,
    }

    pub struct file {
        pub f_flag: ::c_long,
        pub f_count: ::c_int,
        pub f_options: ::c_short,
        pub f_type: ::c_short,
        // Should be pointer to 'vnode'
        pub f_data: *mut ::c_void,
        pub f_offset: ::c_longlong,
        pub f_dir_off: ::c_long,
        // Should be pointer to 'cred'
        pub f_cred: *mut ::c_void,
        pub f_lock: _kernel_simple_lock,
        pub f_offset_lock: _kernel_simple_lock,
        pub f_vinfo: ::caddr_t,
        pub f_ops: *mut fileops_t,
        pub f_parentp: ::caddr_t,
        pub f_fnamep: ::caddr_t,
        pub f_fdata: [::c_char; 160],
    }

    pub union __ld_info_file {
        pub _ldinfo_fd: ::c_int,
        pub _ldinfo_fp: *mut file,
        pub _core_offset: ::c_long,
    }

    pub struct ld_info {
        pub ldinfo_next: ::c_uint,
        pub ldinfo_flags: ::c_uint,
        pub _file: __ld_info_file,
        pub ldinfo_textorg: *mut ::c_void,
        pub ldinfo_textsize: ::c_ulong,
        pub ldinfo_dataorg: *mut ::c_void,
        pub ldinfo_datasize: ::c_ulong,
        pub ldinfo_filename: [::c_char; 2],
    }

    pub union __pollfd_ext_u {
        pub addr: *mut ::c_void,
        pub data32: u32,
        pub data: u64,
    }

    pub struct pollfd_ext {
        pub fd: ::c_int,
        pub events: ::c_ushort,
        pub revents: ::c_ushort,
        pub data: __pollfd_ext_u,
    }
}

impl siginfo_t {
    pub unsafe fn si_addr(&self) -> *mut ::c_void {
        self.si_addr
    }

    pub unsafe fn si_value(&self) -> ::sigval {
        self.si_value
    }

    pub unsafe fn si_pid(&self) -> ::pid_t {
        self.si_pid
    }

    pub unsafe fn si_uid(&self) -> ::uid_t {
        self.si_uid
    }

    pub unsafe fn si_status(&self) -> ::c_int {
        self.si_status
    }
}

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __mt_word: [0, 2, 0, 0, 0, 0, 0, 0],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __cv_word: [0, 0, 0, 0, 2, 0],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __rw_word: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};
pub const RLIM_INFINITY: ::c_ulong = 0x7fffffffffffffff;

extern "C" {
    pub fn getsystemcfg(label: ::c_int) -> ::c_ulong;
}
