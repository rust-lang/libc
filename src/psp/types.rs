#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspSRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspL64Rect {
    pub x: u64,
    pub y: u64,
    pub w: u64,
    pub h: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspFRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspSVector2 {
    pub x: i16,
    pub y: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIVector2 {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspL64Vector2 {
    pub x: u64,
    pub y: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspFVector2 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ScePspVector2 {
    pub fv: ScePspFVector2,
    pub iv: ScePspIVector2,
    pub f: [f32; 2usize],
    pub i: [i32; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspSVector3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIVector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspL64Vector3 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}
#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct ScePspFVector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ScePspVector3 {
    pub fv: ScePspFVector3,
    pub iv: ScePspIVector3,
    pub f: [f32; 3usize],
    pub i: [i32; 3usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspSVector4 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIVector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspL64Vector4 {
    pub x: u64,
    pub y: u64,
    pub z: u64,
    pub w: u64,
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct ScePspFVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspFVector4Unaligned {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub union ScePspVector4 {
    pub fv: ScePspFVector4,
    pub iv: ScePspIVector4,
    pub qw: u128,
    pub f: [f32; 4usize],
    pub i: [i32; 4usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIMatrix2 {
    pub x: ScePspIVector2,
    pub y: ScePspIVector2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspFMatrix2 {
    pub x: ScePspFVector2,
    pub y: ScePspFVector2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ScePspMatrix2 {
    pub fm: ScePspFMatrix2,
    pub im: ScePspIMatrix2,
    pub fv: [ScePspFVector2; 2usize],
    pub iv: [ScePspIVector2; 2usize],
    pub v: [ScePspVector2; 2usize],
    pub f: [[f32; 2usize]; 2usize],
    pub i: [[i32; 2usize]; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIMatrix3 {
    pub x: ScePspIVector3,
    pub y: ScePspIVector3,
    pub z: ScePspIVector3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspFMatrix3 {
    pub x: ScePspFVector3,
    pub y: ScePspFVector3,
    pub z: ScePspFVector3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ScePspMatrix3 {
    pub fm: ScePspFMatrix3,
    pub im: ScePspIMatrix3,
    pub fv: [ScePspFVector3; 3usize],
    pub iv: [ScePspIVector3; 3usize],
    pub v: [ScePspVector3; 3usize],
    pub f: [[f32; 3usize]; 3usize],
    pub i: [[i32; 3usize]; 3usize],
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct ScePspIMatrix4 {
    pub x: ScePspIVector4,
    pub y: ScePspIVector4,
    pub z: ScePspIVector4,
    pub w: ScePspIVector4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspIMatrix4Unaligned {
    pub x: ScePspIVector4,
    pub y: ScePspIVector4,
    pub z: ScePspIVector4,
    pub w: ScePspIVector4,
}

#[repr(C, align(16))]
#[derive(Copy, Clone)]
pub struct ScePspFMatrix4 {
    pub x: ScePspFVector4,
    pub y: ScePspFVector4,
    pub z: ScePspFVector4,
    pub w: ScePspFVector4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspFMatrix4Unaligned {
    pub x: ScePspFVector4,
    pub y: ScePspFVector4,
    pub z: ScePspFVector4,
    pub w: ScePspFVector4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ScePspMatrix4 {
    pub fm: ScePspFMatrix4,
    pub im: ScePspIMatrix4,
    pub fv: [ScePspFVector4; 4usize],
    pub iv: [ScePspIVector4; 4usize],
    pub v: [ScePspVector4; 4usize],
    pub f: [[f32; 4usize]; 4usize],
    pub i: [[i32; 4usize]; 4usize],
}
