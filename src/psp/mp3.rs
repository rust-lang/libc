use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceMp3InitArg {
    pub mp3_stream_start: u32,
    pub unk1: u32,
    pub mp3_stream_end: u32,
    pub unk2: u32,
    pub mp3_buf: *mut c_void,
    pub mp3_buf_size: i32,
    pub pcm_buf: *mut c_void,
    pub pcm_buf_size: i32,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Handle(pub i32);

extern "C" {
    pub fn sceMp3ReserveMp3Handle(args: *mut SceMp3InitArg) -> i32;
    pub fn sceMp3ReleaseMp3Handle(handle: Handle) -> i32;
    pub fn sceMp3InitResource() -> i32;
    pub fn sceMp3TermResource() -> i32;
    pub fn sceMp3Init(handle: Handle) -> i32;
    pub fn sceMp3Decode(handle: Handle, dst: *mut *mut i16) -> i32;
    pub fn sceMp3GetInfoToAddStreamData(
        handle: Handle,
        dst: *mut *mut u8,
        to_write: *mut i32,
        src_pos: *mut i32,
    ) -> i32;
    pub fn sceMp3NotifyAddStreamData(handle: Handle, size: i32) -> i32;
    pub fn sceMp3CheckStreamDataNeeded(handle: Handle) -> i32;
    pub fn sceMp3SetLoopNum(handle: Handle, loop_: i32) -> i32;
    pub fn sceMp3GetLoopNum(handle: Handle) -> i32;
    pub fn sceMp3GetSumDecodedSample(handle: Handle) -> i32;
    pub fn sceMp3GetMaxOutputSample(handle: Handle) -> i32;
    pub fn sceMp3GetSamplingRate(handle: Handle) -> i32;
    pub fn sceMp3GetBitRate(handle: Handle) -> i32;
    pub fn sceMp3GetMp3ChannelNum(handle: Handle) -> i32;
    pub fn sceMp3ResetPlayPosition(handle: Handle) -> i32;
}
