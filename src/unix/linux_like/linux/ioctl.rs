const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;

// https://github.com/search?q=repo%3Atorvalds%2Flinux+%22%23define+_IOC_NONE%22&type=code
cfg_if! {
    if #[cfg(any(
            any(target_arch = "powerpc", target_arch = "powerpc64"),
            any(target_arch = "sparc", target_arch = "sparc64"),
            any(target_arch = "mips", target_arch = "mips64"),
        ))]
    {
        // https://github.com/torvalds/linux/blob/b311c1b497e51a628aa89e7cb954481e5f9dced2/arch/powerpc/include/uapi/asm/ioctl.h
        // https://github.com/torvalds/linux/blob/b311c1b497e51a628aa89e7cb954481e5f9dced2/arch/sparc/include/uapi/asm/ioctl.h
        // https://github.com/torvalds/linux/blob/b311c1b497e51a628aa89e7cb954481e5f9dced2/arch/mips/include/uapi/asm/ioctl.h

        const _IOC_SIZEBITS: u32 = 13;
        const _IOC_DIRBITS: u32 = 3;

        const _IOC_NONE: u32 = 1;
        const _IOC_READ: u32 = 2;
        const _IOC_WRITE: u32 = 4;
    } else {
        // https://github.com/torvalds/linux/blob/b311c1b497e51a628aa89e7cb954481e5f9dced2/include/uapi/asm-generic/ioctl.h

        const _IOC_SIZEBITS: u32 = 14;
        const _IOC_DIRBITS: u32 = 2;

        const _IOC_NONE: u32 = 0;
        const _IOC_WRITE: u32 = 1;
        const _IOC_READ: u32 = 2;
    }
}

const _IOC_NRMASK: u32 = (1 << _IOC_NRBITS) - 1;
const _IOC_TYPEMASK: u32 = (1 << _IOC_TYPEBITS) - 1;
const _IOC_SIZEMASK: u32 = (1 << _IOC_SIZEBITS) - 1;
const _IOC_DIRMASK: u32 = (1 << _IOC_DIRBITS) - 1;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

// adopted from https://github.com/torvalds/linux/blob/8a696a29c6905594e4abf78eaafcb62165ac61f1/rust/kernel/ioctl.rs

/// Build an ioctl number, analogous to the C macro of the same name.
#[inline(always)]
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    debug_assert!(dir <= _IOC_DIRMASK);
    debug_assert!(ty <= _IOC_TYPEMASK);
    debug_assert!(nr <= _IOC_NRMASK);
    debug_assert!(size <= (_IOC_SIZEMASK as usize));

    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((size as u32) << _IOC_SIZESHIFT)
}

/// Build an ioctl number for an argumentless ioctl.
#[inline(always)]
pub const fn _IO(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, ty, nr, 0)
}

/// Build an ioctl number for an read-only ioctl.
#[inline(always)]
pub const fn _IOR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, ty, nr, core::mem::size_of::<T>())
}

/// Build an ioctl number for an write-only ioctl.
#[inline(always)]
pub const fn _IOW<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_WRITE, ty, nr, core::mem::size_of::<T>())
}

/// Build an ioctl number for a read-write ioctl.
#[inline(always)]
pub const fn _IOWR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ | _IOC_WRITE, ty, nr, core::mem::size_of::<T>())
}
