//! Header: `uapi/linux/vt.h`

use crate::Ioctl;

pub const VT_OPENQRY: Ioctl = 0x5600;
pub const VT_GETMODE: Ioctl = 0x5601;
pub const VT_SETMODE: Ioctl = 0x5602;
pub const VT_GETSTATE: Ioctl = 0x5603;
pub const VT_RELDISP: Ioctl = 0x5605;
pub const VT_ACTIVATE: Ioctl = 0x5606;
pub const VT_WAITACTIVE: Ioctl = 0x5607;
pub const VT_DISALLOCATE: Ioctl = 0x5608;
pub const VT_RESIZE: Ioctl = 0x5609;
pub const VT_RESIZEX: Ioctl = 0x560A;
pub const VT_GETHIFONTMASK: Ioctl = 0x560D;
pub const VT_WAITEVENT: Ioctl = 0x560E;
pub const VT_SETACTIVATE: Ioctl = 0x560F;
