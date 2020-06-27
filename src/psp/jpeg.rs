use super::c_void;
extern "C" {
    pub fn sceJpegInitMJpeg() -> i32;
    pub fn sceJpegFinishMJpeg() -> i32;
    pub fn sceJpegCreateMJpeg(width: i32, height: i32) -> i32;
    pub fn sceJpegDeleteMJpeg() -> i32;
    pub fn sceJpegDecodeMJpeg(
        jpeg_buf: *mut u8,
        size: usize,
        rgba: *mut c_void,
        unk: u32,
    ) -> i32;
}
