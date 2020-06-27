#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScePspDateTime {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hour: u16,
    pub minutes: u16,
    pub seconds: u16,
    pub microseconds: u32,
}

#[repr(i32)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub enum RtcCheckValidError {
    InvalidYear = -1,
    InvalidMonth = -2,
    InvalidDay = -3,
    InvalidHour = -4,
    InvalidMinutes = -5,
    InvalidSeconds = -6,
    InvalidMicroseconds = -7,
}

extern "C" {
    pub fn sceRtcGetTickResolution() -> u32;
    pub fn sceRtcGetCurrentTick(tick: *mut u64) -> i32;
    pub fn sceRtcGetCurrentClock(tm: *mut ScePspDateTime, tz: i32) -> i32;
    pub fn sceRtcGetCurrentClockLocalTime(tm: *mut ScePspDateTime) -> i32;
    pub fn sceRtcConvertUtcToLocalTime(
        tick_utc: *const u64,
        tick_local: *mut u64,
    ) -> i32;
    pub fn sceRtcConvertLocalTimeToUTC(
        tick_local: *const u64,
        tick_utc: *mut u64,
    ) -> i32;
    pub fn sceRtcIsLeapYear(year: i32) -> i32;
    pub fn sceRtcGetDaysInMonth(year: i32, month: i32) -> i32;
    pub fn sceRtcGetDayOfWeek(year: i32, month: i32, day: i32) -> i32;
    pub fn sceRtcCheckValid(date: *const ScePspDateTime) -> i32;
    pub fn sceRtcSetTick(date: *mut ScePspDateTime, tick: *const u64) -> i32;
    pub fn sceRtcGetTick(date: *const ScePspDateTime, tick: *mut u64) -> i32;
    pub fn sceRtcCompareTick(tick1: *const u64, tick2: *const u64) -> i32;
    pub fn sceRtcTickAddTicks(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_ticks: u64,
    ) -> i32;
    pub fn sceRtcTickAddMicroseconds(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_ms: u64,
    ) -> i32;
    pub fn sceRtcTickAddSeconds(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_seconds: u64,
    ) -> i32;
    pub fn sceRtcTickAddMinutes(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_minutes: u64,
    ) -> i32;
    pub fn sceRtcTickAddHours(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_hours: u64,
    ) -> i32;
    pub fn sceRtcTickAddDays(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_days: u64,
    ) -> i32;
    pub fn sceRtcTickAddWeeks(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_weeks: u64,
    ) -> i32;
    pub fn sceRtcTickAddMonths(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_months: u64,
    ) -> i32;
    pub fn sceRtcTickAddYears(
        dest_tick: *mut u64,
        src_tick: *const u64,
        num_years: u64,
    ) -> i32;
    pub fn sceRtcSetTime_t(date: *mut ScePspDateTime, time: i64) -> i32;
    pub fn sceRtcGetTime_t(date: *const ScePspDateTime, time: *mut i64)
        -> i32;
    pub fn sceRtcSetDosTime(date: *mut ScePspDateTime, dos_time: u32) -> i32;
    pub fn sceRtcGetDosTime(date: *mut ScePspDateTime, dos_time: u32) -> i32;
    pub fn sceRtcSetWin32FileTime(
        date: *mut ScePspDateTime,
        time: *mut u64,
    ) -> i32;
    pub fn sceRtcGetWin32FileTime(
        date: *mut ScePspDateTime,
        time: *mut u64,
    ) -> i32;
    pub fn sceRtcParseDateTime(
        dest_tick: *mut u64,
        date_string: *const u8,
    ) -> i32;
    pub fn sceRtcFormatRFC3339(
        psz_date_time: *mut char,
        p_utc: *const u64,
        time_zone_minutes: i32,
    ) -> i32;
    pub fn sceRtcFormatRFC3339LocalTime(
        psz_date_time: *mut char,
        p_utc: *const u64,
    ) -> i32;
    pub fn sceRtcParseRFC3339(
        p_utc: *mut u64,
        psz_date_time: *const u8,
    ) -> i32;
    pub fn sceRtcFormatRFC2822(
        psz_date_time: *mut char,
        p_utc: *const u64,
        time_zone_minutes: i32,
    ) -> i32;
    pub fn sceRtcFormatRFC2822LocalTime(
        psz_date_time: *mut char,
        p_utc: *const u64,
    ) -> i32;
}
