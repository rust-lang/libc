macro_rules! vector {
    ($name:ident, $ty:ty) => {
        #[repr(C)]
        struct $name {
            pub x: $ty,
            pub y: $ty,
        }
    };
}

vector!(VecU8, u8);
vector!(VecU16, u16);
