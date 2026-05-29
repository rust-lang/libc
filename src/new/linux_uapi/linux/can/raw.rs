//! Header: `linux/can/raw.h`

pub use crate::linux::can::*;

pub const SOL_CAN_RAW: c_int = SOL_CAN_BASE + CAN_RAW;
#[deprecated(
    since = "0.2.187",
    note = "This constant, among others often used in C for the purposes of denoting the latest \
            value or limit in a set of constants, has been deprecated. See #3131 for details and \
            discussion."
)]
pub const CAN_RAW_FILTER_MAX: c_int = 512;

// FIXME(cleanup): use `c_enum!`, which needs to be adapted to allow omitting a type.
pub const CAN_RAW_FILTER: c_int = 1;
pub const CAN_RAW_ERR_FILTER: c_int = 2;
pub const CAN_RAW_LOOPBACK: c_int = 3;
pub const CAN_RAW_RECV_OWN_MSGS: c_int = 4;
pub const CAN_RAW_FD_FRAMES: c_int = 5;
pub const CAN_RAW_JOIN_FILTERS: c_int = 6;
pub const CAN_RAW_XL_FRAMES: c_int = 7;
