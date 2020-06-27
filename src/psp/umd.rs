pub type UmdCallback = fn(unknown: i32, event: i32) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UmdInfo {
    pub size: u32,
    pub type_: UmdType,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UmdType {
    Game = 0x10,
    Video = 0x20,
    Audio = 0x40,
}

pub const UMD_NOT_PRESENT: i32 = 0x01;
pub const UMD_PRESENT: i32 = 0x02;
pub const UMD_CHANGED: i32 = 0x04;
pub const UMD_INITING: i32 = 0x08;
pub const UMD_INITED: i32 = 0x10;
pub const UMD_READY: i32 = 0x20;

extern "C" {
    pub fn sceUmdCheckMedium() -> i32;
    pub fn sceUmdGetDiscInfo(info: *mut UmdInfo) -> i32;
    pub fn sceUmdActivate(unit: i32, drive: *const u8) -> i32;
    pub fn sceUmdDeactivate(unit: i32, drive: *const u8) -> i32;
    pub fn sceUmdWaitDriveStat(state: i32) -> i32;
    pub fn sceUmdWaitDriveStatWithTimer(
        state: i32,
        timeout: u32,
    ) -> i32;
    pub fn sceUmdWaitDriveStatCB(
        state: i32,
        timeout: u32,
    ) -> i32;
    pub fn sceUmdCancelWaitDriveStat() -> i32;
    pub fn sceUmdGetDriveStat() -> i32;
    pub fn sceUmdGetErrorStat() -> i32;
    pub fn sceUmdRegisterUMDCallBack(cbid: i32) -> i32;
    pub fn sceUmdUnRegisterUMDCallBack(cbid: i32) -> i32;
    pub fn sceUmdReplacePermit() -> i32;
    pub fn sceUmdReplaceProhibit() -> i32;
}
