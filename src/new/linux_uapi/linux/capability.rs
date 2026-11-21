//! Header: `uapi/linux/capability.h`

use crate::prelude::*;

pub const _LINUX_CAPABILITY_VERSION_1: c_int = 0x19980330;
pub const _LINUX_CAPABILITY_U32S_1: c_int = 1;

pub const _LINUX_CAPABILITY_VERSION_2: c_int = 0x20071026;
pub const _LINUX_CAPABILITY_U32S_2: c_int = 2;

pub const _LINUX_CAPABILITY_VERSION_3: c_int = 0x20080522;
pub const _LINUX_CAPABILITY_U32S_3: c_int = 2;

s! {
    pub struct __user_cap_header_struct {
        pub version: crate::__u32,
        pub pid: c_int,
    }

    pub struct __user_cap_data_struct {
        pub effective: crate::__u32,
        pub permitted: crate::__u32,
        pub inheritable: crate::__u32,
    }
}

pub type cap_user_header_t = *mut __user_cap_header_struct;
pub type cap_user_data_t = *mut __user_cap_data_struct;

pub const CAP_CHOWN: c_int = 0;
pub const CAP_DAC_OVERRIDE: c_int = 1;
pub const CAP_DAC_READ_SEARCH: c_int = 2;
pub const CAP_FOWNER: c_int = 3;
pub const CAP_FSETID: c_int = 4;
pub const CAP_KILL: c_int = 5;
pub const CAP_SETGID: c_int = 6;
pub const CAP_SETUID: c_int = 7;
pub const CAP_SETPCAP: c_int = 8;
pub const CAP_LINUX_IMMUTABLE: c_int = 9;
pub const CAP_NET_BIND_SERVICE: c_int = 10;
pub const CAP_NET_BROADCAST: c_int = 11;
pub const CAP_NET_ADMIN: c_int = 12;
pub const CAP_NET_RAW: c_int = 13;
pub const CAP_IPC_LOCK: c_int = 14;
pub const CAP_IPC_OWNER: c_int = 15;
pub const CAP_SYS_MODULE: c_int = 16;
pub const CAP_SYS_RAWIO: c_int = 17;
pub const CAP_SYS_CHROOT: c_int = 18;
pub const CAP_SYS_PTRACE: c_int = 19;
pub const CAP_SYS_PACCT: c_int = 20;
pub const CAP_SYS_ADMIN: c_int = 21;
pub const CAP_SYS_BOOT: c_int = 22;
pub const CAP_SYS_NICE: c_int = 23;
pub const CAP_SYS_RESOURCE: c_int = 24;
pub const CAP_SYS_TIME: c_int = 25;
pub const CAP_SYS_TTY_CONFIG: c_int = 26;
pub const CAP_MKNOD: c_int = 27;
pub const CAP_LEASE: c_int = 28;
pub const CAP_AUDIT_WRITE: c_int = 29;
pub const CAP_AUDIT_CONTROL: c_int = 30;
pub const CAP_SETFCAP: c_int = 31;
pub const CAP_MAC_OVERRIDE: c_int = 32;
pub const CAP_MAC_ADMIN: c_int = 33;
pub const CAP_SYSLOG: c_int = 34;
pub const CAP_WAKE_ALARM: c_int = 35;
pub const CAP_BLOCK_SUSPEND: c_int = 36;
pub const CAP_AUDIT_READ: c_int = 37;
pub const CAP_PERFMON: c_int = 38;
pub const CAP_BPF: c_int = 39;
pub const CAP_CHECKPOINT_RESTORE: c_int = 40;
