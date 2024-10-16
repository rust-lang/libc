s! {
    pub struct statx {
        pub stx_mask: ::__u32,
        pub stx_blksize: ::__u32,
        pub stx_attributes: ::__u64,
        pub stx_nlink: ::__u32,
        pub stx_uid: ::__u32,
        pub stx_gid: ::__u32,
        pub stx_mode: ::__u16,
        __statx_pad1: [::__u16; 1],
        pub stx_ino: ::__u64,
        pub stx_size: ::__u64,
        pub stx_blocks: ::__u64,
        pub stx_attributes_mask: ::__u64,
        pub stx_atime: statx_timestamp,
        pub stx_btime: statx_timestamp,
        pub stx_ctime: statx_timestamp,
        pub stx_mtime: statx_timestamp,
        pub stx_rdev_major: ::__u32,
        pub stx_rdev_minor: ::__u32,
        pub stx_dev_major: ::__u32,
        pub stx_dev_minor: ::__u32,
        pub stx_mnt_id: ::__u64,
        pub stx_dio_mem_align: ::__u32,
        pub stx_dio_offset_align: ::__u32,
        pub stx_subvol: ::__u64,
        pub stx_atomic_write_unit_min: ::__u32,
        pub stx_atomic_write_unit_max: ::__u32,
        pub stx_atomic_write_segments_max: ::__u32,
        __statx_pad2: [::__u32; 1],
        __statx_pad3: [::__u64; 9],
    }

    pub struct statx_timestamp {
        pub tv_sec: ::__s64,
        pub tv_nsec: ::__u32,
        __statx_timestamp_pad1: [::__s32; 1],
    }
}

pub const AT_STATX_SYNC_TYPE: ::c_int = 0x6000;
pub const AT_STATX_SYNC_AS_STAT: ::c_int = 0x0000;
pub const AT_STATX_FORCE_SYNC: ::c_int = 0x2000;
pub const AT_STATX_DONT_SYNC: ::c_int = 0x4000;
pub const STATX_TYPE: ::c_uint = 0x0001;
pub const STATX_MODE: ::c_uint = 0x0002;
pub const STATX_NLINK: ::c_uint = 0x0004;
pub const STATX_UID: ::c_uint = 0x0008;
pub const STATX_GID: ::c_uint = 0x0010;
pub const STATX_ATIME: ::c_uint = 0x0020;
pub const STATX_MTIME: ::c_uint = 0x0040;
pub const STATX_CTIME: ::c_uint = 0x0080;
pub const STATX_INO: ::c_uint = 0x0100;
pub const STATX_SIZE: ::c_uint = 0x0200;
pub const STATX_BLOCKS: ::c_uint = 0x0400;
pub const STATX_BASIC_STATS: ::c_uint = 0x07ff;
pub const STATX_BTIME: ::c_uint = 0x0800;
pub const STATX_ALL: ::c_uint = 0x0fff;
pub const STATX_MNT_ID: ::c_uint = 0x1000;
pub const STATX_DIOALIGN: ::c_uint = 0x2000;
pub const STATX_MNT_ID_UNIQUE: ::c_uint = 0x4000;
pub const STATX_SUBVOL: ::c_uint = 0x8000;
pub const STATX_WRITE_ATOMIC: ::c_uint = 0x10000;
pub const STATX__RESERVED: ::c_int = 0x80000000;
pub const STATX_ATTR_COMPRESSED: ::c_int = 0x0004;
pub const STATX_ATTR_IMMUTABLE: ::c_int = 0x0010;
pub const STATX_ATTR_APPEND: ::c_int = 0x0020;
pub const STATX_ATTR_NODUMP: ::c_int = 0x0040;
pub const STATX_ATTR_ENCRYPTED: ::c_int = 0x0800;
pub const STATX_ATTR_AUTOMOUNT: ::c_int = 0x1000;
pub const STATX_ATTR_MOUNT_ROOT: ::c_int = 0x2000;
pub const STATX_ATTR_VERITY: ::c_int = 0x100000;
pub const STATX_ATTR_DAX: ::c_int = 0x200000;
pub const STATX_ATTR_WRITE_ATOMIC: ::c_int = 0x400000;

extern "C" {
    pub fn statx(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        flags: ::c_int,
        mask: ::c_uint,
        statxbuf: *mut statx,
    ) -> ::c_int;
}
