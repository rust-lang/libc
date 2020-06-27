use super::{c_void, ScePspDateTime, SceUid};

pub type IoPermissions = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceIoDirent {
    pub d_stat: SceIoStat,
    pub d_name: [u8; 256usize],
    pub d_private: *mut c_void,
    pub dummy: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceIoStat {
    pub st_mode: i32,
    pub st_attr: i32,
    pub st_size: i64,
    pub st_ctime: ScePspDateTime,
    pub st_atime: ScePspDateTime,
    pub st_mtime: ScePspDateTime,
    pub st_private: [u32; 6usize],
}

pub const FIO_S_IFLNK: i32 = 0x4000;
pub const FIO_S_IFDIR: i32 = 0x1000;
pub const FIO_S_IFREG: i32 = 0x2000;
pub const FIO_S_ISUID: i32 = 0x0800;
pub const FIO_S_ISGID: i32 = 0x0400;
pub const FIO_S_ISVTX: i32 = 0x0200;
pub const FIO_S_IRUSR: i32 = 0x0100;
pub const FIO_S_IWUSR: i32 = 0x0080;
pub const FIO_S_IXUSR: i32 = 0x0040;
pub const FIO_S_IRGRP: i32 = 0x0020;
pub const FIO_S_IWGRP: i32 = 0x0010;
pub const FIO_S_IXGRP: i32 = 0x0008;
pub const FIO_S_IROTH: i32 = 0x0004;
pub const FIO_S_IWOTH: i32 = 0x0002;
pub const FIO_S_IXOTH: i32 = 0x0001;

pub const FIO_SO_IFLNK: i32 = 0x0008;
pub const FIO_SO_IFDIR: i32 = 0x0010;
pub const FIO_SO_IFREG: i32 = 0x0020;
pub const FIO_SO_IROTH: i32 = 0x0004;
pub const FIO_SO_IWOTH: i32 = 0x0002;
pub const FIO_SO_IXOTH: i32 = 0x0001;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum IoAssignPerms {
    RdWr = 0,
    RdOnly = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum IoWhence {
    Set = 0,
    Cur = 1,
    End = 2,
}

pub const PSP_O_RD_ONLY: i32 = 0x0001;
pub const PSP_O_WR_ONLY: i32 = 0x0002;
pub const PSP_O_RD_WR: i32 = 0x0003;
pub const PSP_O_NBLOCK: i32 = 0x0004;
pub const PSP_O_DIR: i32 = 0x0008;
pub const PSP_O_APPEND: i32 = 0x0100;
pub const PSP_O_CREAT: i32 = 0x0200;
pub const PSP_O_TRUNC: i32 = 0x0400;
pub const PSP_O_EXCL: i32 = 0x0800;
pub const PSP_O_NO_WAIT: i32 = 0x8000;

extern "C" {
    pub fn sceIoOpen(
        file: *const u8,
        flags: i32,
        permissions: IoPermissions,
    ) -> SceUid;
    pub fn sceIoOpenAsync(
        file: *const u8,
        flags: i32,
        permissions: IoPermissions,
    ) -> SceUid;
    pub fn sceIoClose(fd: SceUid) -> i32;
    pub fn sceIoCloseAsync(fd: SceUid) -> i32;
    pub fn sceIoRead(fd: SceUid, data: *mut c_void, size: u32) -> i32;
    pub fn sceIoReadAsync(fd: SceUid, data: *mut c_void, size: u32) -> i32;
    pub fn sceIoWrite(fd: SceUid, data: *const c_void, size: usize) -> i32;
    pub fn sceIoWriteAsync(fd: SceUid, data: *const c_void, size: u32) -> i32;
    pub fn sceIoLseek(fd: SceUid, offset: i64, whence: IoWhence) -> i64;
    pub fn sceIoLseekAsync(fd: SceUid, offset: i64, whence: IoWhence) -> i32;
    pub fn sceIoLseek32(fd: SceUid, offset: i32, whence: IoWhence) -> i32;
    pub fn sceIoLseek32Async(fd: SceUid, offset: i32, whence: IoWhence)
        -> i32;
    pub fn sceIoRemove(file: *const u8) -> i32;
    pub fn sceIoMkdir(dir: *const u8, mode: IoPermissions) -> i32;
    pub fn sceIoRmdir(path: *const u8) -> i32;
    pub fn sceIoChdir(path: *const u8) -> i32;
    pub fn sceIoRename(oldname: *const u8, newname: *const u8) -> i32;
    pub fn sceIoDopen(dirname: *const u8) -> SceUid;
    pub fn sceIoDread(fd: SceUid, dir: *mut SceIoDirent) -> i32;
    pub fn sceIoDclose(fd: SceUid) -> i32;
    pub fn sceIoDevctl(
        dev: *const u8,
        cmd: u32,
        indata: *mut c_void,
        inlen: i32,
        outdata: *mut c_void,
        outlen: i32,
    ) -> i32;
    pub fn sceIoAssign(
        dev1: *const u8,
        dev2: *const u8,
        dev3: *const u8,
        mode: IoAssignPerms,
        unk1: *mut c_void,
        unk2: i32,
    ) -> i32;
    pub fn sceIoUnassign(dev: *const u8) -> i32;
    pub fn sceIoGetstat(file: *const u8, stat: *mut SceIoStat) -> i32;
    pub fn sceIoChstat(
        file: *const u8,
        stat: *mut SceIoStat,
        bits: i32,
    ) -> i32;
    pub fn sceIoIoctl(
        fd: SceUid,
        cmd: u32,
        indata: *mut c_void,
        inlen: i32,
        outdata: *mut c_void,
        outlen: i32,
    ) -> i32;
    pub fn sceIoIoctlAsync(
        fd: SceUid,
        cmd: u32,
        indata: *mut c_void,
        inlen: i32,
        outdata: *mut c_void,
        outlen: i32,
    ) -> i32;
    pub fn sceIoSync(device: *const u8, unk: u32) -> i32;
    pub fn sceIoWaitAsync(fd: SceUid, res: *mut i64) -> i32;
    pub fn sceIoWaitAsyncCB(fd: SceUid, res: *mut i64) -> i32;
    pub fn sceIoPollAsync(fd: SceUid, res: *mut i64) -> i32;
    pub fn sceIoGetAsyncStat(fd: SceUid, poll: i32, res: *mut i64) -> i32;
    pub fn sceIoCancel(fd: SceUid) -> i32;
    pub fn sceIoGetDevType(fd: SceUid) -> i32;
    pub fn sceIoChangeAsyncPriority(fd: SceUid, pri: i32) -> i32;
    pub fn sceIoSetAsyncCallback(
        fd: SceUid,
        cb: SceUid,
        argp: *mut c_void,
    ) -> i32;
}
