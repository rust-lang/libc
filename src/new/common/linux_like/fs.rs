//! Header: `uapi/linux/fs.h`
//!
//! <https://github.com/torvalds/linux/blob/3389648819829299517a8d482fda213fd7166a2c/include/uapi/linux/fs.h>

use crate::prelude::*;
use crate::{
    Ioctl,
    _IOWR,
};

s! {
    pub struct page_region {
        pub start: crate::__u64,
        pub end: crate::__u64,
        pub categories: crate::__u64,
    }

    pub struct pm_scan_arg {
        pub size: crate::__u64,
        pub flags: crate::__u64,
        pub start: crate::__u64,
        pub end: crate::__u64,
        pub walk_end: crate::__u64,
        pub vec: crate::__u64,
        pub vec_len: crate::__u64,
        pub max_pages: crate::__u64,
        pub category_inverted: crate::__u64,
        pub category_mask: crate::__u64,
        pub category_anyof_mask: crate::__u64,
        pub return_mask: crate::__u64,
    }

    pub struct procmap_query {
        pub size: crate::__u64,
        pub query_flags: crate::__u64,
        pub query_addr: crate::__u64,
        pub vma_start: crate::__u64,
        pub vma_end: crate::__u64,
        pub vma_flags: crate::__u64,
        pub vma_page_size: crate::__u64,
        pub vma_offset: crate::__u64,
        pub inode: crate::__u64,
        pub dev_major: crate::__u32,
        pub dev_minor: crate::__u32,
        pub vma_name_size: crate::__u32,
        pub build_id_size: crate::__u32,
        pub vma_name_addr: crate::__u64,
        pub build_id_addr: crate::__u64,
    }
}

const PROCFS_IOCTL_MAGIC: c_uint = b'f' as c_uint;
pub const PAGEMAP_SCAN: Ioctl = _IOWR::<pm_scan_arg>(PROCFS_IOCTL_MAGIC, 16);
pub const PROCMAP_QUERY: Ioctl = _IOWR::<procmap_query>(PROCFS_IOCTL_MAGIC, 17);

pub const PROCMAP_QUERY_VMA_READABLE: c_uint = 0x01;
pub const PROCMAP_QUERY_VMA_WRITABLE: c_uint = 0x02;
pub const PROCMAP_QUERY_VMA_EXECUTABLE: c_uint = 0x04;
pub const PROCMAP_QUERY_VMA_SHARED: c_uint = 0x08;
pub const PROCMAP_QUERY_COVERING_OR_NEXT_VMA: c_uint = 0x10;
pub const PROCMAP_QUERY_FILE_BACKED_VMA: c_uint = 0x20;

pub const PAGE_IS_WPALLOWED: c_uint = 1 << 0;
pub const PAGE_IS_WRITTEN: c_uint = 1 << 1;
pub const PAGE_IS_FILE: c_uint = 1 << 2;
pub const PAGE_IS_PRESENT: c_uint = 1 << 3;
pub const PAGE_IS_SWAPPED: c_uint = 1 << 4;
pub const PAGE_IS_PFNZERO: c_uint = 1 << 5;
pub const PAGE_IS_HUGE: c_uint = 1 << 6;
pub const PAGE_IS_SOFT_DIRTY: c_uint = 1 << 7;

pub const PM_SCAN_WP_MATCHING: c_uint = 1 << 0;
pub const PM_SCAN_CHECK_WPASYNC: c_uint = 1 << 1;
