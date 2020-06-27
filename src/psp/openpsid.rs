#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpenPSID {
    pub data: [u8; 16usize],
}

extern "C" {
    pub fn sceOpenPSIDGetOpenPSID(openpsid: *mut OpenPSID) -> i32;
}
