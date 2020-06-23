use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetMallocStat {
    pub pool: i32,
    pub maximum: i32,
    pub free: i32,
}

extern {
    pub fn sceNetInit(
        poolsize: i32,
        calloutprio: i32,
        calloutstack: i32,
        netintrprio: i32,
        netintrstack: i32,
    ) -> i32;
    pub fn sceNetTerm() -> i32;
    pub fn sceNetFreeThreadinfo(thid: i32) -> i32;
    pub fn sceNetThreadAbort(thid: i32) -> i32;
    pub fn sceNetEtherStrton(name: *mut u8, mac: *mut u8);
    pub fn sceNetEtherNtostr(mac: *mut u8, name: *mut u8);
    pub fn sceNetGetLocalEtherAddr(mac: *mut u8) -> i32;
    pub fn sceNetGetMallocStat(stat: *mut SceNetMallocStat) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlAdhocId {
    pub unknown: i32,
    pub adhoc_id: [u8; 9usize],
    pub unk: [u8; 3usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlPeerInfo {
    pub next: *mut SceNetAdhocctlPeerInfo,
    pub nickname: [u8; 128usize],
    pub mac: [u8; 6usize],
    pub unknown: [u8; 6usize],
    pub timestamp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlScanInfo {
    pub next: *mut SceNetAdhocctlScanInfo,
    pub channel: i32,
    pub name: [u8; 8usize],
    pub bssid: [u8; 6usize],
    pub unknown: [u8; 2usize],
    pub unknown2: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlGameModeInfo {
    pub count: i32,
    pub macs: [[u8; 6usize]; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocctlParams {
    pub channel: i32,
    pub name: [u8; 8usize],
    pub bssid: [u8; 6usize],
    pub nickname: [u8; 128usize],
}

pub type SceNetAdhocctlHandler =
    Option<unsafe extern "C" fn(flag: i32, error: i32, unknown: *mut c_void)>;

extern {
    pub fn sceNetAdhocctlInit(
        stacksize: i32,
        priority: i32,
        adhoc_id: *mut SceNetAdhocctlAdhocId,
    ) -> i32;
    pub fn sceNetAdhocctlTerm() -> i32;
    pub fn sceNetAdhocctlConnect(name: *const u8) -> i32;
    pub fn sceNetAdhocctlDisconnect() -> i32;
    pub fn sceNetAdhocctlGetState(event: *mut i32) -> i32;
    pub fn sceNetAdhocctlCreate(name: *const u8) -> i32;
    pub fn sceNetAdhocctlJoin(scaninfo: *mut SceNetAdhocctlScanInfo) -> i32;
    pub fn sceNetAdhocctlGetAdhocId(id: *mut SceNetAdhocctlAdhocId) -> i32;
    pub fn sceNetAdhocctlCreateEnterGameMode(
        name: *const u8,
        unknown: i32,
        num: i32,
        macs: *mut u8,
        timeout: u32,
        unknown2: i32,
    ) -> i32;
    pub fn sceNetAdhocctlJoinEnterGameMode(
        name: *const u8,
        hostmac: *mut u8,
        timeout: u32,
        unknown: i32,
    ) -> i32;
    pub fn sceNetAdhocctlGetGameModeInfo(
        gamemodeinfo: *mut SceNetAdhocctlGameModeInfo,
    ) -> i32;
    pub fn sceNetAdhocctlExitGameMode() -> i32;
    pub fn sceNetAdhocctlGetPeerList(
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocctlGetPeerInfo(
        mac: *mut u8,
        size: i32,
        peerinfo: *mut SceNetAdhocctlPeerInfo,
    ) -> i32;
    pub fn sceNetAdhocctlScan() -> i32;
    pub fn sceNetAdhocctlGetScanInfo(
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocctlAddHandler(
        handler: SceNetAdhocctlHandler,
        unknown: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocctlDelHandler(id: i32) -> i32;
    pub fn sceNetAdhocctlGetNameByAddr(
        mac: *mut u8,
        nickname: *mut u8,
    ) -> i32;
    pub fn sceNetAdhocctlGetAddrByName(
        nickname: *mut u8,
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocctlGetParameter(params: *mut SceNetAdhocctlParams) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocPtpStat {
    pub next: *mut SceNetAdhocPtpStat,
    pub ptp_id: i32,
    pub mac: [u8; 6usize],
    pub peermac: [u8; 6usize],
    pub port: u16,
    pub peerport: u16,
    pub sent_data: u32,
    pub rcvd_data: u32,
    pub state: ScePspnetAdhocPtpState,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ScePspnetAdhocPtpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SceNetAdhocPdpStat {
    pub next: *mut SceNetAdhocPdpStat,
    pub pdp_id: i32,
    pub mac: [u8; 6usize],
    pub port: u16,
    pub rcvd_data: u32,
}

extern {
    pub fn sceNetAdhocInit() -> i32;
    pub fn sceNetAdhocTerm() -> i32;
    pub fn sceNetAdhocPdpCreate(
        mac: *mut u8,
        port: u16,
        buf_size: u32,
        unk1: i32,
    ) -> i32;
    pub fn sceNetAdhocPdpDelete(
        id: i32,
        unk1: i32,
    ) -> i32;
    pub fn sceNetAdhocPdpSend(
        id: i32,
        dest_mac_addr: *mut u8,
        port: u16,
        data: *mut c_void,
        len: u32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocPdpRecv(
        id: i32,
        src_mac_addr: *mut u8,
        port: *mut u16,
        data: *mut c_void,
        data_length: *mut c_void,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocGetPdpStat(
        size: *mut i32,
        stat: *mut SceNetAdhocPdpStat,
    ) -> i32;
    pub fn sceNetAdhocGameModeCreateMaster(
        data: *mut c_void,
        size: i32,
    ) -> i32;
    pub fn sceNetAdhocGameModeCreateReplica(
        mac: *mut u8,
        data: *mut c_void,
        size: i32,
    ) -> i32;
    pub fn sceNetAdhocGameModeUpdateMaster() -> i32;
    pub fn sceNetAdhocGameModeUpdateReplica(
        id: i32,
        unk1: i32,
    ) -> i32;
    pub fn sceNetAdhocGameModeDeleteMaster() -> i32;
    pub fn sceNetAdhocGameModeDeleteReplica(id: i32) -> i32;
    pub fn sceNetAdhocPtpOpen(
        srcmac: *mut u8,
        srcport: u16,
        destmac: *mut u8,
        destport: u16,
        buf_size: u32,
        delay: u32,
        count: i32,
        unk1: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpConnect(
        id: i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpListen(
        srcmac: *mut u8,
        srcport: u16,
        buf_size: u32,
        delay: u32,
        count: i32,
        queue: i32,
        unk1: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpAccept(
        id: i32,
        mac: *mut u8,
        port: *mut u16,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpSend(
        id: i32,
        data: *mut c_void,
        data_size: *mut i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpRecv(
        id: i32,
        data: *mut c_void,
        data_size: *mut i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpFlush(
        id: i32,
        timeout: u32,
        nonblock: i32,
    ) -> i32;
    pub fn sceNetAdhocPtpClose(
        id: i32,
        unk1: i32,
    ) -> i32;
    pub fn sceNetAdhocGetPtpStat(
        size: *mut i32,
        stat: *mut SceNetAdhocPtpStat,
    ) -> i32;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AdhocPoolStat {
    pub size: i32,
    pub maxsize: i32,
    pub freesize: i32,
}

pub type AdhocMatchingCallback = Option<
    unsafe extern "C" fn(
        matching_id: i32,
        event: i32,
        mac: *mut u8,
        opt_len: i32,
        opt_data: *mut c_void,
    ),
>;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum AdhocMatchingMode {
    Host = 1,
    Client,
    Ptp,
}

extern {
    pub fn sceNetAdhocMatchingInit(memsize: i32) -> i32;
    pub fn sceNetAdhocMatchingTerm() -> i32;
    pub fn sceNetAdhocMatchingCreate(
        mode: AdhocMatchingMode,
        max_peers: i32,
        port: u16,
        buf_size: i32,
        hello_delay: u32,
        ping_delay: u32,
        init_count: i32,
        msg_delay: u32,
        callback: AdhocMatchingCallback,
    ) -> i32;
    pub fn sceNetAdhocMatchingDelete(matching_id: i32) -> i32;
    pub fn sceNetAdhocMatchingStart(
        matching_id: i32,
        evth_pri: i32,
        evth_stack: i32,
        inth_pri: i32,
        inth_stack: i32,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingStop(matching_id: i32) -> i32;
    pub fn sceNetAdhocMatchingSelectTarget(
        matching_id: i32,
        mac: *mut u8,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingCancelTarget(
        matching_id: i32,
        mac: *mut u8,
    ) -> i32;
    pub fn sceNetAdhocMatchingCancelTargetWithOpt(
        matching_id: i32,
        mac: *mut u8,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingSendData(
        matching_id: i32,
        mac: *mut u8,
        data_len: i32,
        data: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingAbortSendData(
        matching_id: i32,
        mac: *mut u8,
    ) -> i32;
    pub fn sceNetAdhocMatchingSetHelloOpt(
        matching_id: i32,
        opt_len: i32,
        opt_data: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingGetHelloOpt(
        matching_id: i32,
        opt_len: *mut i32,
        opt_data: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingGetMembers(
        matching_id: i32,
        length: *mut i32,
        buf: *mut c_void,
    ) -> i32;
    pub fn sceNetAdhocMatchingGetPoolMaxAlloc() -> i32;
    pub fn sceNetAdhocMatchingGetPoolStat(poolstat: *mut AdhocPoolStat) -> i32;
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ApctlState {
    Disconnected,
    Scanning,
    Joining,
    GettingIp,
    GotIp,
    EapAuth,
    KeyExchange,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ApctlEvent {
    ConnectRequest,
    ScanRequest,
    ScanComplete,
    Established,
    GetIp,
    DisconnectRequest,
    Error,
    Info,
    EapAuth,
    KeyExchange,
    Reconnect,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ApctlInfo {
    ProfileName,
    Bssid,
    Ssid,
    SsidLength,
    SecurityType,
    Strength,
    Channel,
    PowerSave,
    Ip,
    SubnetMask,
    Gateway,
    PrimaryDns,
    SecondaryDns,
    UseProxy,
    ProxyUrl,
    ProxyPort,
    EapType,
    StartBrowser,
    Wifisp,
}

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ApctlInfoSecurityType {
    None,
    Wep,
    Wpa,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SceNetApctlInfo {
    pub name: [u8; 64usize],
    pub bssid: [u8; 6usize],
    pub ssid: [u8; 32usize],
    pub ssid_length: u32,
    pub security_type: u32,
    pub strength: u8,
    pub channel: u8,
    pub power_save: u8,
    pub ip: [u8; 16usize],
    pub sub_net_mask: [u8; 16usize],
    pub gateway: [u8; 16usize],
    pub primary_dns: [u8; 16usize],
    pub secondary_dns: [u8; 16usize],
    pub use_proxy: u32,
    pub proxy_url: [u8; 128usize],
    pub proxy_port: u16,
    pub eap_type: u32,
    pub start_browser: u32,
    pub wifisp: u32,
}

pub type SceNetApctlHandler = Option<
    unsafe extern "C" fn(oldState: i32, newState: i32, event: i32, error: i32, pArg: *mut c_void),
>;

extern {
    pub fn sceNetApctlInit(
        stack_size: i32,
        init_priority: i32,
    ) -> i32;
    pub fn sceNetApctlTerm() -> i32;
    pub fn sceNetApctlGetInfo(
        code: ApctlInfo,
        pinfo: *mut SceNetApctlInfo,
    ) -> i32;
    pub fn sceNetApctlAddHandler(
        handler: SceNetApctlHandler,
        parg: *mut c_void,
    ) -> i32;
    pub fn sceNetApctlDelHandler(handler_id: i32) -> i32;
    pub fn sceNetApctlConnect(conn_index: i32) -> i32;
    pub fn sceNetApctlDisconnect() -> i32;
    pub fn sceNetApctlGetState(pstate: *mut ApctlState) -> i32;
}

#[allow(non_camel_case_types)]
pub type socklen_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr(pub u32);

extern {
    pub fn sceNetInetInit() -> i32;
    pub fn sceNetInetTerm() -> i32;
    pub fn sceNetInetAccept(
        s: i32,
        addr: *mut sockaddr,
        addr_len: *mut socklen_t,
    ) -> i32;
    pub fn sceNetInetBind(
        s: i32,
        my_addr: *const sockaddr,
        addr_len: socklen_t,
    ) -> i32;
    pub fn sceNetInetConnect(
        s: i32,
        serv_addr: *const sockaddr,
        addr_len: socklen_t,
    ) -> i32;
    pub fn sceNetInetGetsockopt(
        s: i32,
        level: i32,
        opt_name: i32,
        opt_val: *mut c_void,
        optl_en: *mut socklen_t,
    ) -> i32;
    pub fn sceNetInetListen(
        s: i32,
        backlog: i32,
    ) -> i32;
    pub fn sceNetInetRecv(
        s: i32,
        buf: *mut c_void,
        len: usize,
        flags: i32,
    ) -> usize;
    pub fn sceNetInetRecvfrom(
        s: i32,
        buf: *mut c_void,
        flags: usize,
        arg1: i32,
        from: *mut sockaddr,
        from_len: *mut socklen_t,
    ) -> usize;
    pub fn sceNetInetSend(
        s: i32,
        buf: *const c_void,
        len: usize,
        flags: i32,
    ) -> usize;
    pub fn sceNetInetSendto(
        s: i32,
        buf: *const c_void,
        len: usize,
        flags: i32,
        to: *const sockaddr,
        to_len: socklen_t,
    ) -> usize;
    pub fn sceNetInetSetsockopt(
        s: i32,
        level: i32,
        opt_name: i32,
        opt_val: *const c_void,
        opt_len: socklen_t,
    ) -> i32;
    pub fn sceNetInetShutdown(
        s: i32,
        how: i32,
    ) -> i32;
    pub fn sceNetInetSocket(
        domain: i32,
        type_: i32,
        protocol: i32,
    ) -> i32;
    pub fn sceNetInetClose(s: i32) -> i32;
    pub fn sceNetInetGetErrno() -> i32;
}

extern {
    pub fn sceSslInit(unknown1: i32) -> i32;
    pub fn sceSslEnd() -> i32;
    pub fn sceSslGetUsedMemoryMax(memory: *mut u32) -> i32;
    pub fn sceSslGetUsedMemoryCurrent(memory: *mut u32) -> i32;
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Head,
}

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum HttpAuthType {
    Basic,
    Digest,
}

pub type HttpMallocFunction = Option<unsafe extern "C" fn(size: usize) -> *mut c_void>;
pub type HttpReallocFunction =
    Option<unsafe extern "C" fn(p: *mut c_void, size: usize) -> *mut c_void>;

pub type HttpFreeFunction = Option<unsafe extern "C" fn(p: *mut c_void)>;
pub type HttpPasswordCB = Option<
    unsafe extern "C" fn(
        request: i32,
        auth_type: HttpAuthType,
        realm: *const u8,
        username: *mut u8,
        password: *mut u8,
        need_entity: i32,
        entity_body: *mut *mut u8,
        entity_size: *mut usize,
        save: *mut i32,
    ) -> i32,
>;

extern {
    pub fn sceHttpInit(unknown1: u32) -> i32;
    pub fn sceHttpEnd() -> i32;
    pub fn sceHttpCreateTemplate(
        agent: *mut u8,
        unknown1: i32,
        unknown2: i32,
    ) -> i32;
    pub fn sceHttpDeleteTemplate(templateid: i32) -> i32;
    pub fn sceHttpCreateConnection(
        templateid: i32,
        host: *mut u8,
        unknown1: *mut u8,
        port: u16,
        unknown2: i32,
    ) -> i32;
    pub fn sceHttpCreateConnectionWithURL(
        templateid: i32,
        url: *const u8,
        unknown1: i32,
    ) -> i32;
    pub fn sceHttpDeleteConnection(connection_id: i32) -> i32;
    pub fn sceHttpCreateRequest(
        connection_id: i32,
        method: HttpMethod,
        path: *mut u8,
        content_length: u64,
    ) -> i32;
    pub fn sceHttpCreateRequestWithURL(
        connection_id: i32,
        method: HttpMethod,
        url: *mut u8,
        content_length: u64,
    ) -> i32;
    pub fn sceHttpDeleteRequest(request_id: i32) -> i32;
    pub fn sceHttpSendRequest(
        request_id: i32,
        data: *mut c_void,
        data_size: u32,
    ) -> i32;
    pub fn sceHttpAbortRequest(request_id: i32) -> i32;
    pub fn sceHttpReadData(
        request_id: i32,
        data: *mut c_void,
        data_size: u32,
    ) -> i32;
    pub fn sceHttpGetContentLength(
        request_id: i32,
        content_length: *mut u64,
    ) -> i32;
    pub fn sceHttpGetStatusCode(
        request_id: i32,
        status_code: *mut i32,
    ) -> i32;
    pub fn sceHttpSetResolveTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;
    pub fn sceHttpSetResolveRetry(
        id: i32,
        count: i32,
    ) -> i32;
    pub fn sceHttpSetConnectTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;
    pub fn sceHttpSetSendTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;
    pub fn sceHttpSetRecvTimeOut(
        id: i32,
        timeout: u32,
    ) -> i32;
    pub fn sceHttpEnableKeepAlive(id: i32) -> i32;
    pub fn sceHttpDisableKeepAlive(id: i32) -> i32;
    pub fn sceHttpEnableRedirect(id: i32) -> i32;
    pub fn sceHttpDisableRedirect(id: i32) -> i32;
    pub fn sceHttpEnableCookie(id: i32) -> i32;
    pub fn sceHttpDisableCookie(id: i32) -> i32;
    pub fn sceHttpSaveSystemCookie() -> i32;
    pub fn sceHttpLoadSystemCookie() -> i32;
    pub fn sceHttpAddExtraHeader(
        id: i32,
        name: *mut u8,
        value: *mut u8,
        unknown1: i32,
    ) -> i32;
    pub fn sceHttpDeleteHeader(
        id: i32,
        name: *const u8,
    ) -> i32;
    pub fn sceHttpsInit(
        unknown1: i32,
        unknown2: i32,
        unknown3: i32,
        unknown4: i32,
    ) -> i32;
    pub fn sceHttpsEnd() -> i32;
    pub fn sceHttpsLoadDefaultCert(
        unknown1: i32,
        unknown2: i32,
    ) -> i32;
    pub fn sceHttpDisableAuth(id: i32) -> i32;
    pub fn sceHttpDisableCache(id: i32) -> i32;
    pub fn sceHttpEnableAuth(id: i32) -> i32;
    pub fn sceHttpEnableCache(id: i32) -> i32;
    pub fn sceHttpEndCache() -> i32;
    pub fn sceHttpGetAllHeader(
        request: i32,
        header: *mut *mut u8,
        header_size: *mut u32,
    ) -> i32;
    pub fn sceHttpGetNetworkErrno(
        request: i32,
        err_num: *mut i32,
    ) -> i32;
    pub fn sceHttpGetProxy(
        id: i32,
        activate_flag: *mut i32,
        mode: *mut i32,
        proxy_host: *mut u8,
        len: usize,
        proxy_port: *mut u16,
    ) -> i32;
    pub fn sceHttpInitCache(max_size: usize) -> i32;
    pub fn sceHttpSetAuthInfoCB(
        id: i32,
        cbfunc: HttpPasswordCB,
    ) -> i32;
    pub fn sceHttpSetProxy(
        id: i32,
        activate_flag: i32,
        mode: i32,
        new_proxy_host: *const u8,
        new_proxy_port: u16,
    ) -> i32;
    pub fn sceHttpSetResHeaderMaxSize(
        id: i32,
        header_size: u32,
    ) -> i32;
    pub fn sceHttpSetMallocFunction(
        malloc_func: HttpMallocFunction,
        free_func: HttpFreeFunction,
        realloc_func: HttpReallocFunction,
    ) -> i32;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr(pub u32);

extern {
    pub fn sceNetResolverInit() -> i32;
    pub fn sceNetResolverCreate(
        rid: *mut i32,
        buf: *mut c_void,
        buf_length: u32,
    ) -> i32;
    pub fn sceNetResolverDelete(rid: i32) -> i32;
    pub fn sceNetResolverStartNtoA(
        rid: i32,
        hostname: *const u8,
        addr: *mut in_addr,
        timeout: u32,
        retry: i32,
    ) -> i32;
    pub fn sceNetResolverStartAtoN(
        rid: i32,
        addr: *const in_addr,
        hostname: *mut u8,
        hostname_len: u32,
        timeout: u32,
        retry: i32,
    ) -> i32;
    pub fn sceNetResolverStop(rid: i32) -> i32;
    pub fn sceNetResolverTerm() -> i32;
}
