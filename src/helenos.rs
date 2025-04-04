use crate::prelude::*;

pub type intmax_t = i64;
pub type uintmax_t = u64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type size_t = usize;
pub type ssize_t = isize;

// abi/include/_bits/errno.h
pub type errno_t = c_int;

// abi/include/_bits/native.h
pub type sysarg_t = uintptr_t;

// uspace/lib/c/include/fibril.h
pub type fid_t = *mut fibril_t;

// uspace/lib/c/include/ipc/loc.h
pub type service_id_t = sysarg_t;

// uspace/lib/c/include/ipc/vfs.h
pub type fs_handle_t = i16;
pub type fs_index_t = u32;

// uspace/lib/c/include/offset.h
pub type aoff64_t = u64;

// uspace/lib/c/include/time.h
pub type time_t = c_longlong;
pub type usec_t = c_longlong;

// uspace/lib/inet/include/inet/addr.h
pub type addr32_t = u32;
pub type addr128_t = [u8; 16];

// uspace/lib/posix/include/posix/pthread.h
pub type pthread_key_t = c_int;

// uspace/lib/posix/include/posix/sys/types.h
pub type clockid_t = c_int;
pub type pid_t = c_int;

s! {
    // common/include/adt/list.h
    pub struct link_t {
        pub next: *mut link_t,
        pub prev: *mut link_t,
    }

    pub struct list_t {
        pub head: link_t,
    }

    // uspace/lib/c/include/dirent.h
    pub struct dirent {
        pub d_name: [c_char; 256],
    }

    // uspace/lib/c/include/fibril.h
    pub struct fibril_owner_info_t {
        pub owned_by: *mut fibril_t,
    }

    // uspace/lib/c/include/fibril_synch.h
    pub struct fibril_mutex_t {
        pub oi: fibril_owner_info_t,
        pub counter: c_int,
        pub waiters: list_t,
    }

    pub struct fibril_condvar_t {
        pub waiters: list_t,
    }

    // uspace/lib/c/include/time.h
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    // uspace/lib/c/include/vfs/vfs.h
    pub struct vfs_stat_t {
        pub fs_handle: fs_handle_t,
        pub service_id: service_id_t,
        pub index: fs_index_t,
        pub lnkcnt: c_uint,
        pub is_file: bool,
        pub is_directory: bool,
        pub size: aoff64_t,
        pub service: service_id_t,
    }

    // uspace/lib/inet/include/inet/dnsr.h
    pub struct dnsr_hostinfo_t {
        pub cname: *mut c_char,
        pub addr: inet_addr_t,
    }

    // uspace/lib/inet/include/inet/endpoint.h
    pub struct inet_ep2_t {
        pub local_link: service_id_t,
        pub local: inet_ep_t,
        pub remote: inet_ep_t,
    }

    pub struct inet_ep_t {
        pub addr: inet_addr_t,
        pub port: u16,
    }

    // uspace/lib/inet/include/inet/tcp.h
    pub struct tcp_cb_t {
        pub connected: extern "C" fn(conn: *mut tcp_conn_t),
        pub conn_failed: extern "C" fn(conn: *mut tcp_conn_t),
        pub conn_reset: extern "C" fn(conn: *mut tcp_conn_t),
        pub data_avail: extern "C" fn(conn: *mut tcp_conn_t),
        pub urg_data: extern "C" fn(conn: *mut tcp_conn_t),
    }
    pub struct tcp_listen_cb_t {
        pub new_conn: extern "C" fn(listener: *mut tcp_listener_t, conn: *mut tcp_conn_t),
    }
}

// uspace/lib/inet/include/inet/addr.h
e! {
    #[repr(u32)]
    pub enum ip_ver_t {
        ip_any,
        ip_v4,
        ip_v6,
    }
}

// uspace/lib/inet/include/inet/addr.h
s_no_extra_traits! {
    pub union __inet_addr_t_addr_union {
        pub addr: addr32_t,
        pub addr6: addr128_t,
    }

    pub struct inet_addr_t {
        pub version: ip_ver_t,
        pub addr: __inet_addr_t_addr_union,
    }
}

// abi/include/abi/errno.h
pub const EOK: errno_t = 0;
pub const ENOENT: errno_t = 1;
pub const ENOMEM: errno_t = 2;
pub const ELIMIT: errno_t = 3;
pub const EREFUSED: errno_t = 4;
pub const EFORWARD: errno_t = 5;
pub const EPERM: errno_t = 6;
pub const EHANGUP: errno_t = 7;
pub const EPARTY: errno_t = 8;
pub const EEXIST: errno_t = 9;
pub const EBADMEM: errno_t = 10;
pub const ENOTSUP: errno_t = 11;
pub const EADDRNOTAVAIL: errno_t = 12;
pub const ETIMEOUT: errno_t = 13;
pub const EINVAL: errno_t = 14;
pub const EBUSY: errno_t = 15;
pub const EOVERFLOW: errno_t = 16;
pub const EINTR: errno_t = 17;
pub const EMFILE: errno_t = 18;
pub const ENAMETOOLONG: errno_t = 19;
pub const EISDIR: errno_t = 20;
pub const ENOTDIR: errno_t = 21;
pub const ENOSPC: errno_t = 22;
pub const ENOTEMPTY: errno_t = 23;
pub const EBADF: errno_t = 24;
pub const EDOM: errno_t = 25;
pub const ERANGE: errno_t = 26;
pub const EXDEV: errno_t = 27;
pub const EIO: errno_t = 28;
pub const EMLINK: errno_t = 29;
pub const ENXIO: errno_t = 30;
pub const ENOFS: errno_t = 31;
pub const EBADCHECKSUM: errno_t = 32;
pub const ESTALL: errno_t = 33;
pub const EEMPTY: errno_t = 34;
pub const ENAK: errno_t = 35;
pub const EAGAIN: errno_t = 36;

// uspace/lib/posix/include/posix/time.h
pub const CLOCK_REALTIME: clockid_t = 0;

// uspace/lib/c/include/stdio.h
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;

// 'static inline' functions from libc
// common/include/adt/list.h
f! {
    pub fn list_initialize(list: *mut list_t) -> () {
        let list = &mut *list;
        list.head.next = &mut list.head;
        list.head.prev = &mut list.head;
    }

    // uspace/lib/c/include/fibril_synch.h
    pub fn fibril_mutex_initialize(fm: *mut fibril_mutex_t) -> () {
        let fm = &mut *fm;
        fm.oi.owned_by = core::ptr::null_mut();
        fm.counter = 1;
        list_initialize(&mut fm.waiters);
    }
}

missing! {
    // uspace/lib/c/include/dirent.h
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum DIR {}

    // uspace/lib/c/include/fibril.h
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum fibril_t {}

    // uspace/lib/c/include/stdio.h
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum FILE {}

    // uspace/lib/inet/include/inet/tcp.h
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum tcp_t {}
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum tcp_conn_t {}
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum tcp_listener_t {}
}

#[link(name = "c")]
#[link(name = "posix")]
#[link(name = "inet")]
#[link(name = "startfiles")] // FIXME: when I tried to explicitly specify that this is a native library, it broke, I should later figure out why
extern "C" {}

extern "C" {
    // common/include/stdlib.h
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    pub fn realloc(addr: *mut c_void, size: usize) -> *mut c_void;
    pub fn free(addr: *mut c_void);

    pub fn memalign(align: usize, size: usize) -> *mut c_void;

    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);

    pub fn exit(code: c_int) -> !;
    pub fn abort() -> !;

    pub fn getenv(env: *const c_char) -> *mut c_char;

    // common/include/str_error.h
    pub fn str_error(err: errno_t) -> *const c_char;
    pub fn str_error_name(err: errno_t) -> *const c_char;

    // uspace/lib/c/include/dirent.h
    pub fn opendir(name: *const c_char) -> *mut DIR;
    pub fn readdir(dir: *mut DIR) -> *mut dirent;
    pub fn closedir(dir: *mut DIR) -> c_int;
    pub fn rewinddir(dir: *mut DIR);

    // uspace/lib/c/include/errno.h
    pub fn __errno() -> *mut errno_t;

    // uspace/lib/c/include/fibril.h
    pub fn fibril_create_generic(
        func: extern "C" fn(*mut c_void) -> errno_t,
        arg: *mut c_void,
        stacksize: size_t,
    ) -> fid_t;
    pub fn fibril_start(f: fid_t);
    pub fn fibril_exit(retval: c_long) -> !;
    pub fn fibril_detach(f: fid_t);
    pub fn fibril_yield();
    pub fn fibril_usleep(usec: usec_t);
    pub fn fibril_get_id() -> fid_t;

    // uspace/lib/c/include/fibril_synch.h
    pub fn fibril_mutex_lock(mutex: *mut fibril_mutex_t);
    pub fn fibril_mutex_unlock(mutex: *mut fibril_mutex_t);
    pub fn fibril_mutex_trylock(mutex: *mut fibril_mutex_t) -> bool;
    pub fn fibril_mutex_is_locked(mutex: *mut fibril_mutex_t) -> bool;

    pub fn fibril_condvar_initialize(condvar: *mut fibril_condvar_t);
    pub fn fibril_condvar_wait(condvar: *mut fibril_condvar_t, mutex: *mut fibril_mutex_t);
    pub fn fibril_condvar_wait_timeout(
        condvar: *mut fibril_condvar_t,
        mutex: *mut fibril_mutex_t,
        timeout: usec_t,
    ) -> errno_t;
    pub fn fibril_condvar_signal(condvar: *mut fibril_condvar_t);
    pub fn fibril_condvar_broadcast(condvar: *mut fibril_condvar_t);

    // uspace/lib/c/include/stdio.h
    pub static stdin: *mut FILE;
    pub static stdout: *mut FILE;
    pub static stderr: *mut FILE;

    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;

    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn fclose(stream: *mut FILE) -> c_int;
    pub fn fflush(stream: *mut FILE) -> c_int;

    pub fn fread(buf: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(buf: *const c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;

    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;

    pub fn fileno(stream: *mut FILE) -> c_int;

    // uspace/lib/c/include/time.h
    pub fn getuptime(tp: *mut timespec);

    // uspace/lib/c/include/vfs/vfs.h
    pub fn vfs_stat_path(path: *const c_char, stat: *mut vfs_stat_t) -> errno_t;

    // uspace/lib/inet/include/inet/dnsr.h
    pub fn dnsr_name2host(
        name: *const c_char,
        info: *mut *mut dnsr_hostinfo_t,
        ipver: ip_ver_t,
    ) -> errno_t;
    pub fn dnsr_hostinfo_destroy(info: *mut dnsr_hostinfo_t);

    // uspace/lib/inet/include/inet/endpoint.h
    pub fn inet_ep_init(ep: *mut inet_ep_t);
    pub fn inet_ep2_init(epp: *mut inet_ep2_t);

    // uspace/lib/inet/include/inet/tcp.h
    pub fn tcp_create(tcp: *mut *mut tcp_t) -> errno_t;
    pub fn tcp_destroy(tcp: *mut tcp_t);

    pub fn tcp_conn_create(
        tcp: *mut tcp_t,
        epp: *mut inet_ep2_t,
        callbacks: *mut tcp_cb_t,
        arg: *mut c_void,
        conn: *mut *mut tcp_conn_t,
    ) -> errno_t;
    pub fn tcp_conn_destroy(conn: *mut tcp_conn_t);

    pub fn tcp_listener_create(
        tcp: *mut tcp_t,
        ep: *mut inet_ep_t,
        listen_callbacks: *mut tcp_listen_cb_t,
        listen_arg: *mut c_void,
        conn_callbacks: *mut tcp_cb_t,
        conn_arg: *mut c_void,
        listener: *mut *mut tcp_listener_t,
    ) -> errno_t;
    pub fn tcp_listener_destroy(listener: *mut tcp_listener_t);

    pub fn tcp_conn_userptr(conn: *mut tcp_conn_t) -> *mut c_void;
    pub fn tcp_listener_userptr(listener: *mut tcp_listener_t) -> *mut c_void;

    pub fn tcp_conn_wait_connected(conn: *mut tcp_conn_t) -> errno_t;
    pub fn tcp_conn_send(conn: *mut tcp_conn_t, buf: *const c_void, len: size_t) -> errno_t;
    pub fn tcp_conn_send_fin(conn: *mut tcp_conn_t) -> errno_t;
    pub fn tcp_conn_push(conn: *mut tcp_conn_t) -> errno_t;
    pub fn tcp_conn_reset(conn: *mut tcp_conn_t) -> errno_t;

    pub fn tcp_conn_recv(
        conn: *mut tcp_conn_t,
        buf: *mut c_void,
        bufsize: size_t,
        received_len: *mut size_t,
    ) -> errno_t;
    pub fn tcp_conn_recv_wait(
        conn: *mut tcp_conn_t,
        buf: *mut c_void,
        bufsize: size_t,
        received_len: *mut size_t,
    ) -> errno_t;

    // uspace/lib/posix/include/posix/pthread.h
    pub fn pthread_key_create(
        key: *mut pthread_key_t,
        destructor: unsafe extern "C" fn(*mut c_void),
    ) -> c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;

    // uspace/lib/posix/include/posix/time.h
    pub fn clock_getres(clock_id: clockid_t, res: *mut timespec) -> c_int;
    pub fn clock_gettime(clock_id: clockid_t, tp: *mut timespec) -> c_int;
    pub fn clock_settime(clock_id: clockid_t, tp: *const timespec) -> c_int;
    pub fn clock_nanosleep(
        clock_id: clockid_t,
        flags: c_int,
        rqtp: *const timespec,
        rmtp: *mut timespec,
    ) -> c_int;

    // uspace/lib/posix/include/posix/unistd.h
    pub fn getpid() -> pid_t;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn chdir(buf: *const c_char) -> c_int;
    pub static environ: *const *const c_char;
    pub fn isatty(fd: c_int) -> c_int;
}
