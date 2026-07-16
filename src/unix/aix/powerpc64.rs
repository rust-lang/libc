use crate::prelude::*;

pub type simple_lock_data = c_int;
pub type complex_lock_status = c_int;
pub type tid_t = c_long;

extern_ty! {
    /// This is meant to be the `file` type upstream under `sys/ldr.h`. We
    /// currently expose the kernel definition but that is slated for removal.
    /// This opaque type will then be renamed to `file`.
    pub type _file;
}

s! {
    pub struct lock_data_instrumented {
        lock_control_word: __c_anonymous_lock_data_instrumented_lock_control_word,
        li_flags: c_uint,
        reserved: Padding<[c_int; 1]>,
        _lockname: __c_anonymous_lock_data_instrumented__lockname,

        #[cfg(debug_assertions)]
        lock_lr: c_int,
        #[cfg(debug_assertions)]
        unlock_lr: c_int,
        #[cfg(debug_assertions)]
        lock_caller: tid_t,
        #[cfg(debug_assertions)]
        unlock_caller: tid_t,
        #[cfg(debug_assertions)]
        lock_cpuid: c_int,
        #[cfg(debug_assertions)]
        dbg_zero: c_int,
        #[cfg(debug_assertions)]
        unlock_cpuid: c_int,
        #[cfg(debug_assertions)]
        dbg_flags: c_int,
    }

    pub struct __c_anonymous__lockname__lock_id {
        _id: c_uint,
        ocurrence: c_uint,
    }

    pub struct complex_lock_data {
        status: complex_lock_status,
        flags: c_short,
        recursion_depth: c_short,
        reserved: c_uint,
    }

    pub struct drw_lock_data {
        status: complex_lock_status,
    }

    pub struct sigset_t {
        pub ss_set: [c_ulong; 4],
    }

    pub struct fd_set {
        pub fds_bits: [c_long; 1024],
    }

    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_sysid: c_uint,
        pub l_pid: crate::pid_t,
        pub l_vfs: c_int,
        pub l_start: crate::off_t,
        pub l_len: crate::off_t,
    }

    pub struct statvfs {
        pub f_bsize: c_ulong,
        pub f_frsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsfilcnt_t,
        pub f_ffree: crate::fsfilcnt_t,
        pub f_favail: crate::fsfilcnt_t,
        pub f_fsid: crate::fsid_t,
        pub f_basetype: [c_char; 16],
        pub f_flag: c_ulong,
        pub f_namemax: c_ulong,
        pub f_fstr: [c_char; 32],
        pub f_filler: [c_ulong; 16],
    }

    pub struct pthread_rwlock_t {
        __rw_word: [c_long; 10],
    }

    pub struct pthread_cond_t {
        __cv_word: [c_long; 6],
    }

    pub struct pthread_mutex_t {
        __mt_word: [c_long; 8],
    }

    pub struct pthread_once_t {
        __on_word: [c_long; 9],
    }

    pub struct stat {
        pub st_dev: crate::dev_t,
        pub st_ino: crate::ino_t,
        pub st_mode: crate::mode_t,
        pub st_nlink: crate::nlink_t,
        pub st_flag: c_ushort,
        pub st_uid: crate::uid_t,
        pub st_gid: crate::gid_t,
        pub st_rdev: crate::dev_t,
        pub st_ssize: c_int,
        pub st_atim: crate::st_timespec,
        pub st_mtim: crate::st_timespec,
        pub st_ctim: crate::st_timespec,
        pub st_blksize: crate::blksize_t,
        pub st_blocks: crate::blkcnt_t,
        pub st_vfstype: c_int,
        pub st_vfs: c_uint,
        pub st_type: c_uint,
        pub st_gen: c_uint,
        st_reserved: Padding<[c_uint; 9]>,
        st_padto_ll: Padding<c_uint>,
        pub st_size: crate::off_t,
    }

    pub struct statfs {
        pub f_version: c_int,
        pub f_type: c_int,
        pub f_bsize: c_ulong,
        pub f_blocks: crate::fsblkcnt_t,
        pub f_bfree: crate::fsblkcnt_t,
        pub f_bavail: crate::fsblkcnt_t,
        pub f_files: crate::fsblkcnt_t,
        pub f_ffree: crate::fsblkcnt_t,
        pub f_fsid: crate::fsid64_t,
        pub f_vfstype: c_int,
        pub f_fsize: c_ulong,
        pub f_vfsnumber: c_int,
        pub f_vfsoff: c_int,
        pub f_vfslen: c_int,
        pub f_vfsvers: c_int,
        pub f_fname: [c_char; 32],
        pub f_fpack: [c_char; 32],
        pub f_name_max: c_int,
    }

    pub struct aiocb {
        pub aio_lio_opcode: c_int,
        pub aio_fildes: c_int,
        pub aio_word1: c_int,
        pub aio_offset: crate::off_t,
        pub aio_buf: *mut c_void,
        pub aio_return: ssize_t,
        pub aio_errno: c_int,
        pub aio_nbytes: size_t,
        pub aio_reqprio: c_int,
        pub aio_sigevent: crate::sigevent,
        pub aio_word2: c_int,
        pub aio_fp: c_int,
        pub aio_handle: *mut aiocb,
        aio_reserved: Padding<[c_uint; 2]>,
        pub aio_sigev_tid: c_long,
    }

    pub struct __vmxreg_t {
        __v: [c_uint; 4],
    }

    pub struct __vmx_context_t {
        pub __vr: [__vmxreg_t; 32],
        __pad1: Padding<[c_uint; 3]>,
        pub __vscr: c_uint,
        pub __vrsave: c_uint,
        __pad2: Padding<[c_uint; 3]>,
    }

    pub struct __vsx_context_t {
        pub __vsr_dw1: [c_ulonglong; 32],
    }

    pub struct __tm_context_t {
        pub vmx: __vmx_context_t,
        pub vsx: __vsx_context_t,
        pub gpr: [c_ulonglong; 32],
        pub lr: c_ulonglong,
        pub ctr: c_ulonglong,
        pub cr: c_uint,
        pub xer: c_uint,
        pub amr: c_ulonglong,
        pub texasr: c_ulonglong,
        pub tfiar: c_ulonglong,
        pub tfhar: c_ulonglong,
        pub ppr: c_ulonglong,
        pub dscr: c_ulonglong,
        pub tar: c_ulonglong,
        pub fpscr: c_uint,
        pub fpscrx: c_uint,
        pub fpr: [fpreg_t; 32],
        pub tmcontext: c_char,
        pub tmstate: c_char,
        pub prevowner: c_char,
        pad: Padding<[c_char; 5]>,
    }

    pub struct __context64 {
        pub gpr: [c_ulonglong; 32],
        pub msr: c_ulonglong,
        pub iar: c_ulonglong,
        pub lr: c_ulonglong,
        pub ctr: c_ulonglong,
        pub cr: c_uint,
        pub xer: c_uint,
        pub fpscr: c_uint,
        pub fpscrx: c_uint,
        pub except: [c_ulonglong; 1],
        pub fpr: [fpreg_t; 32],
        pub fpeu: c_char,
        pub fpinfo: c_char,
        pub fpscr24_31: c_char,
        pad: Padding<[c_char; 1]>,
        pub excp_type: c_int,
    }

    pub struct mcontext_t {
        pub jmp_context: __context64,
    }

    pub struct __extctx_t {
        pub __flags: c_uint,
        pub __rsvd1: [c_uint; 3],
        pub __vmx: __vmx_context_t,
        pub __ukeys: [c_uint; 2],
        pub __vsx: __vsx_context_t,
        pub __tm: __tm_context_t,
        __reserved: Padding<[c_char; 1860]>,
        pub __extctx_magic: c_int,
    }

    pub struct ucontext_t {
        __sc_onstack: c_int,
        pub uc_sigmask: sigset_t,
        __sc_uerror: c_int,
        pub uc_mcontext: mcontext_t,
        pub uc_link: *mut ucontext_t,
        pub uc_stack: crate::stack_t,
        __extctx: *mut __extctx_t,
        __extctx_magic: c_int,
        __pad: Padding<[c_int; 1]>,
    }

    pub struct utmpx {
        pub ut_user: [c_char; 256],
        pub ut_id: [c_char; 14],
        pub ut_line: [c_char; 64],
        pub ut_pid: crate::pid_t,
        pub ut_type: c_short,
        pub ut_tv: crate::timeval,
        pub ut_host: [c_char; 256],
        __dbl_word_pad: Padding<c_int>,
        __reservedA: Padding<[c_int; 2]>,
        __reservedV: Padding<[c_int; 6]>,
    }

    pub struct pthread_spinlock_t {
        __sp_word: [c_long; 3],
    }

    pub struct pthread_barrier_t {
        __br_word: [c_long; 5],
    }

    pub struct msqid_ds {
        pub msg_perm: crate::ipc_perm,
        pub msg_first: c_uint,
        pub msg_last: c_uint,
        pub msg_cbytes: c_uint,
        pub msg_qnum: c_uint,
        pub msg_qbytes: c_ulong,
        pub msg_lspid: crate::pid_t,
        pub msg_lrpid: crate::pid_t,
        pub msg_stime: crate::time_t,
        pub msg_rtime: crate::time_t,
        pub msg_ctime: crate::time_t,
        pub msg_rwait: c_int,
        pub msg_wwait: c_int,
        pub msg_reqevents: c_ushort,
    }

    pub struct siginfo_t {
        pub si_signo: c_int,
        pub si_errno: c_int,
        pub si_code: c_int,
        pub si_pid: crate::pid_t,
        pub si_uid: crate::uid_t,
        pub si_status: c_int,
        pub si_addr: *mut c_void,
        pub si_band: c_long,
        pub si_value: crate::sigval,
        __si_flags: c_int,
        __pad: Padding<[c_int; 3]>,
    }

    pub struct pollfd_ext_t {
        pub fd: c_int,
        pub events: c_short,
        pub revents: c_short,
        pub u: __c_anonymous_pollfd_ext_t_u,
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

s_no_extra_traits! {
    pub union _simple_lock {
        _slock: simple_lock_data,
        _slockp: *mut lock_data_instrumented,
    }

    pub union _complex_lock {
        _clock: complex_lock_data,
        clockp: *mut lock_data_instrumented,
    }

    pub union _drw_lock {
        _drwlock: complex_lock_status,
        _drwlockp: *mut lock_data_instrumented,
    }

    pub union __c_anonymous_lock_data_instrumented_lock_control_word {
        s_lock: simple_lock_data,
        c_lock: complex_lock_data,
        drw_lock: drw_lock_data,
        lock_next: *mut lock_data_instrumented,
    }

    pub union __c_anonymous_lock_data_instrumented__lockname {
        name: c_long,
        _lock_id: __c_anonymous__lockname__lock_id,
    }

    #[deprecated(
        since = "0.2.187",
        note = "Use `_simple_lock` instead. This type doesn't exist upstream."
    )]
    pub union _kernel_simple_lock {
        pub _slock: c_long,
        pub _slockp: *mut lock_data_instrumented,
    }

    #[deprecated(
        since = "0.2.187",
        note = "This type is only available when programming against the kernel."
    )]
    #[allow(deprecated)]
    pub struct fileops_t {
        pub fo_rw: Option<
            extern "C" fn(
                file: *mut file,
                rw: crate::uio_rw,
                io: *mut c_void,
                ext: c_long,
                secattr: *mut c_void,
            ) -> c_int,
        >,
        pub fo_ioctl: Option<
            extern "C" fn(
                file: *mut file,
                a: c_long,
                b: crate::caddr_t,
                c: c_long,
                d: c_long,
            ) -> c_int,
        >,
        pub fo_select: Option<
            extern "C" fn(
                file: *mut file,
                a: c_int,
                b: c_ushort,
                c: *mut c_ushort,
                c: extern "C" fn(),
            ) -> c_int,
        >,
        pub fo_close: Option<extern "C" fn(file: *mut file) -> c_int>,
        pub fo_fstat: Option<extern "C" fn(file: *mut file, sstat: *mut crate::stat) -> c_int>,
    }

    #[deprecated(
        since = "0.2.187",
        note = "Use `_file` instead. This type is only available when programming against the \
                kernel, and is otherwise an opaque type."
    )]
    #[allow(deprecated)]
    #[repr(align(256))]
    pub struct file {
        pub f_flag: c_long,
        pub f_count: c_int,
        pub f_options: c_short,
        pub f_type: c_short,
        // Should be pointer to 'vnode'
        pub f_data: *mut c_void,
        pub f_offset: crate::offset_t,
        pub f_dir_off: crate::off_t,
        // Should be pointer to 'cred'
        pub f_cred: *mut c_void,
        pub f_lock: _simple_lock,
        pub f_offset_lock: _simple_lock,
        pub f_vinfo: crate::caddr_t,
        pub f_ops: *mut fileops_t,
        pub f_parentp: crate::caddr_t,
        pub f_fnamep: crate::caddr_t,
        pub f_fdata: [c_char; 160],
    }

    pub struct ld_info {
        pub ldinfo_next: c_uint,
        pub ldinfo_flags: c_uint,
        pub _file: __c_anonymous_ld_info__file,
        pub ldinfo_textorg: *mut c_void,
        pub ldinfo_textsize: c_ulong,
        pub ldinfo_dataorg: *mut c_void,
        pub ldinfo_datasize: c_ulong,
        pub ldinfo_filename: [c_char; 2],
    }

    pub union __c_anonymous_ld_info__file {
        pub _ldinfo_fd: c_int,
        pub _ldinfo_fp: *mut _file,
        pub _core_offset: c_long,
    }

    pub union __c_anonymous_pollfd_ext_t_u {
        pub addr: *mut c_void,
        pub data32: u32,
        pub data: u64,
    }

    pub struct fpreg_t {
        pub d: c_double,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for __c_anonymous_pollfd_ext_t_u {
            fn eq(&self, other: &__c_anonymous_pollfd_ext_t_u) -> bool {
                unsafe {
                    self.addr == other.addr
                        && self.data32 == other.data32
                        && self.data == other.data
                }
            }
        }
        impl Eq for __c_anonymous_pollfd_ext_t_u {}
        impl hash::Hash for __c_anonymous_pollfd_ext_t_u {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                unsafe {
                    self.addr.hash(state);
                    self.data.hash(state);
                    self.data32.hash(state);
                }
            }
        }

        impl PartialEq for fpreg_t {
            fn eq(&self, other: &fpreg_t) -> bool {
                self.d == other.d
            }
        }
        impl Eq for fpreg_t {}
        impl hash::Hash for fpreg_t {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                let d: u64 = self.d.to_bits();
                d.hash(state);
            }
        }
    }
}

// pthread.h
pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __mt_word: [0, 2, 0, 0, 0, 0, 0, 0],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __cv_word: [0, 0, 0, 0, 2, 0],
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __rw_word: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};
pub const PTHREAD_ONCE_INIT: pthread_once_t = pthread_once_t {
    __on_word: [0, 0, 0, 0, 0, 2, 0, 0, 0],
};

// sys/resource.h
pub const RLIM_INFINITY: c_ulong = 0x7fffffffffffffff;

extern "C" {
    pub fn getsystemcfg(label: c_int) -> c_ulong;
}
