use manifest_dep::Byte;

#[cfg(manifest_build_script)]
pub const ANSWER: Byte = 42;

#[repr(C)]
pub struct Pair {
    pub first: Byte,
    pub second: Byte,
}
