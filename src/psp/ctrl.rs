pub const PSP_CTRL_SELECT: i32 = 0x000001;
pub const PSP_CTRL_START: i32 = 0x000008;
pub const PSP_CTRL_UP: i32 = 0x000010;
pub const PSP_CTRL_RIGHT: i32 = 0x000020;
pub const PSP_CTRL_DOWN: i32 = 0x000040;
pub const PSP_CTRL_LEFT: i32 = 0x000080;
pub const PSP_CTRL_LTRIGGER: i32 = 0x000100;
pub const PSP_CTRL_RTRIGGER: i32 = 0x000200;
pub const PSP_CTRL_TRIANGLE: i32 = 0x001000;
pub const PSP_CTRL_CIRCLE: i32 = 0x002000;
pub const PSP_CTRL_CROSS: i32 = 0x004000;
pub const PSP_CTRL_SQUARE: i32 = 0x008000;
pub const PSP_CTRL_HOME: i32 = 0x010000;
pub const PSP_CTRL_HOLD: i32 = 0x020000;
pub const PSP_CTRL_NOTE: i32 = 0x800000;
pub const PSP_CTRL_SCREEN: i32 = 0x400000;
pub const PSP_CTRL_VOLUP: i32 = 0x100000;
pub const PSP_CTRL_VOLDOWN: i32 = 0x200000;
pub const PSP_CTRL_WLAN_UP: i32 = 0x040000;
pub const PSP_CTRL_REMOTE: i32 = 0x080000;
pub const PSP_CTRL_DISC: i32 = 0x1000000;
pub const PSP_CTRL_MS: i32 = 0x2000000;
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum CtrlMode {
    Digital = 0,
    Analaog,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceCtrlData {
    pub timestamp: u32,
    pub buttons: i32,
    pub lx: u8,
    pub ly: u8,
    pub rsrv: [u8; 6],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct SceCtrlLatch {
    pub ui_make: u32,
    pub ui_break: u32,
    pub ui_press: u32,
    pub ui_release: u32,
}

extern "C" {
    pub fn sceCtrlSetSamplingCycle(cycle: i32) -> i32;
    pub fn sceCtrlGetSamplingCycle(pcycle: *mut i32) -> i32;
    pub fn sceCtrlSetSamplingMode(mode: CtrlMode) -> i32;
    pub fn sceCtrlGetSamplingMode(pmode: *mut i32) -> i32;
    pub fn sceCtrlPeekBufferPositive(
        pad_data: *mut SceCtrlData,
        count: i32,
    ) -> i32;
    pub fn sceCtrlPeekBufferNegative(
        pad_data: *mut SceCtrlData,
        count: i32,
    ) -> i32;
    pub fn sceCtrlReadBufferPositive(
        pad_data: *mut SceCtrlData,
        count: i32,
    ) -> i32;
    pub fn sceCtrlReadBufferNegative(
        pad_data: *mut SceCtrlData,
        count: i32,
    ) -> i32;
    pub fn sceCtrlPeekLatch(latch_data: *mut SceCtrlLatch) -> i32;
    pub fn sceCtrlReadLatch(latch_data: *mut SceCtrlLatch) -> i32;
    pub fn sceCtrlSetIdleCancelThreshold(idlereset: i32, idleback: i32)
        -> i32;
    pub fn sceCtrlGetIdleCancelThreshold(
        idlereset: *mut i32,
        idleback: *mut i32,
    ) -> i32;
}
