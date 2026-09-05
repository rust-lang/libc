s! {
    pub struct mmsghdr {
        pub msg_hdr: crate::msghdr,
        pub msg_len: c_uint,
    }
}

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

#[deprecated(since = "0.2.70", note = "AF_PACKET must be used instead")]
pub const SOCK_PACKET: c_int = 10;

// x86-64
pub const SOCK_DGRAM: c_int = 2; // connectionless, unreliable datagrams
pub const SOCK_STREAM: c_int = 1; // …/common/bits/socket_type.h

// mips
pub const SOCK_STREAM: c_int = 2;
pub const SOCK_DGRAM: c_int = 1;
pub const SOCK_SEQPACKET: c_int = 5;
pub const SOCK_NONBLOCK: c_int = 128;

// arm
pub const SOCK_DGRAM: c_int = 0x2;
pub const SOCK_NONBLOCK: c_int = 0o0004000;
pub const SOCK_SEQPACKET: c_int = 0x5;
pub const SOCK_STREAM: c_int = 0x1;

/*
copy to the rest of linux-like

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;
pub const SOCK_CLOEXEC: c_int = O_CLOEXEC;
pub const SOCK_DCCP: c_int = 6;

    pub struct mmsghdr {
        pub msg_hdr: crate::msghdr,
        pub msg_len: c_uint,
    }
*/
