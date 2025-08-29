//! `linux/can/bcm.h`

pub use crate::linux::can::*;

s! {
    pub struct bcm_timeval {
        pub tv_sec: c_long,
        pub tv_usec: c_long,
    }

    pub struct bcm_msg_head {
        pub opcode: crate::__u32,
        pub flags: crate::__u32,
        pub count: crate::__u32,
        pub ival1: bcm_timeval,
        pub ival2: bcm_timeval,
        pub can_id: canid_t,
        pub nframes: crate::__u32,
        pub frames: [can_frame; 0],
    }
}

pub const TX_SETUP: crate::__u32 = 1;
pub const TX_DELETE: crate::__u32 = 2;
pub const TX_READ: crate::__u32 = 3;
pub const TX_SEND: crate::__u32 = 4;
pub const RX_SETUP: crate::__u32 = 5;
pub const RX_DELETE: crate::__u32 = 6;
pub const RX_READ: crate::__u32 = 7;
pub const TX_STATUS: crate::__u32 = 8;
pub const TX_EXPIRED: crate::__u32 = 9;
pub const RX_STATUS: crate::__u32 = 10;
pub const RX_TIMEOUT: crate::__u32 = 11;
pub const RX_CHANGED: crate::__u32 = 12;

pub const SETTIMER: crate::__u32 = 0x0001;
pub const STARTTIMER: crate::__u32 = 0x0002;
pub const TX_COUNTEVT: crate::__u32 = 0x0004;
pub const TX_ANNOUNCE: crate::__u32 = 0x0008;
pub const TX_CP_CAN_ID: crate::__u32 = 0x0010;
pub const RX_FILTER_ID: crate::__u32 = 0x0020;
pub const RX_CHECK_DLC: crate::__u32 = 0x0040;
pub const RX_NO_AUTOTIMER: crate::__u32 = 0x0080;
pub const RX_ANNOUNCE_RESUME: crate::__u32 = 0x0100;
pub const TX_RESET_MULTI_IDX: crate::__u32 = 0x0200;
pub const RX_RTR_FRAME: crate::__u32 = 0x0400;
pub const CAN_FD_FRAME: crate::__u32 = 0x0800;
