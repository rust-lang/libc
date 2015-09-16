pub type T2Foo = u32;
pub type T2Bar = u32;

#[repr(C)]
pub struct T2Baz {
    pub a: u8,
    pub b: i32,
}

pub const T2C: i32 = 5;

extern {
    pub fn T2a();
}
