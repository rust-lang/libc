#[repr(C)]
pub struct Foo {
    // We use a 4 byte type with 4 byte alignment.
    pub a: i32,
    pub b: i32,
}
