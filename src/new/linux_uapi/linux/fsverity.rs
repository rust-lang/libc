//! Header: uapi/linux/fsverity.h
//!
//! Source:
//! <https://git.kernel.org/pub/scm/linux/kernel/git/stable/linux.git/tree/include/uapi/linux/fsverity.h?id=07c99001312cbf90a357d4877a358f796eede65b>

use crate::prelude::*;
use crate::{
    __u16,
    __u32,
    __u64,
    __u8,
    Ioctl,
    _IOW,
    _IOWR,
};

pub const FS_VERITY_HASH_ALG_SHA256: __u8 = 1;
pub const FS_VERITY_HASH_ALG_SHA512: __u8 = 2;

s! {
    pub struct fsverity_enable_arg {
        pub version: __u32,
        pub hash_algorithm: __u32,
        pub block_size: __u32,
        pub salt_size: __u32,
        pub salt_ptr: __u64,
        pub sig_size: __u32,
        __reserved1: Padding<__u32>,
        pub sig_ptr: __u64,
        __reserved2: Padding<[__u64; 11]>,
    }

    pub struct fsverity_digest {
        pub digest_algorithm: __u16,
        pub digest_size: __u16,
        pub digest: [__u8; 0],
    }

    pub struct fsverity_descriptor {
        pub version: __u8,
        pub hash_algorithm: __u8,
        pub log_blocksize: __u8,
        pub salt_size: __u8,
        __reserved_0x04: Padding<__u32>,
        pub data_size: __u64,
        pub root_hash: [__u8; 64],
        pub salt: [__u8; 32],
        __reserved: Padding<[__u8; 144]>,
    }

    pub struct fsverity_formatted_digest {
        pub magic: [c_char; 8],
        pub digest_algorithm: __u16,
        pub digest_size: __u16,
        pub digest: [__u8; 0],
    }
}

pub const FS_VERITY_METADATA_TYPE_MERKLE_TREE: __u64 = 1;
pub const FS_VERITY_METADATA_TYPE_DESCRIPTOR: __u64 = 2;
pub const FS_VERITY_METADATA_TYPE_SIGNATURE: __u64 = 3;

s! {
    pub struct fsverity_read_metadata_arg {
        pub metadata_type: __u64,
        pub offset: __u64,
        pub length: __u64,
        pub buf_ptr: __u64,
        __reserved: Padding<__u64>,
    }
}

pub const FS_IOC_ENABLE_VERITY: Ioctl = _IOW::<fsverity_enable_arg>('f' as u32, 133);
pub const FS_IOC_MEASURE_VERITY: Ioctl = _IOWR::<fsverity_digest>('f' as u32, 134);
pub const FS_IOC_READ_VERITY_METADATA: Ioctl = _IOWR::<fsverity_read_metadata_arg>('f' as u32, 135);
