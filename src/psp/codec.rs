extern {
    pub fn sceVideocodecOpen(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceVideocodecGetEDRAM(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceVideocodecInit(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceVideocodecDecode(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceVideocodecReleaseEDRAM(buffer: *mut u32) -> i32;
}

pub enum AudioCodec {
    At3Plus = 0x00001000,
    At3 = 0x00001001,
    Mp3 = 0x00001002,
    Aac = 0x00001003,
}

extern {
    pub fn sceAudiocodecCheckNeedMem(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceAudiocodecInit(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceAudiocodecDecode(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceAudiocodecGetEDRAM(
        buffer: *mut u32,
        type_: i32,
    ) -> i32;
    pub fn sceAudiocodecReleaseEDRAM(buffer: *mut u32) -> i32;
}
