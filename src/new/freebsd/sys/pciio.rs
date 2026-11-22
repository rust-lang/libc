//! Header: `sys/pciio.h`
//!
//! <https://github.com/freebsd/freebsd-src/blob/main/sys/sys/pciio.h>

use crate::prelude::*;
use crate::_IOWR;

pub const PCI_MAXNAMELEN: usize = 16;

c_enum! {
    #[repr(c_uint)]
    pub enum pci_getconf_status {
        pub PCI_GETCONF_LAST_DEVICE = 0,
        pub PCI_GETCONF_LIST_CHANGED = 1,
        pub PCI_GETCONF_MORE_DEVS = 2,
        pub PCI_GETCONF_ERROR = 3,
    }

    #[repr(c_uint)]
    pub enum pci_getconf_flags {
        pub PCI_GETCONF_NO_MATCH = 0x0000,
        pub PCI_GETCONF_MATCH_DOMAIN = 0x0001,
        pub PCI_GETCONF_MATCH_BUS = 0x0002,
        pub PCI_GETCONF_MATCH_DEV = 0x0004,
        pub PCI_GETCONF_MATCH_FUNC = 0x0008,
        pub PCI_GETCONF_MATCH_NAME = 0x0010,
        pub PCI_GETCONF_MATCH_UNIT = 0x0020,
        pub PCI_GETCONF_MATCH_VENDOR = 0x0040,
        pub PCI_GETCONF_MATCH_DEVICE = 0x0080,
        pub PCI_GETCONF_MATCH_CLASS = 0x0100,
    }
}

s2! {
    pub struct pcisel {
        pub pc_domain: u32,
        pub pc_bus: u8,
        pub pc_dev: u8,
        pub pc_func: u8,
    }

    pub struct pci_conf {
        pub pc_sel: pcisel,
        pub pc_hdr: u8,
        pub pc_subvendor: u16,
        pub pc_subdevice: u16,
        pub pc_vendor: u16,
        pub pc_device: u16,
        pub pc_class: u8,
        pub pc_subclass: u8,
        pub pc_progif: u8,
        pub pc_revid: u8,
        pub pd_name: [c_char; PCI_MAXNAMELEN + 1],
        pub pd_unit: c_ulong,
    }

    pub struct pci_match_conf {
        pub pc_sel: pcisel,
        pub pd_name: [c_char; PCI_MAXNAMELEN + 1],
        pub pd_unit: c_ulong,
        pub pc_vendor: u16,
        pub pc_device: u16,
        pub pc_class: u8,
        pub flags: pci_getconf_flags,
    }

    pub struct pci_conf_io {
        pub pat_buf_len: u32,
        pub num_patterns: u32,
        #[custom_default(ptr::null_mut())]
        pub patterns: *mut pci_match_conf,
        pub match_buf_len: u32,
        pub num_matches: u32,
        #[custom_default(ptr::null_mut())]
        pub matches: *mut pci_conf,
        pub offset: u32,
        pub generation: u32,
        pub status: pci_getconf_status,
    }

    pub struct pci_io {
        pub pi_sel: pcisel,
        pub pi_reg: c_int,
        pub pi_width: c_int,
        pub pi_data: u32,
    }

    pub struct pci_bar_io {
        pub pbi_sel: pcisel,
        pub pbi_reg: c_int,
        pub pbi_enabled: c_int,
        pub pbi_base: u64,
        pub pbi_length: u64,
    }

    pub struct pci_vpd_element {
        pub pve_keyword: [c_char; 2],
        pub pve_flags: u8,
        pub pve_datalen: u8,
        pub pve_data: [u8; 0],
    }
}

pub const PVE_FLAG_IDENT: u8 = 0x01;
pub const PVE_FLAG_RW: u8 = 0x02;

f! {
    pub unsafe fn PVE_NEXT(pve: *mut pci_vpd_element) -> *mut pci_vpd_element {
        pve.cast::<u8>()
            .add(size_of::<pci_vpd_element>() + (*pve).pve_datalen as usize)
            .cast::<pci_vpd_element>()
    }
}

s2! {
    pub struct pci_list_vpd_io {
        pub plvi_sel: pcisel,
        pub plvi_len: size_t,
        #[custom_default(ptr::null_mut())]
        pub plvi_data: *mut pci_vpd_element,
    }

    pub struct pci_bar_mmap {
        #[custom_default(ptr::null_mut())]
        pub pbm_map_base: *mut c_void,
        pub pbm_map_length: size_t,
        pub pbm_bar_length: u64,
        pub pbm_bar_off: c_int,
        pub pbm_sel: pcisel,
        pub pbm_reg: c_int,
        pub pbm_flags: c_int,
        pub pbm_memattr: c_int,
    }
}

pub const PCIBARIO_READ: c_int = 0x1;
pub const PCIBARIO_WRITE: c_int = 0x2;

s2! {
    pub struct pci_bar_ioreq {
        pub pbi_sel: pcisel,
        pub pbi_op: c_int,
        pub pbi_bar: u32,
        pub pbi_offset: u32,
        pub pbi_width: u32,
        pub pbi_value: u32,
    }
}

pub const PCIIO_BAR_MMAP_FIXED: c_int = 0x01;
pub const PCIIO_BAR_MMAP_EXCL: c_int = 0x02;
pub const PCIIO_BAR_MMAP_RW: c_int = 0x04;
pub const PCIIO_BAR_MMAP_ACTIVATE: c_int = 0x08;

pub const PCIOCREAD: c_ulong = _IOWR::<pci_io>(b'p' as c_ulong, 2);
pub const PCIOCWRITE: c_ulong = _IOWR::<pci_io>(b'p' as c_ulong, 3);
pub const PCIOCATTACHED: c_ulong = _IOWR::<pci_io>(b'p' as c_ulong, 4);
pub const PCIOCGETCONF: c_ulong = _IOWR::<pci_conf_io>(b'p' as c_ulong, 5);
pub const PCIOCGETBAR: c_ulong = _IOWR::<pci_bar_io>(b'p' as c_ulong, 6);
pub const PCIOCLISTVPD: c_ulong = _IOWR::<pci_list_vpd_io>(b'p' as c_ulong, 7);
pub const PCIOCBARMMAP: c_ulong = _IOWR::<pci_bar_mmap>(b'p' as c_ulong, 8);
pub const PCIOCBARIO: c_ulong = _IOWR::<pci_bar_ioreq>(b'p' as c_ulong, 9);
