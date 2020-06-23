use super::{
    c_void,
    ge::{GeContext, GeCommand, GeListState},
    display::DisplayPixelFormat,
    types::{ScePspFMatrix4, ScePspFVector3, ScePspIMatrix4},
};

pub const GU_PI: f32 = 3.141593;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum GuPrimitive {
    Points = 0,
    Lines = 1,
    LineStrip = 2,
    Triangles = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    Sprites = 6,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum PatchPrimitive {
    Points = 0,
    LineStrip = 2,
    TriangleStrip = 4,
}

#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum GuState {
    AlphaTest = 0,
    DepthTest = 1,
    ScissorTest = 2,
    StencilTest = 3,
    Blend = 4,
    CullFace = 5,
    Dither = 6,
    Fog = 7,
    ClipPlanes = 8,
    Texture2D = 9,
    Lighting = 10,
    Light0 = 11,
    Light1 = 12,
    Light2 = 13,
    Light3 = 14,
    LineSmooth = 15,
    PatchCullFace = 16,
    ColorTest = 17,
    ColorLogicOp = 18,
    FaceNormalReverse = 19,
    PatchFace = 20,
    Fragment2X = 21,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum MatrixMode {
    Projection = 0,
    View = 1,
    Model = 2,
    Texture = 3,
}

pub const GU_TEXTURE_8BIT: i32 = 1;
pub const GU_TEXTURE_16BIT: i32 = 2;
pub const GU_TEXTURE_32BITF: i32 = 3;
pub const GU_COLOR_5650: i32 = 4 << 2;
pub const GU_COLOR_5551: i32 = 5 << 2;
pub const GU_COLOR_4444: i32 = 6 << 2;
pub const GU_COLOR_8888: i32 = 7 << 2;
pub const GU_NORMAL_8BIT: i32 = 1 << 5;
pub const GU_NORMAL_16BIT: i32 = 2 << 5;
pub const GU_NORMAL_32BITF: i32 = 3 << 5;
pub const GU_VERTEX_8BIT: i32 = 1 << 7;
pub const GU_VERTEX_16BIT: i32 = 2 << 7;
pub const GU_VERTEX_32BITF: i32 = 3 << 7;
pub const GU_WEIGHT_8BIT: i32 = 1 << 9;
pub const GU_WEIGHT_16BIT: i32 = 2 << 9;
pub const GU_WEIGHT_32BITF: i32 = 3 << 9;
pub const GU_INDEX_8BIT: i32 = 1 << 11;
pub const GU_INDEX_16BIT: i32 = 2 << 11;
pub const GU_WEIGHTS1: i32 = num_weights(1);
pub const GU_WEIGHTS2: i32 = num_weights(2);
pub const GU_WEIGHTS3: i32 = num_weights(3);
pub const GU_WEIGHTS4: i32 = num_weights(4);
pub const GU_WEIGHTS5: i32 = num_weights(5);
pub const GU_WEIGHTS6: i32 = num_weights(6);
pub const GU_WEIGHTS7: i32 = num_weights(7);
pub const GU_WEIGHTS8: i32 = num_weights(8);
pub const GU_VERTICES1: i32 = num_vertices(1);
pub const GU_VERTICES2: i32 = num_vertices(2);
pub const GU_VERTICES3: i32 = num_vertices(3);
pub const GU_VERTICES4: i32 = num_vertices(4);
pub const GU_VERTICES5: i32 = num_vertices(5);
pub const GU_VERTICES6: i32 = num_vertices(6);
pub const GU_VERTICES7: i32 = num_vertices(7);
pub const GU_VERTICES8: i32 = num_vertices(8);
pub const GU_TRANSFORM_2D: i32 = 1 << 23;
pub const GU_TRANSFORM_3D: i32 = 0;

const fn num_weights(n: u32) -> i32 {
    (((n - 1) & 7) << 14) as i32
}

const fn num_vertices(n: u32) -> i32 {
    (((n - 1) & 7) << 18) as i32
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TexturePixelFormat {
    Psm5650 = 0,
    Psm5551 = 1,
    Psm4444 = 2,
    Psm8888 = 3,
    PsmT4 = 4,
    PsmT8 = 5,
    PsmT16 = 6,
    PsmT32 = 7,
    PsmDxt1 = 8,
    PsmDxt3 = 9,
    PsmDxt5 = 10,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SplineMode {
    FillFill = 0,
    OpenFill = 1,
    FillOpen = 2,
    OpenOpen = 3,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum ShadingModel {
    Flat = 0,
    Smooth = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum LogicalOperation {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    Noop = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equiv = 9,
    Inverted = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum TextureFilter {
    Nearest = 0,
    Linear = 1,
    NearestMipmapNearest = 4,
    LinearMipmapNearest = 5,
    NearestMipmapLinear = 6,
    LinearMipmapLinear = 7,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TextureMapMode {
    TextureCoords = 0,
    TextureMatrix = 1,
    EnvironmentMap = 2,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum TextureLevelMode {
    Auto = 0,
    Const = 1,
    Slope = 2,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TextureProjectionMapMode {
    Position = 0,
    Uv = 1,
    NormalizedNormal = 2,
    Normal = 3,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum GuTexWrapMode {
    Repeat = 0,
    Clamp = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum FrontFaceDirection {
    Clockwise = 0,
    CounterClockwise = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum AlphaFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum StencilFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum ColorFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum DepthFunc {
    Never = 0,
    Always,
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

pub const GU_COLOR_BUFFER_BIT: i32 = 1;
pub const GU_STENCIL_BUFFER_BIT: i32 = 2;
pub const GU_DEPTH_BUFFER_BIT: i32 = 4;
pub const GU_FAST_CLEAR_BIT: i32 = 16;

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TextureEffect {
    Modulate = 0,
    Decal = 1,
    Blend = 2,
    Replace = 3,
    Add = 4,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TextureColorComponent {
    Rgb = 0,
    Rgba = 1,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum MipmapLevel {
    None = 0,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum BlendOp {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    Abs = 5,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum BlendSrc {
    SrcColor = 0,
    OneMinusSrcColor = 1,
    SrcAlpha = 2,
    OneMinusSrcAlpha = 3,
    Fix = 10,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum BlendDst {
    DstColor = 0,
    OneMinusDstColor = 1,
    DstAlpha = 4,
    OneMinusDstAlpha = 5,
    Fix = 10,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum StencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    Invert = 3,
    Incr = 4,
    Decr = 5,
}

pub const GU_AMBIENT: i32 = 1;
pub const GU_DIFFUSE: i32 = 2;
pub const GU_SPECULAR: i32 = 4;
pub const GU_UNKNOWN_LIGHT_COMPONENT: i32 = 8;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum LightMode {
    SingleColor = 0,
    SeparateSpecularColor = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum LightType {
    Directional = 0,
    Pointlight = 1,
    Spotlight = 2,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum GuContextType {
    Direct = 0,
    Call = 1,
    Send = 2,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum GuQueueMode {
    Tail = 0,
    Head = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum GuSyncMode {
    Finish = 0,
    Signal = 1,
    Done = 2,
    List = 3,
    Send = 4,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum GuSyncBehavior {
    Wait = 0,
    NoWait = 1,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum GuCallbackId {
    Signal = 1,
    Finish = 4,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SignalBehavior {
    Suspend = 1,
    Continue = 2,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum ClutPixelFormat {
    Psm5650 = 0,
    Psm5551 = 1,
    Psm4444 = 2,
    Psm8888 = 3,
}

pub type GuCallback = Option<extern fn(id: i32, arg: *mut c_void)>;
pub type GuSwapBuffersCallback = Option<extern fn(display: *mut *mut c_void, render: *mut *mut c_void)>;

extern {
    pub fn sceGuDepthBuffer(zbp: *mut c_void, zbw: i32);
    pub fn sceGuDispBuffer(width: i32, height: i32, dispbp: *mut c_void, dispbw: i32);
    pub fn sceGuDrawBuffer(psm: DisplayPixelFormat, fbp: *mut c_void, fbw: i32);
    pub fn sceGuDrawBufferList(psm: DisplayPixelFormat, fbp: *mut c_void, fbw: i32);
    pub fn sceGuDisplay(state: bool) -> bool;
    pub fn sceGuDepthFunc(function: DepthFunc);
    pub fn sceGuDepthMask(mask: i32);

    pub fn sceGuDepthOffset(offset: i32);
    pub fn sceGuDepthRange(near: i32, far: i32);

    pub fn sceGuFog(near: f32, far: f32, color: u32);
    pub fn sceGuInit();
    pub fn sceGuTerm();
    pub fn sceGuBreak(mode: i32);

    pub fn sceGuContinue();
    pub fn sceGuSetCallback(
        signal: GuCallbackId,
        callback: GuCallback,
    ) -> GuCallback;
    pub fn sceGuSignal(behavior: SignalBehavior, signal: i32);
    pub fn sceGuSendCommandf(cmd: GeCommand, argument: f32);
    pub fn sceGuSendCommandi(cmd: GeCommand, argument: i32);
    pub fn sceGuGetMemory(size: i32) -> *mut c_void;
    pub fn sceGuStart(context_type: GuContextType, list: *mut c_void);
    pub fn sceGuFinish() -> i32;
    pub fn sceGuFinishId(id: u32) -> i32;
    pub fn sceGuCallList(list: *const c_void);
    pub fn sceGuCallMode(mode: i32);
    pub fn sceGuCheckList() -> i32;
    pub fn sceGuSendList(mode: GuQueueMode, list: *const c_void, context: *mut GeContext);
    pub fn sceGuSwapBuffers() -> *mut c_void;
    pub fn sceGuSync(mode: GuSyncMode, behavior: GuSyncBehavior) -> GeListState;
    pub fn sceGuDrawArray(
        prim: GuPrimitive,
        vtype: i32,
        count: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );
    pub fn sceGuBeginObject(
        vtype: i32,
        count: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );
    pub fn sceGuEndObject();
    pub fn sceGuSetStatus(state: GuState, status: i32);
    pub fn sceGuGetStatus(state: GuState) -> bool;
    pub fn sceGuSetAllStatus(status: i32);
    pub fn sceGuGetAllStatus() -> i32;
    pub fn sceGuEnable(state: GuState);
    pub fn sceGuDisable(state: GuState);
    pub fn sceGuLight(
        light: i32,
        type_: LightType,
        components: i32,
        position: &ScePspFVector3,
    );
    pub fn sceGuLightAtt(light: i32, atten0: f32, atten1: f32, atten2: f32);
    pub fn sceGuLightColor(light: i32, component: i32, color: u32);
    pub fn sceGuLightMode(mode: LightMode);
    pub fn sceGuLightSpot(light: i32, direction: &ScePspFVector3, exponent: f32, cutoff: f32);
    pub fn sceGuClear(flags: i32);
    pub fn sceGuClearColor(color: u32);
    pub fn sceGuClearDepth(depth: u32);
    pub fn sceGuClearStencil(stencil: u32);
    pub fn sceGuPixelMask(mask: u32);
    pub fn sceGuColor(color: u32);
    pub fn sceGuColorFunc(func: ColorFunc, color: u32, mask: u32);
    pub fn sceGuColorMaterial(components: i32);
    pub fn sceGuAlphaFunc(func: AlphaFunc, value: i32, mask: i32);

    pub fn sceGuAmbient(color: u32);

    pub fn sceGuAmbientColor(color: u32);
    pub fn sceGuBlendFunc(
        op: BlendOp,
        src: BlendSrc,
        dest: BlendDst,
        src_fix: u32,
        dest_fix: u32,
    );
    pub fn sceGuMaterial(components: i32, color: u32);
    pub fn sceGuModelColor(emissive: u32, ambient: u32, diffuse: u32, specular: u32);
    pub fn sceGuStencilFunc(func: StencilFunc, ref_: i32, mask: i32);
    pub fn sceGuStencilOp(
        fail: StencilOperation,
        zfail: StencilOperation,
        zpass: StencilOperation,
    );
    pub fn sceGuSpecular(power: f32);
    pub fn sceGuFrontFace(order: FrontFaceDirection);
    pub fn sceGuLogicalOp(op: LogicalOperation);
    pub fn sceGuSetDither(matrix: &ScePspIMatrix4);
    pub fn sceGuShadeModel(mode: ShadingModel);
    pub fn sceGuCopyImage(
        psm: DisplayPixelFormat,
        sx: i32,
        sy: i32,
        width: i32,
        height: i32,
        srcw: i32,
        src: *mut c_void,
        dx: i32,
        dy: i32,
        destw: i32,
        dest: *mut c_void,
    );
    pub fn sceGuTexEnvColor(color: u32);
    pub fn sceGuTexFilter(min: TextureFilter, mag: TextureFilter);
    pub fn sceGuTexFlush();
    pub fn sceGuTexFunc(tfx: TextureEffect, tcc: TextureColorComponent);
    pub fn sceGuTexImage(mipmap: MipmapLevel, width: i32, height: i32, tbw: i32, tbp: *const c_void);
    pub fn sceGuTexLevelMode(mode: TextureLevelMode, bias: f32);
    pub fn sceGuTexMapMode(mode: TextureMapMode, a1: u32, a2: u32);
    pub fn sceGuTexMode(tpsm: TexturePixelFormat, maxmips: i32, a2: i32, swizzle: i32);
    pub fn sceGuTexOffset(u: f32, v: f32);
    pub fn sceGuTexProjMapMode(mode: TextureProjectionMapMode);
    pub fn sceGuTexScale(u: f32, v: f32);

    pub fn sceGuTexSlope(slope: f32);
    pub fn sceGuTexSync();
    pub fn sceGuTexWrap(u: GuTexWrapMode, v: GuTexWrapMode);
    pub fn sceGuClutLoad(num_blocks: i32, cbp: *const c_void);
    pub fn sceGuClutMode(cpsm: ClutPixelFormat, shift: u32, mask: u32, a3: u32);
    pub fn sceGuOffset(x: u32, y: u32);
    pub fn sceGuScissor(x: i32, y: i32, w: i32, h: i32);
    pub fn sceGuViewport(cx: i32, cy: i32, width: i32, height: i32);
    pub fn sceGuDrawBezier(
        v_type: i32,
        u_count: i32,
        v_count: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );
    pub fn sceGuPatchDivide(ulevel: u32, vlevel: u32);

    pub fn sceGuPatchFrontFace(a0: u32);
    pub fn sceGuPatchPrim(prim: PatchPrimitive);

    pub fn sceGuDrawSpline(
        v_type: i32,
        u_count: i32,
        v_count: i32,
        u_edge: i32,
        v_edge: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );
    pub fn sceGuSetMatrix(type_: MatrixMode, matrix: &ScePspFMatrix4);
    pub fn sceGuBoneMatrix(index: u32, matrix: &ScePspFMatrix4);
    pub fn sceGuMorphWeight(index: i32, weight: f32);
    pub fn sceGuDrawArrayN(
        primitive_type: GuPrimitive,
        v_type: i32,
        count: i32,
        a3: i32,
        indices: *const c_void,
        vertices: *const c_void,
    );
}
