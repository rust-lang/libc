s! {
    /* Header <fcntl.h> */
    pub struct file_handle  {
        pub handle_bytes: ::c_uint,
        pub handle_type: ::c_int,
//        pub f_handle: [::c_uchar],
    }
}

/* Header <fcntl.h> */
pub const F_OFD_GETLK: ::c_int = 36;
pub const F_OFD_SETLK: ::c_int = 37;
pub const F_OFD_SETLKW: ::c_int = 38;

// Here start non POSIX definitions.
pub const FSYNC: ::c_int = ::O_SYNC;

pub const F_OWNER_TID: ::c_int = 0;
pub const F_OWNER_PID: ::c_int = 1;
pub const F_OWNER_PGRP: ::c_int = 2;
pub const F_OWNER_GID: ::c_int = ::F_OWNER_PGRP;

pub const FALLOC_FL_COLLAPSE_RANGE: ::c_int = 0x08;
pub const FALLOC_FL_ZERO_RANGE: ::c_int = 0x10;

pub const MAX_HANDLE_SZ: ::size_t = 128;

pub const O_SHLOCK: ::c_int = 0x0010;
pub const O_EXLOCK: ::c_int = 0x0020;

pub const FREAD: ::c_int = 1;
pub const FWRITE: ::c_int = 2;

pub const F_SETOWN_EX: ::c_int = 15;
pub const F_GETOWN_EX: ::c_int = 16;

extern {
    /* Header <fcntl.h> */
    pub fn name_to_handle_at(dfd: ::c_int,
                             name: *const ::c_char,
			                 handle: *mut ::file_handle,
			                 mnt_id: *mut ::c_int,
			                 flags: ::c_int) -> ::c_int;
    pub fn open_by_handle_at(mountdirfd: ::c_int,
                             handle: *mut ::file_handle,
			                 flags: ::c_int) -> ::c_int;
}
