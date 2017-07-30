pub type T2Foo = u32;
pub type T2Bar = u32;

macro_rules! i {
    ($i:item) => ($i)
}

#[repr(C)]
pub struct T2Baz {
    pub a: i64,
    pub b: u32,
}

pub const T2C: i32 = 5;

i! {
    pub const T2S: &'static str = "b";
}

extern {
    pub fn T2a();
}
