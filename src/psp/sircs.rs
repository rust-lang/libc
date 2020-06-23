

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SircsData {
    pub type_: u8,
    pub cmd: u8,
    pub dev: u16,
}

extern {
    pub fn sceSircsSend(sd: *mut SircsData, count: i32) -> i32;
}
