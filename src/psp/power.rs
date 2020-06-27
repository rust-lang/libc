use super::SceUid;

pub type PowerCallback = extern "C" fn(unknown: i32, power_info: i32);

pub const POWER_INFO_POWER_SWITCH: i32 = 0x80000000;
pub const POWER_INFO_HOLD_SWITCH: i32 = 0x40000000;
pub const POWER_INFO_STANDBY: i32 = 0x00080000;
pub const POWER_INFO_RESUME_COMPLETE: i32 = 0x00040000;
pub const POWER_INFO_RESUMING: i32 = 0x00020000;
pub const POWER_INFO_SUSPENDING: i32 = 0x00010000;
pub const POWER_INFO_AC_POWER: i32 = 0x00001000;
pub const POWER_INFO_BATTERY_LOW: i32 = 0x00000100;
pub const POWER_INFO_BATTERY_EXIST: i32 = 0x00000080;
pub const POWER_INFO_BATTERY_POWER: i32 = 0x0000007;

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum PowerTick {
    All = 0,
    Suspend = 1,
    Display = 6,
}

extern "C" {
    pub fn scePowerRegisterCallback(slot: i32, cbid: SceUid) -> i32;
    pub fn scePowerUnregisterCallback(slot: i32) -> i32;
    pub fn scePowerIsPowerOnline() -> i32;
    pub fn scePowerIsBatteryExist() -> i32;
    pub fn scePowerIsBatteryCharging() -> i32;
    pub fn scePowerGetBatteryChargingStatus() -> i32;
    pub fn scePowerIsLowBattery() -> i32;
    pub fn scePowerGetBatteryLifePercent() -> i32;
    pub fn scePowerGetBatteryLifeTime() -> i32;
    pub fn scePowerGetBatteryTemp() -> i32;
    pub fn scePowerGetBatteryElec() -> i32;
    pub fn scePowerGetBatteryVolt() -> i32;
    pub fn scePowerSetCpuClockFrequency(cpufreq: i32) -> i32;
    pub fn scePowerSetBusClockFrequency(busfreq: i32) -> i32;
    pub fn scePowerGetCpuClockFrequency() -> i32;
    pub fn scePowerGetCpuClockFrequencyInt() -> i32;
    pub fn scePowerGetCpuClockFrequencyFloat() -> f32;
    pub fn scePowerGetBusClockFrequency() -> i32;
    pub fn scePowerGetBusClockFrequencyInt() -> i32;
    pub fn scePowerGetBusClockFrequencyFloat() -> f32;
    pub fn scePowerSetClockFrequency(
        pllfreq: i32,
        cpufreq: i32,
        busfreq: i32,
    ) -> i32;
    pub fn scePowerLock(unknown: i32) -> i32;
    pub fn scePowerUnlock(unknown: i32) -> i32;
    pub fn scePowerTick(t: PowerTick) -> i32;
    pub fn scePowerGetIdleTimer() -> i32;
    pub fn scePowerIdleTimerEnable(unknown: i32) -> i32;
    pub fn scePowerIdleTimerDisable(unknown: i32) -> i32;
    pub fn scePowerRequestStandby() -> i32;
    pub fn scePowerRequestSuspend() -> i32;
}
