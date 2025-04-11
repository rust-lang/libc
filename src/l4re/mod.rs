//! Definitions found commonly among almost all Unix derivatives
//!
//! More functions and definitions can be found in the more specific modules
//! according to the platform in question.

use core::mem::size_of;

use crate::prelude::*;

// PUB_TYPE

pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type locale_t = *mut c_void;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = size_t;
pub type cc_t = c_uchar;
pub type sa_family_t = u16;
pub type pthread_key_t = c_uint;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;
pub type clockid_t = c_int;
pub type key_t = c_int;
pub type id_t = c_uint;
pub type useconds_t = u32;
pub type dev_t = u64;
pub type socklen_t = u32;
pub type mode_t = u32;
pub type ino64_t = u64;
pub type off64_t = i64;
pub type blkcnt64_t = i64;
pub type rlim64_t = u64;
pub type nfds_t = c_ulong;
pub type nl_item = c_int;
pub type idtype_t = c_uint;
pub type loff_t = c_longlong;

pub type Elf32_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Off = u32;
pub type Elf32_Addr = u32;
pub type Elf32_Xword = u64;
pub type Elf32_Sword = i32;

pub type Elf64_Half = u16;
pub type Elf64_Word = u32;
pub type Elf64_Off = u64;
pub type Elf64_Addr = u64;
pub type Elf64_Xword = u64;
pub type Elf64_Sword = i32;
pub type Elf64_Sxword = i64;

pub type Elf32_Section = u16;
pub type Elf64_Section = u16;

pub type Elf32_Relr = Elf32_Word;
pub type Elf64_Relr = Elf32_Xword;
pub type Elf32_Rel = __c_anonymous_elf32_rel;
pub type Elf64_Rel = __c_anonymous_elf64_rel;

pub type clock_t = c_long;
pub type time_t = c_long;
pub type suseconds_t = c_long;
pub type ino_t = c_ulong;
pub type off_t = c_long;
pub type blkcnt_t = c_long;

pub type shmatt_t = c_ulong;
pub type fsblkcnt_t = c_ulong;
pub type fsfilcnt_t = c_ulong;
pub type rlim_t = c_ulong;

pub type timer_t = *mut c_void;

pub type pthread_once_t = c_int;
pub type pthread_spinlock_t = c_int;

pub type iconv_t = *mut c_void;

pub type Elf32_Rela = __c_anonymous_elf32_rela;
pub type Elf64_Rela = __c_anonymous_elf64_rela;

// PUB_STRUCT

s! {
    pub struct group {
        pub gr_name: *mut c_char,
        pub gr_passwd: *mut c_char,
        pub gr_gid: uid_t,
        pub gr_mem: *mut *mut c_char,
    }

    pub struct utimbuf {
        pub actime: time_t,
        pub modtime: time_t,
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }

    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }

    pub struct rusage {
        pub ru_utime: timeval,
        pub ru_stime: timeval,
        pub ru_maxrss: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad1: u32,
        pub ru_ixrss: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad2: u32,
        pub ru_idrss: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad3: u32,
        pub ru_isrss: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad4: u32,
        pub ru_minflt: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad5: u32,
        pub ru_majflt: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad6: u32,
        pub ru_nswap: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad7: u32,
        pub ru_inblock: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad8: u32,
        pub ru_oublock: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad9: u32,
        pub ru_msgsnd: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad10: u32,
        pub ru_msgrcv: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad11: u32,
        pub ru_nsignals: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad12: u32,
        pub ru_nvcsw: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad13: u32,
        pub ru_nivcsw: c_long,
        #[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
        __pad14: u32,
    }

    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: c_uint,
    }

    pub struct hostent {
        pub h_name: *mut c_char,
        pub h_aliases: *mut *mut c_char,
        pub h_addrtype: c_int,
        pub h_length: c_int,
        pub h_addr_list: *mut *mut c_char,
    }

    pub struct iovec {
        pub iov_base: *mut c_void,
        pub iov_len: size_t,
    }

    pub struct pollfd {
        pub fd: c_int,
        pub events: c_short,
        pub revents: c_short,
    }

    pub struct winsize {
        pub ws_row: c_ushort,
        pub ws_col: c_ushort,
        pub ws_xpixel: c_ushort,
        pub ws_ypixel: c_ushort,
    }

    pub struct linger {
        pub l_onoff: c_int,
        pub l_linger: c_int,
    }

    pub struct sigval {
        // Actually a union of an int and a void*
        pub sival_ptr: *mut c_void,
    }

    // <sys/time.h>
    pub struct itimerval {
        pub it_interval: crate::timeval,
        pub it_value: crate::timeval,
    }

    // <sys/times.h>
    pub struct tms {
        pub tms_utime: crate::clock_t,
        pub tms_stime: crate::clock_t,
        pub tms_cutime: crate::clock_t,
        pub tms_cstime: crate::clock_t,
    }

    pub struct servent {
        pub s_name: *mut c_char,
        pub s_aliases: *mut *mut c_char,
        pub s_port: c_int,
        pub s_proto: *mut c_char,
    }

    pub struct protoent {
        pub p_name: *mut c_char,
        pub p_aliases: *mut *mut c_char,
        pub p_proto: c_int,
    }

    pub struct __c_anonymous_sigev_thread {
        pub _function: Option<extern "C" fn(crate::sigval) -> *mut c_void>,
        pub _attribute: *mut crate::pthread_attr_t,
    }

    pub struct in_addr {
        pub s_addr: in_addr_t,
    }

    #[repr(align(4))]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct ip_mreqn {
        pub imr_multiaddr: in_addr,
        pub imr_address: in_addr,
        pub imr_ifindex: c_int,
    }

    pub struct ip_mreq_source {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
        pub imr_sourceaddr: in_addr,
    }

    pub struct in_pktinfo {
        pub ipi_ifindex: c_int,
        pub ipi_spec_dst: crate::in_addr,
        pub ipi_addr: crate::in_addr,
    }

    pub struct in6_rtmsg {
        rtmsg_dst: crate::in6_addr,
        rtmsg_src: crate::in6_addr,
        rtmsg_gateway: crate::in6_addr,
        rtmsg_type: u32,
        rtmsg_dst_len: u16,
        rtmsg_src_len: u16,
        rtmsg_metric: u32,
        rtmsg_info: c_ulong,
        rtmsg_flags: u32,
        rtmsg_ifindex: c_int,
    }

    pub struct arpreq {
        pub arp_pa: crate::sockaddr,
        pub arp_ha: crate::sockaddr,
        pub arp_flags: c_int,
        pub arp_netmask: crate::sockaddr,
        pub arp_dev: [c_char; 16],
    }

    pub struct arpreq_old {
        pub arp_pa: crate::sockaddr,
        pub arp_ha: crate::sockaddr,
        pub arp_flags: c_int,
        pub arp_netmask: crate::sockaddr,
    }
    pub struct arpd_request {
        pub req: c_ushort,
        pub ip: u32,
        pub dev: c_ulong,
        pub stamp: c_ulong,
        pub updated: c_ulong,
        pub ha: [c_uchar; crate::MAX_ADDR_LEN],
    }

    pub struct arphdr {
        pub ar_hrd: u16,
        pub ar_pro: u16,
        pub ar_hln: u8,
        pub ar_pln: u8,
        pub ar_op: u16,
    }

    pub struct packet_mreq {
        pub mr_ifindex: c_int,
        pub mr_type: c_ushort,
        pub mr_alen: c_ushort,
        pub mr_address: [c_uchar; 8],
    }

    pub struct regmatch_t {
        pub rm_so: regoff_t,
        pub rm_eo: regoff_t,
    }

    pub struct option {
        pub name: *const c_char,
        pub has_arg: c_int,
        pub flag: *mut c_int,
        pub val: c_int,
    }

    pub struct termios {
        pub c_iflag: crate::tcflag_t,
        pub c_oflag: crate::tcflag_t,
        pub c_cflag: crate::tcflag_t,
        pub c_lflag: crate::tcflag_t,
        pub c_line: crate::cc_t,
        pub c_cc: [crate::cc_t; crate::NCCS],
        pub c_ispeed: crate::speed_t,
        pub c_ospeed: crate::speed_t,
    }

    pub struct flock {
        pub l_type: c_short,
        pub l_whence: c_short,
        pub l_start: off_t,
        pub l_len: off_t,
        pub l_pid: crate::pid_t,
    }

    pub struct ucred {
        pub pid: crate::pid_t,
        pub uid: uid_t,
        pub gid: uid_t,
    }

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [c_char; 14],
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: crate::in_port_t,
        pub sin_addr: crate::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: crate::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: crate::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct addrinfo {
        pub ai_flags: c_int,
        pub ai_family: c_int,
        pub ai_socktype: c_int,
        pub ai_protocol: c_int,
        pub ai_addrlen: socklen_t,

        pub ai_addr: *mut crate::sockaddr,

        pub ai_canonname: *mut c_char,

        pub ai_next: *mut addrinfo,
    }

    pub struct sockaddr_ll {
        pub sll_family: c_ushort,
        pub sll_protocol: c_ushort,
        pub sll_ifindex: c_int,
        pub sll_hatype: c_ushort,
        pub sll_pkttype: c_uchar,
        pub sll_halen: c_uchar,
        pub sll_addr: [c_uchar; 8],
    }

    pub struct fd_set {
        fds_bits: [c_ulong; FD_SETSIZE as usize / ULONG_SIZE],
    }

    pub struct tm {
        pub tm_sec: c_int,
        pub tm_min: c_int,
        pub tm_hour: c_int,
        pub tm_mday: c_int,
        pub tm_mon: c_int,
        pub tm_year: c_int,
        pub tm_wday: c_int,
        pub tm_yday: c_int,
        pub tm_isdst: c_int,
        pub tm_gmtoff: c_long,
        pub tm_zone: *const c_char,
    }

    pub struct sched_param {
        pub sched_priority: c_int,
        #[cfg(target_env = "musl")]
        pub sched_ss_low_priority: c_int,
        #[cfg(target_env = "musl")]
        pub sched_ss_repl_period: crate::timespec,
        #[cfg(target_env = "musl")]
        pub sched_ss_init_budget: crate::timespec,
        #[cfg(target_env = "musl")]
        pub sched_ss_max_repl: c_int,
    }

    pub struct Dl_info {
        pub dli_fname: *const c_char,
        pub dli_fbase: *mut c_void,
        pub dli_sname: *const c_char,
        pub dli_saddr: *mut c_void,
    }

    pub struct lconv {
        pub decimal_point: *mut c_char,
        pub thousands_sep: *mut c_char,
        pub grouping: *mut c_char,
        pub int_curr_symbol: *mut c_char,
        pub currency_symbol: *mut c_char,
        pub mon_decimal_point: *mut c_char,
        pub mon_thousands_sep: *mut c_char,
        pub mon_grouping: *mut c_char,
        pub positive_sign: *mut c_char,
        pub negative_sign: *mut c_char,
        pub int_frac_digits: c_char,
        pub frac_digits: c_char,
        pub p_cs_precedes: c_char,
        pub p_sep_by_space: c_char,
        pub n_cs_precedes: c_char,
        pub n_sep_by_space: c_char,
        pub p_sign_posn: c_char,
        pub n_sign_posn: c_char,
        pub int_p_cs_precedes: c_char,
        pub int_p_sep_by_space: c_char,
        pub int_n_cs_precedes: c_char,
        pub int_n_sep_by_space: c_char,
        pub int_p_sign_posn: c_char,
        pub int_n_sign_posn: c_char,
    }

    pub struct rlimit64 {
        pub rlim_cur: rlim64_t,
        pub rlim_max: rlim64_t,
    }

    pub struct glob_t {
        pub gl_pathc: size_t,
        pub gl_pathv: *mut *mut c_char,
        pub gl_offs: size_t,
        pub gl_flags: c_int,

        __unused1: *mut c_void,
        __unused2: *mut c_void,
        __unused3: *mut c_void,
        __unused4: *mut c_void,
        __unused5: *mut c_void,
    }

    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut c_char,
        pub ifa_flags: c_uint,
        pub ifa_addr: *mut crate::sockaddr,
        pub ifa_netmask: *mut crate::sockaddr,
        pub ifa_ifu: *mut crate::sockaddr, // FIXME(union) This should be a union
        pub ifa_data: *mut c_void,
    }

    pub struct __c_anonymous_ifru_map {
        pub mem_start: c_ulong,
        pub mem_end: c_ulong,
        pub base_addr: c_ushort,
        pub irq: c_uchar,
        pub dma: c_uchar,
        pub port: c_uchar,
    }

    pub struct passwd {
        pub pw_name: *mut c_char,
        pub pw_passwd: *mut c_char,
        pub pw_uid: uid_t,
        pub pw_gid: uid_t,
        pub pw_gecos: *mut c_char,
        pub pw_dir: *mut c_char,
        pub pw_shell: *mut c_char,
    }

    pub struct spwd {
        pub sp_namp: *mut c_char,
        pub sp_pwdp: *mut c_char,
        pub sp_lstchg: c_long,
        pub sp_min: c_long,
        pub sp_max: c_long,
        pub sp_warn: c_long,
        pub sp_inact: c_long,
        pub sp_expire: c_long,
        pub sp_flag: c_ulong,
    }

    pub struct itimerspec {
        pub it_interval: crate::timespec,
        pub it_value: crate::timespec,
    }

    pub struct fsid_t {
        __val: [c_int; 2],
    }

    pub struct cpu_set_t {
        #[cfg(all(target_pointer_width = "32", not(target_arch = "x86_64")))]
        bits: [u32; 32],
        #[cfg(not(all(target_pointer_width = "32", not(target_arch = "x86_64"))))]
        bits: [u64; 16],
    }

    pub struct if_nameindex {
        pub if_index: c_uint,
        pub if_name: *mut c_char,
    }

    pub struct mmsghdr {
        pub msg_hdr: crate::msghdr,
        pub msg_len: c_uint,
    }

    pub struct sembuf {
        pub sem_num: c_ushort,
        pub sem_op: c_short,
        pub sem_flg: c_short,
    }

    pub struct dl_phdr_info {
        #[cfg(target_pointer_width = "64")]
        pub dlpi_addr: Elf64_Addr,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_addr: Elf32_Addr,

        pub dlpi_name: *const c_char,

        #[cfg(target_pointer_width = "64")]
        pub dlpi_phdr: *const Elf64_Phdr,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_phdr: *const Elf32_Phdr,

        #[cfg(target_pointer_width = "64")]
        pub dlpi_phnum: Elf64_Half,
        #[cfg(target_pointer_width = "32")]
        pub dlpi_phnum: Elf32_Half,

        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_adds: c_ulonglong,
        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_subs: c_ulonglong,
        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_tls_modid: size_t,
        #[cfg(not(target_env = "uclibc"))]
        pub dlpi_tls_data: *mut c_void,
    }

    pub struct Elf32_Phdr {
        pub p_type: Elf32_Word,
        pub p_offset: Elf32_Off,
        pub p_vaddr: Elf32_Addr,
        pub p_paddr: Elf32_Addr,
        pub p_filesz: Elf32_Word,
        pub p_memsz: Elf32_Word,
        pub p_flags: Elf32_Word,
        pub p_align: Elf32_Word,
    }

    pub struct Elf64_Phdr {
        pub p_type: Elf64_Word,
        pub p_flags: Elf64_Word,
        pub p_offset: Elf64_Off,
        pub p_vaddr: Elf64_Addr,
        pub p_paddr: Elf64_Addr,
        pub p_filesz: Elf64_Xword,
        pub p_memsz: Elf64_Xword,
        pub p_align: Elf64_Xword,
    }

    pub struct Elf32_Shdr {
        pub sh_name: Elf32_Word,
        pub sh_type: Elf32_Word,
        pub sh_flags: Elf32_Word,
        pub sh_addr: Elf32_Addr,
        pub sh_offset: Elf32_Off,
        pub sh_size: Elf32_Word,
        pub sh_link: Elf32_Word,
        pub sh_info: Elf32_Word,
        pub sh_addralign: Elf32_Word,
        pub sh_entsize: Elf32_Word,
    }

    pub struct Elf64_Shdr {
        pub sh_name: Elf64_Word,
        pub sh_type: Elf64_Word,
        pub sh_flags: Elf64_Xword,
        pub sh_addr: Elf64_Addr,
        pub sh_offset: Elf64_Off,
        pub sh_size: Elf64_Xword,
        pub sh_link: Elf64_Word,
        pub sh_info: Elf64_Word,
        pub sh_addralign: Elf64_Xword,
        pub sh_entsize: Elf64_Xword,
    }

    pub struct __c_anonymous_elf32_rela {
        pub r_offset: Elf32_Addr,
        pub r_info: Elf32_Word,
        pub r_addend: Elf32_Sword,
    }

    pub struct __c_anonymous_elf64_rela {
        pub r_offset: Elf64_Addr,
        pub r_info: Elf64_Xword,
        pub r_addend: Elf64_Sxword,
    }

    pub struct mntent {
        pub mnt_fsname: *mut c_char,
        pub mnt_dir: *mut c_char,
        pub mnt_type: *mut c_char,
        pub mnt_opts: *mut c_char,
        pub mnt_freq: c_int,
        pub mnt_passno: c_int,
    }

    pub struct __c_anonymous_elf32_rel {
        pub r_offset: Elf32_Addr,
        pub r_info: Elf32_Word,
    }

    pub struct __c_anonymous_elf64_rel {
        pub r_offset: Elf64_Addr,
        pub r_info: Elf64_Xword,
    }

    pub struct Elf32_Ehdr {
        pub e_ident: [c_uchar; 16],
        pub e_type: Elf32_Half,
        pub e_machine: Elf32_Half,
        pub e_version: Elf32_Word,
        pub e_entry: Elf32_Addr,
        pub e_phoff: Elf32_Off,
        pub e_shoff: Elf32_Off,
        pub e_flags: Elf32_Word,
        pub e_ehsize: Elf32_Half,
        pub e_phentsize: Elf32_Half,
        pub e_phnum: Elf32_Half,
        pub e_shentsize: Elf32_Half,
        pub e_shnum: Elf32_Half,
        pub e_shstrndx: Elf32_Half,
    }

    pub struct Elf64_Ehdr {
        pub e_ident: [c_uchar; 16],
        pub e_type: Elf64_Half,
        pub e_machine: Elf64_Half,
        pub e_version: Elf64_Word,
        pub e_entry: Elf64_Addr,
        pub e_phoff: Elf64_Off,
        pub e_shoff: Elf64_Off,
        pub e_flags: Elf64_Word,
        pub e_ehsize: Elf64_Half,
        pub e_phentsize: Elf64_Half,
        pub e_phnum: Elf64_Half,
        pub e_shentsize: Elf64_Half,
        pub e_shnum: Elf64_Half,
        pub e_shstrndx: Elf64_Half,
    }

    pub struct Elf32_Sym {
        pub st_name: Elf32_Word,
        pub st_value: Elf32_Addr,
        pub st_size: Elf32_Word,
        pub st_info: c_uchar,
        pub st_other: c_uchar,
        pub st_shndx: Elf32_Section,
    }

    pub struct Elf64_Sym {
        pub st_name: Elf64_Word,
        pub st_info: c_uchar,
        pub st_other: c_uchar,
        pub st_shndx: Elf64_Section,
        pub st_value: Elf64_Addr,
        pub st_size: Elf64_Xword,
    }

    pub struct termios2 {
        pub c_iflag: crate::tcflag_t,
        pub c_oflag: crate::tcflag_t,
        pub c_cflag: crate::tcflag_t,
        pub c_lflag: crate::tcflag_t,
        pub c_line: crate::cc_t,
        pub c_cc: [crate::cc_t; 19],
        pub c_ispeed: crate::speed_t,
        pub c_ospeed: crate::speed_t,
    }

    pub struct in6_pktinfo {
        pub ipi6_addr: crate::in6_addr,
        pub ipi6_ifindex: c_uint,
    }

    #[cfg_attr(
        any(
            target_pointer_width = "32",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64",
            target_arch = "riscv32",
        ),
        repr(align(4))
    )]
    #[cfg_attr(
        not(any(
            target_pointer_width = "32",
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "riscv64",
            target_arch = "riscv32",
        )),
        repr(align(8))
    )]
    pub struct pthread_mutexattr_t {
        size: [u8; crate::__SIZEOF_PTHREAD_MUTEXATTR_T],
    }

    #[repr(align(4))]
    pub struct pthread_rwlockattr_t {
        size: [u8; crate::__SIZEOF_PTHREAD_RWLOCKATTR_T],
    }

    #[repr(align(4))]
    pub struct pthread_condattr_t {
        size: [u8; crate::__SIZEOF_PTHREAD_CONDATTR_T],
    }

    #[repr(align(4))]
    pub struct pthread_barrierattr_t {
        size: [u8; crate::__SIZEOF_PTHREAD_BARRIERATTR_T],
    }
}

s_no_extra_traits! {
    pub union __c_anonymous_ifr_ifru {
        pub ifru_addr: crate::sockaddr,
        pub ifru_dstaddr: crate::sockaddr,
        pub ifru_broadaddr: crate::sockaddr,
        pub ifru_netmask: crate::sockaddr,
        pub ifru_hwaddr: crate::sockaddr,
        pub ifru_flags: c_short,
        pub ifru_ifindex: c_int,
        pub ifru_metric: c_int,
        pub ifru_mtu: c_int,
        pub ifru_map: __c_anonymous_ifru_map,
        pub ifru_slave: [c_char; crate::IFNAMSIZ],
        pub ifru_newname: [c_char; crate::IFNAMSIZ],
        pub ifru_data: *mut c_char,
    }

    pub struct ifreq {
        /// interface name, e.g. "en0"
        pub ifr_name: [c_char; crate::IFNAMSIZ],
        pub ifr_ifru: __c_anonymous_ifr_ifru,
    }

    pub union __c_anonymous_ifc_ifcu {
        pub ifcu_buf: *mut c_char,
        pub ifcu_req: *mut crate::ifreq,
    }

    /// Structure used in SIOCGIFCONF request.  Used to retrieve interface configuration for
    /// machine (useful for programs which must know all networks accessible).
    pub struct ifconf {
        /// Size of buffer
        pub ifc_len: c_int,
        pub ifc_ifcu: __c_anonymous_ifc_ifcu,
    }

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [c_char; 108],
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        #[cfg(target_pointer_width = "32")]
        __ss_pad2: [u8; 128 - 2 - 4],
        #[cfg(target_pointer_width = "64")]
        __ss_pad2: [u8; 128 - 2 - 8],
        __ss_align: size_t,
    }

    pub struct utsname {
        pub sysname: [c_char; 65],
        pub nodename: [c_char; 65],
        pub release: [c_char; 65],
        pub version: [c_char; 65],
        pub machine: [c_char; 65],
        pub domainname: [c_char; 65],
    }

    pub struct dirent {
        pub d_ino: crate::ino_t,
        pub d_off: off_t,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    pub struct dirent64 {
        pub d_ino: crate::ino64_t,
        pub d_off: off64_t,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    pub union __c_anonymous_sigev_un {
        _pad: [c_int; SIGEV_PAD_SIZE],
        pub _tid: c_int,
        pub _sigev_thread: __c_anonymous_sigev_thread,
    }

    pub struct sigevent {
        pub sigev_value: crate::sigval,
        pub sigev_signo: c_int,
        pub sigev_notify: c_int,
        pub _sigev_un: __c_anonymous_sigev_un,
    }

    #[cfg_attr(
        all(
            target_pointer_width = "32",
            any(target_arch = "arm", target_arch = "x86_64", target_arch = "x86")
        ),
        repr(align(4))
    )]
    #[cfg_attr(
        any(
            target_pointer_width = "64",
            not(any(target_arch = "arm", target_arch = "x86_64", target_arch = "x86"))
        ),
        repr(align(8))
    )]
    pub struct pthread_mutex_t {
        size: [u8; crate::__SIZEOF_PTHREAD_MUTEX_T],
    }

    #[cfg_attr(
        all(
            target_pointer_width = "32",
            any(target_arch = "arm", target_arch = "x86_64", target_arch = "x86")
        ),
        repr(align(4))
    )]
    #[cfg_attr(
        any(
            target_pointer_width = "64",
            not(any(target_arch = "arm", target_arch = "x86_64", target_arch = "x86"))
        ),
        repr(align(8))
    )]
    pub struct pthread_rwlock_t {
        size: [u8; crate::__SIZEOF_PTHREAD_RWLOCK_T],
    }

    #[cfg_attr(all(target_env = "musl", target_pointer_width = "32"), repr(align(4)))]
    #[cfg_attr(all(target_env = "musl", target_pointer_width = "64"), repr(align(8)))]
    #[cfg_attr(all(not(target_env = "musl"), target_arch = "x86"), repr(align(4)))]
    #[cfg_attr(
        all(not(target_env = "musl"), not(target_arch = "x86")),
        repr(align(8))
    )]
    pub struct pthread_cond_t {
        size: [u8; crate::__SIZEOF_PTHREAD_COND_T],
    }

    #[cfg_attr(
        all(
            target_pointer_width = "32",
            not(any(target_arch = "arm", target_arch = "x86_64", target_arch = "x86"))
        ),
        repr(align(4))
    )]
    #[cfg_attr(
        any(
            target_pointer_width = "64",
            not(any(target_arch = "arm", target_arch = "x86_64", target_arch = "x86"))
        ),
        repr(align(8))
    )]
    pub struct pthread_barrier_t {
        size: [u8; crate::__SIZEOF_PTHREAD_BARRIER_T],
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl fmt::Debug for ifreq {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("ifreq")
                    .field("ifr_name", &self.ifr_name)
                    .field("ifr_ifru", &self.ifr_ifru)
                    .finish()
            }
        }
        impl fmt::Debug for ifconf {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("ifconf")
                    .field("ifc_len", &self.ifc_len)
                    .field("ifc_ifcu", &self.ifc_ifcu)
                    .finish()
            }
        }

        impl PartialEq for sockaddr_un {
            fn eq(&self, other: &sockaddr_un) -> bool {
                self.sun_family == other.sun_family
                    && self
                        .sun_path
                        .iter()
                        .zip(other.sun_path.iter())
                        .all(|(a, b)| a == b)
            }
        }
        impl Eq for sockaddr_un {}
        impl fmt::Debug for sockaddr_un {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("sockaddr_un")
                    .field("sun_family", &self.sun_family)
                    // FIXME(debug): .field("sun_path", &self.sun_path)
                    .finish()
            }
        }
        impl hash::Hash for sockaddr_un {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.sun_family.hash(state);
                self.sun_path.hash(state);
            }
        }

        impl PartialEq for sockaddr_storage {
            fn eq(&self, other: &sockaddr_storage) -> bool {
                self.ss_family == other.ss_family
                    && self.__ss_align == other.__ss_align
                    && self
                        .__ss_pad2
                        .iter()
                        .zip(other.__ss_pad2.iter())
                        .all(|(a, b)| a == b)
            }
        }
        impl Eq for sockaddr_storage {}
        impl fmt::Debug for sockaddr_storage {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("sockaddr_storage")
                    .field("ss_family", &self.ss_family)
                    .field("__ss_align", &self.__ss_align)
                    // FIXME(debug): .field("__ss_pad2", &self.__ss_pad2)
                    .finish()
            }
        }
        impl hash::Hash for sockaddr_storage {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.ss_family.hash(state);
                self.__ss_align.hash(state);
                self.__ss_pad2.hash(state);
            }
        }

        impl PartialEq for utsname {
            fn eq(&self, other: &utsname) -> bool {
                self.sysname
                    .iter()
                    .zip(other.sysname.iter())
                    .all(|(a, b)| a == b)
                    && self
                        .nodename
                        .iter()
                        .zip(other.nodename.iter())
                        .all(|(a, b)| a == b)
                    && self
                        .release
                        .iter()
                        .zip(other.release.iter())
                        .all(|(a, b)| a == b)
                    && self
                        .version
                        .iter()
                        .zip(other.version.iter())
                        .all(|(a, b)| a == b)
                    && self
                        .machine
                        .iter()
                        .zip(other.machine.iter())
                        .all(|(a, b)| a == b)
                    && self
                        .domainname
                        .iter()
                        .zip(other.domainname.iter())
                        .all(|(a, b)| a == b)
            }
        }
        impl Eq for utsname {}
        impl fmt::Debug for utsname {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("utsname")
                    // FIXME(debug): .field("sysname", &self.sysname)
                    // FIXME(debug): .field("nodename", &self.nodename)
                    // FIXME(debug): .field("release", &self.release)
                    // FIXME(debug): .field("version", &self.version)
                    // FIXME(debug): .field("machine", &self.machine)
                    // FIXME(debug): .field("domainname", &self.domainname)
                    .finish()
            }
        }
        impl hash::Hash for utsname {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.sysname.hash(state);
                self.nodename.hash(state);
                self.release.hash(state);
                self.version.hash(state);
                self.machine.hash(state);
                self.domainname.hash(state);
            }
        }

        impl PartialEq for dirent {
            fn eq(&self, other: &dirent) -> bool {
                self.d_ino == other.d_ino
                    && self.d_off == other.d_off
                    && self.d_reclen == other.d_reclen
                    && self.d_type == other.d_type
                    && self
                        .d_name
                        .iter()
                        .zip(other.d_name.iter())
                        .all(|(a, b)| a == b)
            }
        }
        impl Eq for dirent {}
        impl fmt::Debug for dirent {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("dirent")
                    .field("d_ino", &self.d_ino)
                    .field("d_off", &self.d_off)
                    .field("d_reclen", &self.d_reclen)
                    .field("d_type", &self.d_type)
                    // FIXME(debug): .field("d_name", &self.d_name)
                    .finish()
            }
        }
        impl hash::Hash for dirent {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.d_ino.hash(state);
                self.d_off.hash(state);
                self.d_reclen.hash(state);
                self.d_type.hash(state);
                self.d_name.hash(state);
            }
        }

        impl PartialEq for dirent64 {
            fn eq(&self, other: &dirent64) -> bool {
                self.d_ino == other.d_ino
                    && self.d_off == other.d_off
                    && self.d_reclen == other.d_reclen
                    && self.d_type == other.d_type
                    && self
                        .d_name
                        .iter()
                        .zip(other.d_name.iter())
                        .all(|(a, b)| a == b)
            }
        }
        impl Eq for dirent64 {}
        impl fmt::Debug for dirent64 {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("dirent64")
                    .field("d_ino", &self.d_ino)
                    .field("d_off", &self.d_off)
                    .field("d_reclen", &self.d_reclen)
                    .field("d_type", &self.d_type)
                    // FIXME(debug): .field("d_name", &self.d_name)
                    .finish()
            }
        }
        impl hash::Hash for dirent64 {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.d_ino.hash(state);
                self.d_off.hash(state);
                self.d_reclen.hash(state);
                self.d_type.hash(state);
                self.d_name.hash(state);
            }
        }

        impl fmt::Debug for sigevent {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("sigevent")
                    .field("sigev_value", &self.sigev_value)
                    .field("sigev_signo", &self.sigev_signo)
                    .field("sigev_notify", &self.sigev_notify)
                    // Skip _sigev_un, since we can't guarantee that it will be
                    // properly initialized.
                    .finish()
            }
        }

        impl PartialEq for pthread_cond_t {
            fn eq(&self, other: &pthread_cond_t) -> bool {
                self.size.iter().zip(other.size.iter()).all(|(a, b)| a == b)
            }
        }
        impl Eq for pthread_cond_t {}
        impl fmt::Debug for pthread_cond_t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("pthread_cond_t")
                    // FIXME(debug): .field("size", &self.size)
                    .finish()
            }
        }
        impl hash::Hash for pthread_cond_t {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.size.hash(state);
            }
        }

        impl PartialEq for pthread_mutex_t {
            fn eq(&self, other: &pthread_mutex_t) -> bool {
                self.size.iter().zip(other.size.iter()).all(|(a, b)| a == b)
            }
        }
        impl Eq for pthread_mutex_t {}
        impl fmt::Debug for pthread_mutex_t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("pthread_mutex_t")
                    // FIXME(debug): .field("size", &self.size)
                    .finish()
            }
        }
        impl hash::Hash for pthread_mutex_t {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.size.hash(state);
            }
        }

        impl PartialEq for pthread_rwlock_t {
            fn eq(&self, other: &pthread_rwlock_t) -> bool {
                self.size.iter().zip(other.size.iter()).all(|(a, b)| a == b)
            }
        }
        impl Eq for pthread_rwlock_t {}
        impl fmt::Debug for pthread_rwlock_t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("pthread_rwlock_t")
                    // FIXME(debug): .field("size", &self.size)
                    .finish()
            }
        }
        impl hash::Hash for pthread_rwlock_t {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.size.hash(state);
            }
        }

        impl PartialEq for pthread_barrier_t {
            fn eq(&self, other: &pthread_barrier_t) -> bool {
                self.size.iter().zip(other.size.iter()).all(|(a, b)| a == b)
            }
        }
        impl Eq for pthread_barrier_t {}
        impl fmt::Debug for pthread_barrier_t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct("pthread_barrier_t")
                    .field("size", &self.size)
                    .finish()
            }
        }
        impl hash::Hash for pthread_barrier_t {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                self.size.hash(state);
            }
        }
    }
}

// PUB_CONST

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

pub const DT_UNKNOWN: u8 = 0;
pub const DT_FIFO: u8 = 1;
pub const DT_CHR: u8 = 2;
pub const DT_DIR: u8 = 4;
pub const DT_BLK: u8 = 6;
pub const DT_REG: u8 = 8;
pub const DT_LNK: u8 = 10;
pub const DT_SOCK: u8 = 12;

pub const FD_CLOEXEC: c_int = 0x1;

pub const SIGIOT: c_int = 6;

pub const S_ISUID: crate::mode_t = 0o4000;
pub const S_ISGID: crate::mode_t = 0o2000;
pub const S_ISVTX: crate::mode_t = 0o1000;

pub const IF_NAMESIZE: size_t = 16;
pub const IFNAMSIZ: size_t = IF_NAMESIZE;

pub const LOG_EMERG: c_int = 0;
pub const LOG_ALERT: c_int = 1;
pub const LOG_CRIT: c_int = 2;
pub const LOG_ERR: c_int = 3;
pub const LOG_WARNING: c_int = 4;
pub const LOG_NOTICE: c_int = 5;
pub const LOG_INFO: c_int = 6;
pub const LOG_DEBUG: c_int = 7;

pub const LOG_KERN: c_int = 0;
pub const LOG_USER: c_int = 1 << 3;
pub const LOG_MAIL: c_int = 2 << 3;
pub const LOG_DAEMON: c_int = 3 << 3;
pub const LOG_AUTH: c_int = 4 << 3;
pub const LOG_SYSLOG: c_int = 5 << 3;
pub const LOG_LPR: c_int = 6 << 3;
pub const LOG_NEWS: c_int = 7 << 3;
pub const LOG_UUCP: c_int = 8 << 3;
pub const LOG_LOCAL0: c_int = 16 << 3;
pub const LOG_LOCAL1: c_int = 17 << 3;
pub const LOG_LOCAL2: c_int = 18 << 3;
pub const LOG_LOCAL3: c_int = 19 << 3;
pub const LOG_LOCAL4: c_int = 20 << 3;
pub const LOG_LOCAL5: c_int = 21 << 3;
pub const LOG_LOCAL6: c_int = 22 << 3;
pub const LOG_LOCAL7: c_int = 23 << 3;

pub const LOG_PID: c_int = 0x01;
pub const LOG_CONS: c_int = 0x02;
pub const LOG_ODELAY: c_int = 0x04;
pub const LOG_NDELAY: c_int = 0x08;
pub const LOG_NOWAIT: c_int = 0x10;

pub const LOG_PRIMASK: c_int = 7;
pub const LOG_FACMASK: c_int = 0x3f8;

pub const PRIO_MIN: c_int = -20;
pub const PRIO_MAX: c_int = 20;

pub const IPPROTO_ICMP: c_int = 1;
pub const IPPROTO_ICMPV6: c_int = 58;
pub const IPPROTO_TCP: c_int = 6;
pub const IPPROTO_UDP: c_int = 17;
pub const IPPROTO_IP: c_int = 0;
pub const IPPROTO_IPV6: c_int = 41;

pub const INADDR_LOOPBACK: in_addr_t = 2130706433;
pub const INADDR_ANY: in_addr_t = 0;
pub const INADDR_BROADCAST: in_addr_t = 4294967295;
pub const INADDR_NONE: in_addr_t = 4294967295;

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;
pub const RAND_MAX: c_int = 2147483647;
pub const EOF: c_int = -1;
pub const SEEK_SET: c_int = 0;
pub const SEEK_CUR: c_int = 1;
pub const SEEK_END: c_int = 2;
pub const _IOFBF: c_int = 0;
pub const _IONBF: c_int = 2;
pub const _IOLBF: c_int = 1;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;

// Linux-specific fcntls
pub const F_SETLEASE: c_int = 1024;
pub const F_GETLEASE: c_int = 1025;
pub const F_NOTIFY: c_int = 1026;
pub const F_CANCELLK: c_int = 1029;
pub const F_DUPFD_CLOEXEC: c_int = 1030;
pub const F_SETPIPE_SZ: c_int = 1031;
pub const F_GETPIPE_SZ: c_int = 1032;
pub const F_ADD_SEALS: c_int = 1033;
pub const F_GET_SEALS: c_int = 1034;

pub const F_SEAL_SEAL: c_int = 0x0001;
pub const F_SEAL_SHRINK: c_int = 0x0002;
pub const F_SEAL_GROW: c_int = 0x0004;
pub const F_SEAL_WRITE: c_int = 0x0008;

// FIXME(#235): Include file sealing fcntls once we have a way to verify them.

pub const SIGTRAP: c_int = 5;

pub const PTHREAD_CREATE_JOINABLE: c_int = 0;
pub const PTHREAD_CREATE_DETACHED: c_int = 1;

pub const CLOCK_REALTIME: crate::clockid_t = 0;
pub const CLOCK_MONOTONIC: crate::clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: crate::clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: crate::clockid_t = 3;
pub const CLOCK_MONOTONIC_RAW: crate::clockid_t = 4;
pub const CLOCK_REALTIME_COARSE: crate::clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: crate::clockid_t = 6;
pub const CLOCK_BOOTTIME: crate::clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: crate::clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: crate::clockid_t = 9;
pub const CLOCK_TAI: crate::clockid_t = 11;
pub const TIMER_ABSTIME: c_int = 1;

cfg_if! {
    if #[cfg(target_env = "uclibc")] {
        pub const RLIMIT_CPU: crate::__rlimit_resource_t = 0;
        pub const RLIMIT_FSIZE: crate::__rlimit_resource_t = 1;
        pub const RLIMIT_DATA: crate::__rlimit_resource_t = 2;
        pub const RLIMIT_STACK: crate::__rlimit_resource_t = 3;
        pub const RLIMIT_CORE: crate::__rlimit_resource_t = 4;
        pub const RLIMIT_RSS: crate::__rlimit_resource_t = 5;
        pub const RLIMIT_NPROC: crate::__rlimit_resource_t = 6;
        pub const RLIMIT_NOFILE: crate::__rlimit_resource_t = 7;
        pub const RLIMIT_MEMLOCK: crate::__rlimit_resource_t = 8;
        pub const RLIMIT_AS: crate::__rlimit_resource_t = 9;
        pub const RLIMIT_LOCKS: crate::__rlimit_resource_t = 10;
        pub const RLIMIT_SIGPENDING: crate::__rlimit_resource_t = 11;
        pub const RLIMIT_MSGQUEUE: crate::__rlimit_resource_t = 12;
        pub const RLIMIT_NICE: crate::__rlimit_resource_t = 13;
        pub const RLIMIT_RTPRIO: crate::__rlimit_resource_t = 14;
        pub const RLIMIT_RTTIME: crate::__rlimit_resource_t = 15;
        pub const RLIM_NLIMITS: crate::__rlimit_resource_t = 16;
        pub const RLIMIT_NLIMITS: crate::__rlimit_resource_t = RLIM_NLIMITS;
    } else if #[cfg(target_env = "musl")] {
        pub const RLIMIT_CPU: c_int = 0;
        pub const RLIMIT_FSIZE: c_int = 1;
        pub const RLIMIT_DATA: c_int = 2;
        pub const RLIMIT_STACK: c_int = 3;
        pub const RLIMIT_CORE: c_int = 4;
        pub const RLIMIT_RSS: c_int = 5;
        pub const RLIMIT_NPROC: c_int = 6;
        pub const RLIMIT_NOFILE: c_int = 7;
        pub const RLIMIT_MEMLOCK: c_int = 8;
        pub const RLIMIT_AS: c_int = 9;
        pub const RLIMIT_LOCKS: c_int = 10;
        pub const RLIMIT_SIGPENDING: c_int = 11;
        pub const RLIMIT_MSGQUEUE: c_int = 12;
        pub const RLIMIT_NICE: c_int = 13;
        pub const RLIMIT_RTPRIO: c_int = 14;
        pub const RLIMIT_RTTIME: c_int = 15;
        pub const RLIM_NLIMITS: c_int = 15;
        pub const RLIMIT_NLIMITS: c_int = RLIM_NLIMITS;
    }
}

pub const RLIM_INFINITY: crate::rlim_t = !0;
pub const RLIM64_INFINITY: crate::rlim64_t = !0;

pub const O_ACCMODE: c_int = 0o003;
pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_CREAT: c_int = 0o100;
pub const O_EXCL: c_int = 0o200;
pub const O_NOCTTY: c_int = 0x100;
pub const O_TRUNC: c_int = 0o1000;
pub const O_APPEND: c_int = 0o2000;
pub const O_NONBLOCK: c_int = 0o4000;
pub const O_NDELAY: c_int = O_NONBLOCK;
pub const O_SYNC: c_int = 0o10000;
pub const O_FSYNC: c_int = O_SYNC;
pub const O_DSYNC: c_int = O_SYNC;
pub const O_RSYNC: c_int = O_SYNC;
pub const O_ASYNC: c_int = 0o20000;
pub const O_CLOEXEC: c_int = 0x80000;
pub const O_NOATIME: c_int = 0o1000000;
pub const O_PATH: c_int = 0o10000000;

pub const S_IFIFO: crate::mode_t = 0o1_0000;
pub const S_IFCHR: crate::mode_t = 0o2_0000;
pub const S_IFBLK: crate::mode_t = 0o6_0000;
pub const S_IFDIR: crate::mode_t = 0o4_0000;
pub const S_IFREG: crate::mode_t = 0o10_0000;
pub const S_IFLNK: crate::mode_t = 0o12_0000;
pub const S_IFSOCK: crate::mode_t = 0o14_0000;
pub const S_IFMT: crate::mode_t = 0o17_0000;
pub const S_IRWXU: crate::mode_t = 0o0700;
pub const S_IXUSR: crate::mode_t = 0o0100;
pub const S_IWUSR: crate::mode_t = 0o0200;
pub const S_IRUSR: crate::mode_t = 0o0400;
pub const S_IRWXG: crate::mode_t = 0o0070;
pub const S_IXGRP: crate::mode_t = 0o0010;
pub const S_IWGRP: crate::mode_t = 0o0020;
pub const S_IRGRP: crate::mode_t = 0o0040;
pub const S_IRWXO: crate::mode_t = 0o0007;
pub const S_IXOTH: crate::mode_t = 0o0001;
pub const S_IWOTH: crate::mode_t = 0o0002;
pub const S_IROTH: crate::mode_t = 0o0004;
pub const F_OK: c_int = 0;
pub const R_OK: c_int = 4;
pub const W_OK: c_int = 2;
pub const X_OK: c_int = 1;
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int = 1;
pub const STDERR_FILENO: c_int = 2;
pub const SIGHUP: c_int = 1;
pub const SIGINT: c_int = 2;
pub const SIGQUIT: c_int = 3;
pub const SIGILL: c_int = 4;
pub const SIGABRT: c_int = 6;
pub const SIGFPE: c_int = 8;
pub const SIGKILL: c_int = 9;
pub const SIGSEGV: c_int = 11;
pub const SIGPIPE: c_int = 13;
pub const SIGALRM: c_int = 14;
pub const SIGTERM: c_int = 15;

pub const PROT_NONE: c_int = 0;
pub const PROT_READ: c_int = 1;
pub const PROT_WRITE: c_int = 2;
pub const PROT_EXEC: c_int = 4;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const LC_CTYPE: c_int = 0;
        pub const LC_NUMERIC: c_int = 1;
        pub const LC_TIME: c_int = 2;
        pub const LC_COLLATE: c_int = 3;
        pub const LC_MONETARY: c_int = 4;
        pub const LC_MESSAGES: c_int = 5;
        pub const LC_ALL: c_int = 6;
    }
}
pub const LC_CTYPE_MASK: c_int = 1 << LC_CTYPE;
pub const LC_NUMERIC_MASK: c_int = 1 << LC_NUMERIC;
pub const LC_TIME_MASK: c_int = 1 << LC_TIME;
pub const LC_COLLATE_MASK: c_int = 1 << LC_COLLATE;
pub const LC_MONETARY_MASK: c_int = 1 << LC_MONETARY;
pub const LC_MESSAGES_MASK: c_int = 1 << LC_MESSAGES;
// LC_ALL_MASK defined per platform

pub const MAP_FILE: c_int = 0x0000;
pub const MAP_SHARED: c_int = 0x0001;
pub const MAP_PRIVATE: c_int = 0x0002;
pub const MAP_FIXED: c_int = 0x0010;

pub const MAP_FAILED: *mut c_void = !0 as *mut c_void;

// MS_ flags for msync(2)
pub const MS_ASYNC: c_int = 0x0001;
pub const MS_INVALIDATE: c_int = 0x0002;
pub const MS_SYNC: c_int = 0x0004;

// MS_ flags for mount(2)
pub const MS_RDONLY: c_ulong = 0x01;
pub const MS_NOSUID: c_ulong = 0x02;
pub const MS_NODEV: c_ulong = 0x04;
pub const MS_NOEXEC: c_ulong = 0x08;
pub const MS_SYNCHRONOUS: c_ulong = 0x10;
pub const MS_REMOUNT: c_ulong = 0x20;
pub const MS_MANDLOCK: c_ulong = 0x40;
pub const MS_DIRSYNC: c_ulong = 0x80;
pub const MS_NOATIME: c_ulong = 0x0400;
pub const MS_NODIRATIME: c_ulong = 0x0800;
pub const MS_BIND: c_ulong = 0x1000;
pub const MS_MOVE: c_ulong = 0x2000;
pub const MS_REC: c_ulong = 0x4000;
pub const MS_SILENT: c_ulong = 0x8000;
pub const MS_POSIXACL: c_ulong = 0x010000;
pub const MS_UNBINDABLE: c_ulong = 0x020000;
pub const MS_PRIVATE: c_ulong = 0x040000;
pub const MS_SLAVE: c_ulong = 0x080000;
pub const MS_SHARED: c_ulong = 0x100000;
pub const MS_RELATIME: c_ulong = 0x200000;
pub const MS_KERNMOUNT: c_ulong = 0x400000;
pub const MS_I_VERSION: c_ulong = 0x800000;
pub const MS_STRICTATIME: c_ulong = 0x1000000;
pub const MS_LAZYTIME: c_ulong = 0x2000000;
pub const MS_ACTIVE: c_ulong = 0x40000000;
pub const MS_NOUSER: c_ulong = 0x80000000;
pub const MS_MGC_VAL: c_ulong = 0xc0ed0000;
pub const MS_MGC_MSK: c_ulong = 0xffff0000;
pub const MS_RMT_MASK: c_ulong = 0x800051;

pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const ENOTBLK: c_int = 15;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOTTY: c_int = 25;
pub const ETXTBSY: c_int = 26;
pub const EFBIG: c_int = 27;
pub const ENOSPC: c_int = 28;
pub const ESPIPE: c_int = 29;
pub const EROFS: c_int = 30;
pub const EMLINK: c_int = 31;
pub const EPIPE: c_int = 32;
pub const EDOM: c_int = 33;
pub const ERANGE: c_int = 34;
pub const EWOULDBLOCK: c_int = EAGAIN;

pub const SCM_RIGHTS: c_int = 0x01;
pub const SCM_CREDENTIALS: c_int = 0x02;

pub const PROT_GROWSDOWN: c_int = 0x1000000;
pub const PROT_GROWSUP: c_int = 0x2000000;

pub const MAP_TYPE: c_int = 0x000f;

pub const MADV_NORMAL: c_int = 0;
pub const MADV_RANDOM: c_int = 1;
pub const MADV_SEQUENTIAL: c_int = 2;
pub const MADV_WILLNEED: c_int = 3;
pub const MADV_DONTNEED: c_int = 4;
pub const MADV_FREE: c_int = 8;
pub const MADV_REMOVE: c_int = 9;
pub const MADV_DONTFORK: c_int = 10;
pub const MADV_DOFORK: c_int = 11;
pub const MADV_MERGEABLE: c_int = 12;
pub const MADV_UNMERGEABLE: c_int = 13;
pub const MADV_HUGEPAGE: c_int = 14;
pub const MADV_NOHUGEPAGE: c_int = 15;
pub const MADV_DONTDUMP: c_int = 16;
pub const MADV_DODUMP: c_int = 17;
pub const MADV_WIPEONFORK: c_int = 18;
pub const MADV_KEEPONFORK: c_int = 19;
pub const MADV_HWPOISON: c_int = 100;
cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const MADV_COLD: c_int = 20;
        pub const MADV_PAGEOUT: c_int = 21;
        pub const MADV_POPULATE_READ: c_int = 22;
        pub const MADV_POPULATE_WRITE: c_int = 23;
        pub const MADV_DONTNEED_LOCKED: c_int = 24;
    }
}

pub const IFF_UP: c_int = 0x1;
pub const IFF_BROADCAST: c_int = 0x2;
pub const IFF_DEBUG: c_int = 0x4;
pub const IFF_LOOPBACK: c_int = 0x8;
pub const IFF_POINTOPOINT: c_int = 0x10;
pub const IFF_NOTRAILERS: c_int = 0x20;
pub const IFF_RUNNING: c_int = 0x40;
pub const IFF_NOARP: c_int = 0x80;
pub const IFF_PROMISC: c_int = 0x100;
pub const IFF_ALLMULTI: c_int = 0x200;
pub const IFF_MASTER: c_int = 0x400;
pub const IFF_SLAVE: c_int = 0x800;
pub const IFF_MULTICAST: c_int = 0x1000;
pub const IFF_PORTSEL: c_int = 0x2000;
pub const IFF_AUTOMEDIA: c_int = 0x4000;
pub const IFF_DYNAMIC: c_int = 0x8000;

pub const SOL_IP: c_int = 0;
pub const SOL_TCP: c_int = 6;
pub const SOL_UDP: c_int = 17;
pub const SOL_IPV6: c_int = 41;
pub const SOL_ICMPV6: c_int = 58;
pub const SOL_RAW: c_int = 255;
pub const SOL_DECNET: c_int = 261;
pub const SOL_X25: c_int = 262;
pub const SOL_PACKET: c_int = 263;
pub const SOL_ATM: c_int = 264;
pub const SOL_AAL: c_int = 265;
pub const SOL_IRDA: c_int = 266;
pub const SOL_NETBEUI: c_int = 267;
pub const SOL_LLC: c_int = 268;
pub const SOL_DCCP: c_int = 269;
pub const SOL_NETLINK: c_int = 270;
pub const SOL_TIPC: c_int = 271;
pub const SOL_BLUETOOTH: c_int = 274;
pub const SOL_ALG: c_int = 279;

pub const AF_UNSPEC: c_int = 0;
pub const AF_UNIX: c_int = 1;
pub const AF_LOCAL: c_int = 1;
pub const AF_INET: c_int = 2;
pub const AF_AX25: c_int = 3;
pub const AF_IPX: c_int = 4;
pub const AF_APPLETALK: c_int = 5;
pub const AF_NETROM: c_int = 6;
pub const AF_BRIDGE: c_int = 7;
pub const AF_ATMPVC: c_int = 8;
pub const AF_X25: c_int = 9;
pub const AF_INET6: c_int = 10;
pub const AF_ROSE: c_int = 11;
pub const AF_DECnet: c_int = 12;
pub const AF_NETBEUI: c_int = 13;
pub const AF_SECURITY: c_int = 14;
pub const AF_KEY: c_int = 15;
pub const AF_NETLINK: c_int = 16;
pub const AF_ROUTE: c_int = AF_NETLINK;
pub const AF_PACKET: c_int = 17;
pub const AF_ASH: c_int = 18;
pub const AF_ECONET: c_int = 19;
pub const AF_ATMSVC: c_int = 20;
pub const AF_RDS: c_int = 21;
pub const AF_SNA: c_int = 22;
pub const AF_IRDA: c_int = 23;
pub const AF_PPPOX: c_int = 24;
pub const AF_WANPIPE: c_int = 25;
pub const AF_LLC: c_int = 26;
pub const AF_CAN: c_int = 29;
pub const AF_TIPC: c_int = 30;
pub const AF_BLUETOOTH: c_int = 31;
pub const AF_IUCV: c_int = 32;
pub const AF_RXRPC: c_int = 33;
pub const AF_ISDN: c_int = 34;
pub const AF_PHONET: c_int = 35;
pub const AF_IEEE802154: c_int = 36;
pub const AF_CAIF: c_int = 37;
pub const AF_ALG: c_int = 38;

pub const PF_UNSPEC: c_int = AF_UNSPEC;
pub const PF_UNIX: c_int = AF_UNIX;
pub const PF_LOCAL: c_int = AF_LOCAL;
pub const PF_INET: c_int = AF_INET;
pub const PF_AX25: c_int = AF_AX25;
pub const PF_IPX: c_int = AF_IPX;
pub const PF_APPLETALK: c_int = AF_APPLETALK;
pub const PF_NETROM: c_int = AF_NETROM;
pub const PF_BRIDGE: c_int = AF_BRIDGE;
pub const PF_ATMPVC: c_int = AF_ATMPVC;
pub const PF_X25: c_int = AF_X25;
pub const PF_INET6: c_int = AF_INET6;
pub const PF_ROSE: c_int = AF_ROSE;
pub const PF_DECnet: c_int = AF_DECnet;
pub const PF_NETBEUI: c_int = AF_NETBEUI;
pub const PF_SECURITY: c_int = AF_SECURITY;
pub const PF_KEY: c_int = AF_KEY;
pub const PF_NETLINK: c_int = AF_NETLINK;
pub const PF_ROUTE: c_int = AF_ROUTE;
pub const PF_PACKET: c_int = AF_PACKET;
pub const PF_ASH: c_int = AF_ASH;
pub const PF_ECONET: c_int = AF_ECONET;
pub const PF_ATMSVC: c_int = AF_ATMSVC;
pub const PF_RDS: c_int = AF_RDS;
pub const PF_SNA: c_int = AF_SNA;
pub const PF_IRDA: c_int = AF_IRDA;
pub const PF_PPPOX: c_int = AF_PPPOX;
pub const PF_WANPIPE: c_int = AF_WANPIPE;
pub const PF_LLC: c_int = AF_LLC;
pub const PF_CAN: c_int = AF_CAN;
pub const PF_TIPC: c_int = AF_TIPC;
pub const PF_BLUETOOTH: c_int = AF_BLUETOOTH;
pub const PF_IUCV: c_int = AF_IUCV;
pub const PF_RXRPC: c_int = AF_RXRPC;
pub const PF_ISDN: c_int = AF_ISDN;
pub const PF_PHONET: c_int = AF_PHONET;
pub const PF_IEEE802154: c_int = AF_IEEE802154;
pub const PF_CAIF: c_int = AF_CAIF;
pub const PF_ALG: c_int = AF_ALG;

pub const SOMAXCONN: c_int = 128;

pub const MSG_OOB: c_int = 1;
pub const MSG_PEEK: c_int = 2;
pub const MSG_DONTROUTE: c_int = 4;
pub const MSG_CTRUNC: c_int = 8;
pub const MSG_TRUNC: c_int = 0x20;
pub const MSG_DONTWAIT: c_int = 0x40;
pub const MSG_EOR: c_int = 0x80;
pub const MSG_WAITALL: c_int = 0x100;
pub const MSG_FIN: c_int = 0x200;
pub const MSG_SYN: c_int = 0x400;
pub const MSG_CONFIRM: c_int = 0x800;
pub const MSG_RST: c_int = 0x1000;
pub const MSG_ERRQUEUE: c_int = 0x2000;
pub const MSG_NOSIGNAL: c_int = 0x4000;
pub const MSG_MORE: c_int = 0x8000;
pub const MSG_WAITFORONE: c_int = 0x10000;
pub const MSG_FASTOPEN: c_int = 0x20000000;
pub const MSG_CMSG_CLOEXEC: c_int = 0x40000000;

pub const SCM_TIMESTAMP: c_int = SO_TIMESTAMP;

pub const SOCK_RAW: c_int = 3;
pub const SOCK_RDM: c_int = 4;

pub const IP_TOS: c_int = 1;
pub const IP_TTL: c_int = 2;
pub const IP_HDRINCL: c_int = 3;
pub const IP_OPTIONS: c_int = 4;
pub const IP_ROUTER_ALERT: c_int = 5;
pub const IP_RECVOPTS: c_int = 6;
pub const IP_RETOPTS: c_int = 7;
pub const IP_PKTINFO: c_int = 8;
pub const IP_PKTOPTIONS: c_int = 9;
pub const IP_MTU_DISCOVER: c_int = 10;
pub const IP_RECVERR: c_int = 11;
pub const IP_RECVTTL: c_int = 12;
pub const IP_RECVTOS: c_int = 13;
pub const IP_MTU: c_int = 14;
pub const IP_FREEBIND: c_int = 15;
pub const IP_IPSEC_POLICY: c_int = 16;
pub const IP_XFRM_POLICY: c_int = 17;
pub const IP_PASSSEC: c_int = 18;
pub const IP_TRANSPARENT: c_int = 19;
pub const IP_ORIGDSTADDR: c_int = 20;
pub const IP_RECVORIGDSTADDR: c_int = IP_ORIGDSTADDR;
pub const IP_MINTTL: c_int = 21;
cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const IP_NODEFRAG: c_int = 22;
        pub const IP_CHECKSUM: c_int = 23;
        pub const IP_BIND_ADDRESS_NO_PORT: c_int = 24;
    }
}
pub const IP_MULTICAST_IF: c_int = 32;
pub const IP_MULTICAST_TTL: c_int = 33;
pub const IP_MULTICAST_LOOP: c_int = 34;
pub const IP_ADD_MEMBERSHIP: c_int = 35;
pub const IP_DROP_MEMBERSHIP: c_int = 36;
pub const IP_UNBLOCK_SOURCE: c_int = 37;
pub const IP_BLOCK_SOURCE: c_int = 38;
pub const IP_ADD_SOURCE_MEMBERSHIP: c_int = 39;
pub const IP_DROP_SOURCE_MEMBERSHIP: c_int = 40;
pub const IP_MSFILTER: c_int = 41;
pub const IP_MULTICAST_ALL: c_int = 49;
pub const IP_UNICAST_IF: c_int = 50;

pub const IP_DEFAULT_MULTICAST_TTL: c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: c_int = 1;

pub const IP_PMTUDISC_DONT: c_int = 0;
pub const IP_PMTUDISC_WANT: c_int = 1;
pub const IP_PMTUDISC_DO: c_int = 2;
pub const IP_PMTUDISC_PROBE: c_int = 3;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const IP_PMTUDISC_INTERFACE: c_int = 4;
        pub const IP_PMTUDISC_OMIT: c_int = 5;
    }
}

pub const MCAST_EXCLUDE: c_int = 0;
pub const MCAST_INCLUDE: c_int = 1;
pub const MCAST_JOIN_GROUP: c_int = 42;
pub const MCAST_BLOCK_SOURCE: c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: c_int = 44;
pub const MCAST_LEAVE_GROUP: c_int = 45;
pub const MCAST_JOIN_SOURCE_GROUP: c_int = 46;
pub const MCAST_LEAVE_SOURCE_GROUP: c_int = 47;
pub const MCAST_MSFILTER: c_int = 48;

pub const IPV6_ADDRFORM: c_int = 1;
pub const IPV6_2292PKTINFO: c_int = 2;
pub const IPV6_2292HOPOPTS: c_int = 3;
pub const IPV6_2292DSTOPTS: c_int = 4;
pub const IPV6_2292RTHDR: c_int = 5;
pub const IPV6_2292PKTOPTIONS: c_int = 6;
pub const IPV6_CHECKSUM: c_int = 7;
pub const IPV6_2292HOPLIMIT: c_int = 8;
pub const IPV6_NEXTHOP: c_int = 9;
pub const IPV6_AUTHHDR: c_int = 10;
pub const IPV6_UNICAST_HOPS: c_int = 16;
pub const IPV6_MULTICAST_IF: c_int = 17;
pub const IPV6_MULTICAST_HOPS: c_int = 18;
pub const IPV6_MULTICAST_LOOP: c_int = 19;
pub const IPV6_ADD_MEMBERSHIP: c_int = 20;
pub const IPV6_DROP_MEMBERSHIP: c_int = 21;
pub const IPV6_ROUTER_ALERT: c_int = 22;
pub const IPV6_MTU_DISCOVER: c_int = 23;
pub const IPV6_MTU: c_int = 24;
pub const IPV6_RECVERR: c_int = 25;
pub const IPV6_V6ONLY: c_int = 26;
pub const IPV6_JOIN_ANYCAST: c_int = 27;
pub const IPV6_LEAVE_ANYCAST: c_int = 28;
#[cfg(not(target_env = "uclibc"))]
pub const IPV6_MULTICAST_ALL: c_int = 29;
#[cfg(not(target_env = "uclibc"))]
pub const IPV6_ROUTER_ALERT_ISOLATE: c_int = 30;
pub const IPV6_IPSEC_POLICY: c_int = 34;
pub const IPV6_XFRM_POLICY: c_int = 35;
pub const IPV6_RECVPKTINFO: c_int = 49;
pub const IPV6_PKTINFO: c_int = 50;
pub const IPV6_RECVHOPLIMIT: c_int = 51;
pub const IPV6_HOPLIMIT: c_int = 52;
pub const IPV6_RECVHOPOPTS: c_int = 53;
pub const IPV6_HOPOPTS: c_int = 54;
pub const IPV6_RTHDRDSTOPTS: c_int = 55;
pub const IPV6_RECVRTHDR: c_int = 56;
pub const IPV6_RTHDR: c_int = 57;
pub const IPV6_RECVDSTOPTS: c_int = 58;
pub const IPV6_DSTOPTS: c_int = 59;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const IPV6_RECVPATHMTU: c_int = 60;
        pub const IPV6_PATHMTU: c_int = 61;
        pub const IPV6_DONTFRAG: c_int = 62;
    }
}

pub const IPV6_RECVTCLASS: c_int = 66;
pub const IPV6_TCLASS: c_int = 67;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const IPV6_AUTOFLOWLABEL: c_int = 70;
        pub const IPV6_ADDR_PREFERENCES: c_int = 72;
        pub const IPV6_MINHOPCOUNT: c_int = 73;
        pub const IPV6_ORIGDSTADDR: c_int = 74;
        pub const IPV6_RECVORIGDSTADDR: c_int = IPV6_ORIGDSTADDR;
        pub const IPV6_TRANSPARENT: c_int = 75;
        pub const IPV6_UNICAST_IF: c_int = 76;
        pub const IPV6_PREFER_SRC_TMP: c_int = 0x0001;
        pub const IPV6_PREFER_SRC_PUBLIC: c_int = 0x0002;
        pub const IPV6_PREFER_SRC_PUBTMP_DEFAULT: c_int = 0x0100;
        pub const IPV6_PREFER_SRC_COA: c_int = 0x0004;
        pub const IPV6_PREFER_SRC_HOME: c_int = 0x0400;
        pub const IPV6_PREFER_SRC_CGA: c_int = 0x0008;
        pub const IPV6_PREFER_SRC_NONCGA: c_int = 0x0800;
    }
}

pub const IPV6_PMTUDISC_DONT: c_int = 0;
pub const IPV6_PMTUDISC_WANT: c_int = 1;
pub const IPV6_PMTUDISC_DO: c_int = 2;
pub const IPV6_PMTUDISC_PROBE: c_int = 3;

pub const TCP_NODELAY: c_int = 1;
pub const TCP_MAXSEG: c_int = 2;
pub const TCP_CORK: c_int = 3;
pub const TCP_KEEPIDLE: c_int = 4;
pub const TCP_KEEPINTVL: c_int = 5;
pub const TCP_KEEPCNT: c_int = 6;
pub const TCP_SYNCNT: c_int = 7;
pub const TCP_LINGER2: c_int = 8;
pub const TCP_DEFER_ACCEPT: c_int = 9;
pub const TCP_WINDOW_CLAMP: c_int = 10;
pub const TCP_INFO: c_int = 11;
pub const TCP_QUICKACK: c_int = 12;
pub const TCP_CONGESTION: c_int = 13;
pub const TCP_MD5SIG: c_int = 14;
pub const TCP_COOKIE_TRANSACTIONS: c_int = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: c_int = 16;
pub const TCP_THIN_DUPACK: c_int = 17;
pub const TCP_USER_TIMEOUT: c_int = 18;
pub const TCP_REPAIR: c_int = 19;
pub const TCP_REPAIR_QUEUE: c_int = 20;
pub const TCP_QUEUE_SEQ: c_int = 21;
pub const TCP_REPAIR_OPTIONS: c_int = 22;
pub const TCP_FASTOPEN: c_int = 23;
pub const TCP_TIMESTAMP: c_int = 24;

pub const SO_DEBUG: c_int = 1;

pub const SHUT_RD: c_int = 0;
pub const SHUT_WR: c_int = 1;
pub const SHUT_RDWR: c_int = 2;

pub const LOCK_SH: c_int = 1;
pub const LOCK_EX: c_int = 2;
pub const LOCK_NB: c_int = 4;
pub const LOCK_UN: c_int = 8;

pub const SS_ONSTACK: c_int = 1;
pub const SS_DISABLE: c_int = 2;

pub const PATH_MAX: c_int = 4096;

pub const FD_SETSIZE: c_int = 1024;

pub const MNT_FORCE: c_int = 0x1;
pub const MNT_DETACH: c_int = 0x2;
pub const MNT_EXPIRE: c_int = 0x4;
pub const UMOUNT_NOFOLLOW: c_int = 0x8;

pub const TCIOFF: c_int = 2;
pub const TCION: c_int = 3;
pub const TCOOFF: c_int = 0;
pub const TCOON: c_int = 1;
pub const TCIFLUSH: c_int = 0;
pub const TCOFLUSH: c_int = 1;
pub const TCIOFLUSH: c_int = 2;
pub const NL0: crate::tcflag_t = 0x00000000;
pub const NL1: crate::tcflag_t = 0x00000100;
pub const TAB0: crate::tcflag_t = 0x00000000;
pub const CR0: crate::tcflag_t = 0x00000000;
pub const FF0: crate::tcflag_t = 0x00000000;
pub const BS0: crate::tcflag_t = 0x00000000;
pub const VT0: crate::tcflag_t = 0x00000000;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VLNEXT: usize = 15;
pub const IGNBRK: crate::tcflag_t = 0x00000001;
pub const BRKINT: crate::tcflag_t = 0x00000002;
pub const IGNPAR: crate::tcflag_t = 0x00000004;
pub const PARMRK: crate::tcflag_t = 0x00000008;
pub const INPCK: crate::tcflag_t = 0x00000010;
pub const ISTRIP: crate::tcflag_t = 0x00000020;
pub const INLCR: crate::tcflag_t = 0x00000040;
pub const IGNCR: crate::tcflag_t = 0x00000080;
pub const ICRNL: crate::tcflag_t = 0x00000100;
pub const IXANY: crate::tcflag_t = 0x00000800;
pub const IMAXBEL: crate::tcflag_t = 0x00002000;
pub const OPOST: crate::tcflag_t = 0x1;
pub const CS5: crate::tcflag_t = 0x00000000;
pub const CRTSCTS: crate::tcflag_t = 0x80000000;
pub const ECHO: crate::tcflag_t = 0x00000008;
pub const OCRNL: crate::tcflag_t = 0o000010;
pub const ONOCR: crate::tcflag_t = 0o000020;
pub const ONLRET: crate::tcflag_t = 0o000040;
pub const OFILL: crate::tcflag_t = 0o000100;
pub const OFDEL: crate::tcflag_t = 0o000200;

pub const CLONE_VM: c_int = 0x100;
pub const CLONE_FS: c_int = 0x200;
pub const CLONE_FILES: c_int = 0x400;
pub const CLONE_SIGHAND: c_int = 0x800;
pub const CLONE_PTRACE: c_int = 0x2000;
pub const CLONE_VFORK: c_int = 0x4000;
pub const CLONE_PARENT: c_int = 0x8000;
pub const CLONE_THREAD: c_int = 0x10000;
pub const CLONE_NEWNS: c_int = 0x20000;
pub const CLONE_SYSVSEM: c_int = 0x40000;
pub const CLONE_SETTLS: c_int = 0x80000;
pub const CLONE_PARENT_SETTID: c_int = 0x100000;
pub const CLONE_CHILD_CLEARTID: c_int = 0x200000;
pub const CLONE_DETACHED: c_int = 0x400000;
pub const CLONE_UNTRACED: c_int = 0x800000;
pub const CLONE_CHILD_SETTID: c_int = 0x01000000;
pub const CLONE_NEWUTS: c_int = 0x04000000;
pub const CLONE_NEWIPC: c_int = 0x08000000;
pub const CLONE_NEWUSER: c_int = 0x10000000;
pub const CLONE_NEWPID: c_int = 0x20000000;
pub const CLONE_NEWNET: c_int = 0x40000000;
pub const CLONE_IO: c_int = 0x80000000;

pub const WNOHANG: c_int = 0x00000001;
pub const WUNTRACED: c_int = 0x00000002;
pub const WSTOPPED: c_int = WUNTRACED;
pub const WEXITED: c_int = 0x00000004;
pub const WCONTINUED: c_int = 0x00000008;
pub const WNOWAIT: c_int = 0x01000000;

pub const __WNOTHREAD: c_int = 0x20000000;
pub const __WALL: c_int = 0x40000000;
pub const __WCLONE: c_int = 0x80000000;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const SPLICE_F_MOVE: c_uint = 0x01;
        pub const SPLICE_F_NONBLOCK: c_uint = 0x02;
        pub const SPLICE_F_MORE: c_uint = 0x04;
        pub const SPLICE_F_GIFT: c_uint = 0x08;
    }
}

pub const RTLD_LOCAL: c_int = 0;
pub const RTLD_LAZY: c_int = 1;

pub const POSIX_FADV_NORMAL: c_int = 0;
pub const POSIX_FADV_RANDOM: c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: c_int = 2;
pub const POSIX_FADV_WILLNEED: c_int = 3;

pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_EACCESS: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_NO_AUTOMOUNT: c_int = 0x800;
pub const AT_EMPTY_PATH: c_int = 0x1000;
pub const AT_RECURSIVE: c_int = 0x8000;

pub const LOG_CRON: c_int = 9 << 3;
pub const LOG_AUTHPRIV: c_int = 10 << 3;
pub const LOG_FTP: c_int = 11 << 3;
pub const LOG_PERROR: c_int = 0x20;

pub const SI_LOAD_SHIFT: c_uint = 16;

pub const CLD_EXITED: c_int = 1;
pub const CLD_KILLED: c_int = 2;
pub const CLD_DUMPED: c_int = 3;
pub const CLD_TRAPPED: c_int = 4;
pub const CLD_STOPPED: c_int = 5;
pub const CLD_CONTINUED: c_int = 6;

pub const SIGEV_SIGNAL: c_int = 0;
pub const SIGEV_NONE: c_int = 1;
pub const SIGEV_THREAD: c_int = 2;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const UTIME_OMIT: c_long = 1073741822;
pub const UTIME_NOW: c_long = 1073741823;

pub const POLLIN: c_short = 0x1;
pub const POLLPRI: c_short = 0x2;
pub const POLLOUT: c_short = 0x4;
pub const POLLERR: c_short = 0x8;
pub const POLLHUP: c_short = 0x10;
pub const POLLNVAL: c_short = 0x20;
pub const POLLRDNORM: c_short = 0x040;
pub const POLLWRNORM: c_short = 0x100;
pub const POLLWRBAND: c_short = 0x200;
pub const POLLRDBAND: c_short = 0x080;
pub const POLLRDHUP: c_short = 0x2000;

#[cfg(not(target_env = "uclibc"))]
const base: crate::nl_item = 0x20000;
#[cfg(target_env = "uclibc")]
const base: crate::nl_item = 768;

pub const ABDAY_1: crate::nl_item = base;
pub const ABDAY_2: crate::nl_item = base + 0x1;
pub const ABDAY_3: crate::nl_item = base + 0x2;
pub const ABDAY_4: crate::nl_item = base + 0x3;
pub const ABDAY_5: crate::nl_item = base + 0x4;
pub const ABDAY_6: crate::nl_item = base + 0x5;
pub const ABDAY_7: crate::nl_item = base + 0x6;

pub const DAY_1: crate::nl_item = base + 0x7;
pub const DAY_2: crate::nl_item = base + 0x8;
pub const DAY_3: crate::nl_item = base + 0x9;
pub const DAY_4: crate::nl_item = base + 0xA;
pub const DAY_5: crate::nl_item = base + 0xB;
pub const DAY_6: crate::nl_item = base + 0xC;
pub const DAY_7: crate::nl_item = base + 0xD;

pub const ABMON_1: crate::nl_item = base + 0xE;
pub const ABMON_2: crate::nl_item = base + 0xF;
pub const ABMON_3: crate::nl_item = base + 0x10;
pub const ABMON_4: crate::nl_item = base + 0x11;
pub const ABMON_5: crate::nl_item = base + 0x12;
pub const ABMON_6: crate::nl_item = base + 0x13;
pub const ABMON_7: crate::nl_item = base + 0x14;
pub const ABMON_8: crate::nl_item = base + 0x15;
pub const ABMON_9: crate::nl_item = base + 0x16;
pub const ABMON_10: crate::nl_item = base + 0x17;
pub const ABMON_11: crate::nl_item = base + 0x18;
pub const ABMON_12: crate::nl_item = base + 0x19;

pub const MON_1: crate::nl_item = base + 0x1A;
pub const MON_2: crate::nl_item = base + 0x1B;
pub const MON_3: crate::nl_item = base + 0x1C;
pub const MON_4: crate::nl_item = base + 0x1D;
pub const MON_5: crate::nl_item = base + 0x1E;
pub const MON_6: crate::nl_item = base + 0x1F;
pub const MON_7: crate::nl_item = base + 0x20;
pub const MON_8: crate::nl_item = base + 0x21;
pub const MON_9: crate::nl_item = base + 0x22;
pub const MON_10: crate::nl_item = base + 0x23;
pub const MON_11: crate::nl_item = base + 0x24;
pub const MON_12: crate::nl_item = base + 0x25;

pub const AM_STR: crate::nl_item = base + 0x26;
pub const PM_STR: crate::nl_item = base + 0x27;

pub const D_T_FMT: crate::nl_item = base + 0x28;
pub const D_FMT: crate::nl_item = base + 0x29;
pub const T_FMT: crate::nl_item = base + 0x2A;
pub const T_FMT_AMPM: crate::nl_item = base + 0x2B;

pub const ERA: crate::nl_item = base + 0x2C;
pub const ERA_D_FMT: crate::nl_item = base + 0x2E;
pub const ALT_DIGITS: crate::nl_item = base + 0x2F;
pub const ERA_D_T_FMT: crate::nl_item = base + 0x30;
pub const ERA_T_FMT: crate::nl_item = base + 0x31;

pub const SI_USER: c_int = 0;
pub const SI_KERNEL: c_int = 0x80;
pub const SI_QUEUE: c_int = -1;
pub const SI_TIMER: c_int = -2;
pub const SI_MESGQ: c_int = -3;
pub const SI_ASYNCIO: c_int = -4;
pub const SI_SIGIO: c_int = -5;
pub const SI_TKILL: c_int = -6;
pub const SI_ASYNCNL: c_int = -60;

pub const RUSAGE_CHILDREN: c_int = -1;
pub const RUSAGE_SELF: c_int = 0;
#[cfg(not(target_env = "uclibc"))]
pub const RUSAGE_THREAD: c_int = 1;

pub const L_tmpnam: c_uint = 20;
pub const _PC_LINK_MAX: c_int = 0;
pub const _PC_MAX_CANON: c_int = 1;
pub const _PC_MAX_INPUT: c_int = 2;
pub const _PC_NAME_MAX: c_int = 3;
pub const _PC_PATH_MAX: c_int = 4;
pub const _PC_PIPE_BUF: c_int = 5;
pub const _PC_CHOWN_RESTRICTED: c_int = 6;
pub const _PC_NO_TRUNC: c_int = 7;
pub const _PC_VDISABLE: c_int = 8;
pub const _PC_SYNC_IO: c_int = 9;
pub const _PC_ASYNC_IO: c_int = 10;
pub const _PC_PRIO_IO: c_int = 11;
pub const _PC_SOCK_MAXBUF: c_int = 12;
pub const _PC_FILESIZEBITS: c_int = 13;
pub const _PC_REC_INCR_XFER_SIZE: c_int = 14;
pub const _PC_REC_MAX_XFER_SIZE: c_int = 15;
pub const _PC_REC_MIN_XFER_SIZE: c_int = 16;
pub const _PC_REC_XFER_ALIGN: c_int = 17;
pub const _PC_ALLOC_SIZE_MIN: c_int = 18;
pub const _PC_SYMLINK_MAX: c_int = 19;
pub const _PC_2_SYMLINKS: c_int = 20;

pub const _SC_ARG_MAX: c_int = 0;
pub const _SC_CHILD_MAX: c_int = 1;
pub const _SC_CLK_TCK: c_int = 2;
pub const _SC_NGROUPS_MAX: c_int = 3;
pub const _SC_OPEN_MAX: c_int = 4;
pub const _SC_STREAM_MAX: c_int = 5;
pub const _SC_TZNAME_MAX: c_int = 6;
pub const _SC_JOB_CONTROL: c_int = 7;
pub const _SC_SAVED_IDS: c_int = 8;
pub const _SC_REALTIME_SIGNALS: c_int = 9;
pub const _SC_PRIORITY_SCHEDULING: c_int = 10;
pub const _SC_TIMERS: c_int = 11;
pub const _SC_ASYNCHRONOUS_IO: c_int = 12;
pub const _SC_PRIORITIZED_IO: c_int = 13;
pub const _SC_SYNCHRONIZED_IO: c_int = 14;
pub const _SC_FSYNC: c_int = 15;
pub const _SC_MAPPED_FILES: c_int = 16;
pub const _SC_MEMLOCK: c_int = 17;
pub const _SC_MEMLOCK_RANGE: c_int = 18;
pub const _SC_MEMORY_PROTECTION: c_int = 19;
pub const _SC_MESSAGE_PASSING: c_int = 20;
pub const _SC_SEMAPHORES: c_int = 21;
pub const _SC_SHARED_MEMORY_OBJECTS: c_int = 22;
pub const _SC_AIO_LISTIO_MAX: c_int = 23;
pub const _SC_AIO_MAX: c_int = 24;
pub const _SC_AIO_PRIO_DELTA_MAX: c_int = 25;
pub const _SC_DELAYTIMER_MAX: c_int = 26;
pub const _SC_MQ_OPEN_MAX: c_int = 27;
pub const _SC_MQ_PRIO_MAX: c_int = 28;
pub const _SC_VERSION: c_int = 29;
pub const _SC_PAGESIZE: c_int = 30;
pub const _SC_PAGE_SIZE: c_int = _SC_PAGESIZE;
pub const _SC_RTSIG_MAX: c_int = 31;
pub const _SC_SEM_NSEMS_MAX: c_int = 32;
pub const _SC_SEM_VALUE_MAX: c_int = 33;
pub const _SC_SIGQUEUE_MAX: c_int = 34;
pub const _SC_TIMER_MAX: c_int = 35;
pub const _SC_BC_BASE_MAX: c_int = 36;
pub const _SC_BC_DIM_MAX: c_int = 37;
pub const _SC_BC_SCALE_MAX: c_int = 38;
pub const _SC_BC_STRING_MAX: c_int = 39;
pub const _SC_COLL_WEIGHTS_MAX: c_int = 40;
pub const _SC_EXPR_NEST_MAX: c_int = 42;
pub const _SC_LINE_MAX: c_int = 43;
pub const _SC_RE_DUP_MAX: c_int = 44;
pub const _SC_2_VERSION: c_int = 46;
pub const _SC_2_C_BIND: c_int = 47;
pub const _SC_2_C_DEV: c_int = 48;
pub const _SC_2_FORT_DEV: c_int = 49;
pub const _SC_2_FORT_RUN: c_int = 50;
pub const _SC_2_SW_DEV: c_int = 51;
pub const _SC_2_LOCALEDEF: c_int = 52;
pub const _SC_UIO_MAXIOV: c_int = 60;
pub const _SC_IOV_MAX: c_int = 60;
pub const _SC_THREADS: c_int = 67;
pub const _SC_THREAD_SAFE_FUNCTIONS: c_int = 68;
pub const _SC_GETGR_R_SIZE_MAX: c_int = 69;
pub const _SC_GETPW_R_SIZE_MAX: c_int = 70;
pub const _SC_LOGIN_NAME_MAX: c_int = 71;
pub const _SC_TTY_NAME_MAX: c_int = 72;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: c_int = 73;
pub const _SC_THREAD_KEYS_MAX: c_int = 74;
pub const _SC_THREAD_STACK_MIN: c_int = 75;
pub const _SC_THREAD_THREADS_MAX: c_int = 76;
pub const _SC_THREAD_ATTR_STACKADDR: c_int = 77;
pub const _SC_THREAD_ATTR_STACKSIZE: c_int = 78;
pub const _SC_THREAD_PRIORITY_SCHEDULING: c_int = 79;
pub const _SC_THREAD_PRIO_INHERIT: c_int = 80;
pub const _SC_THREAD_PRIO_PROTECT: c_int = 81;
pub const _SC_THREAD_PROCESS_SHARED: c_int = 82;
pub const _SC_NPROCESSORS_CONF: c_int = 83;
pub const _SC_NPROCESSORS_ONLN: c_int = 84;
pub const _SC_PHYS_PAGES: c_int = 85;
pub const _SC_AVPHYS_PAGES: c_int = 86;
pub const _SC_ATEXIT_MAX: c_int = 87;
pub const _SC_PASS_MAX: c_int = 88;
pub const _SC_XOPEN_VERSION: c_int = 89;
pub const _SC_XOPEN_XCU_VERSION: c_int = 90;
pub const _SC_XOPEN_UNIX: c_int = 91;
pub const _SC_XOPEN_CRYPT: c_int = 92;
pub const _SC_XOPEN_ENH_I18N: c_int = 93;
pub const _SC_XOPEN_SHM: c_int = 94;
pub const _SC_2_CHAR_TERM: c_int = 95;
pub const _SC_2_UPE: c_int = 97;
pub const _SC_XOPEN_XPG2: c_int = 98;
pub const _SC_XOPEN_XPG3: c_int = 99;
pub const _SC_XOPEN_XPG4: c_int = 100;
pub const _SC_NZERO: c_int = 109;
pub const _SC_XBS5_ILP32_OFF32: c_int = 125;
pub const _SC_XBS5_ILP32_OFFBIG: c_int = 126;
pub const _SC_XBS5_LP64_OFF64: c_int = 127;
pub const _SC_XBS5_LPBIG_OFFBIG: c_int = 128;
pub const _SC_XOPEN_LEGACY: c_int = 129;
pub const _SC_XOPEN_REALTIME: c_int = 130;
pub const _SC_XOPEN_REALTIME_THREADS: c_int = 131;
pub const _SC_ADVISORY_INFO: c_int = 132;
pub const _SC_BARRIERS: c_int = 133;
pub const _SC_CLOCK_SELECTION: c_int = 137;
pub const _SC_CPUTIME: c_int = 138;
pub const _SC_THREAD_CPUTIME: c_int = 139;
pub const _SC_MONOTONIC_CLOCK: c_int = 149;
pub const _SC_READER_WRITER_LOCKS: c_int = 153;
pub const _SC_SPIN_LOCKS: c_int = 154;
pub const _SC_REGEXP: c_int = 155;
pub const _SC_SHELL: c_int = 157;
pub const _SC_SPAWN: c_int = 159;
pub const _SC_SPORADIC_SERVER: c_int = 160;
pub const _SC_THREAD_SPORADIC_SERVER: c_int = 161;
pub const _SC_TIMEOUTS: c_int = 164;
pub const _SC_TYPED_MEMORY_OBJECTS: c_int = 165;
pub const _SC_2_PBS: c_int = 168;
pub const _SC_2_PBS_ACCOUNTING: c_int = 169;
pub const _SC_2_PBS_LOCATE: c_int = 170;
pub const _SC_2_PBS_MESSAGE: c_int = 171;
pub const _SC_2_PBS_TRACK: c_int = 172;
pub const _SC_SYMLOOP_MAX: c_int = 173;
pub const _SC_STREAMS: c_int = 174;
pub const _SC_2_PBS_CHECKPOINT: c_int = 175;
pub const _SC_V6_ILP32_OFF32: c_int = 176;
pub const _SC_V6_ILP32_OFFBIG: c_int = 177;
pub const _SC_V6_LP64_OFF64: c_int = 178;
pub const _SC_V6_LPBIG_OFFBIG: c_int = 179;
pub const _SC_HOST_NAME_MAX: c_int = 180;
pub const _SC_TRACE: c_int = 181;
pub const _SC_TRACE_EVENT_FILTER: c_int = 182;
pub const _SC_TRACE_INHERIT: c_int = 183;
pub const _SC_TRACE_LOG: c_int = 184;
pub const _SC_IPV6: c_int = 235;
pub const _SC_RAW_SOCKETS: c_int = 236;
pub const _SC_V7_ILP32_OFF32: c_int = 237;
pub const _SC_V7_ILP32_OFFBIG: c_int = 238;
pub const _SC_V7_LP64_OFF64: c_int = 239;
pub const _SC_V7_LPBIG_OFFBIG: c_int = 240;
pub const _SC_SS_REPL_MAX: c_int = 241;
pub const _SC_TRACE_EVENT_NAME_MAX: c_int = 242;
pub const _SC_TRACE_NAME_MAX: c_int = 243;
pub const _SC_TRACE_SYS_MAX: c_int = 244;
pub const _SC_TRACE_USER_EVENT_MAX: c_int = 245;
pub const _SC_XOPEN_STREAMS: c_int = 246;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: c_int = 247;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: c_int = 248;

pub const _CS_PATH: c_int = 0;
pub const _CS_POSIX_V6_WIDTH_RESTRICTED_ENVS: c_int = 1;
pub const _CS_POSIX_V5_WIDTH_RESTRICTED_ENVS: c_int = 4;
pub const _CS_POSIX_V7_WIDTH_RESTRICTED_ENVS: c_int = 5;
pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: c_int = 1116;
pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: c_int = 1117;
pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: c_int = 1118;
pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: c_int = 1119;
pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: c_int = 1120;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: c_int = 1121;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: c_int = 1122;
pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: c_int = 1123;
pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: c_int = 1124;
pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: c_int = 1125;
pub const _CS_POSIX_V6_LP64_OFF64_LIBS: c_int = 1126;
pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: c_int = 1127;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: c_int = 1128;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: c_int = 1129;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: c_int = 1130;
pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: c_int = 1131;
pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: c_int = 1132;
pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: c_int = 1133;
pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: c_int = 1134;
pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: c_int = 1135;
pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: c_int = 1136;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: c_int = 1137;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: c_int = 1138;
pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: c_int = 1139;
pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: c_int = 1140;
pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: c_int = 1141;
pub const _CS_POSIX_V7_LP64_OFF64_LIBS: c_int = 1142;
pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: c_int = 1143;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: c_int = 1144;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: c_int = 1145;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: c_int = 1146;
pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: c_int = 1147;

pub const RLIM_SAVED_MAX: crate::rlim_t = RLIM_INFINITY;
pub const RLIM_SAVED_CUR: crate::rlim_t = RLIM_INFINITY;

pub const GLOB_ERR: c_int = 1 << 0;
pub const GLOB_MARK: c_int = 1 << 1;
pub const GLOB_NOSORT: c_int = 1 << 2;
pub const GLOB_DOOFFS: c_int = 1 << 3;
pub const GLOB_NOCHECK: c_int = 1 << 4;
pub const GLOB_APPEND: c_int = 1 << 5;
pub const GLOB_NOESCAPE: c_int = 1 << 6;

pub const GLOB_NOSPACE: c_int = 1;
pub const GLOB_ABORTED: c_int = 2;
pub const GLOB_NOMATCH: c_int = 3;

pub const POSIX_MADV_NORMAL: c_int = 0;
pub const POSIX_MADV_RANDOM: c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: c_int = 2;
pub const POSIX_MADV_WILLNEED: c_int = 3;
pub const POSIX_MADV_DONTNEED: c_int = 4;

pub const S_IEXEC: mode_t = 0o0100;
pub const S_IWRITE: mode_t = 0o0200;
pub const S_IREAD: mode_t = 0o0400;

pub const F_LOCK: c_int = 1;
pub const F_TEST: c_int = 3;
pub const F_TLOCK: c_int = 2;
pub const F_ULOCK: c_int = 0;

pub const ST_RDONLY: c_ulong = 1;
pub const ST_NOSUID: c_ulong = 2;
pub const ST_NODEV: c_ulong = 4;
pub const ST_NOEXEC: c_ulong = 8;
pub const ST_SYNCHRONOUS: c_ulong = 16;
pub const ST_MANDLOCK: c_ulong = 64;
pub const ST_WRITE: c_ulong = 128;
pub const ST_APPEND: c_ulong = 256;
pub const ST_IMMUTABLE: c_ulong = 512;
pub const ST_NOATIME: c_ulong = 1024;
pub const ST_NODIRATIME: c_ulong = 2048;

pub const RTLD_NEXT: *mut c_void = -1i64 as *mut c_void;
pub const RTLD_DEFAULT: *mut c_void = 0i64 as *mut c_void;
pub const RTLD_NODELETE: c_int = 0x1000;
pub const RTLD_NOW: c_int = 0x2;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    size: [0; __SIZEOF_PTHREAD_MUTEX_T],
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    size: [0; __SIZEOF_PTHREAD_COND_T],
};

#[cfg(not(target_env = "uclibc"))]
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
};

pub const PTHREAD_BARRIER_SERIAL_THREAD: c_int = -1;
pub const PTHREAD_ONCE_INIT: pthread_once_t = 0;
pub const PTHREAD_MUTEX_NORMAL: c_int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: c_int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: c_int = PTHREAD_MUTEX_NORMAL;
#[cfg(not(target_env = "uclibc"))]
pub const PTHREAD_MUTEX_STALLED: c_int = 0;
#[cfg(not(target_env = "uclibc"))]
pub const PTHREAD_MUTEX_ROBUST: c_int = 1;
#[cfg(not(target_env = "uclibc"))]
pub const PTHREAD_PRIO_NONE: c_int = 0;
#[cfg(not(target_env = "uclibc"))]
pub const PTHREAD_PRIO_INHERIT: c_int = 1;
#[cfg(not(target_env = "uclibc"))]
pub const PTHREAD_PRIO_PROTECT: c_int = 2;
pub const PTHREAD_PROCESS_PRIVATE: c_int = 0;
pub const PTHREAD_PROCESS_SHARED: c_int = 1;
pub const PTHREAD_INHERIT_SCHED: c_int = 0;
pub const PTHREAD_EXPLICIT_SCHED: c_int = 1;
#[cfg(not(target_env = "uclibc"))]
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;

pub const SCHED_OTHER: c_int = 0;
pub const SCHED_FIFO: c_int = 1;
pub const SCHED_RR: c_int = 2;
pub const SCHED_BATCH: c_int = 3;
pub const SCHED_IDLE: c_int = 5;
pub const SCHED_DEADLINE: c_int = 6;
pub const SCHED_RESET_ON_FORK: c_int = 0x40000000;

// netinet/in.h
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

// IPPROTO_IP defined in src/unix/mod.rs
/// Hop-by-hop option header
pub const IPPROTO_HOPOPTS: c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: c_int = 2;
/// for compatibility
pub const IPPROTO_IPIP: c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// exterior gateway protocol
pub const IPPROTO_EGP: c_int = 8;
/// pup
pub const IPPROTO_PUP: c_int = 12;
// IPPROTO_UDP defined in src/unix/mod.rs
/// xns idp
pub const IPPROTO_IDP: c_int = 22;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: c_int = 29;
/// DCCP
pub const IPPROTO_DCCP: c_int = 33;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// IP6 routing header
pub const IPPROTO_ROUTING: c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: c_int = 44;
/// resource reservation
pub const IPPROTO_RSVP: c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: c_int = 47;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: c_int = 51;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: c_int = 60;
pub const IPPROTO_MTP: c_int = 92;
/// encapsulation header
pub const IPPROTO_ENCAP: c_int = 98;
/// Protocol indep. multicast
pub const IPPROTO_PIM: c_int = 103;
/// IP Payload Comp. Protocol
pub const IPPROTO_COMP: c_int = 108;
/// SCTP
pub const IPPROTO_SCTP: c_int = 132;
pub const IPPROTO_MH: c_int = 135;
pub const IPPROTO_UDPLITE: c_int = 136;
/// raw IP packet
pub const IPPROTO_RAW: c_int = 255;

pub const IN6ADDR_LOOPBACK_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
};

pub const IN6ADDR_ANY_INIT: in6_addr = in6_addr {
    s6_addr: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const ARPOP_REQUEST: u16 = 1;
pub const ARPOP_REPLY: u16 = 2;

// System V IPC
pub const IPC_PRIVATE: crate::key_t = 0;

pub const IPC_CREAT: c_int = 0o1000;
pub const IPC_EXCL: c_int = 0o2000;
pub const IPC_NOWAIT: c_int = 0o4000;

pub const IPC_RMID: c_int = 0;
pub const IPC_SET: c_int = 1;
pub const IPC_STAT: c_int = 2;
pub const IPC_INFO: c_int = 3;

pub const SHM_R: c_int = 0o400;
pub const SHM_W: c_int = 0o200;

pub const SHM_RDONLY: c_int = 0o10000;
pub const SHM_RND: c_int = 0o20000;
pub const SHM_REMAP: c_int = 0o40000;
#[cfg(not(target_env = "uclibc"))]
pub const SHM_EXEC: c_int = 0o100000;

pub const SHM_LOCK: c_int = 11;
pub const SHM_UNLOCK: c_int = 12;

pub const SHM_HUGETLB: c_int = 0o4000;
pub const SHM_NORESERVE: c_int = 0o10000;

pub const LOG_NFACILITIES: c_int = 24;

pub const SEM_FAILED: *mut crate::sem_t = 0 as *mut sem_t;

pub const AI_PASSIVE: c_int = 0x0001;
pub const AI_CANONNAME: c_int = 0x0002;
pub const AI_NUMERICHOST: c_int = 0x0004;
pub const AI_V4MAPPED: c_int = 0x0008;
pub const AI_ALL: c_int = 0x0010;
pub const AI_ADDRCONFIG: c_int = 0x0020;

pub const AI_NUMERICSERV: c_int = 0x0400;

pub const EAI_BADFLAGS: c_int = -1;
pub const EAI_NONAME: c_int = -2;
pub const EAI_AGAIN: c_int = -3;
pub const EAI_FAIL: c_int = -4;
pub const EAI_NODATA: c_int = -5;
pub const EAI_FAMILY: c_int = -6;
pub const EAI_SOCKTYPE: c_int = -7;
pub const EAI_SERVICE: c_int = -8;
pub const EAI_MEMORY: c_int = -10;
pub const EAI_SYSTEM: c_int = -11;
pub const EAI_OVERFLOW: c_int = -12;

pub const NI_NUMERICHOST: c_int = 1;
pub const NI_NUMERICSERV: c_int = 2;
pub const NI_NOFQDN: c_int = 4;
pub const NI_NAMEREQD: c_int = 8;
pub const NI_DGRAM: c_int = 16;
#[cfg(not(target_env = "uclibc"))]
pub const NI_IDN: c_int = 32;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const AIO_CANCELED: c_int = 0;
        pub const AIO_NOTCANCELED: c_int = 1;
        pub const AIO_ALLDONE: c_int = 2;
        pub const LIO_READ: c_int = 0;
        pub const LIO_WRITE: c_int = 1;
        pub const LIO_NOP: c_int = 2;
        pub const LIO_WAIT: c_int = 0;
        pub const LIO_NOWAIT: c_int = 1;
    }
}

pub const PR_SET_PDEATHSIG: c_int = 1;
pub const PR_GET_PDEATHSIG: c_int = 2;

pub const PR_GET_DUMPABLE: c_int = 3;
pub const PR_SET_DUMPABLE: c_int = 4;

pub const PR_GET_UNALIGN: c_int = 5;
pub const PR_SET_UNALIGN: c_int = 6;
pub const PR_UNALIGN_NOPRINT: c_int = 1;
pub const PR_UNALIGN_SIGBUS: c_int = 2;

pub const PR_GET_KEEPCAPS: c_int = 7;
pub const PR_SET_KEEPCAPS: c_int = 8;

pub const PR_GET_FPEMU: c_int = 9;
pub const PR_SET_FPEMU: c_int = 10;
pub const PR_FPEMU_NOPRINT: c_int = 1;
pub const PR_FPEMU_SIGFPE: c_int = 2;

pub const PR_GET_FPEXC: c_int = 11;
pub const PR_SET_FPEXC: c_int = 12;
pub const PR_FP_EXC_SW_ENABLE: c_int = 0x80;
pub const PR_FP_EXC_DIV: c_int = 0x010000;
pub const PR_FP_EXC_OVF: c_int = 0x020000;
pub const PR_FP_EXC_UND: c_int = 0x040000;
pub const PR_FP_EXC_RES: c_int = 0x080000;
pub const PR_FP_EXC_INV: c_int = 0x100000;
pub const PR_FP_EXC_DISABLED: c_int = 0;
pub const PR_FP_EXC_NONRECOV: c_int = 1;
pub const PR_FP_EXC_ASYNC: c_int = 2;
pub const PR_FP_EXC_PRECISE: c_int = 3;

pub const PR_GET_TIMING: c_int = 13;
pub const PR_SET_TIMING: c_int = 14;
pub const PR_TIMING_STATISTICAL: c_int = 0;
pub const PR_TIMING_TIMESTAMP: c_int = 1;

pub const PR_SET_NAME: c_int = 15;
pub const PR_GET_NAME: c_int = 16;

pub const PR_GET_ENDIAN: c_int = 19;
pub const PR_SET_ENDIAN: c_int = 20;
pub const PR_ENDIAN_BIG: c_int = 0;
pub const PR_ENDIAN_LITTLE: c_int = 1;
pub const PR_ENDIAN_PPC_LITTLE: c_int = 2;

pub const PR_GET_SECCOMP: c_int = 21;
pub const PR_SET_SECCOMP: c_int = 22;

pub const PR_CAPBSET_READ: c_int = 23;
pub const PR_CAPBSET_DROP: c_int = 24;

pub const PR_GET_TSC: c_int = 25;
pub const PR_SET_TSC: c_int = 26;
pub const PR_TSC_ENABLE: c_int = 1;
pub const PR_TSC_SIGSEGV: c_int = 2;

pub const PR_GET_SECUREBITS: c_int = 27;
pub const PR_SET_SECUREBITS: c_int = 28;

pub const PR_SET_TIMERSLACK: c_int = 29;
pub const PR_GET_TIMERSLACK: c_int = 30;

pub const PR_TASK_PERF_EVENTS_DISABLE: c_int = 31;
pub const PR_TASK_PERF_EVENTS_ENABLE: c_int = 32;

pub const PR_MCE_KILL: c_int = 33;
pub const PR_MCE_KILL_CLEAR: c_int = 0;
pub const PR_MCE_KILL_SET: c_int = 1;

pub const PR_MCE_KILL_LATE: c_int = 0;
pub const PR_MCE_KILL_EARLY: c_int = 1;
pub const PR_MCE_KILL_DEFAULT: c_int = 2;

pub const PR_MCE_KILL_GET: c_int = 34;

pub const PR_SET_MM: c_int = 35;
pub const PR_SET_MM_START_CODE: c_int = 1;
pub const PR_SET_MM_END_CODE: c_int = 2;
pub const PR_SET_MM_START_DATA: c_int = 3;
pub const PR_SET_MM_END_DATA: c_int = 4;
pub const PR_SET_MM_START_STACK: c_int = 5;
pub const PR_SET_MM_START_BRK: c_int = 6;
pub const PR_SET_MM_BRK: c_int = 7;
pub const PR_SET_MM_ARG_START: c_int = 8;
pub const PR_SET_MM_ARG_END: c_int = 9;
pub const PR_SET_MM_ENV_START: c_int = 10;
pub const PR_SET_MM_ENV_END: c_int = 11;
pub const PR_SET_MM_AUXV: c_int = 12;
pub const PR_SET_MM_EXE_FILE: c_int = 13;
pub const PR_SET_MM_MAP: c_int = 14;
pub const PR_SET_MM_MAP_SIZE: c_int = 15;

pub const PR_SET_PTRACER: c_int = 0x59616d61;
pub const PR_SET_PTRACER_ANY: c_ulong = 0xffffffffffffffff;

pub const PR_SET_CHILD_SUBREAPER: c_int = 36;
pub const PR_GET_CHILD_SUBREAPER: c_int = 37;

pub const PR_SET_NO_NEW_PRIVS: c_int = 38;
pub const PR_GET_NO_NEW_PRIVS: c_int = 39;

pub const PR_GET_TID_ADDRESS: c_int = 40;

pub const PR_SET_THP_DISABLE: c_int = 41;
pub const PR_GET_THP_DISABLE: c_int = 42;

pub const PR_MPX_ENABLE_MANAGEMENT: c_int = 43;
pub const PR_MPX_DISABLE_MANAGEMENT: c_int = 44;

pub const PR_SET_FP_MODE: c_int = 45;
pub const PR_GET_FP_MODE: c_int = 46;
pub const PR_FP_MODE_FR: c_int = 1 << 0;
pub const PR_FP_MODE_FRE: c_int = 1 << 1;

pub const PR_CAP_AMBIENT: c_int = 47;
pub const PR_CAP_AMBIENT_IS_SET: c_int = 1;
pub const PR_CAP_AMBIENT_RAISE: c_int = 2;
pub const PR_CAP_AMBIENT_LOWER: c_int = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: c_int = 4;

pub const PR_SET_VMA: c_int = 0x53564d41;
pub const PR_SET_VMA_ANON_NAME: c_int = 0;

pub const PR_SCHED_CORE: c_int = 62;
pub const PR_SCHED_CORE_GET: c_int = 0;
pub const PR_SCHED_CORE_CREATE: c_int = 1;
pub const PR_SCHED_CORE_SHARE_TO: c_int = 2;
pub const PR_SCHED_CORE_SHARE_FROM: c_int = 3;
pub const PR_SCHED_CORE_MAX: c_int = 4;
pub const PR_SCHED_CORE_SCOPE_THREAD: c_int = 0;
pub const PR_SCHED_CORE_SCOPE_THREAD_GROUP: c_int = 1;
pub const PR_SCHED_CORE_SCOPE_PROCESS_GROUP: c_int = 2;

pub const ITIMER_REAL: c_int = 0;
pub const ITIMER_VIRTUAL: c_int = 1;
pub const ITIMER_PROF: c_int = 2;

pub const _POSIX_VDISABLE: crate::cc_t = 0;
pub const IUTF8: crate::tcflag_t = 0x00004000;
pub const CMSPAR: crate::tcflag_t = 0o10000000000;

pub const MFD_CLOEXEC: c_uint = 0x0001;
pub const MFD_ALLOW_SEALING: c_uint = 0x0002;
pub const MFD_HUGETLB: c_uint = 0x0004;
cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const MFD_NOEXEC_SEAL: c_uint = 0x0008;
        pub const MFD_EXEC: c_uint = 0x0010;
        pub const MFD_HUGE_64KB: c_uint = 0x40000000;
        pub const MFD_HUGE_512KB: c_uint = 0x4c000000;
        pub const MFD_HUGE_1MB: c_uint = 0x50000000;
        pub const MFD_HUGE_2MB: c_uint = 0x54000000;
        pub const MFD_HUGE_8MB: c_uint = 0x5c000000;
        pub const MFD_HUGE_16MB: c_uint = 0x60000000;
        pub const MFD_HUGE_32MB: c_uint = 0x64000000;
        pub const MFD_HUGE_256MB: c_uint = 0x70000000;
        pub const MFD_HUGE_512MB: c_uint = 0x74000000;
        pub const MFD_HUGE_1GB: c_uint = 0x78000000;
        pub const MFD_HUGE_2GB: c_uint = 0x7c000000;
        pub const MFD_HUGE_16GB: c_uint = 0x88000000;
        pub const MFD_HUGE_MASK: c_uint = 63;
        pub const MFD_HUGE_SHIFT: c_uint = 26;

        pub const NLMSG_NOOP: c_int = 0x1;
        pub const NLMSG_ERROR: c_int = 0x2;
        pub const NLMSG_DONE: c_int = 0x3;
        pub const NLMSG_OVERRUN: c_int = 0x4;
        pub const NLMSG_MIN_TYPE: c_int = 0x10;
    }
}

// elf.h - Legal values for p_type (segment type).
pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_NUM: u32 = 8;
pub const PT_LOOS: u32 = 0x60000000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;
pub const PT_LOSUNW: u32 = 0x6ffffffa;
pub const PT_SUNWBSS: u32 = 0x6ffffffa;
pub const PT_SUNWSTACK: u32 = 0x6ffffffb;
pub const PT_HISUNW: u32 = 0x6fffffff;
pub const PT_HIOS: u32 = 0x6fffffff;
pub const PT_LOPROC: u32 = 0x70000000;
pub const PT_HIPROC: u32 = 0x7fffffff;

pub const NCCS: usize = 32;

pub const EBFONT: c_int = 59;
pub const ENOSTR: c_int = 60;
pub const ENODATA: c_int = 61;
pub const ETIME: c_int = 62;
pub const ENOSR: c_int = 63;
pub const ENONET: c_int = 64;
pub const ENOPKG: c_int = 65;
pub const EREMOTE: c_int = 66;
pub const ENOLINK: c_int = 67;
pub const EADV: c_int = 68;
pub const ESRMNT: c_int = 69;
pub const ECOMM: c_int = 70;
pub const EPROTO: c_int = 71;
pub const EDOTDOT: c_int = 73;

pub const SOCK_DCCP: c_int = 6;
pub const SOCK_PACKET: c_int = 10;

pub const SIGUNUSED: c_int = crate::SIGSYS;

pub const CBAUD: crate::tcflag_t = 0x100f;
pub const TAB1: c_int = 0x800;
pub const TAB2: c_int = 0x1000;
pub const TAB3: c_int = 0x1800;
pub const TABDLY: c_int = 0x1800;
pub const CR1: c_int = 0x200;
pub const CR2: c_int = 0x400;
pub const CR3: c_int = 0x600;
pub const FF1: c_int = 0x8000;
pub const BS1: c_int = 0x2000;
pub const VT1: c_int = 0x4000;
pub const VWERASE: usize = 0xe;
pub const VREPRINT: usize = 0xc;
pub const VSUSP: usize = 0xa;
pub const VDISCARD: usize = 0xd;
pub const IXON: crate::tcflag_t = 0x400;
pub const IXOFF: crate::tcflag_t = 0x1000;
pub const ONLCR: crate::tcflag_t = 0x4;
pub const CSIZE: crate::tcflag_t = 0x30;
pub const CS6: crate::tcflag_t = 0x10;
pub const CS7: crate::tcflag_t = 0x20;
pub const CS8: crate::tcflag_t = 0x30;
pub const CSTOPB: crate::tcflag_t = 0x40;
pub const CREAD: crate::tcflag_t = 0x80;
pub const PARENB: crate::tcflag_t = 0x100;
pub const PARODD: crate::tcflag_t = 0x200;
pub const HUPCL: crate::tcflag_t = 0x400;
pub const CLOCAL: crate::tcflag_t = 0x800;
pub const ECHOKE: crate::tcflag_t = 0x800;
pub const ECHOE: crate::tcflag_t = 0x10;
pub const ECHOK: crate::tcflag_t = 0x20;
pub const ECHONL: crate::tcflag_t = 0x40;
pub const ECHOPRT: crate::tcflag_t = 0x400;
pub const ECHOCTL: crate::tcflag_t = 0x200;
pub const ISIG: crate::tcflag_t = 0x1;
pub const ICANON: crate::tcflag_t = 0x2;
pub const PENDIN: crate::tcflag_t = 0x4000;
pub const NOFLSH: crate::tcflag_t = 0x80;
pub const CIBAUD: crate::tcflag_t = 0x100f0000;
pub const CBAUDEX: crate::tcflag_t = 0x1000;
pub const OLCUC: crate::tcflag_t = 0x2;
pub const NLDLY: crate::tcflag_t = 0x100;
pub const CRDLY: c_int = 0x600;
pub const BSDLY: c_int = 0x2000;
pub const FFDLY: c_int = 0x8000;
pub const XTABS: crate::tcflag_t = 0x1800;

pub const B0: crate::speed_t = 0;
pub const B1000000: crate::speed_t = 0x1008;
pub const B110: crate::speed_t = 0x3;
pub const B115200: crate::speed_t = 0x1002;
pub const B1152000: crate::speed_t = 0x1009;
pub const B1200: crate::speed_t = 0x9;
pub const B134: crate::speed_t = 0x4;
pub const B150: crate::speed_t = 0x5;
pub const B1500000: crate::speed_t = 0x100a;
pub const B1800: crate::speed_t = 0xa;
pub const B19200: crate::speed_t = 0xe;
pub const B200: crate::speed_t = 0x6;
pub const B2000000: crate::speed_t = 0x100b;
pub const B230400: crate::speed_t = 0x1003;
pub const B2400: crate::speed_t = 0xb;
pub const B2500000: crate::speed_t = 0x100c;
pub const B300: crate::speed_t = 0x7;
pub const B3000000: crate::speed_t = 0x100d;
pub const B3500000: crate::speed_t = 0x100e;
pub const B38400: crate::speed_t = 0xf;
pub const B4000000: crate::speed_t = 0x100f;
pub const B460800: crate::speed_t = 0x1004;
pub const B4800: crate::speed_t = 0xc;
pub const B50: crate::speed_t = 0x1;
pub const B500000: crate::speed_t = 0x1005;
pub const B57600: crate::speed_t = 0x1001;
pub const B576000: crate::speed_t = 0x1006;
pub const B600: crate::speed_t = 0x8;
pub const B75: crate::speed_t = 0x2;
pub const B921600: crate::speed_t = 0x1007;
pub const B9600: crate::speed_t = 0xd;
pub const EXTA: c_uint = B19200;
pub const EXTB: c_uint = B38400;
pub const SOCK_CLOEXEC: c_int = 0o2000000;

pub const MAP_ANON: c_int = 0x0020;
pub const MAP_ANONYMOUS: c_int = MAP_ANON;
pub const MAP_GROWSDOWN: c_int = 0x0100;
pub const MAP_DENYWRITE: c_int = 0x0800;
pub const MAP_EXECUTABLE: c_int = 0x01000;
pub const MAP_LOCKED: c_int = 0x02000;
pub const MAP_NORESERVE: c_int = 0x04000;
pub const MAP_POPULATE: c_int = 0x08000;
pub const MAP_NONBLOCK: c_int = 0x010000;
pub const MAP_STACK: c_int = 0x020000;
pub const MAP_HUGETLB: c_int = 0x040000;

pub const SOCK_STREAM: c_int = 1;
pub const SOCK_DGRAM: c_int = 2;
pub const SOCK_NONBLOCK: c_int = 0o0004000;
pub const SOCK_SEQPACKET: c_int = 0x5;

pub const SOL_SOCKET: c_int = 1;

pub const EDEADLK: c_int = 35;
pub const EDEADLOCK: c_int = EDEADLK;
pub const ENAMETOOLONG: c_int = 36;
pub const ENOLCK: c_int = 37;
pub const ENOSYS: c_int = 38;
pub const ENOTEMPTY: c_int = 39;
pub const ELOOP: c_int = 40;
pub const ENOMSG: c_int = 42;
pub const EIDRM: c_int = 43;
pub const ECHRNG: c_int = 44;
pub const EL2NSYNC: c_int = 45;
pub const EL3HLT: c_int = 46;
pub const EL3RST: c_int = 47;
pub const ELNRNG: c_int = 48;
pub const EUNATCH: c_int = 49;
pub const ENOCSI: c_int = 50;
pub const EL2HLT: c_int = 51;
pub const EBADE: c_int = 52;
pub const EBADR: c_int = 53;
pub const EXFULL: c_int = 54;
pub const ENOANO: c_int = 55;
pub const EBADRQC: c_int = 56;
pub const EBADSLT: c_int = 57;
pub const EMULTIHOP: c_int = 72;
pub const EBADMSG: c_int = 74;
pub const EOVERFLOW: c_int = 75;
pub const ENOTUNIQ: c_int = 76;
pub const EBADFD: c_int = 77;
pub const EREMCHG: c_int = 78;
pub const ELIBACC: c_int = 79;
pub const ELIBBAD: c_int = 80;
pub const ELIBSCN: c_int = 81;
pub const ELIBMAX: c_int = 82;
pub const ELIBEXEC: c_int = 83;
pub const EILSEQ: c_int = 84;
pub const ERESTART: c_int = 85;
pub const ESTRPIPE: c_int = 86;
pub const EUSERS: c_int = 87;
pub const ENOTSOCK: c_int = 88;
pub const EDESTADDRREQ: c_int = 89;
pub const EMSGSIZE: c_int = 90;
pub const EPROTOTYPE: c_int = 91;
pub const ENOPROTOOPT: c_int = 92;
pub const EPROTONOSUPPORT: c_int = 93;
pub const ESOCKTNOSUPPORT: c_int = 94;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUP: c_int = EOPNOTSUPP;
pub const EPFNOSUPPORT: c_int = 96;
pub const EAFNOSUPPORT: c_int = 97;
pub const EADDRINUSE: c_int = 98;
pub const EADDRNOTAVAIL: c_int = 99;
pub const ENETDOWN: c_int = 100;
pub const ENETUNREACH: c_int = 101;
pub const ENETRESET: c_int = 102;
pub const ECONNABORTED: c_int = 103;
pub const ECONNRESET: c_int = 104;
pub const ENOBUFS: c_int = 105;
pub const EISCONN: c_int = 106;
pub const ENOTCONN: c_int = 107;
pub const ESHUTDOWN: c_int = 108;
pub const ETOOMANYREFS: c_int = 109;
pub const ETIMEDOUT: c_int = 110;
pub const ECONNREFUSED: c_int = 111;
pub const EHOSTDOWN: c_int = 112;
pub const EHOSTUNREACH: c_int = 113;
pub const EALREADY: c_int = 114;
pub const EINPROGRESS: c_int = 115;
pub const ESTALE: c_int = 116;
pub const EUCLEAN: c_int = 117;
pub const ENOTNAM: c_int = 118;
pub const ENAVAIL: c_int = 119;
pub const EISNAM: c_int = 120;
pub const EREMOTEIO: c_int = 121;
pub const EDQUOT: c_int = 122;
pub const ENOMEDIUM: c_int = 123;
pub const EMEDIUMTYPE: c_int = 124;
pub const ECANCELED: c_int = 125;
pub const ENOKEY: c_int = 126;
pub const EKEYEXPIRED: c_int = 127;
pub const EKEYREVOKED: c_int = 128;
pub const EKEYREJECTED: c_int = 129;
pub const EOWNERDEAD: c_int = 130;
pub const ENOTRECOVERABLE: c_int = 131;

pub const SO_REUSEADDR: c_int = 2;
pub const SO_TYPE: c_int = 3;
pub const SO_ERROR: c_int = 4;
pub const SO_DONTROUTE: c_int = 5;
pub const SO_BROADCAST: c_int = 6;
pub const SO_SNDBUF: c_int = 7;
pub const SO_RCVBUF: c_int = 8;
pub const SO_KEEPALIVE: c_int = 9;
pub const SO_OOBINLINE: c_int = 10;
pub const SO_NO_CHECK: c_int = 11;
pub const SO_PRIORITY: c_int = 12;
pub const SO_LINGER: c_int = 13;
pub const SO_BSDCOMPAT: c_int = 14;
pub const SO_PASSCRED: c_int = 16;
pub const SO_PEERCRED: c_int = 17;
pub const SO_RCVLOWAT: c_int = 18;
pub const SO_SNDLOWAT: c_int = 19;
const SO_RCVTIMEO_OLD: c_int = 20;
const SO_SNDTIMEO_OLD: c_int = 21;
pub const SO_SECURITY_AUTHENTICATION: c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: c_int = 24;
pub const SO_BINDTODEVICE: c_int = 25;
pub const SO_ATTACH_FILTER: c_int = 26;
pub const SO_DETACH_FILTER: c_int = 27;
pub const SO_PEERNAME: c_int = 28;
const SO_TIMESTAMP_OLD: c_int = 29;
const SO_TIMESTAMPNS_OLD: c_int = 35;
const SO_TIMESTAMPING_OLD: c_int = 37;
pub const SO_TIMESTAMP: c_int = SO_TIMESTAMP_OLD;
pub const SO_RCVTIMEO: c_int = SO_RCVTIMEO_OLD;
pub const SO_SNDTIMEO: c_int = SO_SNDTIMEO_OLD;

pub const SO_ACCEPTCONN: c_int = 30;
pub const SO_PEERSEC: c_int = 31;

pub const SA_NOCLDSTOP: c_ulong = 0x1;
pub const SA_NOCLDWAIT: c_ulong = 0x2;
pub const SA_SIGINFO: c_ulong = 0x4;
pub const SA_NODEFER: c_ulong = 0x40000000;
pub const SA_ONSTACK: c_ulong = 0x8000000;
pub const SA_RESETHAND: c_ulong = 0x80000000;
pub const SA_RESTART: c_ulong = 0x10000000;

pub const SIGBUS: c_int = 0x7;
pub const SIGCHLD: c_int = 0x11;
pub const SIGCONT: c_int = 0x12;
pub const SIGIO: c_int = 0x1d;
pub const SIGPOLL: c_int = SIGIO;
pub const SIGPROF: c_int = 0x1b;
pub const SIGPWR: c_int = 0x1e;
pub const SIGSTKFLT: c_int = 0x10;
pub const SIGSTOP: c_int = 0x13;
pub const SIGSYS: c_int = 0x1f;
pub const SIGTSTP: c_int = 0x14;
pub const SIGTTIN: c_int = 0x15;
pub const SIGTTOU: c_int = 0x16;
pub const SIGURG: c_int = 0x17;
pub const SIGUSR1: c_int = 0xa;
pub const SIGUSR2: c_int = 0xc;
pub const SIGVTALRM: c_int = 0x1a;
pub const SIGWINCH: c_int = 0x1c;
pub const SIGXCPU: c_int = 0x18;
pub const SIGXFSZ: c_int = 0x19;
pub const SIG_BLOCK: c_int = 0;
pub const SIG_UNBLOCK: c_int = 0x1;
pub const SIG_SETMASK: c_int = 0x2;
pub const SIG_DFL: sighandler_t = 0 as sighandler_t;
pub const SIG_IGN: sighandler_t = 1 as sighandler_t;
pub const SIG_ERR: sighandler_t = !0 as sighandler_t;

pub const F_GETLK: c_int = 0x5;
pub const F_SETLK: c_int = 0x6;
pub const F_SETLKW: c_int = 0x7;
pub const F_SETOWN: c_int = 0x8;
pub const F_GETOWN: c_int = 0x9;

pub const VEOF: usize = 0x4;
pub const VEOL: usize = 0xb;
pub const VEOL2: usize = 0x10;
pub const VMIN: usize = 0x6;
pub const VSTART: usize = 0x8;
pub const VSTOP: usize = 0x9;
pub const VSWTC: usize = 0x7;
pub const VTDLY: c_int = 0x4000;
pub const VTIME: usize = 0x5;
pub const IEXTEN: crate::tcflag_t = 0x8000;
pub const TOSTOP: crate::tcflag_t = 0x100;
pub const FLUSHO: crate::tcflag_t = 0x1000;

pub const TCGETS: Ioctl = 0x5401;
pub const TCSETS: Ioctl = 0x5402;
pub const TCSETSW: Ioctl = 0x5403;
pub const TCSETSF: Ioctl = 0x5404;
pub const TCGETA: Ioctl = 0x5405;
pub const TCSETA: Ioctl = 0x5406;
pub const TCSETAW: Ioctl = 0x5407;
pub const TCSETAF: Ioctl = 0x5408;
pub const TCSBRK: Ioctl = 0x5409;
pub const TCXONC: Ioctl = 0x540A;
pub const TCFLSH: Ioctl = 0x540B;
pub const TIOCM_LE: c_int = 0x001;
pub const TIOCM_DTR: c_int = 0x002;
pub const TIOCM_RTS: c_int = 0x004;
pub const TIOCM_ST: c_int = 0x008;
pub const TIOCM_SR: c_int = 0x010;
pub const TIOCM_CTS: c_int = 0x020;
pub const TIOCM_CAR: c_int = 0x040;
pub const TIOCM_CD: c_int = TIOCM_CAR;
pub const TIOCM_RNG: c_int = 0x080;
pub const TIOCM_RI: c_int = TIOCM_RNG;
pub const TIOCM_DSR: c_int = 0x100;
pub const TIOCEXCL: Ioctl = 0x540C;
pub const TIOCNXCL: Ioctl = 0x540D;
pub const TIOCSCTTY: Ioctl = 0x540E;
pub const TIOCGPGRP: Ioctl = 0x540F;
pub const TIOCSPGRP: Ioctl = 0x5410;
pub const TIOCOUTQ: Ioctl = 0x5411;
pub const TIOCSTI: Ioctl = 0x5412;
pub const TIOCGWINSZ: Ioctl = 0x5413;
pub const TIOCSWINSZ: Ioctl = 0x5414;
pub const TIOCMGET: Ioctl = 0x5415;
pub const TIOCMBIS: Ioctl = 0x5416;
pub const TIOCMBIC: Ioctl = 0x5417;
pub const TIOCMSET: Ioctl = 0x5418;
pub const TIOCGSOFTCAR: Ioctl = 0x5419;
pub const TIOCSSOFTCAR: Ioctl = 0x541A;
pub const FIONREAD: Ioctl = 0x541B;
pub const TIOCINQ: Ioctl = FIONREAD;
pub const TIOCLINUX: Ioctl = 0x541C;
pub const TIOCCONS: Ioctl = 0x541D;
pub const TIOCGSERIAL: Ioctl = 0x541E;
pub const TIOCSSERIAL: Ioctl = 0x541F;
pub const TIOCPKT: Ioctl = 0x5420;
pub const FIONBIO: Ioctl = 0x5421;
pub const TIOCNOTTY: Ioctl = 0x5422;
pub const TIOCSETD: Ioctl = 0x5423;
pub const TIOCGETD: Ioctl = 0x5424;
pub const TCSBRKP: Ioctl = 0x5425;
pub const TIOCSBRK: Ioctl = 0x5427;
pub const TIOCCBRK: Ioctl = 0x5428;
pub const TIOCGSID: Ioctl = 0x5429;
pub const TIOCGPTN: Ioctl = 0x80045430;
pub const TIOCSPTLCK: Ioctl = 0x40045431;
pub const FIONCLEX: Ioctl = 0x5450;
pub const FIOCLEX: Ioctl = 0x5451;
pub const FIOASYNC: Ioctl = 0x5452;
pub const TIOCSERCONFIG: Ioctl = 0x5453;
pub const TIOCSERGWILD: Ioctl = 0x5454;
pub const TIOCSERSWILD: Ioctl = 0x5455;
pub const TIOCGLCKTRMIOS: Ioctl = 0x5456;
pub const TIOCSLCKTRMIOS: Ioctl = 0x5457;
pub const TIOCSERGSTRUCT: Ioctl = 0x5458;
pub const TIOCSERGETLSR: Ioctl = 0x5459;
pub const TIOCSERGETMULTI: Ioctl = 0x545A;
pub const TIOCSERSETMULTI: Ioctl = 0x545B;
pub const TIOCMIWAIT: Ioctl = 0x545C;
pub const TIOCGICOUNT: Ioctl = 0x545D;
pub const BLKSSZGET: Ioctl = 0x1268;

// intentionally not public, only used for fd_set
cfg_if! {
    if #[cfg(target_pointer_width = "32")] {
        const ULONG_SIZE: usize = 32;
    } else if #[cfg(target_pointer_width = "64")] {
        const ULONG_SIZE: usize = 64;
    } else {
        // Unknown target_pointer_width
    }
}

pub const ATF_COM: c_int = 0x02;
pub const ATF_PERM: c_int = 0x04;
pub const ATF_PUBL: c_int = 0x08;
pub const ATF_USETRAILERS: c_int = 0x10;

pub const FNM_PERIOD: c_int = 1 << 2;
pub const FNM_NOMATCH: c_int = 1;

cfg_if! {
    if #[cfg(any(target_os = "illumos", target_os = "solaris",))] {
        pub const FNM_CASEFOLD: c_int = 1 << 3;
    } else {
        pub const FNM_CASEFOLD: c_int = 1 << 4;
    }
}

cfg_if! {
    if #[cfg(any(
        target_os = "macos",
        target_os = "freebsd",
        target_os = "android",
        target_os = "openbsd",
    ))] {
        pub const FNM_PATHNAME: c_int = 1 << 1;
        pub const FNM_NOESCAPE: c_int = 1 << 0;
    } else {
        pub const FNM_PATHNAME: c_int = 1 << 0;
        pub const FNM_NOESCAPE: c_int = 1 << 1;
    }
}

const SIGEV_MAX_SIZE: usize = 64;
cfg_if! {
    if #[cfg(target_pointer_width = "64")] {
        const __ARCH_SIGEV_PREAMBLE_SIZE: usize = 4 * 2 + 8;
    } else {
        const __ARCH_SIGEV_PREAMBLE_SIZE: usize = 4 * 2 + 4;
    }
}
const SIGEV_PAD_SIZE: usize = (SIGEV_MAX_SIZE - __ARCH_SIGEV_PREAMBLE_SIZE) / 4;

pub const UIO_MAXIOV: c_int = 1024;

cfg_if! {
    if #[cfg(not(target_env = "uclibc"))] {
        pub const CLONE_NEWCGROUP: c_int = 0x02000000;
    }
}

// si_code values for SIGBUS signal
pub const BUS_ADRALN: c_int = 1;
pub const BUS_ADRERR: c_int = 2;
pub const BUS_OBJERR: c_int = 3;

// si_code values for SIGTRAP
pub const TRAP_BRKPT: c_int = 1;
pub const TRAP_TRACE: c_int = 2;

pub const IPTOS_LOWDELAY: u8 = 0x10;
pub const IPTOS_THROUGHPUT: u8 = 0x08;
pub const IPTOS_RELIABILITY: u8 = 0x04;
pub const IPTOS_MINCOST: u8 = 0x02;

pub const IPTOS_PREC_NETCONTROL: u8 = 0xe0;
pub const IPTOS_PREC_INTERNETCONTROL: u8 = 0xc0;
pub const IPTOS_PREC_CRITIC_ECP: u8 = 0xa0;
pub const IPTOS_PREC_FLASHOVERRIDE: u8 = 0x80;
pub const IPTOS_PREC_FLASH: u8 = 0x60;
pub const IPTOS_PREC_IMMEDIATE: u8 = 0x40;
pub const IPTOS_PREC_PRIORITY: u8 = 0x20;
pub const IPTOS_PREC_ROUTINE: u8 = 0x00;

pub const IPTOS_ECN_MASK: u8 = 0x03;
pub const IPTOS_ECN_ECT1: u8 = 0x01;
pub const IPTOS_ECN_ECT0: u8 = 0x02;
pub const IPTOS_ECN_CE: u8 = 0x03;

pub const IPOPT_COPY: u8 = 0x80;
pub const IPOPT_CLASS_MASK: u8 = 0x60;
pub const IPOPT_NUMBER_MASK: u8 = 0x1f;

pub const IPOPT_CONTROL: u8 = 0x00;
pub const IPOPT_RESERVED1: u8 = 0x20;
pub const IPOPT_MEASUREMENT: u8 = 0x40;
pub const IPOPT_RESERVED2: u8 = 0x60;
pub const IPOPT_END: u8 = 0 | IPOPT_CONTROL;
pub const IPOPT_NOOP: u8 = 1 | IPOPT_CONTROL;
pub const IPOPT_SEC: u8 = 2 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_LSRR: u8 = 3 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_TIMESTAMP: u8 = 4 | IPOPT_MEASUREMENT;
pub const IPOPT_RR: u8 = 7 | IPOPT_CONTROL;
pub const IPOPT_SID: u8 = 8 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_SSRR: u8 = 9 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPOPT_RA: u8 = 20 | IPOPT_CONTROL | IPOPT_COPY;
pub const IPVERSION: u8 = 4;
pub const MAXTTL: u8 = 255;
pub const IPDEFTTL: u8 = 64;
pub const IPOPT_OPTVAL: u8 = 0;
pub const IPOPT_OLEN: u8 = 1;
pub const IPOPT_OFFSET: u8 = 2;
pub const IPOPT_MINOFF: u8 = 4;
pub const MAX_IPOPTLEN: u8 = 40;
pub const IPOPT_NOP: u8 = IPOPT_NOOP;
pub const IPOPT_EOL: u8 = IPOPT_END;
pub const IPOPT_TS: u8 = IPOPT_TIMESTAMP;
pub const IPOPT_TS_TSONLY: u8 = 0;
pub const IPOPT_TS_TSANDADDR: u8 = 1;
pub const IPOPT_TS_PRESPEC: u8 = 3;

pub const ARPOP_RREQUEST: u16 = 3;
pub const ARPOP_RREPLY: u16 = 4;
pub const ARPOP_InREQUEST: u16 = 8;
pub const ARPOP_InREPLY: u16 = 9;
pub const ARPOP_NAK: u16 = 10;

pub const ATF_NETMASK: c_int = 0x20;
pub const ATF_DONTPUB: c_int = 0x40;

pub const ARPHRD_NETROM: u16 = 0;
pub const ARPHRD_ETHER: u16 = 1;
pub const ARPHRD_EETHER: u16 = 2;
pub const ARPHRD_AX25: u16 = 3;
pub const ARPHRD_PRONET: u16 = 4;
pub const ARPHRD_CHAOS: u16 = 5;
pub const ARPHRD_IEEE802: u16 = 6;
pub const ARPHRD_ARCNET: u16 = 7;
pub const ARPHRD_APPLETLK: u16 = 8;
pub const ARPHRD_DLCI: u16 = 15;
pub const ARPHRD_ATM: u16 = 19;
pub const ARPHRD_METRICOM: u16 = 23;
pub const ARPHRD_IEEE1394: u16 = 24;
pub const ARPHRD_EUI64: u16 = 27;
pub const ARPHRD_INFINIBAND: u16 = 32;

pub const ARPHRD_SLIP: u16 = 256;
pub const ARPHRD_CSLIP: u16 = 257;
pub const ARPHRD_SLIP6: u16 = 258;
pub const ARPHRD_CSLIP6: u16 = 259;
pub const ARPHRD_RSRVD: u16 = 260;
pub const ARPHRD_ADAPT: u16 = 264;
pub const ARPHRD_ROSE: u16 = 270;
pub const ARPHRD_X25: u16 = 271;
pub const ARPHRD_HWX25: u16 = 272;
pub const ARPHRD_CAN: u16 = 280;
pub const ARPHRD_PPP: u16 = 512;
pub const ARPHRD_CISCO: u16 = 513;
pub const ARPHRD_HDLC: u16 = ARPHRD_CISCO;
pub const ARPHRD_LAPB: u16 = 516;
pub const ARPHRD_DDCMP: u16 = 517;
pub const ARPHRD_RAWHDLC: u16 = 518;

pub const ARPHRD_TUNNEL: u16 = 768;
pub const ARPHRD_TUNNEL6: u16 = 769;
pub const ARPHRD_FRAD: u16 = 770;
pub const ARPHRD_SKIP: u16 = 771;
pub const ARPHRD_LOOPBACK: u16 = 772;
pub const ARPHRD_LOCALTLK: u16 = 773;
pub const ARPHRD_FDDI: u16 = 774;
pub const ARPHRD_BIF: u16 = 775;
pub const ARPHRD_SIT: u16 = 776;
pub const ARPHRD_IPDDP: u16 = 777;
pub const ARPHRD_IPGRE: u16 = 778;
pub const ARPHRD_PIMREG: u16 = 779;
pub const ARPHRD_HIPPI: u16 = 780;
pub const ARPHRD_ASH: u16 = 781;
pub const ARPHRD_ECONET: u16 = 782;
pub const ARPHRD_IRDA: u16 = 783;
pub const ARPHRD_FCPP: u16 = 784;
pub const ARPHRD_FCAL: u16 = 785;
pub const ARPHRD_FCPL: u16 = 786;
pub const ARPHRD_FCFABRIC: u16 = 787;
pub const ARPHRD_IEEE802_TR: u16 = 800;
pub const ARPHRD_IEEE80211: u16 = 801;
pub const ARPHRD_IEEE80211_PRISM: u16 = 802;
pub const ARPHRD_IEEE80211_RADIOTAP: u16 = 803;
pub const ARPHRD_IEEE802154: u16 = 804;

pub const ARPHRD_VOID: u16 = 0xFFFF;
pub const ARPHRD_NONE: u16 = 0xFFFE;

// elf.h - Fields in the e_ident array.
pub const EI_NIDENT: usize = 16;

pub const EI_MAG0: usize = 0;
pub const ELFMAG0: u8 = 0x7f;
pub const EI_MAG1: usize = 1;
pub const ELFMAG1: u8 = b'E';
pub const EI_MAG2: usize = 2;
pub const ELFMAG2: u8 = b'L';
pub const EI_MAG3: usize = 3;
pub const ELFMAG3: u8 = b'F';
pub const SELFMAG: usize = 4;

pub const EI_CLASS: usize = 4;
pub const ELFCLASSNONE: u8 = 0;
pub const ELFCLASS32: u8 = 1;
pub const ELFCLASS64: u8 = 2;
pub const ELFCLASSNUM: usize = 3;

pub const EI_DATA: usize = 5;
pub const ELFDATANONE: u8 = 0;
pub const ELFDATA2LSB: u8 = 1;
pub const ELFDATA2MSB: u8 = 2;
pub const ELFDATANUM: usize = 3;

pub const EI_VERSION: usize = 6;

pub const EI_OSABI: usize = 7;
pub const ELFOSABI_NONE: u8 = 0;
pub const ELFOSABI_SYSV: u8 = 0;
pub const ELFOSABI_HPUX: u8 = 1;
pub const ELFOSABI_NETBSD: u8 = 2;
pub const ELFOSABI_GNU: u8 = 3;
pub const ELFOSABI_LINUX: u8 = ELFOSABI_GNU;
pub const ELFOSABI_SOLARIS: u8 = 6;
pub const ELFOSABI_AIX: u8 = 7;
pub const ELFOSABI_IRIX: u8 = 8;
pub const ELFOSABI_FREEBSD: u8 = 9;
pub const ELFOSABI_TRU64: u8 = 10;
pub const ELFOSABI_MODESTO: u8 = 11;
pub const ELFOSABI_OPENBSD: u8 = 12;
pub const ELFOSABI_ARM: u8 = 97;
pub const ELFOSABI_STANDALONE: u8 = 255;

pub const EI_ABIVERSION: usize = 8;

pub const EI_PAD: usize = 9;

// elf.h - Legal values for e_type (object file type).
pub const ET_NONE: u16 = 0;
pub const ET_REL: u16 = 1;
pub const ET_EXEC: u16 = 2;
pub const ET_DYN: u16 = 3;
pub const ET_CORE: u16 = 4;
pub const ET_NUM: u16 = 5;
pub const ET_LOOS: u16 = 0xfe00;
pub const ET_HIOS: u16 = 0xfeff;
pub const ET_LOPROC: u16 = 0xff00;
pub const ET_HIPROC: u16 = 0xffff;

// elf.h - Legal values for e_machine (architecture).
pub const EM_NONE: u16 = 0;
pub const EM_M32: u16 = 1;
pub const EM_SPARC: u16 = 2;
pub const EM_386: u16 = 3;
pub const EM_68K: u16 = 4;
pub const EM_88K: u16 = 5;
pub const EM_860: u16 = 7;
pub const EM_MIPS: u16 = 8;
pub const EM_S370: u16 = 9;
pub const EM_MIPS_RS3_LE: u16 = 10;
pub const EM_PARISC: u16 = 15;
pub const EM_VPP500: u16 = 17;
pub const EM_SPARC32PLUS: u16 = 18;
pub const EM_960: u16 = 19;
pub const EM_PPC: u16 = 20;
pub const EM_PPC64: u16 = 21;
pub const EM_S390: u16 = 22;
pub const EM_V800: u16 = 36;
pub const EM_FR20: u16 = 37;
pub const EM_RH32: u16 = 38;
pub const EM_RCE: u16 = 39;
pub const EM_ARM: u16 = 40;
pub const EM_FAKE_ALPHA: u16 = 41;
pub const EM_SH: u16 = 42;
pub const EM_SPARCV9: u16 = 43;
pub const EM_TRICORE: u16 = 44;
pub const EM_ARC: u16 = 45;
pub const EM_H8_300: u16 = 46;
pub const EM_H8_300H: u16 = 47;
pub const EM_H8S: u16 = 48;
pub const EM_H8_500: u16 = 49;
pub const EM_IA_64: u16 = 50;
pub const EM_MIPS_X: u16 = 51;
pub const EM_COLDFIRE: u16 = 52;
pub const EM_68HC12: u16 = 53;
pub const EM_MMA: u16 = 54;
pub const EM_PCP: u16 = 55;
pub const EM_NCPU: u16 = 56;
pub const EM_NDR1: u16 = 57;
pub const EM_STARCORE: u16 = 58;
pub const EM_ME16: u16 = 59;
pub const EM_ST100: u16 = 60;
pub const EM_TINYJ: u16 = 61;
pub const EM_X86_64: u16 = 62;
pub const EM_PDSP: u16 = 63;
pub const EM_FX66: u16 = 66;
pub const EM_ST9PLUS: u16 = 67;
pub const EM_ST7: u16 = 68;
pub const EM_68HC16: u16 = 69;
pub const EM_68HC11: u16 = 70;
pub const EM_68HC08: u16 = 71;
pub const EM_68HC05: u16 = 72;
pub const EM_SVX: u16 = 73;
pub const EM_ST19: u16 = 74;
pub const EM_VAX: u16 = 75;
pub const EM_CRIS: u16 = 76;
pub const EM_JAVELIN: u16 = 77;
pub const EM_FIREPATH: u16 = 78;
pub const EM_ZSP: u16 = 79;
pub const EM_MMIX: u16 = 80;
pub const EM_HUANY: u16 = 81;
pub const EM_PRISM: u16 = 82;
pub const EM_AVR: u16 = 83;
pub const EM_FR30: u16 = 84;
pub const EM_D10V: u16 = 85;
pub const EM_D30V: u16 = 86;
pub const EM_V850: u16 = 87;
pub const EM_M32R: u16 = 88;
pub const EM_MN10300: u16 = 89;
pub const EM_MN10200: u16 = 90;
pub const EM_PJ: u16 = 91;
#[cfg(not(target_env = "uclibc"))]
pub const EM_OPENRISC: u16 = 92;
#[cfg(target_env = "uclibc")]
pub const EM_OR1K: u16 = 92;
#[cfg(not(target_env = "uclibc"))]
pub const EM_ARC_A5: u16 = 93;
pub const EM_XTENSA: u16 = 94;
pub const EM_AARCH64: u16 = 183;
pub const EM_TILEPRO: u16 = 188;
pub const EM_TILEGX: u16 = 191;
pub const EM_ALPHA: u16 = 0x9026;

// elf.h - Legal values for e_version (version).
pub const EV_NONE: u32 = 0;
pub const EV_CURRENT: u32 = 1;
pub const EV_NUM: u32 = 2;

// Legal values for p_flags (segment flags).
pub const PF_X: u32 = 1 << 0;
pub const PF_W: u32 = 1 << 1;
pub const PF_R: u32 = 1 << 2;
pub const PF_MASKOS: u32 = 0x0ff00000;
pub const PF_MASKPROC: u32 = 0xf0000000;

// elf.h - Legal values for a_type (entry type).
pub const AT_NULL: c_ulong = 0;
pub const AT_IGNORE: c_ulong = 1;
pub const AT_EXECFD: c_ulong = 2;
pub const AT_PHDR: c_ulong = 3;
pub const AT_PHENT: c_ulong = 4;
pub const AT_PHNUM: c_ulong = 5;
pub const AT_PAGESZ: c_ulong = 6;
pub const AT_BASE: c_ulong = 7;
pub const AT_FLAGS: c_ulong = 8;
pub const AT_ENTRY: c_ulong = 9;
pub const AT_NOTELF: c_ulong = 10;
pub const AT_UID: c_ulong = 11;
pub const AT_EUID: c_ulong = 12;
pub const AT_GID: c_ulong = 13;
pub const AT_EGID: c_ulong = 14;
pub const AT_PLATFORM: c_ulong = 15;
pub const AT_HWCAP: c_ulong = 16;
pub const AT_CLKTCK: c_ulong = 17;

pub const AT_SECURE: c_ulong = 23;
pub const AT_BASE_PLATFORM: c_ulong = 24;
pub const AT_RANDOM: c_ulong = 25;
pub const AT_HWCAP2: c_ulong = 26;

pub const AT_EXECFN: c_ulong = 31;

// defined in arch/<arch>/include/uapi/asm/auxvec.h but has the same value
// wherever it is defined.
pub const AT_SYSINFO_EHDR: c_ulong = 33;
#[cfg(not(target_env = "uclibc"))]
pub const AT_MINSIGSTKSZ: c_ulong = 51;

#[deprecated(
    since = "0.2.80",
    note = "This value was increased in the newer kernel \
            and we'll change this following upstream in the future release. \
            See #1896 for more info."
)]
pub const IPPROTO_MAX: c_int = 256;

#[cfg(not(target_env = "uclibc"))]
pub const MREMAP_DONTUNMAP: c_int = 4;

pub const IPV6_RTHDR_LOOSE: c_int = 0;
pub const IPV6_RTHDR_STRICT: c_int = 1;

// linux/if_packet.h
pub const PACKET_HOST: c_uchar = 0;
pub const PACKET_BROADCAST: c_uchar = 1;
pub const PACKET_MULTICAST: c_uchar = 2;
pub const PACKET_OTHERHOST: c_uchar = 3;
pub const PACKET_OUTGOING: c_uchar = 4;
pub const PACKET_LOOPBACK: c_uchar = 5;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_USER: c_uchar = 6;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_KERNEL: c_uchar = 7;

pub const PACKET_ADD_MEMBERSHIP: c_int = 1;
pub const PACKET_DROP_MEMBERSHIP: c_int = 2;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_MR_UNICAST: c_int = 3;
pub const PACKET_RX_RING: c_int = 5;
pub const PACKET_STATISTICS: c_int = 6;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_AUXDATA: c_int = 8;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_VERSION: c_int = 10;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_RESERVE: c_int = 12;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_TX_RING: c_int = 13;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_LOSS: c_int = 14;
#[cfg(not(target_env = "uclibc"))]
pub const PACKET_TIMESTAMP: c_int = 17;

pub const PACKET_MR_MULTICAST: c_int = 0;
pub const PACKET_MR_PROMISC: c_int = 1;
pub const PACKET_MR_ALLMULTI: c_int = 2;

pub const SIOCADDRT: c_ulong = 0x0000890B;
pub const SIOCDELRT: c_ulong = 0x0000890C;
pub const SIOCGIFNAME: c_ulong = 0x00008910;
pub const SIOCSIFLINK: c_ulong = 0x00008911;
pub const SIOCGIFCONF: c_ulong = 0x00008912;
pub const SIOCGIFFLAGS: c_ulong = 0x00008913;
pub const SIOCSIFFLAGS: c_ulong = 0x00008914;
pub const SIOCGIFADDR: c_ulong = 0x00008915;
pub const SIOCSIFADDR: c_ulong = 0x00008916;
pub const SIOCGIFDSTADDR: c_ulong = 0x00008917;
pub const SIOCSIFDSTADDR: c_ulong = 0x00008918;
pub const SIOCGIFBRDADDR: c_ulong = 0x00008919;
pub const SIOCSIFBRDADDR: c_ulong = 0x0000891A;
pub const SIOCGIFNETMASK: c_ulong = 0x0000891B;
pub const SIOCSIFNETMASK: c_ulong = 0x0000891C;
pub const SIOCGIFMETRIC: c_ulong = 0x0000891D;
pub const SIOCSIFMETRIC: c_ulong = 0x0000891E;
pub const SIOCGIFMEM: c_ulong = 0x0000891F;
pub const SIOCSIFMEM: c_ulong = 0x00008920;
pub const SIOCGIFMTU: c_ulong = 0x00008921;
pub const SIOCSIFMTU: c_ulong = 0x00008922;
pub const SIOCSIFNAME: c_ulong = 0x00008923;
pub const SIOCSIFHWADDR: c_ulong = 0x00008924;
pub const SIOCGIFENCAP: c_ulong = 0x00008925;
pub const SIOCSIFENCAP: c_ulong = 0x00008926;
pub const SIOCGIFHWADDR: c_ulong = 0x00008927;
pub const SIOCGIFSLAVE: c_ulong = 0x00008929;
pub const SIOCSIFSLAVE: c_ulong = 0x00008930;
pub const SIOCADDMULTI: c_ulong = 0x00008931;
pub const SIOCDELMULTI: c_ulong = 0x00008932;
pub const SIOCGIFINDEX: c_ulong = 0x00008933;
pub const SIOGIFINDEX: c_ulong = SIOCGIFINDEX;
pub const SIOCSIFPFLAGS: c_ulong = 0x00008934;
pub const SIOCGIFPFLAGS: c_ulong = 0x00008935;
pub const SIOCDIFADDR: c_ulong = 0x00008936;
pub const SIOCSIFHWBROADCAST: c_ulong = 0x00008937;
pub const SIOCGIFCOUNT: c_ulong = 0x00008938;
pub const SIOCGIFBR: c_ulong = 0x00008940;
pub const SIOCSIFBR: c_ulong = 0x00008941;
pub const SIOCGIFTXQLEN: c_ulong = 0x00008942;
pub const SIOCSIFTXQLEN: c_ulong = 0x00008943;
pub const SIOCDARP: c_ulong = 0x00008953;
pub const SIOCGARP: c_ulong = 0x00008954;
pub const SIOCSARP: c_ulong = 0x00008955;
pub const SIOCDRARP: c_ulong = 0x00008960;
pub const SIOCGRARP: c_ulong = 0x00008961;
pub const SIOCSRARP: c_ulong = 0x00008962;
pub const SIOCGIFMAP: c_ulong = 0x00008970;
pub const SIOCSIFMAP: c_ulong = 0x00008971;

pub const IPTOS_TOS_MASK: u8 = 0x1E;
pub const IPTOS_PREC_MASK: u8 = 0xE0;

pub const IPTOS_ECN_NOT_ECT: u8 = 0x00;

pub const RTF_UP: c_ushort = 0x0001;
pub const RTF_GATEWAY: c_ushort = 0x0002;

pub const RTF_HOST: c_ushort = 0x0004;
pub const RTF_REINSTATE: c_ushort = 0x0008;
pub const RTF_DYNAMIC: c_ushort = 0x0010;
pub const RTF_MODIFIED: c_ushort = 0x0020;
pub const RTF_MTU: c_ushort = 0x0040;
pub const RTF_MSS: c_ushort = RTF_MTU;
pub const RTF_WINDOW: c_ushort = 0x0080;
pub const RTF_IRTT: c_ushort = 0x0100;
pub const RTF_REJECT: c_ushort = 0x0200;
pub const RTF_STATIC: c_ushort = 0x0400;
pub const RTF_XRESOLVE: c_ushort = 0x0800;
pub const RTF_NOFORWARD: c_ushort = 0x1000;
pub const RTF_THROW: c_ushort = 0x2000;
pub const RTF_NOPMTUDISC: c_ushort = 0x4000;

pub const RTF_DEFAULT: u32 = 0x00010000;
pub const RTF_ALLONLINK: u32 = 0x00020000;
pub const RTF_ADDRCONF: u32 = 0x00040000;
pub const RTF_LINKRT: u32 = 0x00100000;
pub const RTF_NONEXTHOP: u32 = 0x00200000;
pub const RTF_CACHE: u32 = 0x01000000;
pub const RTF_FLOW: u32 = 0x02000000;
pub const RTF_POLICY: u32 = 0x04000000;

pub const RTCF_VALVE: u32 = 0x00200000;
pub const RTCF_MASQ: u32 = 0x00400000;
pub const RTCF_NAT: u32 = 0x00800000;
pub const RTCF_DOREDIRECT: u32 = 0x01000000;
pub const RTCF_LOG: u32 = 0x02000000;
pub const RTCF_DIRECTSRC: u32 = 0x04000000;

pub const RTF_LOCAL: u32 = 0x80000000;
pub const RTF_INTERFACE: u32 = 0x40000000;
pub const RTF_MULTICAST: u32 = 0x20000000;
pub const RTF_BROADCAST: u32 = 0x10000000;
pub const RTF_NAT: u32 = 0x08000000;
pub const RTF_ADDRCLASSMASK: u32 = 0xF8000000;

pub const RT_CLASS_UNSPEC: u8 = 0;
pub const RT_CLASS_DEFAULT: u8 = 253;
pub const RT_CLASS_MAIN: u8 = 254;
pub const RT_CLASS_LOCAL: u8 = 255;
pub const RT_CLASS_MAX: u8 = 255;

pub const MAX_ADDR_LEN: usize = 7;
pub const ARPD_UPDATE: c_ushort = 0x01;
pub const ARPD_LOOKUP: c_ushort = 0x02;
pub const ARPD_FLUSH: c_ushort = 0x03;
pub const ATF_MAGIC: c_int = 0x80;

// include/uapi/linux/udp.h
pub const UDP_CORK: c_int = 1;
pub const UDP_ENCAP: c_int = 100;
pub const UDP_NO_CHECK6_TX: c_int = 101;
pub const UDP_NO_CHECK6_RX: c_int = 102;
#[cfg(not(target_env = "uclibc"))]
pub const UDP_SEGMENT: c_int = 103;
#[cfg(not(target_env = "uclibc"))]
pub const UDP_GRO: c_int = 104;

// include/uapi/asm-generic/mman-common.h
pub const MAP_FIXED_NOREPLACE: c_int = 0x100000;
pub const MLOCK_ONFAULT: c_uint = 0x01;

pub const REG_EXTENDED: c_int = 1;
pub const REG_ICASE: c_int = 2;
pub const REG_NEWLINE: c_int = 4;
pub const REG_NOSUB: c_int = 8;

pub const REG_NOTBOL: c_int = 1;
pub const REG_NOTEOL: c_int = 2;

pub const REG_ENOSYS: c_int = -1;
pub const REG_NOMATCH: c_int = 1;
pub const REG_BADPAT: c_int = 2;
pub const REG_ECOLLATE: c_int = 3;
pub const REG_ECTYPE: c_int = 4;
pub const REG_EESCAPE: c_int = 5;
pub const REG_ESUBREG: c_int = 6;
pub const REG_EBRACK: c_int = 7;
pub const REG_EPAREN: c_int = 8;
pub const REG_EBRACE: c_int = 9;
pub const REG_BADBR: c_int = 10;
pub const REG_ERANGE: c_int = 11;
pub const REG_ESPACE: c_int = 12;
pub const REG_BADRPT: c_int = 13;

pub const CSIGNAL: c_int = 0x000000ff;

// elf.h
pub const NT_PRSTATUS: c_int = 1;
pub const NT_FPREGSET: c_int = 2;
pub const NT_PRPSINFO: c_int = 3;
pub const NT_TASKSTRUCT: c_int = 4;
pub const NT_PLATFORM: c_int = 5;
pub const NT_AUXV: c_int = 6;
pub const NT_GWINDOWS: c_int = 7;
pub const NT_ASRS: c_int = 8;
pub const NT_PSTATUS: c_int = 10;
pub const NT_PSINFO: c_int = 13;
pub const NT_PRCRED: c_int = 14;
pub const NT_UTSNAME: c_int = 15;
pub const NT_LWPSTATUS: c_int = 16;
pub const NT_LWPSINFO: c_int = 17;
pub const NT_PRFPXREG: c_int = 20;

const _IOC_NRBITS: u32 = 8;
const _IOC_TYPEBITS: u32 = 8;

const _IOC_SIZEBITS: u32 = 14;
const _IOC_DIRBITS: u32 = 2;

const _IOC_NONE: u32 = 0;
const _IOC_WRITE: u32 = 1;
const _IOC_READ: u32 = 2;

const _IOC_NRMASK: u32 = (1 << _IOC_NRBITS) - 1;
const _IOC_TYPEMASK: u32 = (1 << _IOC_TYPEBITS) - 1;
const _IOC_SIZEMASK: u32 = (1 << _IOC_SIZEBITS) - 1;
const _IOC_DIRMASK: u32 = (1 << _IOC_DIRBITS) - 1;

const _IOC_NRSHIFT: u32 = 0;
const _IOC_TYPESHIFT: u32 = _IOC_NRSHIFT + _IOC_NRBITS;
const _IOC_SIZESHIFT: u32 = _IOC_TYPESHIFT + _IOC_TYPEBITS;
const _IOC_DIRSHIFT: u32 = _IOC_SIZESHIFT + _IOC_SIZEBITS;

// adapted from https://github.com/torvalds/linux/blob/8a696a29c6905594e4abf78eaafcb62165ac61f1/rust/kernel/ioctl.rs

/// Build an ioctl number, analogous to the C macro of the same name.
const fn _IOC(dir: u32, ty: u32, nr: u32, size: usize) -> u32 {
    // FIXME(ctest) the `garando_syntax` crate (used by ctest2 in the CI test suite)
    // cannot currently parse these `debug_assert!`s
    //
    // debug_assert!(dir <= _IOC_DIRMASK);
    // debug_assert!(ty <= _IOC_TYPEMASK);
    // debug_assert!(nr <= _IOC_NRMASK);
    // debug_assert!(size <= (_IOC_SIZEMASK as usize));

    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((size as u32) << _IOC_SIZESHIFT)
}

/// Build an ioctl number for an argumentless ioctl.
pub(crate) const fn _IO(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_NONE, ty, nr, 0)
}

/// Build an ioctl number for an read-only ioctl.
pub(crate) const fn _IOR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ, ty, nr, size_of::<T>())
}

/// Build an ioctl number for an write-only ioctl.
pub(crate) const fn _IOW<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_WRITE, ty, nr, size_of::<T>())
}

/// Build an ioctl number for a read-write ioctl.
pub(crate) const fn _IOWR<T>(ty: u32, nr: u32) -> u32 {
    _IOC(_IOC_READ | _IOC_WRITE, ty, nr, size_of::<T>())
}

pub const BOTHER: crate::speed_t = 0o010000;

// END_PUB_CONST

f! {
    pub fn FD_CLR(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] &= !(1 << (fd % size));
        return;
    }

    pub fn FD_ISSET(fd: c_int, set: *const fd_set) -> bool {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        return ((*set).fds_bits[fd / size] & (1 << (fd % size))) != 0;
    }

    pub fn FD_SET(fd: c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        let size = mem::size_of_val(&(*set).fds_bits[0]) * 8;
        (*set).fds_bits[fd / size] |= 1 << (fd % size);
        return;
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        for slot in (*set).fds_bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn CPU_ZERO(cpuset: &mut cpu_set_t) -> () {
        for slot in cpuset.bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] |= 1 << offset;
        ()
    }

    pub fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] &= !(1 << offset);
        ()
    }

    pub fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool {
        let size_in_bits = 8 * mem::size_of_val(&cpuset.bits[0]);
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        0 != (cpuset.bits[idx] & (1 << offset))
    }

    pub fn CPU_ALLOC_SIZE(count: c_int) -> size_t {
        let _dummy: cpu_set_t = mem::zeroed();
        let size_in_bits = 8 * mem::size_of_val(&_dummy.bits[0]);
        ((count as size_t + size_in_bits - 1) / 8) as size_t
    }

    pub fn CPU_COUNT_S(size: usize, cpuset: &cpu_set_t) -> c_int {
        let mut s: u32 = 0;
        let size_of_mask = mem::size_of_val(&cpuset.bits[0]);
        for i in cpuset.bits[..(size / size_of_mask)].iter() {
            s += i.count_ones();
        }
        s as c_int
    }

    pub fn CPU_COUNT(cpuset: &cpu_set_t) -> c_int {
        CPU_COUNT_S(size_of::<cpu_set_t>(), cpuset)
    }

    pub fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool {
        set1.bits == set2.bits
    }

    pub fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut c_uchar {
        cmsg.offset(1) as *mut c_uchar
    }

    pub fn CMSG_NXTHDR(mhdr: *const msghdr, cmsg: *const cmsghdr) -> *mut cmsghdr {
        if ((*cmsg).cmsg_len as usize) < size_of::<cmsghdr>() {
            return 0 as *mut cmsghdr;
        };
        let next = (cmsg as usize + CMSG_ALIGN((*cmsg).cmsg_len as usize)) as *mut cmsghdr;
        let max = (*mhdr).msg_control as usize + (*mhdr).msg_controllen as usize;
        if (next.wrapping_offset(1)) as usize > max
            || next as usize + CMSG_ALIGN((*next).cmsg_len as usize) > max
        {
            0 as *mut cmsghdr
        } else {
            next as *mut cmsghdr
        }
    }

    pub fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if (*mhdr).msg_controllen as usize >= size_of::<cmsghdr>() {
            (*mhdr).msg_control as *mut cmsghdr
        } else {
            0 as *mut cmsghdr
        }
    }

    pub {const} fn CMSG_ALIGN(len: usize) -> usize {
        len + size_of::<usize>() - 1 & !(size_of::<usize>() - 1)
    }

    pub {const} fn CMSG_SPACE(length: c_uint) -> c_uint {
        (CMSG_ALIGN(length as usize) + CMSG_ALIGN(size_of::<cmsghdr>())) as c_uint
    }

    pub {const} fn CMSG_LEN(length: c_uint) -> c_uint {
        CMSG_ALIGN(size_of::<cmsghdr>()) as c_uint + length
    }

    pub fn IPTOS_TOS(tos: u8) -> u8 {
        tos & IPTOS_TOS_MASK
    }

    pub fn IPTOS_PREC(tos: u8) -> u8 {
        tos & IPTOS_PREC_MASK
    }

    pub fn RT_TOS(tos: u8) -> u8 {
        tos & crate::IPTOS_TOS_MASK
    }

    pub fn RT_ADDRCLASS(flags: u32) -> u32 {
        flags >> 23
    }

    pub fn RT_LOCALADDR(flags: u32) -> bool {
        (flags & RTF_ADDRCLASSMASK) == (RTF_LOCAL | RTF_INTERFACE)
    }

    pub fn ELF32_R_SYM(val: Elf32_Word) -> Elf32_Word {
        val >> 8
    }

    pub fn ELF32_R_TYPE(val: Elf32_Word) -> Elf32_Word {
        val & 0xff
    }

    pub fn ELF32_R_INFO(sym: Elf32_Word, t: Elf32_Word) -> Elf32_Word {
        sym << 8 + t & 0xff
    }

    pub fn ELF64_R_SYM(val: Elf64_Xword) -> Elf64_Xword {
        val >> 32
    }

    pub fn ELF64_R_TYPE(val: Elf64_Xword) -> Elf64_Xword {
        val & 0xffffffff
    }

    pub fn ELF64_R_INFO(sym: Elf64_Xword, t: Elf64_Xword) -> Elf64_Xword {
        sym << 32 + t
    }
}

safe_f! {
    pub {const} fn WIFSTOPPED(status: c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub {const} fn WSTOPSIG(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub {const} fn WIFCONTINUED(status: c_int) -> bool {
        status == 0xffff
    }

    pub {const} fn WIFSIGNALED(status: c_int) -> bool {
        ((status & 0x7f) + 1) as i8 >= 2
    }

    pub {const} fn WTERMSIG(status: c_int) -> c_int {
        status & 0x7f
    }

    pub {const} fn WIFEXITED(status: c_int) -> bool {
        (status & 0x7f) == 0
    }

    pub {const} fn WEXITSTATUS(status: c_int) -> c_int {
        (status >> 8) & 0xff
    }

    pub {const} fn WCOREDUMP(status: c_int) -> bool {
        (status & 0x80) != 0
    }

    pub {const} fn QCMD(cmd: c_int, type_: c_int) -> c_int {
        (cmd << 8) | (type_ & 0x00ff)
    }

    pub {const} fn makedev(major: c_uint, minor: c_uint) -> crate::dev_t {
        let major = major as crate::dev_t;
        let minor = minor as crate::dev_t;
        let mut dev = 0;
        dev |= (major & 0x00000fff) << 8;
        dev |= (major & 0xfffff000) << 32;
        dev |= (minor & 0x000000ff) << 0;
        dev |= (minor & 0xffffff00) << 12;
        dev
    }

    pub {const} fn major(dev: crate::dev_t) -> c_uint {
        let mut major = 0;
        major |= (dev & 0x00000000000fff00) >> 8;
        major |= (dev & 0xfffff00000000000) >> 32;
        major as c_uint
    }

    pub {const} fn minor(dev: crate::dev_t) -> c_uint {
        let mut minor = 0;
        minor |= (dev & 0x00000000000000ff) >> 0;
        minor |= (dev & 0x00000ffffff00000) >> 12;
        minor as c_uint
    }

    pub fn SIGRTMAX() -> c_int {
        unsafe { __libc_current_sigrtmax() }
    }

    pub fn SIGRTMIN() -> c_int {
        unsafe { __libc_current_sigrtmin() }
    }

    pub {const} fn W_EXITCODE(ret: c_int, sig: c_int) -> c_int {
        (ret << 8) | sig
    }

    pub {const} fn W_STOPCODE(sig: c_int) -> c_int {
        (sig << 8) | 0x7f
    }

    pub {const} fn IPOPT_COPIED(o: u8) -> u8 {
        o & IPOPT_COPY
    }

    pub {const} fn IPOPT_CLASS(o: u8) -> u8 {
        o & IPOPT_CLASS_MASK
    }

    pub {const} fn IPOPT_NUMBER(o: u8) -> u8 {
        o & IPOPT_NUMBER_MASK
    }

    pub {const} fn IPTOS_ECN(x: u8) -> u8 {
        x & crate::IPTOS_ECN_MASK
    }

    #[allow(ellipsis_inclusive_range_patterns)]
    pub {const} fn KERNEL_VERSION(a: u32, b: u32, c: u32) -> u32 {
        ((a << 16) + (b << 8))
            + match c {
                0..256 => c,
                _ => 255,
            }
    }
}

// EXTERN_FN

missing! {
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum timezone {}

    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum DIR {}

    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum FILE {}

    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum fpos_t {} // FIXME(unix): fill this out with a struct

    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum fpos64_t {} // FIXME(linux): fill this out with a struct
}

extern "C" {
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn isblank(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    pub fn qsort(
        base: *mut c_void,
        num: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    pub fn bsearch(
        key: *const c_void,
        base: *const c_void,
        num: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    pub fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn freopen(filename: *const c_char, mode: *const c_char, file: *mut FILE) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int, size: size_t) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t, stream: *mut FILE) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn clearerr(stream: *mut FILE);
    pub fn perror(s: *const c_char);
    pub fn atof(s: *const c_char) -> c_double;
    pub fn atoi(s: *const c_char) -> c_int;
    pub fn atol(s: *const c_char) -> c_long;
    pub fn atoll(s: *const c_char) -> c_longlong;
    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float;
    pub fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long;
    pub fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong;
    pub fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong;
    pub fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    pub fn system(s: *const c_char) -> c_int;
    pub fn getenv(s: *const c_char) -> *mut c_char;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn stpcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn stpncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(s: *mut c_char, ct: *const c_char, n: size_t) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strndup(cs: *const c_char, n: size_t) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    pub fn strncasecmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    pub fn strcasestr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strtok_r(s: *mut c_char, t: *const c_char, p: *mut *mut c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;
    pub fn strsignal(sig: c_int) -> *mut c_char;
    pub fn wcslen(buf: *const wchar_t) -> size_t;
    pub fn wcstombs(dest: *mut c_char, src: *const wchar_t, n: size_t) -> size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn wmemchr(cx: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memccpy(dest: *mut c_void, src: *const c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn abs(i: c_int) -> c_int;
    pub fn labs(i: c_long) -> c_long;
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);

    pub fn drand48() -> c_double;
    pub fn erand48(xseed: *mut c_ushort) -> c_double;
    pub fn lrand48() -> c_long;
    pub fn nrand48(xseed: *mut c_ushort) -> c_long;
    pub fn jrand48(xseed: *mut c_ushort) -> c_long;
    pub fn srand48(seed: c_long);

    pub fn getpwnam(name: *const c_char) -> *mut passwd;
    pub fn getpwuid(uid: uid_t) -> *mut passwd;

    pub fn fprintf(stream: *mut crate::FILE, format: *const c_char, ...) -> c_int;
    pub fn printf(format: *const c_char, ...) -> c_int;
    pub fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    pub fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    pub fn fscanf(stream: *mut crate::FILE, format: *const c_char, ...) -> c_int;
    pub fn scanf(format: *const c_char, ...) -> c_int;
    pub fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    pub fn getchar_unlocked() -> c_int;
    pub fn putchar_unlocked(c: c_int) -> c_int;

    pub fn socket(domain: c_int, ty: c_int, protocol: c_int) -> c_int;
    pub fn connect(socket: c_int, address: *const sockaddr, len: socklen_t) -> c_int;
    pub fn listen(socket: c_int, backlog: c_int) -> c_int;
    pub fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int;
    pub fn getpeername(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t)
        -> c_int;
    pub fn getsockname(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t)
        -> c_int;
    pub fn setsockopt(
        socket: c_int,
        level: c_int,
        name: c_int,
        value: *const c_void,
        option_len: socklen_t,
    ) -> c_int;
    pub fn socketpair(
        domain: c_int,
        type_: c_int,
        protocol: c_int,
        socket_vector: *mut c_int,
    ) -> c_int;
    pub fn sendto(
        socket: c_int,
        buf: *const c_void,
        len: size_t,
        flags: c_int,
        addr: *const sockaddr,
        addrlen: socklen_t,
    ) -> ssize_t;
    pub fn shutdown(socket: c_int, how: c_int) -> c_int;

    pub fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fchmod(fd: c_int, mode: mode_t) -> c_int;

    pub fn fstat(fildes: c_int, buf: *mut stat) -> c_int;

    pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int;

    pub fn stat(path: *const c_char, buf: *mut stat) -> c_int;

    pub fn pclose(stream: *mut crate::FILE) -> c_int;
    pub fn fdopen(fd: c_int, mode: *const c_char) -> *mut crate::FILE;
    pub fn fileno(stream: *mut crate::FILE) -> c_int;

    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn creat(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;

    pub fn opendir(dirname: *const c_char) -> *mut crate::DIR;
    pub fn readdir(dirp: *mut crate::DIR) -> *mut crate::dirent;
    pub fn readdir_r(
        dirp: *mut crate::DIR,
        entry: *mut crate::dirent,
        result: *mut *mut crate::dirent,
    ) -> c_int;
    pub fn closedir(dirp: *mut crate::DIR) -> c_int;
    pub fn rewinddir(dirp: *mut crate::DIR);

    pub fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    pub fn fchmodat(
        dirfd: c_int,
        pathname: *const c_char,
        mode: crate::mode_t,
        flags: c_int,
    ) -> c_int;
    pub fn lockf(fd: c_int, cmd: c_int, len: off_t) -> c_int;
    pub fn fchown(fd: c_int, owner: uid_t, group: uid_t) -> c_int;

    pub fn access(path: *const c_char, amode: c_int) -> c_int;
    pub fn alarm(seconds: c_uint) -> c_uint;
    pub fn chdir(dir: *const c_char) -> c_int;
    pub fn fchdir(dirfd: c_int) -> c_int;
    pub fn chown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int;
    pub fn lchown(path: *const c_char, uid: uid_t, gid: gid_t) -> c_int;
    pub fn close(fd: c_int) -> c_int;
    pub fn dup(fd: c_int) -> c_int;
    pub fn dup2(src: c_int, dst: c_int) -> c_int;

    pub fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;
    pub fn execle(path: *const c_char, arg0: *const c_char, ...) -> c_int;
    pub fn execlp(file: *const c_char, arg0: *const c_char, ...) -> c_int;

    // DIFF(main): changed to `*const *mut` in e77f551de9
    pub fn execv(prog: *const c_char, argv: *const *const c_char) -> c_int;
    pub fn execve(
        prog: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn execvp(c: *const c_char, argv: *const *const c_char) -> c_int;

    pub fn fork() -> pid_t;
    pub fn fpathconf(filedes: c_int, name: c_int) -> c_long;
    pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    pub fn getegid() -> gid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn getgroups(ngroups_max: c_int, groups: *mut gid_t) -> c_int;
    pub fn getlogin() -> *mut c_char;
    pub fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int;
    pub fn getpgid(pid: pid_t) -> pid_t;
    pub fn getpgrp() -> pid_t;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
    pub fn getuid() -> uid_t;
    pub fn isatty(fd: c_int) -> c_int;
    pub fn link(src: *const c_char, dst: *const c_char) -> c_int;
    pub fn lseek(fd: c_int, offset: off_t, whence: c_int) -> off_t;
    pub fn pathconf(path: *const c_char, name: c_int) -> c_long;
    pub fn pipe(fds: *mut c_int) -> c_int;
    pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
    pub fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void;
    pub fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    pub fn rmdir(path: *const c_char) -> c_int;
    pub fn seteuid(uid: uid_t) -> c_int;
    pub fn setegid(gid: gid_t) -> c_int;
    pub fn setgid(gid: gid_t) -> c_int;
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int;
    pub fn setsid() -> pid_t;
    pub fn setuid(uid: uid_t) -> c_int;
    pub fn sleep(secs: c_uint) -> c_uint;
    pub fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int;
    pub fn tcgetpgrp(fd: c_int) -> pid_t;
    pub fn tcsetpgrp(fd: c_int, pgrp: crate::pid_t) -> c_int;
    pub fn ttyname(fd: c_int) -> *mut c_char;
    pub fn ttyname_r(fd: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    pub fn unlink(c: *const c_char) -> c_int;
    pub fn wait(status: *mut c_int) -> pid_t;
    pub fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    pub fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    pub fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn pwrite(fd: c_int, buf: *const c_void, count: size_t, offset: off_t) -> ssize_t;
    pub fn umask(mask: mode_t) -> mode_t;

    pub fn utime(file: *const c_char, buf: *const utimbuf) -> c_int;

    pub fn kill(pid: pid_t, sig: c_int) -> c_int;
    pub fn killpg(pgrp: pid_t, sig: c_int) -> c_int;

    pub fn mlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn munlock(addr: *const c_void, len: size_t) -> c_int;
    pub fn mlockall(flags: c_int) -> c_int;
    pub fn munlockall() -> c_int;

    pub fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;
    pub fn munmap(addr: *mut c_void, len: size_t) -> c_int;

    pub fn if_nametoindex(ifname: *const c_char) -> c_uint;
    pub fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;

    pub fn lstat(path: *const c_char, buf: *mut stat) -> c_int;

    pub fn fsync(fd: c_int) -> c_int;

    pub fn setenv(name: *const c_char, val: *const c_char, overwrite: c_int) -> c_int;
    pub fn unsetenv(name: *const c_char) -> c_int;

    pub fn symlink(path1: *const c_char, path2: *const c_char) -> c_int;

    pub fn truncate(path: *const c_char, length: off_t) -> c_int;
    pub fn ftruncate(fd: c_int, length: off_t) -> c_int;

    pub fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;

    pub fn getrusage(resource: c_int, usage: *mut rusage) -> c_int;

    pub fn realpath(pathname: *const c_char, resolved: *mut c_char) -> *mut c_char;

    pub fn flock(fd: c_int, operation: c_int) -> c_int;

    pub fn times(buf: *mut crate::tms) -> crate::clock_t;

    pub fn pthread_self() -> crate::pthread_t;
    pub fn pthread_equal(t1: crate::pthread_t, t2: crate::pthread_t) -> c_int;
    pub fn pthread_join(native: crate::pthread_t, value: *mut *mut c_void) -> c_int;
    pub fn pthread_exit(value: *mut c_void) -> !;
    pub fn pthread_attr_init(attr: *mut crate::pthread_attr_t) -> c_int;
    pub fn pthread_attr_destroy(attr: *mut crate::pthread_attr_t) -> c_int;
    pub fn pthread_attr_getstacksize(
        attr: *const crate::pthread_attr_t,
        stacksize: *mut size_t,
    ) -> c_int;
    pub fn pthread_attr_setstacksize(attr: *mut crate::pthread_attr_t, stack_size: size_t)
        -> c_int;
    pub fn pthread_attr_setdetachstate(attr: *mut crate::pthread_attr_t, state: c_int) -> c_int;
    pub fn pthread_detach(thread: crate::pthread_t) -> c_int;
    pub fn sched_yield() -> c_int;
    pub fn pthread_key_create(
        key: *mut pthread_key_t,
        dtor: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
    pub fn pthread_key_delete(key: pthread_key_t) -> c_int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut c_void;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const c_void) -> c_int;
    pub fn pthread_mutex_init(
        lock: *mut pthread_mutex_t,
        attr: *const pthread_mutexattr_t,
    ) -> c_int;
    pub fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_trylock(lock: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> c_int;

    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, _type: c_int) -> c_int;

    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> c_int;
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, lock: *mut pthread_mutex_t) -> c_int;
    pub fn pthread_cond_timedwait(
        cond: *mut pthread_cond_t,
        lock: *mut pthread_mutex_t,
        abstime: *const crate::timespec,
    ) -> c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> c_int;
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> c_int;
    pub fn pthread_rwlock_init(
        lock: *mut pthread_rwlock_t,
        attr: *const pthread_rwlockattr_t,
    ) -> c_int;
    pub fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_tryrdlock(lock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_trywrlock(lock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> c_int;
    pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> c_int;
    pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> c_int;
    #[cfg_attr(not(target_env = "musl"), link_name = "__xpg_strerror_r")]
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;

    pub fn getsockopt(
        sockfd: c_int,
        level: c_int,
        optname: c_int,
        optval: *mut c_void,
        optlen: *mut crate::socklen_t,
    ) -> c_int;
    pub fn raise(signum: c_int) -> c_int;
    pub fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;

    pub fn utimes(filename: *const c_char, times: *const crate::timeval) -> c_int;
    pub fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void;
    pub fn dlerror() -> *mut c_char;
    pub fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    pub fn dlclose(handle: *mut c_void) -> c_int;
    pub fn dladdr(addr: *const c_void, info: *mut Dl_info) -> c_int;

    pub fn getaddrinfo(
        node: *const c_char,
        service: *const c_char,
        hints: *const addrinfo,
        res: *mut *mut addrinfo,
    ) -> c_int;
    pub fn freeaddrinfo(res: *mut addrinfo);
    pub fn hstrerror(errcode: c_int) -> *const c_char;
    pub fn gai_strerror(errcode: c_int) -> *const c_char;

    pub fn gmtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
    pub fn localtime_r(time_p: *const time_t, result: *mut tm) -> *mut tm;
    pub fn mktime(tm: *mut tm) -> time_t;
    pub fn time(time: *mut time_t) -> time_t;
    pub fn gmtime(time_p: *const time_t) -> *mut tm;
    pub fn localtime(time_p: *const time_t) -> *mut tm;
    pub fn difftime(time1: time_t, time0: time_t) -> c_double;
    pub fn timegm(tm: *mut crate::tm) -> time_t;

    pub fn mknod(pathname: *const c_char, mode: crate::mode_t, dev: crate::dev_t) -> c_int;
    pub fn uname(buf: *mut crate::utsname) -> c_int;
    pub fn gethostname(name: *mut c_char, len: size_t) -> c_int;
    pub fn endservent();
    pub fn getservbyname(name: *const c_char, proto: *const c_char) -> *mut servent;
    pub fn getservbyport(port: c_int, proto: *const c_char) -> *mut servent;
    pub fn getservent() -> *mut servent;
    pub fn setservent(stayopen: c_int);
    pub fn getprotobyname(name: *const c_char) -> *mut protoent;
    pub fn getprotobynumber(proto: c_int) -> *mut protoent;
    pub fn chroot(name: *const c_char) -> c_int;
    pub fn usleep(secs: c_uint) -> c_int;
    pub fn pause() -> c_int;
    pub fn send(socket: c_int, buf: *const c_void, len: size_t, flags: c_int) -> ssize_t;
    pub fn recv(socket: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> ssize_t;
    pub fn putenv(string: *mut c_char) -> c_int;
    pub fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: c_int) -> c_int;
    pub fn select(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        errorfds: *mut fd_set,
        timeout: *mut timeval,
    ) -> c_int;
    pub fn setlocale(category: c_int, locale: *const c_char) -> *mut c_char;
    pub fn localeconv() -> *mut lconv;

    pub fn sem_destroy(sem: *mut sem_t) -> c_int;
    pub fn sem_wait(sem: *mut sem_t) -> c_int;
    pub fn sem_trywait(sem: *mut sem_t) -> c_int;
    pub fn sem_post(sem: *mut sem_t) -> c_int;
    pub fn sem_init(sem: *mut sem_t, pshared: c_int, value: c_uint) -> c_int;
    pub fn statvfs(path: *const c_char, buf: *mut statvfs) -> c_int;
    pub fn fstatvfs(fd: c_int, buf: *mut statvfs) -> c_int;

    pub fn readlink(path: *const c_char, buf: *mut c_char, bufsz: size_t) -> ssize_t;

    pub fn sigemptyset(set: *mut sigset_t) -> c_int;
    pub fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigfillset(set: *mut sigset_t) -> c_int;
    pub fn sigdelset(set: *mut sigset_t, signum: c_int) -> c_int;
    pub fn sigismember(set: *const sigset_t, signum: c_int) -> c_int;

    pub fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    pub fn sigpending(set: *mut sigset_t) -> c_int;

    pub fn getsid(pid: pid_t) -> pid_t;

    pub fn sysconf(name: c_int) -> c_long;
    pub fn confstr(name: c_int, buf: *mut c_char, len: size_t) -> size_t;

    pub fn mkfifo(path: *const c_char, mode: mode_t) -> c_int;

    pub fn pselect(
        nfds: c_int,
        readfds: *mut fd_set,
        writefds: *mut fd_set,
        errorfds: *mut fd_set,
        timeout: *const timespec,
        sigmask: *const sigset_t,
    ) -> c_int;
    pub fn fseeko(stream: *mut crate::FILE, offset: off_t, whence: c_int) -> c_int;
    pub fn ftello(stream: *mut crate::FILE) -> off_t;
    pub fn tcdrain(fd: c_int) -> c_int;
    pub fn cfgetispeed(termios: *const crate::termios) -> crate::speed_t;
    pub fn cfgetospeed(termios: *const crate::termios) -> crate::speed_t;
    pub fn cfmakeraw(termios: *mut crate::termios);
    pub fn cfsetispeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int;
    pub fn cfsetospeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int;
    pub fn tcgetattr(fd: c_int, termios: *mut crate::termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios: *const crate::termios) -> c_int;
    pub fn tcflow(fd: c_int, action: c_int) -> c_int;
    pub fn tcflush(fd: c_int, action: c_int) -> c_int;
    pub fn tcgetsid(fd: c_int) -> crate::pid_t;
    pub fn tcsendbreak(fd: c_int, duration: c_int) -> c_int;
    pub fn mkstemp(template: *mut c_char) -> c_int;
    pub fn mkdtemp(template: *mut c_char) -> *mut c_char;

    pub fn openlog(ident: *const c_char, logopt: c_int, facility: c_int);
    pub fn closelog();
    pub fn setlogmask(maskpri: c_int) -> c_int;
    pub fn syslog(priority: c_int, message: *const c_char, ...);
    pub fn nice(incr: c_int) -> c_int;

    pub fn fdatasync(fd: c_int) -> c_int;
    pub fn clock_getres(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clock_gettime(clk_id: crate::clockid_t, tp: *mut crate::timespec) -> c_int;
    pub fn clock_settime(clk_id: crate::clockid_t, tp: *const crate::timespec) -> c_int;
    pub fn dirfd(dirp: *mut crate::DIR) -> c_int;

    pub fn pthread_getattr_np(native: crate::pthread_t, attr: *mut crate::pthread_attr_t) -> c_int;
    pub fn pthread_attr_getstack(
        attr: *const crate::pthread_attr_t,
        stackaddr: *mut *mut c_void,
        stacksize: *mut size_t,
    ) -> c_int;
    pub fn memalign(align: size_t, size: size_t) -> *mut c_void;
    pub fn setgroups(ngroups: size_t, ptr: *const uid_t) -> c_int;
    pub fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    pub fn memrchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn posix_fadvise(fd: c_int, offset: off_t, len: off_t, advise: c_int) -> c_int;
    pub fn utimensat(
        dirfd: c_int,
        path: *const c_char,
        times: *const crate::timespec,
        flag: c_int,
    ) -> c_int;
    pub fn freelocale(loc: crate::locale_t);
    pub fn newlocale(mask: c_int, locale: *const c_char, base: crate::locale_t) -> crate::locale_t;
    pub fn uselocale(loc: crate::locale_t) -> crate::locale_t;

    pub fn fdopendir(fd: c_int) -> *mut crate::DIR;

    pub fn clearenv() -> c_int;
    pub fn setreuid(ruid: uid_t, euid: uid_t) -> c_int;
    pub fn setregid(rgid: gid_t, egid: gid_t) -> c_int;
    pub fn execvpe(
        file: *const c_char,
        argv: *const *const c_char,
        envp: *const *const c_char,
    ) -> c_int;
    pub fn fexecve(fd: c_int, argv: *const *const c_char, envp: *const *const c_char) -> c_int;

    pub fn setpwent();
    pub fn endpwent();
    pub fn getpwent() -> *mut passwd;

    // System V IPC
    pub fn shmget(key: crate::key_t, size: size_t, shmflg: c_int) -> c_int;
    pub fn shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
    pub fn shmdt(shmaddr: *const c_void) -> c_int;
    pub fn shmctl(shmid: c_int, cmd: c_int, buf: *mut crate::shmid_ds) -> c_int;

    pub fn mprotect(addr: *mut c_void, len: size_t, prot: c_int) -> c_int;
    pub fn __errno_location() -> *mut c_int;

    pub fn pwritev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn preadv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int, offset: off_t) -> ssize_t;
    pub fn mkostemp(template: *mut c_char, flags: c_int) -> c_int;
    pub fn nl_langinfo_l(item: crate::nl_item, locale: crate::locale_t) -> *mut c_char;
    pub fn getnameinfo(
        sa: *const crate::sockaddr,
        salen: crate::socklen_t,
        host: *mut c_char,
        hostlen: crate::socklen_t,
        serv: *mut c_char,
        servlen: crate::socklen_t,
        flags: c_int,
    ) -> c_int;

    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);

    pub fn glob(
        pattern: *const c_char,
        flags: c_int,
        errfunc: Option<extern "C" fn(epath: *const c_char, errno: c_int) -> c_int>,
        pglob: *mut crate::glob_t,
    ) -> c_int;
    pub fn globfree(pglob: *mut crate::glob_t);

    pub fn seekdir(dirp: *mut crate::DIR, loc: c_long);

    pub fn telldir(dirp: *mut crate::DIR) -> c_long;
    pub fn madvise(addr: *mut c_void, len: size_t, advice: c_int) -> c_int;

    pub fn msync(addr: *mut c_void, len: size_t, flags: c_int) -> c_int;

    pub fn recvfrom(
        socket: c_int,
        buf: *mut c_void,
        len: size_t,
        flags: c_int,
        addr: *mut crate::sockaddr,
        addrlen: *mut crate::socklen_t,
    ) -> ssize_t;
    pub fn nl_langinfo(item: crate::nl_item) -> *mut c_char;

    pub fn bind(
        socket: c_int,
        address: *const crate::sockaddr,
        address_len: crate::socklen_t,
    ) -> c_int;

    pub fn writev(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;
    pub fn readv(fd: c_int, iov: *const crate::iovec, iovcnt: c_int) -> ssize_t;

    pub fn sendmsg(fd: c_int, msg: *const crate::msghdr, flags: c_int) -> ssize_t;
    pub fn recvmsg(fd: c_int, msg: *mut crate::msghdr, flags: c_int) -> ssize_t;
    pub fn sched_getaffinity(
        pid: crate::pid_t,
        cpusetsize: size_t,
        cpuset: *mut cpu_set_t,
    ) -> c_int;
    pub fn sched_get_priority_max(policy: c_int) -> c_int;
    pub fn settimeofday(tv: *const crate::timeval, tz: *const crate::timezone) -> c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const crate::timespec) -> c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut c_int) -> c_int;
    pub fn mount(
        src: *const c_char,
        target: *const c_char,
        fstype: *const c_char,
        flags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    pub fn ppoll(
        fds: *mut crate::pollfd,
        nfds: nfds_t,
        timeout: *const crate::timespec,
        sigmask: *const sigset_t,
    ) -> c_int;
    pub fn pthread_getschedparam(
        native: crate::pthread_t,
        policy: *mut c_int,
        param: *mut crate::sched_param,
    ) -> c_int;
    pub fn prctl(option: c_int, ...) -> c_int;
    #[cfg(not(target_env = "uclibc"))]
    pub fn pthread_mutexattr_getprotocol(
        attr: *const pthread_mutexattr_t,
        protocol: *mut c_int,
    ) -> c_int;
    #[cfg(not(target_env = "uclibc"))]
    pub fn pthread_mutexattr_setprotocol(attr: *mut pthread_mutexattr_t, protocol: c_int) -> c_int;

    pub fn pthread_mutex_timedlock(
        lock: *mut pthread_mutex_t,
        abstime: *const crate::timespec,
    ) -> c_int;
    pub fn pthread_barrierattr_init(attr: *mut crate::pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_destroy(attr: *mut crate::pthread_barrierattr_t) -> c_int;
    pub fn pthread_barrierattr_setpshared(
        attr: *mut crate::pthread_barrierattr_t,
        shared: c_int,
    ) -> c_int;
    pub fn pthread_barrier_init(
        barrier: *mut pthread_barrier_t,
        attr: *const crate::pthread_barrierattr_t,
        count: c_uint,
    ) -> c_int;

    pub fn pthread_barrier_destroy(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_barrier_wait(barrier: *mut pthread_barrier_t) -> c_int;
    pub fn pthread_spin_init(lock: *mut crate::pthread_spinlock_t, pshared: c_int) -> c_int;
    pub fn pthread_spin_destroy(lock: *mut crate::pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_lock(lock: *mut crate::pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_trylock(lock: *mut crate::pthread_spinlock_t) -> c_int;
    pub fn pthread_spin_unlock(lock: *mut crate::pthread_spinlock_t) -> c_int;
    pub fn pthread_attr_getguardsize(
        attr: *const crate::pthread_attr_t,
        guardsize: *mut size_t,
    ) -> c_int;
    pub fn pthread_attr_setguardsize(attr: *mut crate::pthread_attr_t, guardsize: size_t) -> c_int;
    pub fn sethostname(name: *const c_char, len: size_t) -> c_int;
    pub fn sched_get_priority_min(policy: c_int) -> c_int;
    pub fn sigsuspend(mask: *const crate::sigset_t) -> c_int;
    pub fn getgrgid_r(
        gid: uid_t,
        grp: *mut crate::group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut crate::group,
    ) -> c_int;
    pub fn sem_close(sem: *mut sem_t) -> c_int;
    pub fn getgrnam_r(
        name: *const c_char,
        grp: *mut crate::group,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut crate::group,
    ) -> c_int;
    pub fn initgroups(user: *const c_char, group: uid_t) -> c_int;
    pub fn pthread_sigmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    pub fn sem_open(name: *const c_char, oflag: c_int, ...) -> *mut sem_t;
    pub fn getgrnam(name: *const c_char) -> *mut crate::group;
    pub fn pthread_cancel(thread: crate::pthread_t) -> c_int;
    pub fn pthread_kill(thread: crate::pthread_t, sig: c_int) -> c_int;
    pub fn sem_unlink(name: *const c_char) -> c_int;
    pub fn daemon(nochdir: c_int, noclose: c_int) -> c_int;
    pub fn getpwnam_r(
        name: *const c_char,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn getpwuid_r(
        uid: uid_t,
        pwd: *mut passwd,
        buf: *mut c_char,
        buflen: size_t,
        result: *mut *mut passwd,
    ) -> c_int;
    pub fn sigwait(set: *const sigset_t, sig: *mut c_int) -> c_int;
    pub fn getgrgid(gid: uid_t) -> *mut crate::group;

    pub fn setgrent();
    pub fn endgrent();
    pub fn getgrent() -> *mut crate::group;

    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut crate::FILE;
    pub fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    pub fn pthread_create(
        native: *mut crate::pthread_t,
        attr: *const crate::pthread_attr_t,
        f: extern "C" fn(*mut c_void) -> *mut c_void,
        value: *mut c_void,
    ) -> c_int;
    pub fn dl_iterate_phdr(
        callback: Option<
            unsafe extern "C" fn(
                info: *mut crate::dl_phdr_info,
                size: size_t,
                data: *mut c_void,
            ) -> c_int,
        >,
        data: *mut c_void,
    ) -> c_int;
    pub fn pthread_attr_getinheritsched(
        attr: *const crate::pthread_attr_t,
        inheritsched: *mut c_int,
    ) -> c_int;
    pub fn pthread_attr_setinheritsched(
        attr: *mut crate::pthread_attr_t,
        inheritsched: c_int,
    ) -> c_int;
    pub fn pthread_attr_getschedpolicy(
        attr: *const crate::pthread_attr_t,
        policy: *mut c_int,
    ) -> c_int;
    pub fn pthread_attr_setschedpolicy(attr: *mut crate::pthread_attr_t, policy: c_int) -> c_int;
    pub fn pthread_attr_getschedparam(
        attr: *const crate::pthread_attr_t,
        param: *mut crate::sched_param,
    ) -> c_int;
    pub fn pthread_attr_setschedparam(
        attr: *mut crate::pthread_attr_t,
        param: *const crate::sched_param,
    ) -> c_int;

    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: c_int) -> c_int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: c_int) -> c_int;
    pub fn pthread_rwlockattr_getpshared(
        attr: *const pthread_rwlockattr_t,
        val: *mut c_int,
    ) -> c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, val: c_int) -> c_int;

    pub fn pthread_getaffinity_np(
        thread: crate::pthread_t,
        cpusetsize: size_t,
        cpuset: *mut crate::cpu_set_t,
    ) -> c_int;
    pub fn pthread_setaffinity_np(
        thread: crate::pthread_t,
        cpusetsize: size_t,
        cpuset: *const crate::cpu_set_t,
    ) -> c_int;
    #[cfg(not(target_env = "uclibc"))]
    pub fn pthread_setschedprio(native: crate::pthread_t, priority: c_int) -> c_int;

    // Not available now on Android
    pub fn mremap(
        addr: *mut c_void,
        len: size_t,
        new_len: size_t,
        flags: c_int,
        ...
    ) -> *mut c_void;

    pub fn pthread_condattr_getpshared(
        attr: *const pthread_condattr_t,
        pshared: *mut c_int,
    ) -> c_int;
    pub fn sysinfo(info: *mut crate::sysinfo) -> c_int;
    pub fn pthread_setschedparam(
        native: crate::pthread_t,
        policy: c_int,
        param: *const crate::sched_param,
    ) -> c_int;
    pub fn pthread_mutexattr_getpshared(
        attr: *const pthread_mutexattr_t,
        pshared: *mut c_int,
    ) -> c_int;

    pub fn setmntent(filename: *const c_char, ty: *const c_char) -> *mut crate::FILE;
    pub fn getmntent(stream: *mut crate::FILE) -> *mut crate::mntent;
    pub fn addmntent(stream: *mut crate::FILE, mnt: *const crate::mntent) -> c_int;
    pub fn endmntent(streamp: *mut crate::FILE) -> c_int;
    pub fn hasmntopt(mnt: *const crate::mntent, opt: *const c_char) -> *mut c_char;

    pub fn fnmatch(pattern: *const c_char, name: *const c_char, flags: c_int) -> c_int;

    pub fn __libc_current_sigrtmax() -> c_int;
    pub fn __libc_current_sigrtmin() -> c_int;

    pub fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;

    pub fn strftime(
        s: *mut c_char,
        max: size_t,
        format: *const c_char,
        tm: *const crate::tm,
    ) -> size_t;
    pub fn strftime_l(
        s: *mut c_char,
        max: size_t,
        format: *const c_char,
        tm: *const crate::tm,
        locale: crate::locale_t,
    ) -> size_t;
    pub fn strptime(s: *const c_char, format: *const c_char, tm: *mut crate::tm) -> *mut c_char;

    pub fn getspnam(name: *const c_char) -> *mut spwd;

    pub fn regcomp(preg: *mut crate::regex_t, pattern: *const c_char, cflags: c_int) -> c_int;

    pub fn regexec(
        preg: *const crate::regex_t,
        input: *const c_char,
        nmatch: size_t,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;

    pub fn regerror(
        errcode: c_int,
        preg: *const crate::regex_t,
        errbuf: *mut c_char,
        errbuf_size: size_t,
    ) -> size_t;

    pub fn regfree(preg: *mut crate::regex_t);

    pub fn iconv_open(tocode: *const c_char, fromcode: *const c_char) -> iconv_t;
    pub fn iconv(
        cd: iconv_t,
        inbuf: *mut *mut c_char,
        inbytesleft: *mut size_t,
        outbuf: *mut *mut c_char,
        outbytesleft: *mut size_t,
    ) -> size_t;
    pub fn iconv_close(cd: iconv_t) -> c_int;

    pub fn gettid() -> crate::pid_t;

    pub fn timer_create(
        clockid: crate::clockid_t,
        sevp: *mut crate::sigevent,
        timerid: *mut crate::timer_t,
    ) -> c_int;
    pub fn timer_delete(timerid: crate::timer_t) -> c_int;
    pub fn timer_gettime(timerid: crate::timer_t, curr_value: *mut crate::itimerspec) -> c_int;
    pub fn timer_settime(
        timerid: crate::timer_t,
        flags: c_int,
        new_value: *const crate::itimerspec,
        old_value: *mut crate::itimerspec,
    ) -> c_int;

    pub fn memmem(
        haystack: *const c_void,
        haystacklen: size_t,
        needle: *const c_void,
        needlelen: size_t,
    ) -> *mut c_void;
    pub fn sched_getcpu() -> c_int;

    pub fn getopt_long(
        argc: c_int,
        argv: *const *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;

    pub fn pthread_once(control: *mut pthread_once_t, routine: extern "C" fn()) -> c_int;

    #[cfg(not(target_env = "uclibc"))]
    pub fn copy_file_range(
        fd_in: c_int,
        off_in: *mut off64_t,
        fd_out: c_int,
        off_out: *mut off64_t,
        len: size_t,
        flags: c_uint,
    ) -> ssize_t;

    pub static in6addr_loopback: in6_addr;

    pub static in6addr_any: in6_addr;
}

#[cfg(not(target_env = "musl"))]
extern "C" {
    pub fn statvfs64(path: *const c_char, buf: *mut statvfs64) -> c_int;
    pub fn fstatvfs64(fd: c_int, buf: *mut statvfs64) -> c_int;
    pub fn statfs64(path: *const c_char, buf: *mut statfs64) -> c_int;
    pub fn creat64(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fstat64(fildes: c_int, buf: *mut stat64) -> c_int;
    pub fn ftruncate64(fd: c_int, length: off64_t) -> c_int;
    pub fn lseek64(fd: c_int, offset: off64_t, whence: c_int) -> off64_t;
    pub fn lstat64(path: *const c_char, buf: *mut stat64) -> c_int;
    pub fn mmap64(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off64_t,
    ) -> *mut c_void;
    pub fn open64(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn openat64(fd: c_int, path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn posix_fadvise64(fd: c_int, offset: off64_t, len: off64_t, advise: c_int) -> c_int;
    pub fn pread64(fd: c_int, buf: *mut c_void, count: size_t, offset: off64_t) -> ssize_t;
    pub fn pwrite64(fd: c_int, buf: *const c_void, count: size_t, offset: off64_t) -> ssize_t;
    pub fn readdir64(dirp: *mut crate::DIR) -> *mut crate::dirent64;
    pub fn readdir64_r(
        dirp: *mut crate::DIR,
        entry: *mut crate::dirent64,
        result: *mut *mut crate::dirent64,
    ) -> c_int;
    pub fn stat64(path: *const c_char, buf: *mut stat64) -> c_int;
    pub fn truncate64(path: *const c_char, length: off64_t) -> c_int;

    pub fn fgetpos64(stream: *mut crate::FILE, ptr: *mut fpos64_t) -> c_int;
    pub fn fopen64(filename: *const c_char, mode: *const c_char) -> *mut crate::FILE;
    pub fn freopen64(
        filename: *const c_char,
        mode: *const c_char,
        file: *mut crate::FILE,
    ) -> *mut crate::FILE;
    pub fn fseeko64(stream: *mut crate::FILE, offset: off64_t, whence: c_int) -> c_int;
    pub fn fsetpos64(stream: *mut crate::FILE, ptr: *const fpos64_t) -> c_int;
    pub fn ftello64(stream: *mut crate::FILE) -> off64_t;
    pub fn tmpfile64() -> *mut crate::FILE;
}

#[cfg(not(target_env = "uclibc"))]
extern "C" {
    // uclibc has separate non-const version of this function
    pub fn forkpty(
        amaster: *mut c_int,
        name: *mut c_char,
        termp: *const termios,
        winp: *const crate::winsize,
    ) -> crate::pid_t;
    // uclibc has separate non-const version of this function
    pub fn openpty(
        amaster: *mut c_int,
        aslave: *mut c_int,
        name: *mut c_char,
        termp: *const termios,
        winp: *const crate::winsize,
    ) -> c_int;

    pub fn aio_read(aiocbp: *mut aiocb) -> c_int;
    pub fn aio_write(aiocbp: *mut aiocb) -> c_int;
    pub fn aio_fsync(op: c_int, aiocbp: *mut aiocb) -> c_int;
    pub fn aio_error(aiocbp: *const aiocb) -> c_int;
    pub fn aio_return(aiocbp: *mut aiocb) -> ssize_t;
    pub fn aio_suspend(
        aiocb_list: *const *const aiocb,
        nitems: c_int,
        timeout: *const crate::timespec,
    ) -> c_int;
    pub fn aio_cancel(fd: c_int, aiocbp: *mut aiocb) -> c_int;
    pub fn lio_listio(
        mode: c_int,
        aiocb_list: *const *mut aiocb,
        nitems: c_int,
        sevp: *mut crate::sigevent,
    ) -> c_int;

    pub fn getloadavg(loadavg: *mut c_double, nelem: c_int) -> c_int;
    pub fn process_vm_readv(
        pid: crate::pid_t,
        local_iov: *const crate::iovec,
        liovcnt: c_ulong,
        remote_iov: *const crate::iovec,
        riovcnt: c_ulong,
        flags: c_ulong,
    ) -> isize;
    pub fn process_vm_writev(
        pid: crate::pid_t,
        local_iov: *const crate::iovec,
        liovcnt: c_ulong,
        remote_iov: *const crate::iovec,
        riovcnt: c_ulong,
        flags: c_ulong,
    ) -> isize;
    pub fn futimes(fd: c_int, times: *const crate::timeval) -> c_int;
}

#[cfg(all(target_env = "musl", any(target_arch = "x86_64", target_arch = "x86")))]
extern "C" {
    pub fn iopl(level: c_int) -> c_int;
    pub fn ioperm(from: c_ulong, num: c_ulong, turn_on: c_int) -> c_int;
}

cfg_if! {
    if #[cfg(target_env = "uclibc")] {
        mod uclibc;
        pub use self::uclibc::*;
    }
}
