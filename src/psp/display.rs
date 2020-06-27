use super::c_void;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DisplayMode {
    Lcd = 0,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum DisplayPixelFormat {
    Psm5650 = 0,
    Psm5551 = 1,
    Psm4444 = 2,
    Psm8888 = 3,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DisplaySetBufSync {
    Immediate = 0,
    NextFrame = 1,
}

extern "C" {
    pub fn sceDisplaySetMode(
        mode: DisplayMode,
        width: usize,
        height: usize,
    ) -> u32;
    pub fn sceDisplayGetMode(
        pmode: *mut i32,
        pwidth: *mut i32,
        pheight: *mut i32,
    ) -> i32;
    pub fn sceDisplaySetFrameBuf(
        top_addr: *const u8,
        buffer_width: usize,
        pixel_format: DisplayPixelFormat,
        sync: DisplaySetBufSync,
    ) -> u32;
    pub fn sceDisplayGetFrameBuf(
        top_addr: *mut *mut c_void,
        buffer_width: *mut usize,
        pixel_format: *mut DisplayPixelFormat,
        sync: DisplaySetBufSync,
    ) -> i32;
    pub fn sceDisplayGetVcount() -> u32;
    pub fn sceDisplayWaitVblank() -> i32;
    pub fn sceDisplayWaitVblankCB() -> i32;
    pub fn sceDisplayWaitVblankStart() -> i32;
    pub fn sceDisplayWaitVblankStartCB() -> i32;
    pub fn sceDisplayGetAccumulatedHcount() -> i32;
    pub fn sceDisplayGetCurrentHcount() -> i32;
    pub fn sceDisplayGetFramePerSec() -> f32;
    pub fn sceDisplayIsForeground() -> i32;
    pub fn sceDisplayIsVblank() -> i32;
}
