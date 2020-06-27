use super::{c_void, GuPrimitive, MatrixMode, ScePspFMatrix4, ScePspFVector3};

extern "C" {
    pub fn sceGumDrawArray(
        prim: GuPrimitive,
        v_type: i32,
        count: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );

    pub fn sceGumDrawArrayN(
        prim: GuPrimitive,
        v_type: i32,
        count: i32,
        a3: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );

    pub fn sceGumDrawBezier(
        v_type: i32,
        u_count: i32,
        v_count: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );

    pub fn sceGumDrawSpline(
        v_type: i32,
        u_count: i32,
        v_count: i32,
        u_edge: i32,
        v_edge: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );

    pub fn sceGumFastInverse();
    pub fn sceGumFullInverse();
    pub fn sceGumLoadIdentity();
    pub fn sceGumLoadMatrix(m: &ScePspFMatrix4);
    pub fn sceGumLookAt(
        eye: &ScePspFVector3,
        center: &ScePspFVector3,
        up: &ScePspFVector3,
    );
    pub fn sceGumMatrixMode(mode: MatrixMode);
    pub fn sceGumMultMatrix(m: &ScePspFMatrix4);
    pub fn sceGumOrtho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    );
    pub fn sceGumPerspective(fovy: f32, aspect: f32, near: f32, far: f32);
    pub fn sceGumPopMatrix();
    pub fn sceGumPushMatrix();
    pub fn sceGumRotateX(angle: f32);
    pub fn sceGumRotateY(angle: f32);
    pub fn sceGumRotateZ(angle: f32);
    pub fn sceGumRotateXYZ(v: &ScePspFVector3);
    pub fn sceGumRotateZYX(v: &ScePspFVector3);
    pub fn sceGumScale(v: &ScePspFVector3);
    pub fn sceGumStoreMatrix(m: &mut ScePspFMatrix4);
    pub fn sceGumTranslate(v: &ScePspFVector3);
    pub fn sceGumUpdateMatrix();
}
