//! Header: `uapi/linux/kd.h`

use crate::Ioctl;

pub const GIO_CMAP: Ioctl = 0x4B70;
pub const PIO_CMAP: Ioctl = 0x4B71;
pub const KDGETLED: Ioctl = 0x4B31;
pub const KDSETLED: Ioctl = 0x4B32;
pub const KDSETMODE: Ioctl = 0x4B3A;
pub const KDGETMODE: Ioctl = 0x4B3B;
pub const GIO_SCRNMAP: Ioctl = 0x4B40;
pub const PIO_SCRNMAP: Ioctl = 0x4B41;
pub const GIO_UNISCRNMAP: Ioctl = 0x4B69;
pub const PIO_UNISCRNMAP: Ioctl = 0x4B6A;
pub const GIO_UNIMAP: Ioctl = 0x4B66;
pub const PIO_UNIMAP: Ioctl = 0x4B67;
pub const PIO_UNIMAPCLR: Ioctl = 0x4B68;
pub const KDGKBMODE: Ioctl = 0x4B44;
pub const KDSKBMODE: Ioctl = 0x4B45;
pub const KDGKBMETA: Ioctl = 0x4B62;
pub const KDSKBMETA: Ioctl = 0x4B63;
pub const KDGKBLED: Ioctl = 0x4B64;
pub const KDSKBLED: Ioctl = 0x4B65;
pub const KDGKBDIACR: Ioctl = 0x4B4A;
pub const KDSKBDIACR: Ioctl = 0x4B4B;
pub const KDGKBDIACRUC: Ioctl = 0x4BFA;
pub const KDSKBDIACRUC: Ioctl = 0x4BFB;
pub const KDFONTOP: Ioctl = 0x4B72;
