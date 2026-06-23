use core::ffi::c_float;

// Union size is 8 bytes.
#[repr(C)]
union Bar {
    pub x: i64,
    pub y: c_float,
}
