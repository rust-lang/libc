use super::c_void;

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SceMpeg(*mut *mut c_void);

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SceMpegStream(*mut c_void);
pub type SceMpegRingbufferCb = Option<
    unsafe extern "C" fn(
        data: *mut c_void,
        num_packets: i32,
        param: *mut c_void,
    ) -> i32,
>;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceMpegRingbuffer {
    pub packets: i32,
    pub unk0: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub data: *mut c_void,
    pub callback: SceMpegRingbufferCb,
    pub cb_param: *mut c_void,
    pub unk4: u32,
    pub unk5: u32,
    pub sce_mpeg: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceMpegAu {
    pub pts_msb: u32,
    pub pts: u32,
    pub dts_msb: u32,
    pub dts: u32,
    pub es_buffer: u32,
    pub au_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceMpegAvcMode {
    pub unk0: i32,
    pub pixel_format: super::DisplayPixelFormat,
}

extern "C" {
    pub fn sceMpegInit() -> i32;
    pub fn sceMpegFinish();
    pub fn sceMpegRingbufferQueryMemSize(packets: i32) -> i32;
    pub fn sceMpegRingbufferConstruct(
        ringbuffer: *mut SceMpegRingbuffer,
        packets: i32,
        data: *mut c_void,
        size: i32,
        callback: SceMpegRingbufferCb,
        cb_param: *mut c_void,
    ) -> i32;
    pub fn sceMpegRingbufferDestruct(ringbuffer: *mut SceMpegRingbuffer);
    pub fn sceMpegRingbufferAvailableSize(
        ringbuffer: *mut SceMpegRingbuffer,
    ) -> i32;
    pub fn sceMpegRingbufferPut(
        ringbuffer: *mut SceMpegRingbuffer,
        num_packets: i32,
        available: i32,
    ) -> i32;
    pub fn sceMpegQueryMemSize(unk: i32) -> i32;
    pub fn sceMpegCreate(
        handle: SceMpeg,
        data: *mut c_void,
        size: i32,
        ringbuffer: *mut SceMpegRingbuffer,
        frame_width: i32,
        unk1: i32,
        unk2: i32,
    ) -> i32;
    pub fn sceMpegDelete(handle: SceMpeg);
    pub fn sceMpegQueryStreamOffset(
        handle: SceMpeg,
        buffer: *mut c_void,
        offset: *mut i32,
    ) -> i32;
    pub fn sceMpegQueryStreamSize(buffer: *mut c_void, size: *mut i32) -> i32;
    pub fn sceMpegRegistStream(
        handle: SceMpeg,
        stream_id: i32,
        unk: i32,
    ) -> SceMpegStream;
    pub fn sceMpegUnRegistStream(handle: SceMpeg, stream: SceMpegStream);
    pub fn sceMpegFlushAllStream(handle: SceMpeg) -> i32;
    pub fn sceMpegMallocAvcEsBuf(handle: SceMpeg) -> *mut c_void;
    pub fn sceMpegFreeAvcEsBuf(handle: SceMpeg, buf: *mut c_void);
    pub fn sceMpegQueryAtracEsSize(
        handle: SceMpeg,
        es_size: *mut i32,
        out_size: *mut i32,
    ) -> i32;
    pub fn sceMpegInitAu(
        handle: SceMpeg,
        es_buffer: *mut c_void,
        au: *mut SceMpegAu,
    ) -> i32;
    pub fn sceMpegGetAvcAu(
        handle: SceMpeg,
        stream: SceMpegStream,
        au: *mut SceMpegAu,
        unk: *mut i32,
    ) -> i32;
    pub fn sceMpegAvcDecodeMode(
        handle: SceMpeg,
        mode: *mut SceMpegAvcMode,
    ) -> i32;
    pub fn sceMpegAvcDecode(
        handle: SceMpeg,
        au: *mut SceMpegAu,
        iframe_width: i32,
        buffer: *mut c_void,
        init: *mut i32,
    ) -> i32;
    pub fn sceMpegAvcDecodeStop(
        handle: SceMpeg,
        frame_width: i32,
        buffer: *mut c_void,
        status: *mut i32,
    ) -> i32;
    pub fn sceMpegGetAtracAu(
        handle: SceMpeg,
        stream: SceMpegStream,
        au: *mut SceMpegAu,
        unk: *mut c_void,
    ) -> i32;
    pub fn sceMpegAtracDecode(
        handle: SceMpeg,
        au: *mut SceMpegAu,
        buffer: *mut c_void,
        init: i32,
    ) -> i32;
}

#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct SceMpegLLI {
    pub src: *mut c_void,
    pub dst: *mut c_void,
    pub next: *mut c_void,
    pub size: i32,
}

#[repr(C)]
#[repr(align(64))]
#[derive(Copy, Clone)]
pub struct SceMpegYCrCbBuffer {
    pub frame_buffer_height16: i32,
    pub frame_buffer_width16: i32,
    pub unknown: i32,
    pub unknown2: i32,
    pub y_buffer: *mut c_void,
    pub y_buffer2: *mut c_void,
    pub cr_buffer: *mut c_void,
    pub cb_buffer: *mut c_void,
    pub cr_buffer2: *mut c_void,
    pub cb_buffer2: *mut c_void,

    pub frame_height: i32,
    pub frame_width: i32,
    pub frame_buffer_width: i32,
    pub unknown3: [i32; 11usize],
}

extern "C" {
    pub fn sceMpegBaseYCrCbCopyVme(
        yuv_buffer: *mut c_void,
        buffer: *mut i32,
        type_: i32,
    ) -> i32;
    pub fn sceMpegBaseCscInit(width: i32) -> i32;
    pub fn sceMpegBaseCscVme(
        rgb_buffer: *mut c_void,
        rgb_buffer2: *mut c_void,
        width: i32,
        y_cr_cb_buffer: *mut SceMpegYCrCbBuffer,
    ) -> i32;
    pub fn sceMpegbase_BEA18F91(lli: *mut SceMpegLLI) -> i32;
}
