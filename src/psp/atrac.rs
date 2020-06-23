use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Atrac3BufferInfo {
    pub puc_write_position_first_buf: *mut u8,
    pub ui_writable_byte_first_buf: u32,
    pub ui_min_write_byte_first_buf: u32,
    pub ui_read_position_first_buf: u32,
    pub puc_write_position_second_buf: *mut u8,
    pub ui_writable_byte_second_buf: u32,
    pub ui_min_write_byte_second_buf: u32,
    pub ui_read_position_second_buf: u32,
}

extern {
    pub fn sceAtracGetAtracID(ui_codec_type: u32) -> i32;
    pub fn sceAtracSetDataAndGetID(
        buf: *mut c_void,
        bufsize: usize,
    ) -> i32;
    pub fn sceAtracDecodeData(
        atrac_id: i32,
        out_samples: *mut u16,
        out_n: *mut i32,
        out_end: *mut i32,
        out_remain_frame: *mut i32,
    ) -> i32;
    pub fn sceAtracGetRemainFrame(
        atrac_id: i32,
        out_remain_frame: *mut i32,
    ) -> i32;
    pub fn sceAtracGetStreamDataInfo(
        atrac_id: i32,
        write_pointer: *mut *mut u8,
        available_bytes: *mut u32,
        read_offset: *mut u32,
    ) -> i32;
    pub fn sceAtracAddStreamData(
        atrac_id: i32,
        bytes_to_add: u32,
    ) -> i32;
    pub fn sceAtracGetBitrate(
        atrac_id: i32,
        out_bitrate: *mut i32,
    ) -> i32;
    pub fn sceAtracSetLoopNum(
        atrac_id: i32,
        nloops: i32,
    ) -> i32;
    pub fn sceAtracReleaseAtracID(atrac_id: i32) -> i32;
    pub fn sceAtracGetNextSample(
        atrac_id: i32,
        out_n: *mut i32,
    ) -> i32;
    pub fn sceAtracGetMaxSample(
        atrac_id: i32,
        out_max: *mut i32,
    ) -> i32;
    pub fn sceAtracGetBufferInfoForReseting(
        atrac_id: i32,
        ui_sample: u32,
        pbuffer_info: *mut Atrac3BufferInfo,
    ) -> i32;
    pub fn sceAtracGetChannel(
        atrac_id: i32,
        pui_channel: *mut u32,
    ) -> i32;
    pub fn sceAtracGetInternalErrorInfo(
        atrac_id: i32,
        pi_result: *mut i32,
    ) -> i32;
    pub fn sceAtracGetLoopStatus(
        atrac_id: i32,
        pi_loop_num: *mut i32,
        pui_loop_status: *mut u32,
    ) -> i32;
    pub fn sceAtracGetNextDecodePosition(
        atrac_id: i32,
        pui_sample_position: *mut u32,
    ) -> i32;
    pub fn sceAtracGetSecondBufferInfo(
        atrac_id: i32,
        pui_position: *mut u32,
        pui_data_byte: *mut u32,
    ) -> i32;
    pub fn sceAtracGetSoundSample(
        atrac_id: i32,
        pi_end_sample: *mut i32,
        pi_loop_start_sample: *mut i32,
        pi_loop_end_sample: *mut i32,
    ) -> i32;
    pub fn sceAtracResetPlayPosition(
        atrac_id: i32,
        ui_sample: u32,
        ui_write_byte_first_buf: u32,
        ui_write_byte_second_buf: u32,
    ) -> i32;
    pub fn sceAtracSetData(
        atrac_id: i32,
        puc_buffer_addr: *mut u8,
        ui_buffer_byte: u32,
    ) -> i32;
    pub fn sceAtracSetHalfwayBuffer(
        atrac_id: i32,
        puc_buffer_addr: *mut u8,
        ui_read_byte: u32,
        ui_buffer_byte: u32,
    ) -> i32;
    pub fn sceAtracSetHalfwayBufferAndGetID(
        puc_buffer_addr: *mut u8,
        ui_read_byte: u32,
        ui_buffer_byte: u32,
    ) -> i32;
    pub fn sceAtracSetSecondBuffer(
        atrac_id: i32,
        puc_second_buffer_addr: *mut u8,
        ui_second_buffer_byte: u32,
    ) -> i32;
}
