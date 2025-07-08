use std::os::raw::c_char;

macro_rules! vector {
    ($name:ident, $ty:ty) => {
        #[repr(C)]
        struct $name {
            x: $ty,
            y: $ty,
        }
    };
}

vector!(VecU8, u8);
vector!(VecU16, u16);

type string = *const c_char;
