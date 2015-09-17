//! Windows CRT definitions

pub type c_char = i8;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_long = i32;
pub type c_ulong = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type wchar_t = u16;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub type intptr_t = i32;
        pub type ptrdiff_t = i32;
        pub type size_t = u32;
        pub type ssize_t = i32;
        pub type uintptr_t = u32;
        pub type LONG_PTR = c_long;
    } else if #[cfg(target_arch = "x86_64")] {
        pub type intptr_t = i64;
        pub type ptrdiff_t = i64;
        pub type size_t = u64;
        pub type ssize_t = i64;
        pub type uintptr_t = u64;
        pub type LONG_PTR = i64;
    } else {
        // unknown arch
    }
}

pub type clock_t = i32;

cfg_if! {
    if #[cfg(all(target_arch = "x86", target_env = "gnu"))] {
        pub type time_t = i32;
    } else {
        pub type time_t = i64;
    }
}

pub type off_t = i32;
pub type dev_t = u32;
pub type ino_t = u16;
pub enum timezone {}
pub type time64_t = i64;

pub type BOOL = c_int;
pub type BYTE = u8;
pub type BOOLEAN = BYTE;
pub type CCHAR = c_char;
pub type CHAR = c_char;
pub type DWORD = c_ulong;
pub type DWORDLONG = c_ulonglong;
pub type HANDLE = LPVOID;
pub type HINSTANCE = HANDLE;
pub type HMODULE = HINSTANCE;
pub type LONG = c_long;
pub type PLONG = *mut c_long;
pub type LARGE_INTEGER = c_longlong;
pub type PLARGE_INTEGER = *mut c_longlong;
pub type LPCWSTR = *const WCHAR;
pub type LPCSTR = *const CHAR;
pub type LPWSTR = *mut WCHAR;
pub type LPSTR = *mut CHAR;
pub type LPWCH = *mut WCHAR;
pub type LPCH = *mut CHAR;
pub type SOCKET = uintptr_t;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPVOID = *mut ::c_void;
pub type LPCVOID = *const ::c_void;
pub type LPBYTE = *mut BYTE;
pub type LPWORD = *mut WORD;
pub type LPDWORD = *mut DWORD;
pub type LPHANDLE = *mut HANDLE;
pub type LRESULT = LONG_PTR;
pub type PBOOL = *mut BOOL;
pub type WCHAR = wchar_t;
pub type WORD = u16;
pub type SIZE_T = size_t;
pub type GROUP = c_uint;
pub type LPSTARTUPINFOW = *mut STARTUPINFOW;
pub type LPPROCESS_INFORMATION = *mut PROCESS_INFORMATION;
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
pub type PMEMORY_BASIC_INFORMATION = *mut MEMORY_BASIC_INFORMATION;
pub type LPOVERLAPPED = *mut OVERLAPPED;
pub type LPFILETIME = *mut FILETIME;
pub type LPWSAPROTOCOL_INFO = *mut WSAPROTOCOL_INFO;
pub type LPWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
pub type LPWSAPROTOCOLCHAIN = *mut WSAPROTOCOLCHAIN;
pub type UCHAR = c_uchar;
pub type ULONG = c_ulong;
cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub type ULONG_PTR = u64;
    } else {
        pub type ULONG_PTR = c_ulong;
    }
}

s! {
    // note this is the struct called stat64 in Windows. Not stat, nor stati64.
    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: u16,
        pub st_nlink: c_short,
        pub st_uid: c_short,
        pub st_gid: c_short,
        pub st_rdev: dev_t,
        pub st_size: i64,
        pub st_atime: time64_t,
        pub st_mtime: time64_t,
        pub st_ctime: time64_t,
    }

    // note that this is called utimbuf64 in Windows
    pub struct utimbuf {
        pub actime: time64_t,
        pub modtime: time64_t,
    }

    pub struct timeval {
        pub tv_sec: c_long,
        pub tv_usec: c_long,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct sockaddr {
        pub sa_family: u16,
        pub sa_data: [CHAR; 14],
    }

    pub struct sockaddr_storage {
        pub ss_family: u16,
        __ss_pad1: [u8; 6],
        __ss_align: i64,
        __ss_pad2: [u8; 112],
    }

    pub struct sockaddr_in {
        pub sin_family: u16,
        pub sin_port: u16,
        pub sin_addr: in_addr,
        pub sin_zero: [CHAR; 8],
    }

    pub struct in_addr {
        pub s_addr: ULONG,
    }

    pub struct sockaddr_in6 {
        pub sin6_family: u16,
        pub sin6_port: u16,
        pub sin6_flowinfo: ULONG,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: ULONG,
    }

    pub struct in6_addr {
        pub s6_addr: [UCHAR; 16],
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: ULONG,
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: size_t,
        pub ai_canonname: *mut c_char,
        pub ai_addr: *mut sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct SECURITY_ATTRIBUTES {
        pub nLength: DWORD,
        pub lpSecurityDescriptor: LPVOID,
        pub bInheritHandle: BOOL,
    }

    pub struct STARTUPINFOW {
        pub cb: DWORD,
        pub lpReserved: LPWSTR,
        pub lpDesktop: LPWSTR,
        pub lpTitle: LPWSTR,
        pub dwX: DWORD,
        pub dwY: DWORD,
        pub dwXSize: DWORD,
        pub dwYSize: DWORD,
        pub dwXCountChars: DWORD,
        pub dwYCountChars: DWORD,
        pub dwFillAttribute: DWORD,
        pub dwFlags: DWORD,
        pub wShowWindow: WORD,
        pub cbReserved2: WORD,
        pub lpReserved2: LPBYTE,
        pub hStdInput: HANDLE,
        pub hStdOutput: HANDLE,
        pub hStdError: HANDLE,
    }

    pub struct PROCESS_INFORMATION {
        pub hProcess: HANDLE,
        pub hThread: HANDLE,
        pub dwProcessId: DWORD,
        pub dwThreadId: DWORD,
    }

    pub struct SYSTEM_INFO {
        pub wProcessorArchitecture: WORD,
        pub wReserved: WORD,
        pub dwPageSize: DWORD,
        pub lpMinimumApplicationAddress: LPVOID,
        pub lpMaximumApplicationAddress: LPVOID,
        pub dwActiveProcessorMask: uintptr_t,
        pub dwNumberOfProcessors: DWORD,
        pub dwProcessorType: DWORD,
        pub dwAllocationGranularity: DWORD,
        pub wProcessorLevel: WORD,
        pub wProcessorRevision: WORD,
    }

    pub struct MEMORY_BASIC_INFORMATION {
        pub BaseAddress: LPVOID,
        pub AllocationBase: LPVOID,
        pub AllocationProtect: DWORD,
        pub RegionSize: SIZE_T,
        pub State: DWORD,
        pub Protect: DWORD,
        pub Type: DWORD,
    }

    pub struct OVERLAPPED {
        pub Internal: ULONG_PTR,
        pub InternalHigh: ULONG_PTR,
        pub Offset: DWORD,
        pub OffsetHigh: DWORD,
        pub hEvent: HANDLE,
    }

    pub struct FILETIME {
        pub dwLowDateTime: DWORD,
        pub dwHighDateTime: DWORD,
    }

    pub struct GUID {
        pub Data1: DWORD,
        pub Data2: WORD,
        pub Data3: WORD,
        pub Data4: [BYTE; 8],
    }

    pub struct WSAPROTOCOLCHAIN {
        pub ChainLen: c_int,
        pub ChainEntries: [DWORD; 7],
    }

    pub struct WSAPROTOCOL_INFO {
        pub dwServiceFlags1: DWORD,
        pub dwServiceFlags2: DWORD,
        pub dwServiceFlags3: DWORD,
        pub dwServiceFlags4: DWORD,
        pub dwProviderFlags: DWORD,
        pub ProviderId: GUID,
        pub dwCatalogEntryId: DWORD,
        pub ProtocolChain: WSAPROTOCOLCHAIN,
        pub iVersion: c_int,
        pub iAddressFamily: c_int,
        pub iMaxSockAddr: c_int,
        pub iMinSockAddr: c_int,
        pub iSocketType: c_int,
        pub iProtocol: c_int,
        pub iProtocolMaxOffset: c_int,
        pub iNetworkByteOrder: c_int,
        pub iSecurityScheme: c_int,
        pub dwMessageSize: DWORD,
        pub dwProviderReserved: DWORD,
        pub szProtocol: [CHAR; 256],
    }

    pub struct WIN32_FIND_DATAW {
        pub dwFileAttributes: DWORD,
        pub ftCreationTime: FILETIME,
        pub ftLastAccessTime: FILETIME,
        pub ftLastWriteTime: FILETIME,
        pub nFileSizeHigh: DWORD,
        pub nFileSizeLow: DWORD,
        pub dwReserved0: DWORD,
        pub dwReserved1: DWORD,
        pub cFileName: [wchar_t; 260], // #define MAX_PATH 260
        pub cAlternateFileName: [wchar_t; 14],
    }
}

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;
pub const RAND_MAX: c_int = 32767;
pub const EOF: c_int = -1;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 0;
pub const _IONBF: c_int = 4;
pub const _IOLBF: c_int = 64;
pub const BUFSIZ: c_uint = 512;
pub const FOPEN_MAX: c_uint = 20;
pub const FILENAME_MAX: c_uint = 260;

pub const WSAEINTR: c_int = 10004;
pub const WSAEBADF: c_int = 10009;
pub const WSAEACCES: c_int = 10013;
pub const WSAEFAULT: c_int = 10014;
pub const WSAEINVAL: c_int = 10022;
pub const WSAEMFILE: c_int = 10024;
pub const WSAEWOULDBLOCK: c_int = 10035;
pub const WSAEINPROGRESS: c_int = 10036;
pub const WSAEALREADY: c_int = 10037;
pub const WSAENOTSOCK: c_int = 10038;
pub const WSAEDESTADDRREQ: c_int = 10039;
pub const WSAEMSGSIZE: c_int = 10040;
pub const WSAEPROTOTYPE: c_int = 10041;
pub const WSAENOPROTOOPT: c_int = 10042;
pub const WSAEPROTONOSUPPORT: c_int = 10043;
pub const WSAESOCKTNOSUPPORT: c_int = 10044;
pub const WSAEOPNOTSUPP: c_int = 10045;
pub const WSAEPFNOSUPPORT: c_int = 10046;
pub const WSAEAFNOSUPPORT: c_int = 10047;
pub const WSAEADDRINUSE: c_int = 10048;
pub const WSAEADDRNOTAVAIL: c_int = 10049;
pub const WSAENETDOWN: c_int = 10050;
pub const WSAENETUNREACH: c_int = 10051;
pub const WSAENETRESET: c_int = 10052;
pub const WSAECONNABORTED: c_int = 10053;
pub const WSAECONNRESET: c_int = 10054;
pub const WSAENOBUFS: c_int = 10055;
pub const WSAEISCONN: c_int = 10056;
pub const WSAENOTCONN: c_int = 10057;
pub const WSAESHUTDOWN: c_int = 10058;
pub const WSAETOOMANYREFS: c_int = 10059;
pub const WSAETIMEDOUT: c_int = 10060;
pub const WSAECONNREFUSED: c_int = 10061;
pub const WSAELOOP: c_int = 10062;
pub const WSAENAMETOOLONG: c_int = 10063;
pub const WSAEHOSTDOWN: c_int = 10064;
pub const WSAEHOSTUNREACH: c_int = 10065;
pub const WSAENOTEMPTY: c_int = 10066;
pub const WSAEPROCLIM: c_int = 10067;
pub const WSAEUSERS: c_int = 10068;
pub const WSAEDQUOT: c_int = 10069;
pub const WSAESTALE: c_int = 10070;
pub const WSAEREMOTE: c_int = 10071;
pub const WSASYSNOTREADY: c_int = 10091;
pub const WSAVERNOTSUPPORTED: c_int = 10092;
pub const WSANOTINITIALISED: c_int = 10093;
pub const WSAEDISCON: c_int = 10101;
pub const WSAENOMORE: c_int = 10102;
pub const WSAECANCELLED: c_int = 10103;
pub const WSAEINVALIDPROCTABLE: c_int = 10104;
pub const WSAEINVALIDPROVIDER: c_int = 10105;
pub const WSAEPROVIDERFAILEDINIT: c_int = 10106;

cfg_if! {
    if #[cfg(all(target_env = "gnu"))] {
        pub const L_tmpnam: c_uint = 14;
        pub const TMP_MAX: c_uint = 0x7fff;
    } else {
        pub const L_tmpnam: c_uint = 260;
        pub const TMP_MAX: c_uint = 0x7fff_ffff;
    }
}

pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_APPEND: c_int = 8;
pub const O_CREAT: c_int = 256;
pub const O_EXCL: c_int = 1024;
pub const O_TRUNC: c_int = 512;
pub const S_IFCHR: c_int = 8192;
pub const S_IFDIR: c_int = 16384;
pub const S_IFREG: c_int = 32768;
pub const S_IFMT: c_int = 61440;
pub const S_IEXEC: c_int = 64;
pub const S_IWRITE: c_int = 128;
pub const S_IREAD: c_int = 256;

pub const AF_INET: c_int = 2;
pub const AF_INET6: c_int = 23;
pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_RAW: c_int = 3;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_IPV6: c_int = 41;
pub const IP_MULTICAST_TTL: c_int = 10;
pub const IP_MULTICAST_LOOP: c_int = 11;
pub const IP_ADD_MEMBERSHIP: c_int = 12;
pub const IP_DROP_MEMBERSHIP: c_int = 13;
pub const IPV6_ADD_MEMBERSHIP: c_int = 12;
pub const IPV6_DROP_MEMBERSHIP: c_int = 13;
pub const IP_TTL: c_int = 4;
pub const IP_HDRINCL: c_int = 2;

pub const TCP_NODELAY: c_int = 0x0001;
pub const SOL_SOCKET: c_int = 0xffff;

pub const SO_DEBUG: c_int = 0x0001;
pub const SO_ACCEPTCONN: c_int = 0x0002;
pub const SO_REUSEADDR: c_int = 0x0004;
pub const SO_KEEPALIVE: c_int = 0x0008;
pub const SO_DONTROUTE: c_int = 0x0010;
pub const SO_BROADCAST: c_int = 0x0020;
pub const SO_USELOOPBACK: c_int = 0x0040;
pub const SO_LINGER: c_int = 0x0080;
pub const SO_OOBINLINE: c_int = 0x0100;
pub const SO_SNDBUF: c_int = 0x1001;
pub const SO_RCVBUF: c_int = 0x1002;
pub const SO_SNDLOWAT: c_int = 0x1003;
pub const SO_RCVLOWAT: c_int = 0x1004;
pub const SO_SNDTIMEO: c_int = 0x1005;
pub const SO_RCVTIMEO: c_int = 0x1006;
pub const SO_ERROR: c_int = 0x1007;
pub const SO_TYPE: c_int = 0x1008;

pub const IFF_LOOPBACK: c_int = 4;

pub const SD_RECEIVE: c_int = 0;
pub const SD_SEND: c_int = 1;
pub const SD_BOTH: c_int = 2;

pub const TRUE: BOOL = 1;
pub const FALSE: BOOL = 0;

pub const O_TEXT: c_int = 16384;
pub const O_BINARY: c_int = 32768;
pub const O_NOINHERIT: c_int = 128;

pub const ERROR_SUCCESS: c_int = 0;
pub const ERROR_INVALID_FUNCTION: c_int = 1;
pub const ERROR_FILE_NOT_FOUND: c_int = 2;
pub const ERROR_ACCESS_DENIED: c_int = 5;
pub const ERROR_INVALID_HANDLE: c_int = 6;
pub const ERROR_BROKEN_PIPE: c_int = 109;
pub const ERROR_DISK_FULL: c_int = 112;
pub const ERROR_CALL_NOT_IMPLEMENTED: c_int = 120;
pub const ERROR_INSUFFICIENT_BUFFER: c_int = 122;
pub const ERROR_INVALID_NAME: c_int = 123;
pub const ERROR_ALREADY_EXISTS: c_int = 183;
pub const ERROR_PIPE_BUSY: c_int = 231;
pub const ERROR_NO_DATA: c_int = 232;
pub const ERROR_INVALID_ADDRESS: c_int = 487;
pub const ERROR_PIPE_CONNECTED: c_int = 535;
pub const ERROR_NOTHING_TO_TERMINATE: c_int = 758;
pub const ERROR_OPERATION_ABORTED: c_int = 995;
pub const ERROR_IO_PENDING: c_int = 997;
pub const ERROR_FILE_INVALID: c_int = 1006;
pub const ERROR_NOT_FOUND: c_int = 1168;
pub const INVALID_HANDLE_VALUE: HANDLE = !0 as HANDLE;

pub const DELETE: DWORD = 0x00010000;
pub const READ_CONTROL: DWORD = 0x00020000;
pub const SYNCHRONIZE: DWORD = 0x00100000;
pub const WRITE_DAC: DWORD = 0x00040000;
pub const WRITE_OWNER: DWORD = 0x00080000;

pub const PROCESS_CREATE_PROCESS: DWORD = 0x0080;
pub const PROCESS_CREATE_THREAD: DWORD = 0x0002;
pub const PROCESS_DUP_HANDLE: DWORD = 0x0040;
pub const PROCESS_QUERY_INFORMATION: DWORD = 0x0400;
pub const PROCESS_QUERY_LIMITED_INFORMATION: DWORD = 0x1000;
pub const PROCESS_SET_INFORMATION: DWORD = 0x0200;
pub const PROCESS_SET_QUOTA: DWORD = 0x0100;
pub const PROCESS_SUSPEND_RESUME: DWORD = 0x0800;
pub const PROCESS_TERMINATE: DWORD = 0x0001;
pub const PROCESS_VM_OPERATION: DWORD = 0x0008;
pub const PROCESS_VM_READ: DWORD = 0x0010;
pub const PROCESS_VM_WRITE: DWORD = 0x0020;

pub const STARTF_FORCEONFEEDBACK: DWORD = 0x00000040;
pub const STARTF_FORCEOFFFEEDBACK: DWORD = 0x00000080;
pub const STARTF_PREVENTPINNING: DWORD = 0x00002000;
pub const STARTF_RUNFULLSCREEN: DWORD = 0x00000020;
pub const STARTF_TITLEISAPPID: DWORD = 0x00001000;
pub const STARTF_TITLEISLINKNAME: DWORD = 0x00000800;
pub const STARTF_USECOUNTCHARS: DWORD = 0x00000008;
pub const STARTF_USEFILLATTRIBUTE: DWORD = 0x00000010;
pub const STARTF_USEHOTKEY: DWORD = 0x00000200;
pub const STARTF_USEPOSITION: DWORD = 0x00000004;
pub const STARTF_USESHOWWINDOW: DWORD = 0x00000001;
pub const STARTF_USESIZE: DWORD = 0x00000002;
pub const STARTF_USESTDHANDLES: DWORD = 0x00000100;

pub const WAIT_ABANDONED: DWORD = 0x00000080;
pub const WAIT_OBJECT_0: DWORD = 0x00000000;
pub const WAIT_TIMEOUT: DWORD = 0x00000102;
pub const WAIT_FAILED: DWORD = !0;

pub const DUPLICATE_CLOSE_SOURCE: DWORD = 0x00000001;
pub const DUPLICATE_SAME_ACCESS: DWORD = 0x00000002;

pub const INFINITE: DWORD = !0;
pub const STILL_ACTIVE: DWORD = 259;

pub const MEM_COMMIT: DWORD = 0x00001000;
pub const MEM_RESERVE: DWORD = 0x00002000;
pub const MEM_DECOMMIT: DWORD = 0x00004000;
pub const MEM_RELEASE: DWORD = 0x00008000;
pub const MEM_RESET: DWORD = 0x00080000;
pub const MEM_RESET_UNDO: DWORD = 0x1000000;
pub const MEM_LARGE_PAGES: DWORD = 0x20000000;
pub const MEM_PHYSICAL: DWORD = 0x00400000;
pub const MEM_TOP_DOWN: DWORD = 0x00100000;
pub const MEM_WRITE_WATCH: DWORD = 0x00200000;

pub const PAGE_EXECUTE: DWORD = 0x10;
pub const PAGE_EXECUTE_READ: DWORD = 0x20;
pub const PAGE_EXECUTE_READWRITE: DWORD = 0x40;
pub const PAGE_EXECUTE_WRITECOPY: DWORD = 0x80;
pub const PAGE_NOACCESS: DWORD = 0x01;
pub const PAGE_READONLY: DWORD = 0x02;
pub const PAGE_READWRITE: DWORD = 0x04;
pub const PAGE_WRITECOPY: DWORD = 0x08;
pub const PAGE_GUARD: DWORD = 0x100;
pub const PAGE_NOCACHE: DWORD = 0x200;
pub const PAGE_WRITECOMBINE: DWORD = 0x400;

pub const SEC_COMMIT: DWORD = 0x8000000;
pub const SEC_IMAGE: DWORD = 0x1000000;
pub const SEC_IMAGE_NO_EXECUTE: DWORD = 0x11000000;
pub const SEC_LARGE_PAGES: DWORD = 0x80000000;
pub const SEC_NOCACHE: DWORD = 0x10000000;
pub const SEC_RESERVE: DWORD = 0x4000000;
pub const SEC_WRITECOMBINE: DWORD = 0x40000000;

pub const FILE_MAP_ALL_ACCESS: DWORD = 0xf001f;
pub const FILE_MAP_READ: DWORD = 0x4;
pub const FILE_MAP_WRITE: DWORD = 0x2;
pub const FILE_MAP_COPY: DWORD = 0x1;
pub const FILE_MAP_EXECUTE: DWORD = 0x20;

pub const PROCESSOR_ARCHITECTURE_INTEL: WORD = 0;
pub const PROCESSOR_ARCHITECTURE_ARM: WORD = 5;
pub const PROCESSOR_ARCHITECTURE_IA64: WORD = 6;
pub const PROCESSOR_ARCHITECTURE_AMD64: WORD = 9;
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: WORD = 0xffff;

pub const MOVEFILE_COPY_ALLOWED: DWORD = 2;
pub const MOVEFILE_CREATE_HARDLINK: DWORD = 16;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: DWORD = 4;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: DWORD = 32;
pub const MOVEFILE_REPLACE_EXISTING: DWORD = 1;
pub const MOVEFILE_WRITE_THROUGH: DWORD = 8;

pub const SYMBOLIC_LINK_FLAG_DIRECTORY: DWORD = 1;

pub const FILE_SHARE_DELETE: DWORD = 0x4;
pub const FILE_SHARE_READ: DWORD = 0x1;
pub const FILE_SHARE_WRITE: DWORD = 0x2;

pub const CREATE_ALWAYS: DWORD = 2;
pub const CREATE_NEW: DWORD = 1;
pub const OPEN_ALWAYS: DWORD = 4;
pub const OPEN_EXISTING: DWORD = 3;
pub const TRUNCATE_EXISTING: DWORD = 5;

pub const FILE_APPEND_DATA: DWORD = 0x00000004;
pub const FILE_READ_DATA: DWORD = 0x00000001;
pub const FILE_WRITE_DATA: DWORD = 0x00000002;

pub const FILE_ATTRIBUTE_ARCHIVE: DWORD = 0x20;
pub const FILE_ATTRIBUTE_COMPRESSED: DWORD = 0x800;
pub const FILE_ATTRIBUTE_DEVICE: DWORD = 0x40;
pub const FILE_ATTRIBUTE_DIRECTORY: DWORD = 0x10;
pub const FILE_ATTRIBUTE_ENCRYPTED: DWORD = 0x4000;
pub const FILE_ATTRIBUTE_HIDDEN: DWORD = 0x2;
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: DWORD = 0x8000;
pub const FILE_ATTRIBUTE_NORMAL: DWORD = 0x80;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: DWORD = 0x2000;
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: DWORD = 0x20000;
pub const FILE_ATTRIBUTE_OFFLINE: DWORD = 0x1000;
pub const FILE_ATTRIBUTE_READONLY: DWORD = 0x1;
pub const FILE_ATTRIBUTE_REPARSE_POINT: DWORD = 0x400;
pub const FILE_ATTRIBUTE_SPARSE_FILE: DWORD = 0x200;
pub const FILE_ATTRIBUTE_SYSTEM: DWORD = 0x4;
pub const FILE_ATTRIBUTE_TEMPORARY: DWORD = 0x100;
pub const FILE_ATTRIBUTE_VIRTUAL: DWORD = 0x10000;

pub const FILE_FLAG_BACKUP_SEMANTICS: DWORD = 0x02000000;
pub const FILE_FLAG_DELETE_ON_CLOSE: DWORD = 0x04000000;
pub const FILE_FLAG_NO_BUFFERING: DWORD = 0x20000000;
pub const FILE_FLAG_OPEN_NO_RECALL: DWORD = 0x00100000;
pub const FILE_FLAG_OPEN_REPARSE_POINT: DWORD = 0x00200000;
pub const FILE_FLAG_OVERLAPPED: DWORD = 0x40000000;
pub const FILE_FLAG_POSIX_SEMANTICS: DWORD = 0x1000000;
pub const FILE_FLAG_RANDOM_ACCESS: DWORD = 0x10000000;
pub const FILE_FLAG_SESSION_AWARE: DWORD = 0x00800000;
pub const FILE_FLAG_SEQUENTIAL_SCAN: DWORD = 0x08000000;
pub const FILE_FLAG_WRITE_THROUGH: DWORD = 0x80000000;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: DWORD = 0x00080000;

pub const FILE_NAME_NORMALIZED: DWORD = 0x0;
pub const FILE_NAME_OPENED: DWORD = 0x8;

pub const VOLUME_NAME_DOS: DWORD = 0x0;
pub const VOLUME_NAME_GUID: DWORD = 0x1;
pub const VOLUME_NAME_NONE: DWORD = 0x4;
pub const VOLUME_NAME_NT: DWORD = 0x2;

pub const GENERIC_READ: DWORD = 0x80000000;
pub const GENERIC_WRITE: DWORD = 0x40000000;
pub const GENERIC_EXECUTE: DWORD = 0x20000000;
pub const GENERIC_ALL: DWORD = 0x10000000;
pub const FILE_WRITE_ATTRIBUTES: DWORD = 0x00000100;
pub const FILE_READ_ATTRIBUTES: DWORD = 0x00000080;

pub const STANDARD_RIGHTS_READ: DWORD = 0x20000;
pub const STANDARD_RIGHTS_WRITE: DWORD = 0x20000;
pub const FILE_WRITE_EA: DWORD = 0x00000010;
pub const FILE_READ_EA: DWORD = 0x00000008;
pub const FILE_GENERIC_READ: DWORD =
    STANDARD_RIGHTS_READ | FILE_READ_DATA |
    FILE_READ_ATTRIBUTES | FILE_READ_EA | SYNCHRONIZE;
pub const FILE_GENERIC_WRITE: DWORD =
    STANDARD_RIGHTS_WRITE | FILE_WRITE_DATA |
    FILE_WRITE_ATTRIBUTES | FILE_WRITE_EA | FILE_APPEND_DATA |
    SYNCHRONIZE;

pub const FILE_BEGIN: DWORD = 0;
pub const FILE_CURRENT: DWORD = 1;
pub const FILE_END: DWORD = 2;

pub const MAX_PROTOCOL_CHAIN: DWORD = 7;
pub const WSAPROTOCOL_LEN: DWORD = 255;
pub const INVALID_SOCKET: SOCKET = !0;

pub const DETACHED_PROCESS: DWORD = 0x00000008;
pub const CREATE_NEW_PROCESS_GROUP: DWORD = 0x00000200;
pub const CREATE_UNICODE_ENVIRONMENT: DWORD = 0x00000400;

pub const PIPE_ACCESS_DUPLEX: DWORD = 0x00000003;
pub const PIPE_ACCESS_INBOUND: DWORD = 0x00000001;
pub const PIPE_ACCESS_OUTBOUND: DWORD = 0x00000002;
pub const PIPE_TYPE_BYTE: DWORD = 0x00000000;
pub const PIPE_TYPE_MESSAGE: DWORD = 0x00000004;
pub const PIPE_READMODE_BYTE: DWORD = 0x00000000;
pub const PIPE_READMODE_MESSAGE: DWORD = 0x00000002;
pub const PIPE_WAIT: DWORD = 0x00000000;
pub const PIPE_NOWAIT: DWORD = 0x00000001;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: DWORD = 0x00000000;
pub const PIPE_REJECT_REMOTE_CLIENTS: DWORD = 0x00000008;
pub const PIPE_UNLIMITED_INSTANCES: DWORD = 255;

pub const IPPROTO_RAW: c_int = 255;

pub const FIONBIO: c_long = -0x7FFB9982;

#[cfg(target_env = "msvc")]
#[link(name = "kernel32")]
#[link(name = "shell32")]
#[link(name = "msvcrt")]
extern {}

extern {
    #[link_name = "_chmod"]
    pub fn chmod(path: *const c_char, mode: c_int) -> c_int;
    #[link_name = "_wchmod"]
    pub fn wchmod(path: *const wchar_t, mode: c_int) -> c_int;
    #[link_name = "_mkdir"]
    pub fn mkdir(path: *const c_char) -> c_int;
    #[link_name = "_wrmdir"]
    pub fn wrmdir(path: *const wchar_t) -> c_int;
    #[link_name = "_fstat64"]
    pub fn fstat(fildes: c_int, buf: *mut stat) -> c_int;
    #[link_name = "_stat64"]
    pub fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    #[link_name = "_wstat64"]
    pub fn wstat(path: *const wchar_t, buf: *mut stat) -> c_int;
    #[link_name = "_wutime64"]
    pub fn wutime(file: *const wchar_t, buf: *mut utimbuf) -> c_int;
    #[link_name = "_popen"]
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut ::FILE;
    #[link_name = "_pclose"]
    pub fn pclose(stream: *mut ::FILE) -> c_int;
    #[link_name = "_fdopen"]
    pub fn fdopen(fd: c_int, mode: *const c_char) -> *mut ::FILE;
    #[link_name = "_fileno"]
    pub fn fileno(stream: *mut ::FILE) -> c_int;
    #[link_name = "_open"]
    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    #[link_name = "_wopen"]
    pub fn wopen(path: *const wchar_t, oflag: c_int, ...) -> c_int;
    #[link_name = "_creat"]
    pub fn creat(path: *const c_char, mode: c_int) -> c_int;
    #[link_name = "_access"]
    pub fn access(path: *const c_char, amode: c_int) -> c_int;
    #[link_name = "_chdir"]
    pub fn chdir(dir: *const c_char) -> c_int;
    #[link_name = "_close"]
    pub fn close(fd: c_int) -> c_int;
    #[link_name = "_dup"]
    pub fn dup(fd: c_int) -> c_int;
    #[link_name = "_dup2"]
    pub fn dup2(src: c_int, dst: c_int) -> c_int;
    #[link_name = "_execv"]
    pub fn execv(prog: *const c_char, argv: *const *const c_char) -> intptr_t;
    #[link_name = "_execve"]
    pub fn execve(prog: *const c_char, argv: *const *const c_char,
                  envp: *const *const c_char) -> c_int;
    #[link_name = "_execvp"]
    pub fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int;
    #[link_name = "_execvpe"]
    pub fn execvpe(c: *const c_char, argv: *const *const c_char,
                   envp: *const *const c_char) -> c_int;
    #[link_name = "_getcwd"]
    pub fn getcwd(buf: *mut c_char, size: c_int) -> *mut c_char;
    #[link_name = "_getpid"]
    pub fn getpid() -> c_int;
    #[link_name = "_isatty"]
    pub fn isatty(fd: c_int) -> c_int;
    #[link_name = "_lseek"]
    pub fn lseek(fd: c_int, offset: c_long, origin: c_int) -> c_long;
    #[link_name = "_pipe"]
    pub fn pipe(fds: *mut c_int, psize: c_uint, textmode: c_int) -> c_int;
    #[link_name = "_read"]
    pub fn read(fd: c_int, buf: *mut ::c_void, count: c_uint) -> c_int;
    #[link_name = "_rmdir"]
    pub fn rmdir(path: *const c_char) -> c_int;
    #[link_name = "_unlink"]
    pub fn unlink(c: *const c_char) -> c_int;
    #[link_name = "_write"]
    pub fn write(fd: c_int, buf: *const ::c_void, count: c_uint) -> c_int;
    #[link_name = "_commit"]
    pub fn commit(fd: c_int) -> c_int;
    #[link_name = "_get_osfhandle"]
    pub fn get_osfhandle(fd: c_int) -> intptr_t;
    #[link_name = "_open_osfhandle"]
    pub fn open_osfhandle(osfhandle: intptr_t, flags: c_int) -> c_int;
}

extern "system" {
    pub fn GetEnvironmentVariableW(n: LPCWSTR,
                                   v: LPWSTR,
                                   nsize: DWORD)
                                   -> DWORD;
    pub fn SetEnvironmentVariableW(n: LPCWSTR, v: LPCWSTR)
                                   -> BOOL;
    pub fn GetEnvironmentStringsW() -> LPWCH;
    pub fn FreeEnvironmentStringsW(env_ptr: LPWCH) -> BOOL;
    pub fn GetModuleFileNameW(hModule: HMODULE,
                              lpFilename: LPWSTR,
                              nSize: DWORD)
                              -> DWORD;
    pub fn CreateDirectoryW(lpPathName: LPCWSTR,
                            lpSecurityAttributes:
                            LPSECURITY_ATTRIBUTES)
                            -> BOOL;
    pub fn CopyFileW(lpExistingFileName: LPCWSTR,
                     lpNewFileName: LPCWSTR,
                     bFailIfExists: BOOL)
                     -> BOOL;
    pub fn DeleteFileW(lpPathName: LPCWSTR) -> BOOL;
    pub fn RemoveDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    pub fn GetCurrentDirectoryW(nBufferLength: DWORD,
                                lpBuffer: LPWSTR)
                                -> DWORD;
    pub fn SetCurrentDirectoryW(lpPathName: LPCWSTR) -> BOOL;
    pub fn GetLastError() -> DWORD;
    pub fn FindFirstFileW(fileName: LPCWSTR, findFileData: LPWIN32_FIND_DATAW)
                          -> HANDLE;
    pub fn FindNextFileW(findFile: HANDLE, findFileData: LPWIN32_FIND_DATAW)
                         -> BOOL;
    pub fn FindClose(findFile: HANDLE) -> BOOL;
    pub fn DuplicateHandle(hSourceProcessHandle: HANDLE,
                           hSourceHandle: HANDLE,
                           hTargetProcessHandle: HANDLE,
                           lpTargetHandle: LPHANDLE,
                           dwDesiredAccess: DWORD,
                           bInheritHandle: BOOL,
                           dwOptions: DWORD)
                           -> BOOL;
    pub fn CloseHandle(hObject: HANDLE) -> BOOL;
    pub fn OpenProcess(dwDesiredAccess: DWORD,
                       bInheritHandle: BOOL,
                       dwProcessId: DWORD)
                       -> HANDLE;
    pub fn GetCurrentProcess() -> HANDLE;
    pub fn CreateProcessW(lpApplicationName: LPCWSTR,
                          lpCommandLine: LPWSTR,
                          lpProcessAttributes:
                          LPSECURITY_ATTRIBUTES,
                          lpThreadAttributes:
                          LPSECURITY_ATTRIBUTES,
                          bInheritHandles: BOOL,
                          dwCreationFlags: DWORD,
                          lpEnvironment: LPVOID,
                          lpCurrentDirectory: LPCWSTR,
                          lpStartupInfo: LPSTARTUPINFOW,
                          lpProcessInformation:
                          LPPROCESS_INFORMATION)
                          -> BOOL;
    pub fn WaitForSingleObject(hHandle: HANDLE,
                               dwMilliseconds: DWORD)
                               -> DWORD;
    pub fn TerminateProcess(hProcess: HANDLE, uExitCode: c_uint)
                            -> BOOL;
    pub fn GetExitCodeProcess(hProcess: HANDLE,
                              lpExitCode: LPDWORD)
                              -> BOOL;
    pub fn GetSystemInfo(lpSystemInfo: LPSYSTEM_INFO);
    pub fn VirtualAlloc(lpAddress: LPVOID,
                        dwSize: SIZE_T,
                        flAllocationType: DWORD,
                        flProtect: DWORD)
                        -> LPVOID;
    pub fn VirtualFree(lpAddress: LPVOID,
                       dwSize: SIZE_T,
                       dwFreeType: DWORD)
                       -> BOOL;
    pub fn VirtualLock(lpAddress: LPVOID, dwSize: SIZE_T) -> BOOL;
    pub fn VirtualUnlock(lpAddress: LPVOID, dwSize: SIZE_T)
                         -> BOOL;
    pub fn VirtualProtect(lpAddress: LPVOID,
                          dwSize: SIZE_T,
                          flNewProtect: DWORD,
                          lpflOldProtect: LPDWORD)
                          -> BOOL;
    pub fn VirtualQuery(lpAddress: LPCVOID,
                        lpBuffer: PMEMORY_BASIC_INFORMATION,
                        dwLength: SIZE_T)
                        -> SIZE_T;
    pub fn CreateFileMappingW(hFile: HANDLE,
                              lpAttributes: LPSECURITY_ATTRIBUTES,
                              flProtect: DWORD,
                              dwMaximumSizeHigh: DWORD,
                              dwMaximumSizeLow: DWORD,
                              lpName: LPCWSTR)
                              -> HANDLE;
    pub fn MapViewOfFile(hFileMappingObject: HANDLE,
                         dwDesiredAccess: DWORD,
                         dwFileOffsetHigh: DWORD,
                         dwFileOffsetLow: DWORD,
                         dwNumberOfBytesToMap: SIZE_T)
                         -> LPVOID;
    pub fn UnmapViewOfFile(lpBaseAddress: LPCVOID) -> BOOL;
    pub fn MoveFileExW(lpExistingFileName: LPCWSTR,
                       lpNewFileName: LPCWSTR,
                       dwFlags: DWORD) -> BOOL;
    pub fn CreateHardLinkW(lpSymlinkFileName: LPCWSTR,
                           lpTargetFileName: LPCWSTR,
                           lpSecurityAttributes: LPSECURITY_ATTRIBUTES)
                            -> BOOL;
    pub fn FlushFileBuffers(hFile: HANDLE) -> BOOL;
    pub fn CreateFileW(lpFileName: LPCWSTR,
                       dwDesiredAccess: DWORD,
                       dwShareMode: DWORD,
                       lpSecurityAttributes: LPSECURITY_ATTRIBUTES,
                       dwCreationDisposition: DWORD,
                       dwFlagsAndAttributes: DWORD,
                       hTemplateFile: HANDLE) -> HANDLE;
    pub fn ReadFile(hFile: HANDLE,
                    lpBuffer: LPVOID,
                    nNumberOfBytesToRead: DWORD,
                    lpNumberOfBytesRead: LPDWORD,
                    lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn WriteFile(hFile: HANDLE,
                     lpBuffer: LPCVOID,
                     nNumberOfBytesToWrite: DWORD,
                     lpNumberOfBytesWritten: LPDWORD,
                     lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn SetFilePointerEx(hFile: HANDLE,
                            liDistanceToMove: LARGE_INTEGER,
                            lpNewFilePointer: PLARGE_INTEGER,
                            dwMoveMethod: DWORD) -> BOOL;
    pub fn SetEndOfFile(hFile: HANDLE) -> BOOL;

    pub fn GetSystemTimeAsFileTime(lpSystemTimeAsFileTime: LPFILETIME);

    pub fn QueryPerformanceFrequency(lpFrequency: *mut LARGE_INTEGER) -> BOOL;
    pub fn QueryPerformanceCounter(lpPerformanceCount: *mut LARGE_INTEGER)
                                   -> BOOL;

    pub fn GetCurrentProcessId() -> DWORD;
    pub fn CreateNamedPipeW(
                lpName: LPCWSTR,
                dwOpenMode: DWORD,
                dwPipeMode: DWORD,
                nMaxInstances: DWORD,
                nOutBufferSize: DWORD,
                nInBufferSize: DWORD,
                nDefaultTimeOut: DWORD,
                lpSecurityAttributes: LPSECURITY_ATTRIBUTES
                ) -> HANDLE;
    pub fn ConnectNamedPipe(hNamedPipe: HANDLE,
                            lpOverlapped: LPOVERLAPPED) -> BOOL;
    pub fn WaitNamedPipeW(lpNamedPipeName: LPCWSTR,
                          nTimeOut: DWORD) -> BOOL;
    pub fn SetNamedPipeHandleState(hNamedPipe: HANDLE,
                                   lpMode: LPDWORD,
                                   lpMaxCollectionCount: LPDWORD,
                                   lpCollectDataTimeout: LPDWORD)
                                                -> BOOL;
    pub fn CreateEventW(lpEventAttributes: LPSECURITY_ATTRIBUTES,
                        bManualReset: BOOL,
                        bInitialState: BOOL,
                        lpName: LPCWSTR) -> HANDLE;
    pub fn GetOverlappedResult(hFile: HANDLE,
                               lpOverlapped: LPOVERLAPPED,
                               lpNumberOfBytesTransferred: LPDWORD,
                               bWait: BOOL) -> BOOL;
    pub fn DisconnectNamedPipe(hNamedPipe: HANDLE) -> BOOL;

    pub fn ioctlsocket(s: SOCKET, cmd: c_long, argp: *mut c_ulong) -> c_int;
    pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> SOCKET;
    pub fn connect(socket: SOCKET, address: *const sockaddr,
                   len: c_int) -> c_int;
    pub fn bind(socket: SOCKET, address: *const sockaddr,
                address_len: c_int) -> c_int;
    pub fn listen(socket: SOCKET, backlog: c_int) -> c_int;
    pub fn accept(socket: SOCKET, address: *mut sockaddr,
                  address_len: *mut c_int) -> SOCKET;
    pub fn getpeername(socket: SOCKET, address: *mut sockaddr,
                       address_len: *mut c_int) -> c_int;
    pub fn getsockname(socket: SOCKET, address: *mut sockaddr,
                       address_len: *mut c_int) -> c_int;
    pub fn setsockopt(socket: SOCKET, level: c_int, name: c_int,
                      value: *const c_char,
                      option_len: c_int) -> c_int;
    pub fn closesocket(socket: SOCKET) -> c_int;
    pub fn recv(socket: SOCKET, buf: *mut c_char, len: c_int,
                flags: c_int) -> c_int;
    pub fn send(socket: SOCKET, buf: *const c_char, len: c_int,
                flags: c_int) -> c_int;
    pub fn recvfrom(socket: SOCKET, buf: *mut c_char, len: c_int,
                    flags: c_int, addr: *mut sockaddr,
                    addrlen: *mut c_int) -> c_int;
    pub fn sendto(socket: SOCKET, buf: *const c_char, len: c_int,
                  flags: c_int, addr: *const sockaddr,
                  addrlen: c_int) -> c_int;
    pub fn shutdown(socket: SOCKET, how: c_int) -> c_int;
}
