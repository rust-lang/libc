//! Header: `uapi/linux/fs.h`
//!
//! <https://github.com/torvalds/linux/blob/3389648819829299517a8d482fda213fd7166a2c/include/uapi/linux/fs.h>

pub use crate::new::common::linux_like::fs::{
    page_region,
    pm_scan_arg,
    procmap_query,
    PAGEMAP_SCAN,
    PAGE_IS_FILE,
    PAGE_IS_HUGE,
    PAGE_IS_PFNZERO,
    PAGE_IS_PRESENT,
    PAGE_IS_SOFT_DIRTY,
    PAGE_IS_SWAPPED,
    PAGE_IS_WPALLOWED,
    PAGE_IS_WRITTEN,
    PM_SCAN_CHECK_WPASYNC,
    PM_SCAN_WP_MATCHING,
    PROCMAP_QUERY,
    PROCMAP_QUERY_COVERING_OR_NEXT_VMA,
    PROCMAP_QUERY_FILE_BACKED_VMA,
    PROCMAP_QUERY_VMA_EXECUTABLE,
    PROCMAP_QUERY_VMA_READABLE,
    PROCMAP_QUERY_VMA_SHARED,
    PROCMAP_QUERY_VMA_WRITABLE,
};
use crate::prelude::*;

pub const PAGE_IS_GUARD: c_uint = 1 << 8;
pub const PAGE_IS_ACCESSED: c_uint = 1 << 9;
