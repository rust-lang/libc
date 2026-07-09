//! Header: `sys/ttycom.h`
//!
//! <https://github.com/apple-oss-distributions/xnu/blob/main/bsd/sys/ttycom.h>

use crate::prelude::*;

pub const TIOCMODG: c_ulong = 0x40047403;
pub const TIOCMODS: c_ulong = 0x80047404;

pub const TIOCM_LE: c_int = 0x1;
pub const TIOCM_DTR: c_int = 0x2;
pub const TIOCM_RTS: c_int = 0x4;
pub const TIOCM_ST: c_int = 0x8;
pub const TIOCM_SR: c_int = 0x10;
pub const TIOCM_CTS: c_int = 0x20;
pub const TIOCM_CAR: c_int = 0x40;
pub const TIOCM_CD: c_int = 0x40;
pub const TIOCM_RNG: c_int = 0x80;
pub const TIOCM_RI: c_int = 0x80;
pub const TIOCM_DSR: c_int = 0x100;

pub const TIOCEXCL: c_int = 0x2000740d;
pub const TIOCNXCL: c_int = 0x2000740e;

pub const TIOCFLUSH: c_ulong = 0x80047410;

pub const TIOCGETA: c_ulong = 0x40487413;
pub const TIOCSETA: c_ulong = 0x80487414;
pub const TIOCSETAW: c_ulong = 0x80487415;
pub const TIOCSETAF: c_ulong = 0x80487416;

pub const TIOCGETD: c_ulong = 0x4004741a;
pub const TIOCSETD: c_ulong = 0x8004741b;
pub const TIOCIXON: c_uint = 0x20007481;
pub const TIOCIXOFF: c_uint = 0x20007480;

pub const TIOCSBRK: c_uint = 0x2000747b;
pub const TIOCCBRK: c_uint = 0x2000747a;
pub const TIOCSDTR: c_uint = 0x20007479;
pub const TIOCCDTR: c_uint = 0x20007478;
pub const TIOCGPGRP: c_ulong = 0x40047477;
pub const TIOCSPGRP: c_ulong = 0x80047476;

pub const TIOCOUTQ: c_ulong = 0x40047473;
pub const TIOCSTI: c_ulong = 0x80017472;
pub const TIOCNOTTY: c_uint = 0x20007471;
pub const TIOCPKT: c_ulong = 0x80047470;

pub const TIOCPKT_DATA: c_int = 0x0;
pub const TIOCPKT_FLUSHREAD: c_int = 0x1;
pub const TIOCPKT_FLUSHWRITE: c_int = 0x2;
pub const TIOCPKT_STOP: c_int = 0x4;
pub const TIOCPKT_START: c_int = 0x8;
pub const TIOCPKT_NOSTOP: c_int = 0x10;
pub const TIOCPKT_DOSTOP: c_int = 0x20;
pub const TIOCPKT_IOCTL: c_int = 0x40;

pub const TIOCSTOP: c_uint = 0x2000746f;
pub const TIOCSTART: c_uint = 0x2000746e;
pub const TIOCMSET: c_ulong = 0x8004746d;
pub const TIOCMBIS: c_ulong = 0x8004746c;
pub const TIOCMBIC: c_ulong = 0x8004746b;
pub const TIOCMGET: c_ulong = 0x4004746a;

#[deprecated(since = "0.2.178", note = "Removed in MacOSX 12.0.1")]
pub const TIOCREMOTE: c_ulong = 0x80047469;

pub const TIOCGWINSZ: c_ulong = 0x40087468;
pub const TIOCSWINSZ: c_ulong = 0x80087467;
pub const TIOCUCNTL: c_ulong = 0x80047466;
pub const TIOCSTAT: c_uint = 0x20007465;

pub const TIOCSCONS: c_uint = 0x20007463;
pub const TIOCCONS: c_ulong = 0x80047462;
pub const TIOCSCTTY: c_uint = 0x20007461;
pub const TIOCEXT: c_ulong = 0x80047460;
pub const TIOCSIG: c_uint = 0x2000745f;
pub const TIOCDRAIN: c_uint = 0x2000745e;
pub const TIOCMSDTRWAIT: c_ulong = 0x8004745b;
pub const TIOCMGDTRWAIT: c_ulong = 0x4004745a;

cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        pub const TIOCTIMESTAMP: c_ulong = 0x40107459;
        pub const TIOCDCDTIMESTAMP: c_ulong = 0x40107458;
    } else {
        pub const TIOCTIMESTAMP: c_ulong = 0x40087459;
        pub const TIOCDCDTIMESTAMP: c_ulong = 0x40087458;
    }
}

pub const TIOCSDRAINWAIT: c_ulong = 0x80047457;
pub const TIOCGDRAINWAIT: c_ulong = 0x40047456;
pub const TIOCDSIMICROCODE: c_uint = 0x20007455;
pub const TIOCPTYGRANT: c_uint = 0x20007454;
pub const TIOCPTYGNAME: c_uint = 0x40807453;
pub const TIOCPTYUNLK: c_uint = 0x20007452;
