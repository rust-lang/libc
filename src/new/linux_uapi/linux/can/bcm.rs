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

c_enum! {
    #[repr(crate::__u32)]
    pub enum {
        TX_SETUP = 1,
        TX_DELETE,
        TX_READ,
        TX_SEND,
        RX_SETUP,
        RX_DELETE,
        RX_READ,
        TX_STATUS,
        TX_EXPIRED,
        RX_STATUS,
        RX_TIMEOUT,
        RX_CHANGED,
    }
}

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
