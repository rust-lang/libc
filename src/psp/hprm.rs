pub const PLAY_PAUSE: i32 = 0x1;
pub const FORWARD: i32 = 0x4;
pub const BACK: i32 = 0x8;
pub const VOL_UP: i32 = 0x10;
pub const VOL_DOWN: i32 = 0x20;
pub const HOLD: i32 = 0x80;

extern "C" {
    pub fn sceHprmPeekCurrentKey(key: *mut i32) -> i32;
    pub fn sceHprmPeekLatch(latch: *mut [u32;4]) -> i32;
    pub fn sceHprmReadLatch(latch: *mut [u32;4]) -> i32;
    pub fn sceHprmIsHeadphoneExist() -> i32;
    pub fn sceHprmIsRemoteExist() -> i32;
    pub fn sceHprmIsMicrophoneExist() -> i32;
}
