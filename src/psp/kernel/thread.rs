use super::{SceUid, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DebugProfilerRegs {
    pub enable: u32,
    pub systemck: u32,
    pub cpuck: u32,
    pub internal: u32,
    pub memory: u32,
    pub copz: u32,
    pub vfpu: u32,
    pub sleep: u32,
    pub bus_access: u32,
    pub uncached_load: u32,
    pub uncached_store: u32,
    pub cached_load: u32,
    pub cached_store: u32,
    pub i_miss: u32,
    pub d_miss: u32,
    pub d_writeback: u32,
    pub cop0_inst: u32,
    pub fpu_inst: u32,
    pub vfpu_inst: u32,
    pub local_bus: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelSysClock {
    pub low: u32,
    pub hi: u32,
}

pub type SceKernelThreadEntry = unsafe extern "C" fn(args: usize, argp: *mut c_void) -> i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelThreadOptParam {
    pub size: usize,
    pub stack_mpid: SceUid,
}

pub const THREAD_ATTR_VFPU: i32 = 0x00004000;
pub const THREAD_ATTR_USER: i32 = 0x80000000;
pub const THREAD_ATTR_USBWLAN: i32 = 0xa0000000;
pub const THREAD_ATTR_VSH: i32 = 0xc0000000;
pub const THREAD_ATTR_SCRATCH_SRAM: i32 = 0x00008000;
pub const THREAD_ATTR_NO_FILLSTACK: i32 = 0x00100000;
pub const THREAD_ATTR_CLEAR_STACK: i32 = 0x00200000;

pub const EVENT_WAIT_MULTIPLE: i32 = 0x200;

pub const EVENT_WAIT_AND: i32 = 0;
pub const EVENT_WAIT_OR: i32 = 1;
pub const EVENT_WAIT_CLEAR: i32 = 0x20;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelThreadInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub status: i32,
    pub entry: SceKernelThreadEntry,
    pub stack: *mut c_void,
    pub stack_size: i32,
    pub gp_reg: *mut c_void,
    pub init_priority: i32,
    pub current_priority: i32,
    pub wait_type: i32,
    pub wait_id: SceUid,
    pub wakeup_count: i32,
    pub exit_status: i32,
    pub run_clocks: SceKernelSysClock,
    pub intr_preempt_count: u32,
    pub thread_preempt_count: u32,
    pub release_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelThreadRunStatus {
    pub size: usize,
    pub status: i32,
    pub current_priority: i32,
    pub wait_type: i32,
    pub wait_id: i32,
    pub wakeup_count: i32,
    pub run_clocks: SceKernelSysClock,
    pub intr_preempt_count: u32,
    pub thread_preempt_count: u32,
    pub release_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelSemaOptParam {
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelSemaInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub init_count: i32,
    pub current_count: i32,
    pub max_count: i32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelEventFlagInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub init_pattern: u32,
    pub current_pattern: u32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelEventFlagOptParam {
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelMbxOptParam {
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelMbxInfo {
    pub size: usize,
    pub name: [u8; 32usize],
    pub attr: u32,
    pub num_wait_threads: i32,
    pub num_messages: i32,
    pub first_message: *mut c_void,
}

pub type SceKernelVTimerHandler = unsafe extern "C" fn(
    uid: SceUid,
    arg1: *mut SceKernelSysClock,
    arg2: *mut SceKernelSysClock,
    arg3: *mut c_void,
) -> u32;

pub type SceKernelVTimerHandlerWide = unsafe extern "C" fn(
    uid: SceUid,
    arg1: i64,
    arg2: i64,
    arg3: *mut c_void,
) -> u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelVTimerInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub active: i32,
    pub base: SceKernelSysClock,
    pub current: SceKernelSysClock,
    pub schedule: SceKernelSysClock,
    pub handler: SceKernelVTimerHandler,
    pub common: *mut c_void,
}

pub type SceKernelThreadEventHandler = unsafe extern "C" fn(
    mask: i32,
    thid: SceUid,
    common: *mut c_void
) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelThreadEventHandlerInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub thread_id: SceUid,
    pub mask: i32,
    pub handler: SceKernelThreadEventHandler,
    pub common: *mut c_void,
}

pub type SceKernelAlarmHandler = unsafe extern "C" fn(common: *mut c_void) -> u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelAlarmInfo {
    pub size: usize,
    pub schedule: SceKernelSysClock,
    pub handler: SceKernelAlarmHandler,
    pub common: *mut c_void,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum SceKernelIdListType {
    Thread = 1,
    Semaphore = 2,
    EventFlag = 3,
    Mbox = 4,
    Vpl = 5,
    Fpl = 6,
    Mpipe = 7,
    Callback = 8,
    ThreadEventHandler = 9,
    Alarm = 10,
    VTimer = 11,
    SleepThread = 64,
    DelayThread = 65,
    SuspendThread = 66,
    DormantThread = 67,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelSystemStatus {
    pub size: usize,
    pub status: u32,
    pub idle_clocks: SceKernelSysClock,
    pub comes_out_of_idle_count: u32,
    pub thread_switch_count: u32,
    pub vfpu_switch_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelMppInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub buf_size: i32,
    pub free_size: i32,
    pub num_send_wait_threads: i32,
    pub num_receive_wait_threads: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelVplOptParam {
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelVplInfo {
    pub size: usize,
    pub name: [u8; 32],
    pub attr: u32,
    pub pool_size: i32,
    pub free_size: i32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelFplOptParam {
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelFplInfo {
    pub size: usize,
    pub name: [u8; 32usize],
    pub attr: u32,
    pub block_size: i32,
    pub num_blocks: i32,
    pub free_blocks: i32,
    pub num_wait_threads: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelVTimerOptParam {
    pub size: usize,
}

pub type SceKernelCallbackFunction = unsafe extern "C" fn(
    arg1: i32,
    arg2: i32,
    arg: *mut c_void,
) -> i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceKernelCallbackInfo {
    pub size: usize,
    pub name: [u8; 32usize],
    pub thread_id: SceUid,
    pub callback: SceKernelCallbackFunction,
    pub common: *mut c_void,
    pub notify_count: i32,
    pub notify_arg: i32,
}

extern {
    pub fn sceKernelGetThreadmanIdType(uid: SceUid) -> SceKernelIdListType;
    pub fn sceKernelCreateThread(
        name: *const u8,
        entry: SceKernelThreadEntry,
        init_priority: i32,
        stack_size: i32,
        attr: i32,
        option: *mut SceKernelThreadOptParam,
    ) -> SceUid;
    pub fn sceKernelDeleteThread(thid: SceUid) -> i32;
    pub fn sceKernelStartThread(
        id: SceUid,
        arg_len: usize,
        arg_p: *mut c_void,
    ) -> i32;
    pub fn sceKernelExitThread(status: i32) -> i32;
    pub fn sceKernelExitDeleteThread(status: i32) -> i32;
    pub fn sceKernelTerminateThread(thid: SceUid) -> i32;
    pub fn sceKernelTerminateDeleteThread(thid: SceUid) -> i32;
    pub fn sceKernelSuspendDispatchThread() -> i32;
    pub fn sceKernelResumeDispatchThread(state: i32) -> i32;
    pub fn sceKernelSleepThread() -> i32;
    pub fn sceKernelSleepThreadCB() -> i32;
    pub fn sceKernelWakeupThread(thid: SceUid) -> i32;
    pub fn sceKernelCancelWakeupThread(thid: SceUid) -> i32;
    pub fn sceKernelSuspendThread(thid: SceUid) -> i32;
    pub fn sceKernelResumeThread(thid: SceUid) -> i32;
    pub fn sceKernelWaitThreadEnd(thid: SceUid, timeout: *mut u32) -> i32;
    pub fn sceKernelWaitThreadEndCB(thid: SceUid, timeout: *mut u32) -> i32;
    pub fn sceKernelDelayThread(delay: u32) -> i32;
    pub fn sceKernelDelayThreadCB(delay: u32) -> i32;
    pub fn sceKernelDelaySysClockThread(delay: *mut SceKernelSysClock) -> i32;
    pub fn sceKernelDelaySysClockThreadCB(delay: *mut SceKernelSysClock) -> i32;
    pub fn sceKernelChangeCurrentThreadAttr(
        unknown: i32,
        attr: i32,
    ) -> i32;
    pub fn sceKernelChangeThreadPriority(
        thid: SceUid,
        priority: i32,
    ) -> i32;
    pub fn sceKernelRotateThreadReadyQueue(
        priority: i32,
    ) -> i32;
    pub fn sceKernelReleaseWaitThread(thid: SceUid) -> i32;
    pub fn sceKernelGetThreadId() -> i32;
    pub fn sceKernelGetThreadCurrentPriority() -> i32;
    pub fn sceKernelGetThreadExitStatus(thid: SceUid) -> i32;
    pub fn sceKernelCheckThreadStack() -> i32;
    pub fn sceKernelGetThreadStackFreeSize(thid: SceUid) -> i32;
    pub fn sceKernelReferThreadStatus(
        thid: SceUid,
        info: *mut SceKernelThreadInfo,
    ) -> i32;
    pub fn sceKernelReferThreadRunStatus(
        thid: SceUid,
        status: *mut SceKernelThreadRunStatus,
    ) -> i32;
    pub fn sceKernelCreateSema(
        name: *const u8,
        attr: u32,
        init_val: i32,
        max_val: i32,
        option: *mut SceKernelSemaOptParam,
    ) -> SceUid;
    pub fn sceKernelDeleteSema(sema_id: SceUid) -> i32;
    pub fn sceKernelSignalSema(
        sema_id: SceUid,
        signal: i32,
    ) -> i32;
    pub fn sceKernelWaitSema(
        sema_id: SceUid,
        signal: i32,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelWaitSemaCB(
        sema_id: SceUid,
        signal: i32,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelPollSema(
        sema_id: SceUid,
        signal: i32,
    ) -> i32;
    pub fn sceKernelReferSemaStatus(
        sema_id: SceUid,
        info: *mut SceKernelSemaInfo,
    ) -> i32;
    pub fn sceKernelCreateEventFlag(
        name: *const u8,
        attr: i32,
        bits: i32,
        opt: *mut SceKernelEventFlagOptParam,
    ) -> SceUid;
    pub fn sceKernelSetEventFlag(ev_id: SceUid, bits: u32) -> i32;
    pub fn sceKernelClearEventFlag(ev_id: SceUid, bits: u32) -> i32;
    pub fn sceKernelPollEventFlag(
        ev_id: SceUid,
        bits: u32,
        wait: i32,
        out_bits: *mut u32,
    ) -> i32;
    pub fn sceKernelWaitEventFlag(
        ev_id: SceUid,
        bits: u32,
        wait: i32,
        out_bits: *mut u32,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelWaitEventFlagCB(
        ev_id: SceUid,
        bits: u32,
        wait: i32,
        out_bits: *mut u32,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelDeleteEventFlag(ev_id: SceUid) -> i32;
    pub fn sceKernelReferEventFlagStatus(
        event: SceUid,
        status: *mut SceKernelEventFlagInfo,
    ) -> i32;
    pub fn sceKernelCreateMbx(
        name: *const u8,
        attr: u32,
        option: *mut SceKernelMbxOptParam,
    ) -> SceUid;
    pub fn sceKernelDeleteMbx(mbx_id: SceUid) -> i32;
    pub fn sceKernelSendMbx(
        mbx_id: SceUid,
        message: *mut c_void,
    ) -> i32;
    pub fn sceKernelReceiveMbx(
        mbx_id: SceUid,
        message: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelReceiveMbxCB(
        mbx_id: SceUid,
        message: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelPollMbx(
        mbx_id: SceUid,
        pmessage: *mut *mut c_void,
    ) -> i32;
    pub fn sceKernelCancelReceiveMbx(
        mbx_id: SceUid,
        num: *mut i32,
    ) -> i32;
    pub fn sceKernelReferMbxStatus(
        mbx_id: SceUid,
        info: *mut SceKernelMbxInfo,
    ) -> i32;
    pub fn sceKernelSetAlarm(
        clock: u32,
        handler: SceKernelAlarmHandler,
        common: *mut c_void,
    ) -> SceUid;
    pub fn sceKernelSetSysClockAlarm(
        clock: *mut SceKernelSysClock,
        handler: *mut SceKernelAlarmHandler,
        common: *mut c_void,
    ) -> SceUid;
    pub fn sceKernelCancelAlarm(alarm_id: SceUid) -> i32;
    pub fn sceKernelReferAlarmStatus(
        alarm_id: SceUid,
        info: *mut SceKernelAlarmInfo,
    ) -> i32;
    pub fn sceKernelCreateCallback(
        name: *const u8,
        func: SceKernelCallbackFunction,
        arg: *mut c_void,
    ) -> SceUid;
    pub fn sceKernelReferCallbackStatus(
        cb: SceUid,
        status: *mut SceKernelCallbackInfo,
    ) -> i32;
    pub fn sceKernelDeleteCallback(cb: SceUid) -> i32;
    pub fn sceKernelNotifyCallback(
        cb: SceUid,
        arg2: i32,
    ) -> i32;
    pub fn sceKernelCancelCallback(cb: SceUid) -> i32;
    pub fn sceKernelGetCallbackCount(cb: SceUid) -> i32;
    pub fn sceKernelCheckCallback() -> i32;
    pub fn sceKernelGetThreadmanIdList(
        type_: SceKernelIdListType,
        read_buf: *mut SceUid,
        read_buf_size: i32,
        id_count: *mut i32,
    ) -> i32;
    pub fn sceKernelReferSystemStatus(status: *mut SceKernelSystemStatus) -> i32;
    pub fn sceKernelCreateMsgPipe(
        name: *const u8,
        part: i32,
        attr: i32,
        unk1: *mut c_void,
        opt: *mut c_void,
    ) -> SceUid;
    pub fn sceKernelDeleteMsgPipe(uid: SceUid) -> i32;
    pub fn sceKernelSendMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelSendMsgPipeCB(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelTrySendMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
    ) -> i32;
    pub fn sceKernelReceiveMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelReceiveMsgPipeCB(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelTryReceiveMsgPipe(
        uid: SceUid,
        message: *mut c_void,
        size: u32,
        unk1: i32,
        unk2: *mut c_void,
    ) -> i32;
    pub fn sceKernelCancelMsgPipe(
        uid: SceUid,
        send: *mut i32,
        recv: *mut i32,
    ) -> i32;
    pub fn sceKernelReferMsgPipeStatus(
        uid: SceUid,
        info: *mut SceKernelMppInfo,
    ) -> i32;
    pub fn sceKernelCreateVpl(
        name: *const u8,
        part: i32,
        attr: i32,
        size: u32,
        opt: *mut SceKernelVplOptParam,
    ) -> SceUid;
    pub fn sceKernelDeleteVpl(uid: SceUid) -> i32;
    pub fn sceKernelAllocateVpl(
        uid: SceUid,
        size: u32,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelAllocateVplCB(
        uid: SceUid,
        size: u32,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelTryAllocateVpl(
        uid: SceUid,
        size: u32,
        data: *mut *mut c_void,
    ) -> i32;
    pub fn sceKernelFreeVpl(
        uid: SceUid,
        data: *mut c_void,
    ) -> i32;
    pub fn sceKernelCancelVpl(
        uid: SceUid,
        num: *mut i32,
    ) -> i32;
    pub fn sceKernelReferVplStatus(
        uid: SceUid,
        info: *mut SceKernelVplInfo,
    ) -> i32;
    pub fn sceKernelCreateFpl(
        name: *const u8,
        part: i32,
        attr: i32,
        size: u32,
        blocks: u32,
        opt: *mut SceKernelFplOptParam,
    ) -> i32;
    pub fn sceKernelDeleteFpl(uid: SceUid) -> i32;
    pub fn sceKernelAllocateFpl(
        uid: SceUid,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelAllocateFplCB(
        uid: SceUid,
        data: *mut *mut c_void,
        timeout: *mut u32,
    ) -> i32;
    pub fn sceKernelTryAllocateFpl(
        uid: SceUid,
        data: *mut *mut c_void,
    ) -> i32;
    pub fn sceKernelFreeFpl(
        uid: SceUid,
        data: *mut c_void,
    ) -> i32;
    pub fn sceKernelCancelFpl(
        uid: SceUid,
        pnum: *mut i32,
    ) -> i32;
    pub fn sceKernelReferFplStatus(
        uid: SceUid,
        info: *mut SceKernelFplInfo,
    ) -> i32;
    pub fn sceKernelUSec2SysClock(
        usec: u32,
        clock: *mut SceKernelSysClock,
    ) -> i32;
    pub fn sceKernelUSec2SysClockWide(usec: u32) -> i64;
    pub fn sceKernelSysClock2USec(
        clock: *mut SceKernelSysClock,
        low: *mut u32,
        high: *mut u32,
    ) -> i32;
    pub fn sceKernelSysClock2USecWide(
        clock: i64,
        low: *mut u32,
        high: *mut u32,
    ) -> i32;
    pub fn sceKernelGetSystemTime(time: *mut SceKernelSysClock) -> i32;
    pub fn sceKernelGetSystemTimeWide() -> i64;
    pub fn sceKernelGetSystemTimeLow() -> u32;
    pub fn sceKernelCreateVTimer(
        name: *const u8,
        opt: *mut SceKernelVTimerOptParam,
    ) -> SceUid;
    pub fn sceKernelDeleteVTimer(uid: SceUid) -> i32;
    pub fn sceKernelGetVTimerBase(
        uid: SceUid,
        base: *mut SceKernelSysClock,
    ) -> i32;
    pub fn sceKernelGetVTimerBaseWide(uid: SceUid) -> i64;
    pub fn sceKernelGetVTimerTime(
        uid: SceUid,
        time: *mut SceKernelSysClock,
    ) -> i32;
    pub fn sceKernelGetVTimerTimeWide(uid: SceUid) -> i64;
    pub fn sceKernelSetVTimerTime(
        uid: SceUid,
        time: *mut SceKernelSysClock,
    ) -> i32;
    pub fn sceKernelSetVTimerTimeWide(uid: SceUid, time: i64) -> i64;
    pub fn sceKernelStartVTimer(uid: SceUid) -> i32;
    pub fn sceKernelStopVTimer(uid: SceUid) -> i32;
    pub fn sceKernelSetVTimerHandler(
        uid: SceUid,
        time: *mut SceKernelSysClock,
        handler: SceKernelVTimerHandler,
        common: *mut c_void,
    ) -> i32;
    pub fn sceKernelSetVTimerHandlerWide(
        uid: SceUid,
        time: i64,
        handler: SceKernelVTimerHandlerWide,
        common: *mut c_void,
    ) -> i32;
    pub fn sceKernelCancelVTimerHandler(uid: SceUid) -> i32;
    pub fn sceKernelReferVTimerStatus(
        uid: SceUid,
        info: *mut SceKernelVTimerInfo,
    ) -> i32;
    pub fn sceKernelRegisterThreadEventHandler(
        name: *const u8,
        thread_id: SceUid,
        mask: i32,
        handler: SceKernelThreadEventHandler,
        common: *mut c_void,
    ) -> SceUid;
    pub fn sceKernelReleaseThreadEventHandler(uid: SceUid) -> i32;
    pub fn sceKernelReferThreadEventHandlerStatus(
        uid: SceUid,
        info: *mut SceKernelThreadEventHandlerInfo,
    ) -> i32;
    pub fn sceKernelReferThreadProfiler() -> *mut DebugProfilerRegs;
    pub fn sceKernelReferGlobalProfiler() -> *mut DebugProfilerRegs;
}
