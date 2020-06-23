use super::c_void;
use super::SceUid;

pub const USB_CAM_PID: i32 = 0x282;
pub const USB_BUS_DRIVER_NAME: &str = "USBBusDriver";
pub const USB_CAM_DRIVER_NAME: &str = "USBCamDriver";
pub const USB_CAM_MIC_DRIVER_NAME: &str = "USBCamMicDriver";
pub const USB_STOR_DRIVER_NAME: &str = "USBStor_Driver";

pub const ACTIVATED: i32 = 0x200;
pub const CONNECTED: i32 = 0x020;
pub const ESTABLISHED: i32 = 0x002;

extern {
    pub fn sceUsbStart(
        driver_name: *const u8,
        size: i32,
        args: *mut c_void,
    ) -> i32;
    pub fn sceUsbStop(
        driver_name: *const u8,
        size: i32,
        args: *mut c_void,
    ) -> i32;
    pub fn sceUsbActivate(pid: u32) -> i32;
    pub fn sceUsbDeactivate(pid: u32) -> i32;
    pub fn sceUsbGetState() -> i32;
    pub fn sceUsbGetDrvState(driver_name: *const u8) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UsbCamSetupStillParam {
    pub size: i32,
    pub resolution: UsbCamResolution,
    pub jpeg_size: i32,
    pub reverse_flags: i32,
    pub delay: UsbCamDelay,
    pub comp_level: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UsbCamSetupStillExParam {
    pub size: i32,
    pub unk: u32,
    pub resolution: UsbCamResolutionEx,
    pub jpeg_size: i32,
    pub comp_level: i32,
    pub unk2: u32,
    pub unk3: u32,
    pub flip: i32,
    pub mirror: i32,
    pub delay: UsbCamDelay,
    pub unk4: [u32; 5usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UsbCamSetupVideoParam {
    pub size: i32,
    pub resolution: UsbCamResolution,
    pub framerate: UsbCamFrameRate,
    pub white_balance: UsbCamWb,
    pub saturation: i32,
    pub brightness: i32,
    pub contrast: i32,
    pub sharpness: i32,
    pub effect_mode: UsbCamEffectMode,
    pub frame_size: i32,
    pub unk: u32,
    pub evl_evel: UsbCamEvLevel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UsbCamSetupVideoExParam {
    pub size: i32,
    pub unk: u32,
    pub resolution: UsbCamResolutionEx,
    pub framerate: UsbCamFrameRate,
    pub unk2: u32,
    pub unk3: u32,
    pub white_balance: UsbCamWb,
    pub saturation: i32,
    pub brightness: i32,
    pub contrast: i32,
    pub sharpness: i32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: [u32; 3usize],
    pub effect_mode: UsbCamEffectMode,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub frame_size: i32,
    pub unk12: u32,
    pub ev_level: UsbCamEvLevel,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum UsbCamResolution {
    Px160_120  = 0,
    Px176_144  = 1,
    Px320_240  = 2,
    Px352_288  = 3,
    Px640_480  = 4,
    Px1024_768 = 5,
    Px1280_960 = 6,
    Px480_272  = 7,
    Px360_272  = 8,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum UsbCamResolutionEx {
    Px160_120  = 0,
    Px176_144  = 1,
    Px320_240  = 2,
    Px352_288  = 3,
    Px360_272  = 4,
    Px480_272  = 5,
    Px640_480  = 6,
    Px1024_768 = 7,
    Px1280_960 = 8,
}

pub const USB_CAM_FLIP: i32 = 1;
pub const USB_CAM_MIRROR: i32 = 0x100;

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum UsbCamDelay {
    NoDelay = 0,
    Delay10Sec = 1,
    Delay20Sec = 2,
    Delay30Sec = 3,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum UsbCamFrameRate {
    Fps3_75 = 0,
    Fps5 = 1,
    Fps7_5 = 2,
    Fps10 = 3,
    Fps15 = 4,
    Fps20 = 5,
    Fps30 = 6,
    Fps60 = 7,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum UsbCamWb {
    Auto = 0,
    Daylight = 1,
    Fluorescent = 2,
    Incadescent = 3,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum UsbCamEffectMode {
    Normal = 0,
    Negative = 1,
    Blackwhite = 2,
    Sepia = 3,
    Blue = 4,
    Red = 5,
    Green = 6,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum UsbCamEvLevel {
    Pos2_0 = 0,
    Pos1_7 = 1,
    Pos1_5 = 2,
    Pos1_3 = 3,
    Pos1_0 = 4,
    Pos0_7 = 5,
    Pos0_5 = 6,
    Pos0_3 = 7,
    Zero = 8,
    Neg0_3,
    Neg0_5,
    Neg0_7,
    Neg1_0,
    Neg1_3,
    Neg1_5,
    Neg1_7,
    Neg2_0,
}

extern {
    pub fn sceUsbCamSetupStill(param: *mut UsbCamSetupStillParam) -> i32;
    pub fn sceUsbCamSetupStillEx(param: *mut UsbCamSetupStillExParam) -> i32;
    pub fn sceUsbCamStillInputBlocking(buf: *mut u8, size: usize) -> i32;
    pub fn sceUsbCamStillInput(buf: *mut u8, size: usize) -> i32;
    pub fn sceUsbCamStillWaitInputEnd() -> i32;
    pub fn sceUsbCamStillPollInputEnd() -> i32;
    pub fn sceUsbCamStillCancelInput() -> i32;
    pub fn sceUsbCamStillGetInputLength() -> i32;
    pub fn sceUsbCamSetupVideo(
        param: *mut UsbCamSetupVideoParam,
        work_area: *mut c_void,
        work_area_size: i32,
    ) -> i32;
    pub fn sceUsbCamSetupVideoEx(
        param: *mut UsbCamSetupVideoExParam,
        work_area: *mut c_void,
        work_area_size: i32,
    ) -> i32;
    pub fn sceUsbCamStartVideo() -> i32;
    pub fn sceUsbCamStopVideo() -> i32;
    pub fn sceUsbCamReadVideoFrameBlocking(buf: *mut u8, size: usize) -> i32;
    pub fn sceUsbCamReadVideoFrame(buf: *mut u8, size: usize) -> i32;
    pub fn sceUsbCamWaitReadVideoFrameEnd() -> i32;
    pub fn sceUsbCamPollReadVideoFrameEnd() -> i32;
    pub fn sceUsbCamGetReadVideoFrameSize() -> i32;
    pub fn sceUsbCamSetSaturation(saturation: i32) -> i32;
    pub fn sceUsbCamSetBrightness(brightness: i32) -> i32;
    pub fn sceUsbCamSetContrast(contrast: i32) -> i32;
    pub fn sceUsbCamSetSharpness(sharpness: i32) -> i32;
    pub fn sceUsbCamSetImageEffectMode(effect_mode: UsbCamEffectMode) -> i32;
    pub fn sceUsbCamSetEvLevel(exposure_level: UsbCamEvLevel) -> i32;
    pub fn sceUsbCamSetReverseMode(reverse_flags: i32) -> i32;
    pub fn sceUsbCamSetZoom(zoom: i32) -> i32;
    pub fn sceUsbCamGetSaturation(saturation: *mut i32) -> i32;
    pub fn sceUsbCamGetBrightness(brightness: *mut i32) -> i32;
    pub fn sceUsbCamGetContrast(contrast: *mut i32) -> i32;
    pub fn sceUsbCamGetSharpness(sharpness: *mut i32) -> i32;
    pub fn sceUsbCamGetImageEffectMode(
        effect_mode: *mut UsbCamEffectMode,
    ) -> i32;
    pub fn sceUsbCamGetEvLevel(exposure_level: *mut UsbCamEvLevel) -> i32;
    pub fn sceUsbCamGetReverseMode(
        reverse_flags: *mut i32,
    ) -> i32;
    pub fn sceUsbCamGetZoom(zoom: *mut i32) -> i32;
    pub fn sceUsbCamAutoImageReverseSW(on: i32) -> i32;
    pub fn sceUsbCamGetAutoImageReverseState() -> i32;
    pub fn sceUsbCamGetLensDirection() -> i32;
}

extern {
    pub fn sceUsbstorBootRegisterNotify(event_flag: SceUid) -> i32;
    pub fn sceUsbstorBootUnregisterNotify(event_flag: u32) -> i32;
    pub fn sceUsbstorBootSetCapacity(size: u32) -> i32;
}
