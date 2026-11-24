//! Header: `uapi/linux/mmc/ioctl.h`
//!
//! <https://github.com/torvalds/linux/blob/3389648819829299517a8d482fda213fd7166a2c/include/uapi/linux/mmc/ioctl.h>

use crate::prelude::*;
use crate::{
    Ioctl,
    _IOWR,
};

/// `MMC_BLOCK_MAJOR` from `linux/major.h`
const MMC_BLOCK_MAJOR: u32 = 179;

s2! {
    pub struct mmc_ioc_cmd {
        pub write_flag: c_int,
        pub is_acmd: c_int,
        pub opcode: crate::__u32,
        pub arg: crate::__u32,
        pub response: [crate::__u32; 4],
        pub flags: c_uint,
        pub blksz: c_uint,
        pub blocks: c_uint,
        pub postsleep_min_us: c_uint,
        pub postsleep_max_us: c_uint,
        pub data_timeout_ns: c_uint,
        pub cmd_timeout_ms: c_uint,
        __pad: Padding<crate::__u32>,
        pub data_ptr: crate::__u64,
    }

    pub struct mmc_ioc_multi_cmd {
        pub num_of_cmds: crate::__u64,
        pub cmds: [mmc_ioc_cmd; 0],
    }
}

pub const MMC_IOC_CMD: Ioctl = _IOWR::<mmc_ioc_cmd>(MMC_BLOCK_MAJOR, 0);
pub const MMC_IOC_MULTI_CMD: Ioctl = _IOWR::<mmc_ioc_multi_cmd>(MMC_BLOCK_MAJOR, 1);

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const MMC_IOC_MAX_BYTES: c_long = 512 * 1024;

/// Constants may change across releases. See the [usage guidelines](crate#usage-guidelines)
/// for details.
pub const MMC_IOC_MAX_CMDS: c_int = 255;
