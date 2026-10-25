//! Header: `bionic/libc/kernel/uapi/linux/fs.h`
//!
//! <https://android.googlesource.com/platform/bionic/+/731631f300090436d7f5df80d50b6275c8c60a93/libc/kernel/uapi/linux/fs.h>

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
