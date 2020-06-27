use super::c_void;

pub const AUDIO_VOLUME_MAX: u32 = 0x8000;
pub const AUDIO_CHANNEL_MAX: u32 = 8;
pub const AUDIO_NEXT_CHANNEL: i32 = -1;
pub const AUDIO_SAMPLE_MIN: u32 = 64;
pub const AUDIO_SAMPLE_MAX: u32 = 65472;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum AudioFormat {
    Stereo = 0,
    Mono = 0x10,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AudioInputParams {
    pub unknown1: i32,
    pub gain: i32,
    pub unknown2: i32,
    pub unknown3: i32,
    pub unknown4: i32,
    pub unknown5: i32,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum AudioOutputFrequency {
    Khz48 = 48000,
    Khz44_1 = 44100,
    Khz32 = 32000,
    Khz24 = 24000,
    Khz22_05 = 22050,
    Khz16 = 16000,
    Khz12 = 12000,
    Khz11_025 = 11025,
    Khz8 = 8000,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum AudioInputFrequency {
    Khz44_1 = 44100,
    Khz22_05 = 22050,
    Khz11_025 = 11025,
}

extern "C" {
    pub fn sceAudioChReserve(
        channel: i32,
        sample_count: i32,
        format: AudioFormat,
    ) -> i32;
    pub fn sceAudioChRelease(channel: i32) -> i32;
    pub fn sceAudioOutput(channel: i32, vol: i32, buf: *mut c_void) -> i32;
    pub fn sceAudioOutputBlocking(
        channel: i32,
        vol: i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceAudioOutputPanned(
        channel: i32,
        left_vol: i32,
        right_vol: i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceAudioOutputPannedBlocking(
        channel: i32,
        left_vol: i32,
        right_vol: i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceAudioGetChannelRestLen(channel: i32) -> i32;
    pub fn sceAudioGetChannelRestLength(channel: i32) -> i32;
    pub fn sceAudioSetChannelDataLen(channel: i32, sample_count: i32) -> i32;
    pub fn sceAudioChangeChannelConfig(
        channel: i32,
        format: AudioFormat,
    ) -> i32;
    pub fn sceAudioChangeChannelVolume(
        channel: i32,
        left_vol: i32,
        right_vol: i32,
    ) -> i32;
    pub fn sceAudioOutput2Reserve(sample_count: i32) -> i32;
    pub fn sceAudioOutput2Release() -> i32;
    pub fn sceAudioOutput2ChangeLength(sample_count: i32) -> i32;
    pub fn sceAudioOutput2OutputBlocking(vol: i32, buf: *mut c_void) -> i32;
    pub fn sceAudioOutput2GetRestSample() -> i32;
    pub fn sceAudioSRCChReserve(
        sample_count: i32,
        freq: AudioOutputFrequency,
        channels: i32,
    ) -> i32;
    pub fn sceAudioSRCChRelease() -> i32;
    pub fn sceAudioSRCOutputBlocking(vol: i32, buf: *mut c_void) -> i32;
    pub fn sceAudioInputInit(unknown1: i32, gain: i32, unknown2: i32) -> i32;
    pub fn sceAudioInputInitEx(params: *mut AudioInputParams) -> i32;
    pub fn sceAudioInputBlocking(
        sample_count: i32,
        freq: AudioInputFrequency,
        buf: *mut c_void,
    );
    pub fn sceAudioInput(
        sample_count: i32,
        freq: AudioInputFrequency,
        buf: *mut c_void,
    );
    pub fn sceAudioGetInputLength() -> i32;
    pub fn sceAudioWaitInputEnd() -> i32;
    pub fn sceAudioPollInputEnd() -> i32;
}
