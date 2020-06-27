use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelLoadExecParam {
    pub size: usize,
    pub args: usize,
    pub argp: *mut c_void,
    pub key: *const u8,
}

extern "C" {
    pub fn sceKernelExitGame();
    pub fn sceKernelRegisterExitCallback(id: SceUid) -> i32;
    pub fn sceKernelLoadExec(
        file: *const u8,
        param: *mut SceKernelLoadExecParam,
    ) -> i32;
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SceUid(pub i32);

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum SceSysMemPartitionId {
    SceKernelUnknownPartition = 0,
    SceKernelPrimaryKernelPartition = 1,
    SceKernelPrimaryUserPartition = 2,
    SceKernelOtherKernelPartition1 = 3,
    SceKernelOtherKernelPartition2 = 4,
    SceKernelVshellPARTITION = 5,
    SceKernelScUserPartition = 6,
    SceKernelMeUserPartition = 7,
    SceKernelExtendedScKernelPartition = 8,
    SceKernelExtendedSc2KernelPartition = 9,
    SceKernelExtendedMeKernelPartition = 10,
    SceKernelVshellKernelPartition = 11,
    SceKernelExtendedKernelPartition = 12,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum SceSysMemBlockTypes {
    Low = 0,
    High,
    Addr,
}

extern "C" {
    pub fn sceKernelAllocPartitionMemory(
        partition: SceSysMemPartitionId,
        name: *const u8,
        type_: SceSysMemBlockTypes,
        size: u32,
        addr: *mut c_void,
    ) -> SceUid;
    pub fn sceKernelGetBlockHeadAddr(blockid: SceUid) -> *mut c_void;
    pub fn sceKernelFreePartitionMemory(blockid: SceUid) -> i32;
    pub fn sceKernelTotalFreeMemSize() -> usize;
    pub fn sceKernelMaxFreeMemSize() -> usize;
    pub fn sceKernelDevkitVersion() -> u32;
    pub fn sceKernelSetCompiledSdkVersion(version: u32) -> i32;
    pub fn sceKernelGetCompiledSdkVersion() -> u32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timeval {
    pub tv_sec: i32,
    pub tv_usec: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timezone {
    pub tz_minutes_west: i32,
    pub tz_dst_time: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelUtilsSha1Context {
    pub h: [u32; 5usize],
    pub us_remains: u16,
    pub us_computed: u16,
    pub ull_total_len: u64,
    pub buf: [u8; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelUtilsMt19937Context {
    pub count: u32,
    pub state: [u32; 624usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelUtilsMd5Context {
    pub h: [u32; 4usize],
    pub pad: u32,
    pub us_remains: u16,
    pub us_computed: u16,
    pub ull_total_len: u64,
    pub buf: [u8; 64usize],
}

extern "C" {
    pub fn sceKernelLibcTime(t: *mut i32) -> i32;
    pub fn sceKernelLibcClock() -> u32;
    pub fn sceKernelLibcGettimeofday(
        tp: *mut timeval,
        tzp: *mut timezone,
    ) -> i32;
    pub fn sceKernelDcacheWritebackAll();
    pub fn sceKernelDcacheWritebackInvalidateAll();
    pub fn sceKernelDcacheWritebackRange(p: *const c_void, size: u32);
    pub fn sceKernelDcacheWritebackInvalidateRange(
        p: *const c_void,
        size: u32,
    );
    pub fn sceKernelDcacheInvalidateRange(p: *const c_void, size: u32);
    pub fn sceKernelIcacheInvalidateAll();
    pub fn sceKernelIcacheInvalidateRange(p: *const c_void, size: u32);
    pub fn sceKernelUtilsMt19937Init(
        ctx: *mut SceKernelUtilsMt19937Context,
        seed: u32,
    ) -> i32;
    pub fn sceKernelUtilsMt19937UInt(
        ctx: *mut SceKernelUtilsMt19937Context,
    ) -> u32;
    pub fn sceKernelUtilsMd5Digest(
        data: *mut u8,
        size: u32,
        digest: *mut u8,
    ) -> i32;
    pub fn sceKernelUtilsMd5BlockInit(
        ctx: *mut SceKernelUtilsMd5Context,
    ) -> i32;
    pub fn sceKernelUtilsMd5BlockUpdate(
        ctx: *mut SceKernelUtilsMd5Context,
        data: *mut u8,
        size: u32,
    ) -> i32;
    pub fn sceKernelUtilsMd5BlockResult(
        ctx: *mut SceKernelUtilsMd5Context,
        digest: *mut u8,
    ) -> i32;
    pub fn sceKernelUtilsSha1Digest(
        data: *mut u8,
        size: u32,
        digest: *mut u8,
    ) -> i32;
    pub fn sceKernelUtilsSha1BlockInit(
        ctx: *mut SceKernelUtilsSha1Context,
    ) -> i32;
    pub fn sceKernelUtilsSha1BlockUpdate(
        ctx: *mut SceKernelUtilsSha1Context,
        data: *mut u8,
        size: u32,
    ) -> i32;
    pub fn sceKernelUtilsSha1BlockResult(
        ctx: *mut SceKernelUtilsSha1Context,
        digest: *mut u8,
    ) -> i32;
}

#[derive(Copy, Clone)]
#[repr(packed, C)]
pub struct IntrHandlerOptionParam {
    size: i32,
    entry: u32,
    common: u32,
    gp: u32,
    intr_code: u16,
    sub_count: u16,
    intr_level: u16,
    enabled: u16,
    calls: u32,
    field_1c: u32,
    total_clock_lo: u32,
    total_clock_hi: u32,
    min_clock_lo: u32,
    min_clock_hi: u32,
    max_clock_lo: u32,
    max_clock_hi: u32,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum Interrupt {
    Gpio = 4,
    Ata = 5,
    Umd = 6,
    Mscm0 = 7,
    Wlan = 8,
    Audio = 10,
    I2c = 12,
    Sircs = 14,
    Systimer0 = 15,
    Systimer1 = 16,
    Systimer2 = 17,
    Systimer3 = 18,
    Thread0 = 19,
    Nand = 20,
    Dmacplus = 21,
    Dma0 = 22,
    Dma1 = 23,
    Memlmd = 24,
    Ge = 25,
    Vblank = 30,
    Mecodec = 31,
    Hpremote = 36,
    Mscm1 = 60,
    Mscm2 = 61,
    Thread1 = 65,
    Interrupt = 66,
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum SubInterrupt {
    Gpio = Interrupt::Gpio as u32,
    Ata = Interrupt::Ata as u32,
    Umd = Interrupt::Umd as u32,
    Dmacplus = Interrupt::Dmacplus as u32,
    Ge = Interrupt::Ge as u32,
    Display = Interrupt::Vblank as u32,
}

extern "C" {
    pub fn sceKernelRegisterSubIntrHandler(
        int_no: i32,
        no: i32,
        handler: *mut c_void,
        arg: *mut c_void,
    ) -> i32;
    pub fn sceKernelReleaseSubIntrHandler(int_no: i32, no: i32) -> i32;
    pub fn sceKernelEnableSubIntr(int_no: i32, no: i32) -> i32;
    pub fn sceKernelDisableSubIntr(int_no: i32, no: i32) -> i32;
    pub fn QueryIntrHandlerInfo(
        intr_code: SceUid,
        sub_intr_code: SceUid,
        data: *mut IntrHandlerOptionParam,
    ) -> i32;
}

extern "C" {
    pub fn sceKernelCpuSuspendIntr() -> u32;
    pub fn sceKernelCpuResumeIntr(flags: u32);
    pub fn sceKernelCpuResumeIntrWithSync(flags: u32);
    pub fn sceKernelIsCpuIntrSuspended(flags: u32) -> i32;
    pub fn sceKernelIsCpuIntrEnable() -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelLMOption {
    pub size: usize,
    pub m_pid_text: SceUid,
    pub m_pid_data: SceUid,
    pub flags: u32,
    pub position: u8,
    pub access: u8,
    pub c_reserved: [u8; 2usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelSMOption {
    pub size: usize,
    pub m_pid_stack: SceUid,
    pub stack_size: usize,
    pub priority: i32,
    pub attribute: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelModuleInfo {
    pub size: usize,
    pub n_segment: u8,
    pub reserved: [u8; 3usize],
    pub segment_addr: [i32; 4usize],
    pub segment_size: [i32; 4usize],
    pub entry_addr: u32,
    pub gp_value: u32,
    pub text_addr: u32,
    pub text_size: u32,
    pub data_size: u32,
    pub bss_size: u32,
    pub attribute: u16,
    pub version: [u8; 2usize],
    pub name: [u8; 28usize],
}

extern "C" {
    pub fn sceKernelLoadModule(
        path: *const u8,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;
    pub fn sceKernelLoadModuleMs(
        path: *const u8,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;
    pub fn sceKernelLoadModuleByID(
        fid: SceUid,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;
    pub fn sceKernelLoadModuleBufferUsbWlan(
        buf_size: usize,
        buf: *mut c_void,
        flags: i32,
        option: *mut SceKernelLMOption,
    ) -> SceUid;
    pub fn sceKernelStartModule(
        mod_id: SceUid,
        arg_size: usize,
        argp: *mut c_void,
        status: *mut i32,
        option: *mut SceKernelSMOption,
    ) -> i32;
    pub fn sceKernelStopModule(
        mod_id: SceUid,
        arg_size: usize,
        argp: *mut c_void,
        status: *mut i32,
        option: *mut SceKernelSMOption,
    ) -> i32;
    pub fn sceKernelUnloadModule(mod_id: SceUid) -> i32;
    pub fn sceKernelSelfStopUnloadModule(
        unknown: i32,
        arg_size: usize,
        argp: *mut c_void,
    ) -> i32;
    pub fn sceKernelStopUnloadSelfModule(
        arg_size: usize,
        argp: *mut c_void,
        status: *mut i32,
        option: *mut SceKernelSMOption,
    ) -> i32;
    pub fn sceKernelQueryModuleInfo(
        mod_id: SceUid,
        info: *mut SceKernelModuleInfo,
    ) -> i32;
    pub fn sceKernelGetModuleIdList(
        read_buf: *mut SceUid,
        read_buf_size: i32,
        id_count: *mut i32,
    ) -> i32;
}

extern "C" {
    pub fn sceKernelVolatileMemLock(
        unk: i32,
        ptr: *mut *mut c_void,
        size: *mut i32,
    ) -> i32;
    pub fn sceKernelVolatileMemTryLock(
        unk: i32,
        ptr: *mut *mut c_void,
        size: *mut i32,
    ) -> i32;
    pub fn sceKernelVolatileMemUnlock(unk: i32) -> i32;
}

extern "C" {
    pub fn sceKernelStdin() -> SceUid;
    pub fn sceKernelStdout() -> SceUid;
    pub fn sceKernelStderr() -> SceUid;
}

mod thread;
pub use self::thread::*;
