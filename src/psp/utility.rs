use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityDialogCommon {
    pub size: u32,
    pub language: SystemParamLanguage,
    pub button_accept: UtilityDialogButtonAccept,
    pub graphics_thread: i32,
    pub access_thread: i32,
    pub font_thread: i32,
    pub sound_thread: i32,
    pub result: i32,
    pub reserved: [i32; 4usize],
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityMsgDialogMode {
    Error,
    Text,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityMsgDialogPressed {
    Unknown1,
    Yes,
    No,
    Back,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityDialogButtonAccept {
    Circle,
    Cross,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SceUtilityOskInputLanguage {
    Default,
    Japanese,
    English,
    French,
    Spanish,
    German,
    Italian,
    Dutch,
    Portugese,
    Russian,
    Korean,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SceUtilityOskInputType {
    All,
    LatinDigit,
    LatinSymbol,
    LatinLowercase = 4,
    LatinUppercase = 8,
    JapaneseDigit = 0x100,
    JapaneseSymbol = 0x200,
    JapaneseLowercase = 0x400,
    JapaneseUppercase = 0x800,
    JapaneseHiragana = 0x1000,
    JapaneseHalfWidthKatakana = 0x2000,
    JapaneseKatakana = 0x4000,
    JapaneseKanji = 0x8000,
    RussianLowercase = 0x10000,
    RussianUppercase = 0x20000,
    Korean = 0x40000,
    Url = 0x80000,
}

#[derive(Clone, Copy)]
pub enum SceUtilityOskState {
    None,
    Initializing,
    Initialized,
    Visible,
    Quit,
    Finished,
}

#[derive(Clone, Copy)]
pub enum SceUtilityOskResult {
    Unchanged,
    Cancelled,
    Changed,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamLanguage {
    Japanese,
    English,
    French,
    Spanish,
    German,
    Italian,
    Dutch,
    Portugese,
    Russian,
    Korean,
    ChineseTraditional,
    ChineseSimplified,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamId {
    StringNickname = 1,
    AdhocChannel,
    WlanPowerSave,
    DateFormat,
    TimeFormat,
    Timezone,
    DaylightSavings,
    Language,
    Unknown,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamAdhocChannel {
    ChannelAutomatic = 0,
    Channel1 = 1,
    Channel6 = 6,
    Channel11 = 11,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamWlanPowerSaveState {
    Off,
    On,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamDateFormat {
    YYYYMMDD,
    MMDDYYYY,
    DDMMYYYY,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamTimeFormat {
    Hour24,
    Hour12,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum SystemParamDaylightSavings {
    Std,
    Dst,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum AvModule {
    AvCodec,
    SasCore,
    Atrac3Plus,
    MpegBase,
    Mp3,
    Vaudio,
    Aac,
    G729,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum Module {
    NetCommon = 0x100,
    NetAdhoc,
    NetInet,
    NetParseUri,
    NetHttp,
    NetSsl,

    UsbPspCm = 0x200,
    UsbMic,
    UsbCam,
    UsbGps,

    AvCodec = 0x300,
    AvSascore,
    AvAtrac3Plus,
    AvMpegBase,
    AvMp3,
    AvVaudio,
    AvAac,
    AvG729,

    NpCommon = 0x400,
    NpService,
    NpMatching2,
    NpDrm = 0x500,

    Irda = 0x600,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum NetModule {
    NetCommon = 1,
    NetAdhoc,
    NetInet,
    NetParseUri,
    NetHttp,
    NetSsl,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UsbModule {
    UsbPspCm = 1,
    UsbAcc,
    UsbMic,
    UsbCam,
    UsbGps,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum NetParam {
    Name,
    Ssid,
    Secure,
    WepKey,
    IsStaticIp,
    Ip,
    NetMask,
    Route,
    ManualDns,
    PrimaryDns,
    SecondaryDns,
    ProxyUser,
    ProxyPass,
    UseProxy,
    ProxyServer,
    ProxyPort,
    Unknown1,
    Unknown2,
}

#[derive(Copy, Clone)]
pub enum UtilityNetconfAction {
    ConnectAP,
    DisplayStatus,
    ConnectAdhoc,
}

pub const UTILITY_MSGDIALOG_ERROR: i32 = 0;
pub const UTILITY_MSGDIALOG_TEXT: i32 = 1;
pub const UTILITY_MSGDIALOG_YES_NO_BUTTONS: i32 = 0x10;
pub const UTILITY_MSGDIALOG_DEFAULT_NO: i32 = 0x100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityMsgDialogParams {
    pub base: UtilityDialogCommon,
    pub unknown: i32,
    pub mode: UtilityMsgDialogMode,
    pub error_value: u32,
    pub message: [u8; 512usize],
    pub options: i32,
    pub button_pressed: UtilityMsgDialogPressed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityNetconfAdhoc {
    pub name: [u8; 8usize],
    pub timeout: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityNetconfData {
    pub base: UtilityDialogCommon,
    pub action: UtilityNetconfAction,
    pub adhocparam: *mut UtilityNetconfAdhoc,
    pub hotspot: i32,
    pub hotspot_connected: i32,
    pub wifisp: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union UtilityNetData {
    pub as_uint: u32,
    pub as_string: [u8; 128usize],
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UtilitySavedataMode {
    AutoLoad,
    AutoSave,
    Load,
    Save,
    ListLoad,
    ListSave,
    ListDelete,
    Delete,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UtilitySavedataFocus {
    Unknown1,
    FirstList,
    LastList,
    Latest,
    Oldest,
    Unknown2,
    Unknown3,
    FirstEmpty,
    LastEmpty,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilitySavedataSFOParam {
    pub title: [u8; 128usize],
    pub savedata_title: [u8; 128usize],
    pub detail: [u8; 1024usize],
    pub parental_level: u8,
    pub unknown: [u8; 3usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilitySavedataFileData {
    pub buf: *mut c_void,
    pub buf_size: usize,
    pub size: usize,
    pub unknown: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilitySavedataListSaveNewData {
    pub icon0: UtilitySavedataFileData,
    pub title: *mut u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceUtilitySavedataParam {
    pub base: UtilityDialogCommon,
    pub mode: UtilitySavedataMode,
    pub unknown1: i32,
    pub overwrite: i32,
    pub game_name: [u8; 13usize],
    pub reserved: [u8; 3usize],
    pub save_name: [u8; 20usize],
    pub save_name_list: *mut [u8; 20usize],
    pub file_name: [u8; 13usize],
    pub reserved1: [u8; 3usize],
    pub data_buf: *mut c_void,
    pub data_buf_size: usize,
    pub data_size: usize,
    pub sfo_param: UtilitySavedataSFOParam,
    pub icon0_file_data: UtilitySavedataFileData,
    pub icon1_file_data: UtilitySavedataFileData,
    pub pic1_file_data: UtilitySavedataFileData,
    pub snd0_file_data: UtilitySavedataFileData,
    pub new_data: *mut UtilitySavedataListSaveNewData,
    pub focus: UtilitySavedataFocus,
    pub unknown2: [i32; 4usize],
    pub key: [u8; 16],
    pub unknown3: [u8; 20],
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UtilityGameSharingMode {
    Single = 1,
    Multiple,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum UtilityGameSharingDataType {
    File = 1,
    Memory,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityGameSharingParams {
    pub base: UtilityDialogCommon,
    pub unknown1: i32,
    pub unknown2: i32,
    pub name: [u8; 8usize],
    pub unknown3: i32,
    pub unknown4: i32,
    pub unknown5: i32,
    pub result: i32,
    pub filepath: *mut u8,
    pub mode: UtilityGameSharingMode,
    pub datatype: UtilityGameSharingDataType,
    pub data: *mut c_void,
    pub datasize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct UtilityHtmlViewerParam {
    pub base: UtilityDialogCommon,
    pub memaddr: *mut c_void,
    pub memsize: u32,
    pub unknown1: i32,
    pub unknown2: i32,
    pub initialurl: *mut u8,
    pub numtabs: u32,
    pub interfacemode: UtilityHtmlViewerInterfaceMode,
    pub options: i32,
    pub dldirname: *mut u8,
    pub dlfilename: *mut u8,
    pub uldirname: *mut u8,
    pub ulfilename: *mut u8,
    pub cookiemode: UtilityHtmlViewerCookieMode,
    pub unknown3: u32,
    pub homeurl: *mut u8,
    pub textsize: UtilityHtmlViewerTextSize,
    pub displaymode: UtilityHtmlViewerDisplayMode,
    pub connectmode: UtilityHtmlViewerConnectMode,
    pub disconnectmode: UtilityHtmlViewerDisconnectMode,
    pub memused: u32,
    pub unknown4: [i32; 10usize],
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityHtmlViewerInterfaceMode {
    Full,
    Limited,
    None,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityHtmlViewerCookieMode {
    Disabled = 0,
    Enabled,
    Confirm,
    Default,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityHtmlViewerTextSize {
    Large,
    Normal,
    Small,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityHtmlViewerDisplayMode {
    Normal,
    Fit,
    SmartFit,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityHtmlViewerConnectMode {
    Last,
    ManualOnce,
    ManualAll,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum UtilityHtmlViewerDisconnectMode {
    Enable,
    Disable,
    Confirm,
}

pub const UTILITY_HTMLVIEWER_OPEN_SCE_START_PAGE: i32 = 0x000001;
pub const UTILITY_HTMLVIEWER_DISABLE_STARTUP_LIMITS: i32 = 0x000002;
pub const UTILITY_HTMLVIEWER_DISABLE_EXIT_DIALOG: i32 = 0x000004;
pub const UTILITY_HTMLVIEWER_DISABLE_CURSOR: i32 = 0x000008;
pub const UTILITY_HTMLVIEWER_DISABLE_DOWNLOAD_COMPLETE_DIALOG: i32 = 0x000010;
pub const UTILITY_HTMLVIEWER_DISABLE_DOWNLOAD_START_DIALOG: i32 = 0x000020;
pub const UTILITY_HTMLVIEWER_DISABLE_DOWNLOAD_DESTINATION_DIALOG: i32 =
    0x000040;
pub const UTILITY_HTMLVIEWER_LOCK_DOWNLOAD_DESTINATION_DIALOG: i32 = 0x000080;
pub const UTILITY_HTMLVIEWER_DISABLE_TAB_DISPLAY: i32 = 0x000100;
pub const UTILITY_HTMLVIEWER_ENABLE_ANALOG_HOLD: i32 = 0x000200;
pub const UTILITY_HTMLVIEWER_ENABLE_FLASH: i32 = 0x000400;
pub const UTILITY_HTMLVIEWER_DISABLE_LRTRIGGER: i32 = 0x000800;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceUtilityOskData {
    pub unk_00: i32,
    pub unk_04: i32,
    pub language: SceUtilityOskInputLanguage,
    pub unk_12: i32,
    pub inputtype: SceUtilityOskInputType,
    pub lines: i32,
    pub unk_24: i32,
    pub desc: *mut u16,
    pub intext: *mut u16,
    pub outtextlength: i32,
    pub outtext: *mut u16,
    pub result: SceUtilityOskResult,
    pub outtextlimit: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceUtilityOskParams {
    pub base: UtilityDialogCommon,
    pub datacount: i32,
    pub data: *mut SceUtilityOskData,
    pub state: SceUtilityOskState,
    pub unk_60: i32,
}

extern "C" {
    pub fn sceUtilityMsgDialogInitStart(
        params: *mut UtilityMsgDialogParams,
    ) -> i32;
    pub fn sceUtilityMsgDialogShutdownStart();
    pub fn sceUtilityMsgDialogGetStatus() -> i32;
    pub fn sceUtilityMsgDialogUpdate(n: i32);
    pub fn sceUtilityMsgDialogAbort() -> i32;
    pub fn sceUtilityNetconfInitStart(data: *mut UtilityNetconfData) -> i32;
    pub fn sceUtilityNetconfShutdownStart() -> i32;
    pub fn sceUtilityNetconfUpdate(unknown: i32) -> i32;
    pub fn sceUtilityNetconfGetStatus() -> i32;
    pub fn sceUtilityCheckNetParam(id: i32) -> i32;
    pub fn sceUtilityGetNetParam(
        conf: i32,
        param: NetParam,
        data: *mut UtilityNetData,
    ) -> i32;
    pub fn sceUtilitySavedataInitStart(
        params: *mut SceUtilitySavedataParam,
    ) -> i32;
    pub fn sceUtilitySavedataGetStatus() -> i32;
    pub fn sceUtilitySavedataShutdownStart() -> i32;
    pub fn sceUtilitySavedataUpdate(unknown: i32);
    pub fn sceUtilityGameSharingInitStart(
        params: *mut UtilityGameSharingParams,
    ) -> i32;
    pub fn sceUtilityGameSharingShutdownStart();
    pub fn sceUtilityGameSharingGetStatus() -> i32;
    pub fn sceUtilityGameSharingUpdate(n: i32);
    pub fn sceUtilityHtmlViewerInitStart(
        params: *mut UtilityHtmlViewerParam,
    ) -> i32;
    pub fn sceUtilityHtmlViewerShutdownStart() -> i32;
    pub fn sceUtilityHtmlViewerUpdate(n: i32) -> i32;
    pub fn sceUtilityHtmlViewerGetStatus() -> i32;
    pub fn sceUtilitySetSystemParamInt(id: SystemParamId, value: i32) -> i32;
    pub fn sceUtilitySetSystemParamString(
        id: SystemParamId,
        str: *const u8,
    ) -> i32;
    pub fn sceUtilityGetSystemParamInt(
        id: SystemParamId,
        value: *mut i32,
    ) -> i32;
    pub fn sceUtilityGetSystemParamString(
        id: SystemParamId,
        str: *mut u8,
        len: i32,
    ) -> i32;
    pub fn sceUtilityOskInitStart(params: *mut SceUtilityOskParams) -> i32;
    pub fn sceUtilityOskShutdownStart() -> i32;
    pub fn sceUtilityOskUpdate(n: i32) -> i32;
    pub fn sceUtilityOskGetStatus() -> i32;
    pub fn sceUtilityLoadNetModule(module: NetModule) -> i32;
    pub fn sceUtilityUnloadNetModule(module: NetModule) -> i32;
    pub fn sceUtilityLoadAvModule(module: AvModule) -> i32;
    pub fn sceUtilityUnloadAvModule(module: AvModule) -> i32;
    pub fn sceUtilityLoadUsbModule(module: UsbModule) -> i32;
    pub fn sceUtilityUnloadUsbModule(module: UsbModule) -> i32;
    pub fn sceUtilityLoadModule(module: Module) -> i32;
    pub fn sceUtilityUnloadModule(module: Module) -> i32;
}

extern "C" {
    pub fn sceUtilityCreateNetParam(conf: i32) -> i32;
    pub fn sceUtilitySetNetParam(param: NetParam, val: *const c_void) -> i32;
    pub fn sceUtilityCopyNetParam(src: i32, dest: i32) -> i32;
    pub fn sceUtilityDeleteNetParam(conf: i32) -> i32;
}
