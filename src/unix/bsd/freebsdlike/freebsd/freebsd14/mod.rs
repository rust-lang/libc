// APIs in FreeBSD 13 that have changed since 11.

pub type nlink_t = u64;
pub type dev_t = u64;
pub type ino_t = ::c_ulong;
pub type shmatt_t = ::c_uint;
pub type kpaddr_t = u64;
pub type kssize_t = i64;
pub type domainset_t = __c_anonymous_domainset;

s! {
    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_lpid: ::pid_t,
        pub shm_cpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
    }

    pub struct kevent {
        pub ident: ::uintptr_t,
        pub filter: ::c_short,
        pub flags: ::c_ushort,
        pub fflags: ::c_uint,
        pub data: i64,
        pub udata: *mut ::c_void,
        pub ext: [u64; 4],
    }

    pub struct kvm_page {
        pub kp_version: ::u_int,
        pub kp_paddr: ::kpaddr_t,
        pub kp_kmap_vaddr: ::kvaddr_t,
        pub kp_dmap_vaddr: ::kvaddr_t,
        pub kp_prot: ::vm_prot_t,
        pub kp_offset: ::off_t,
        pub kp_len: ::size_t,
    }

    pub struct __c_anonymous_domainset {
        _priv: [::uintptr_t; 4],
    }

    pub struct kinfo_proc {
        /// Size of this structure.
        pub ki_structsize: ::c_int,
        /// Reserved: layout identifier.
        pub ki_layout: ::c_int,
        /// Address of command arguments.
        pub ki_args: *mut ::pargs,
        // This is normally "struct proc".
        /// Address of proc.
        pub ki_paddr: *mut ::c_void,
        // This is normally "struct user".
        /// Kernel virtual address of u-area.
        pub ki_addr: *mut ::c_void,
        // This is normally "struct vnode".
        /// Pointer to trace file.
        pub ki_tracep: *mut ::c_void,
        // This is normally "struct vnode".
        /// Pointer to executable file.
        pub ki_textvp: *mut ::c_void,
        // This is normally "struct filedesc".
        /// Pointer to open file info.
        pub ki_fd: *mut ::c_void,
        // This is normally "struct vmspace".
        /// Pointer to kernel vmspace struct.
        pub ki_vmspace: *mut ::c_void,
        /// Sleep address.
        pub ki_wchan: *const ::c_void,
        /// Process identifier.
        pub ki_pid: ::pid_t,
        /// Parent process ID.
        pub ki_ppid: ::pid_t,
        /// Process group ID.
        pub ki_pgid: ::pid_t,
        /// tty process group ID.
        pub ki_tpgid: ::pid_t,
        /// Process session ID.
        pub ki_sid: ::pid_t,
        /// Terminal session ID.
        pub ki_tsid: ::pid_t,
        /// Job control counter.
        pub ki_jobc: ::c_short,
        /// Unused (just here for alignment).
        pub ki_spare_short1: ::c_short,
        /// Controlling tty dev.
        pub ki_tdev_freebsd11: u32,
        /// Signals arrived but not delivered.
        pub ki_siglist: ::sigset_t,
        /// Current signal mask.
        pub ki_sigmask: ::sigset_t,
        /// Signals being ignored.
        pub ki_sigignore: ::sigset_t,
        /// Signals being caught by user.
        pub ki_sigcatch: ::sigset_t,
        /// Effective user ID.
        pub ki_uid: ::uid_t,
        /// Real user ID.
        pub ki_ruid: ::uid_t,
        /// Saved effective user ID.
        pub ki_svuid: ::uid_t,
        /// Real group ID.
        pub ki_rgid: ::gid_t,
        /// Saved effective group ID.
        pub ki_svgid: ::gid_t,
        /// Number of groups.
        pub ki_ngroups: ::c_short,
        /// Unused (just here for alignment).
        pub ki_spare_short2: ::c_short,
        /// Groups.
        pub ki_groups: [::gid_t; ::KI_NGROUPS],
        /// Virtual size.
        pub ki_size: ::vm_size_t,
        /// Current resident set size in pages.
        pub ki_rssize: ::segsz_t,
        /// Resident set size before last swap.
        pub ki_swrss: ::segsz_t,
        /// Text size (pages) XXX.
        pub ki_tsize: ::segsz_t,
        /// Data size (pages) XXX.
        pub ki_dsize: ::segsz_t,
        /// Stack size (pages).
        pub ki_ssize: ::segsz_t,
        /// Exit status for wait & stop signal.
        pub ki_xstat: ::u_short,
        /// Accounting flags.
        pub ki_acflag: ::u_short,
        /// %cpu for process during `ki_swtime`.
        pub ki_pctcpu: ::fixpt_t,
        /// Time averaged value of `ki_cpticks`.
        pub ki_estcpu: ::u_int,
        /// Time since last blocked.
        pub ki_slptime: ::u_int,
        /// Time swapped in or out.
        pub ki_swtime: ::u_int,
        /// Number of copy-on-write faults.
        pub ki_cow: ::u_int,
        /// Real time in microsec.
        pub ki_runtime: u64,
        /// Starting time.
        pub ki_start: ::timeval,
        /// Time used by process children.
        pub ki_childtime: ::timeval,
        /// P_* flags.
        pub ki_flag: ::c_long,
        /// KI_* flags (below).
        pub ki_kiflag: ::c_long,
        /// Kernel trace points.
        pub ki_traceflag: ::c_int,
        /// S* process status.
        pub ki_stat: ::c_char,
        /// Process "nice" value.
        pub ki_nice: i8, // signed char
        /// Process lock (prevent swap) count.
        pub ki_lock: ::c_char,
        /// Run queue index.
        pub ki_rqindex: ::c_char,
        /// Which cpu we are on.
        pub ki_oncpu_old: ::c_uchar,
        /// Last cpu we were on.
        pub ki_lastcpu_old: ::c_uchar,
        /// Thread name.
        pub ki_tdname: [::c_char; ::TDNAMLEN + 1],
        /// Wchan message.
        pub ki_wmesg: [::c_char; ::WMESGLEN + 1],
        /// Setlogin name.
        pub ki_login: [::c_char; ::LOGNAMELEN + 1],
        /// Lock name.
        pub ki_lockname: [::c_char; ::LOCKNAMELEN + 1],
        /// Command name.
        pub ki_comm: [::c_char; ::COMMLEN + 1],
        /// Emulation name.
        pub ki_emul: [::c_char; ::KI_EMULNAMELEN + 1],
        /// Login class.
        pub ki_loginclass: [::c_char; ::LOGINCLASSLEN + 1],
        /// More thread name.
        pub ki_moretdname: [::c_char; ::MAXCOMLEN - ::TDNAMLEN + 1],
        /// Spare string space.
        pub ki_sparestrings: [[::c_char; 23]; 2], // little hack to allow PartialEq
        /// Spare room for growth.
        pub ki_spareints: [::c_int; ::KI_NSPARE_INT],
        /// Controlling tty dev.
        pub ki_tdev: u64,
        /// Which cpu we are on.
        pub ki_oncpu: ::c_int,
        /// Last cpu we were on.
        pub ki_lastcpu: ::c_int,
        /// PID of tracing process.
        pub ki_tracer: ::c_int,
        /// P2_* flags.
        pub ki_flag2: ::c_int,
        /// Default FIB number.
        pub ki_fibnum: ::c_int,
        /// Credential flags.
        pub ki_cr_flags: ::u_int,
        /// Process jail ID.
        pub ki_jid: ::c_int,
        /// Number of threads in total.
        pub ki_numthreads: ::c_int,
        /// Thread ID.
        pub ki_tid: ::lwpid_t,
        /// Process priority.
        pub ki_pri: ::priority,
        /// Process rusage statistics.
        pub ki_rusage: ::rusage,
        /// rusage of children processes.
        pub ki_rusage_ch: ::rusage,
        // This is normally "struct pcb".
        /// Kernel virtual addr of pcb.
        pub ki_pcb: *mut ::c_void,
        /// Kernel virtual addr of stack.
        pub ki_kstack: *mut ::c_void,
        /// User convenience pointer.
        pub ki_udata: *mut ::c_void,
        // This is normally "struct thread".
        pub ki_tdaddr: *mut ::c_void,
        // This is normally "struct pwddesc".
        /// Pointer to process paths info.
        pub ki_pd: *mut ::c_void,
        pub ki_spareptrs: [*mut ::c_void; ::KI_NSPARE_PTR],
        pub ki_sparelongs: [::c_long; ::KI_NSPARE_LONG],
        /// PS_* flags.
        pub ki_sflag: ::c_long,
        /// kthread flag.
        pub ki_tdflags: ::c_long,
    }

    /// Netlink Message Header
    pub struct nlmsghdr {
	    /// Length of message including header
        pub nlmsg_len: u32,
    	/// Message type identifier
        pub nlmsg_type: u16,
	    /// Flags (NLM_F_)
        pub nlmsg_flags: u16,
    	/// Sequence number
        pub nlmsg_seq: u32,
	    /// Sending process port ID
        pub nlmsg_pid: u32,
    }


    /// Netlink ACK Message
    pub struct nlmsgerr {
        pub error: ::c_int,
        pub msg: nlmsghdr,
    }

    /// Base netlink attribute TLV header
    pub struct nlattr {
        /// Total attribute length
        pub nla_len: u16,
        /// Attribute type
        pub nla_type: u16,
    }
}

s_no_extra_traits! {
    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_off: ::off_t,
        pub d_reclen: u16,
        pub d_type: u8,
        d_pad0: u8,
        pub d_namlen: u16,
        d_pad1: u16,
        pub d_name: [::c_char; 256],
    }

    pub struct statfs {
        pub f_version: u32,
        pub f_type: u32,
        pub f_flags: u64,
        pub f_bsize: u64,
        pub f_iosize: u64,
        pub f_blocks: u64,
        pub f_bfree: u64,
        pub f_bavail: i64,
        pub f_files: u64,
        pub f_ffree: i64,
        pub f_syncwrites: u64,
        pub f_asyncwrites: u64,
        pub f_syncreads: u64,
        pub f_asyncreads: u64,
        f_spare: [u64; 10],
        pub f_namemax: u32,
        pub f_owner: ::uid_t,
        pub f_fsid: ::fsid_t,
        f_charspare: [::c_char; 80],
        pub f_fstypename: [::c_char; 16],
        pub f_mntfromname: [::c_char; 1024],
        pub f_mntonname: [::c_char; 1024],
    }

    pub struct vnstat {
        pub vn_fileid: u64,
        pub vn_size: u64,
        pub vn_dev: u64,
        pub vn_fsid: u64,
        pub vn_mntdir: *mut ::c_char,
        pub vn_type: ::c_int,
        pub vn_mode: u16,
        pub vn_devname: [::c_char; ::SPECNAMELEN as usize + 1],
    }

    pub struct sockaddr_nl {
        /// sizeof (sockaddr_nl)
        pub nl_len: u8,
        /// netlink family
        pub nl_family: ::sa_family_t,
        /// reserved, set to 0
        nl_pad: ::c_ushort,
        /// desired port ID, 0 for auto-select
        pub nl_pid: u32,
        /// multicast groups mask to bind to
        pub nl_groups: u32
    }
}

e! {
    #[repr(u32)]
    pub enum nlmsgerr_attrs {
        /// string, error message
        NLMSGERR_ATTR_MSG = 1,
        /// u32, offset of the invalid attr from nl header
        NLMSGERR_ATTR_OFFS	= 2,
        /// binary, data to pass to userland
        NLMSGERR_ATTR_COOKIE = 3,
        // not supported
        NLMSGERR_ATTR_POLICY = 4,
    }
}

cfg_if! {
    if #[cfg(feature = "extra_traits")] {
        impl PartialEq for statfs {
            fn eq(&self, other: &statfs) -> bool {
                self.f_version == other.f_version
                    && self.f_type == other.f_type
                    && self.f_flags == other.f_flags
                    && self.f_bsize == other.f_bsize
                    && self.f_iosize == other.f_iosize
                    && self.f_blocks == other.f_blocks
                    && self.f_bfree == other.f_bfree
                    && self.f_bavail == other.f_bavail
                    && self.f_files == other.f_files
                    && self.f_ffree == other.f_ffree
                    && self.f_syncwrites == other.f_syncwrites
                    && self.f_asyncwrites == other.f_asyncwrites
                    && self.f_syncreads == other.f_syncreads
                    && self.f_asyncreads == other.f_asyncreads
                    && self.f_namemax == other.f_namemax
                    && self.f_owner == other.f_owner
                    && self.f_fsid == other.f_fsid
                    && self.f_fstypename == other.f_fstypename
                    && self
                    .f_mntfromname
                    .iter()
                    .zip(other.f_mntfromname.iter())
                    .all(|(a,b)| a == b)
                    && self
                    .f_mntonname
                    .iter()
                    .zip(other.f_mntonname.iter())
                    .all(|(a,b)| a == b)
            }
        }
        impl Eq for statfs {}
        impl ::fmt::Debug for statfs {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("statfs")
                    .field("f_bsize", &self.f_bsize)
                    .field("f_iosize", &self.f_iosize)
                    .field("f_blocks", &self.f_blocks)
                    .field("f_bfree", &self.f_bfree)
                    .field("f_bavail", &self.f_bavail)
                    .field("f_files", &self.f_files)
                    .field("f_ffree", &self.f_ffree)
                    .field("f_syncwrites", &self.f_syncwrites)
                    .field("f_asyncwrites", &self.f_asyncwrites)
                    .field("f_syncreads", &self.f_syncreads)
                    .field("f_asyncreads", &self.f_asyncreads)
                    .field("f_namemax", &self.f_namemax)
                    .field("f_owner", &self.f_owner)
                    .field("f_fsid", &self.f_fsid)
                    .field("f_fstypename", &self.f_fstypename)
                    .field("f_mntfromname", &&self.f_mntfromname[..])
                    .field("f_mntonname", &&self.f_mntonname[..])
                    .finish()
            }
        }
        impl ::hash::Hash for statfs {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.f_version.hash(state);
                self.f_type.hash(state);
                self.f_flags.hash(state);
                self.f_bsize.hash(state);
                self.f_iosize.hash(state);
                self.f_blocks.hash(state);
                self.f_bfree.hash(state);
                self.f_bavail.hash(state);
                self.f_files.hash(state);
                self.f_ffree.hash(state);
                self.f_syncwrites.hash(state);
                self.f_asyncwrites.hash(state);
                self.f_syncreads.hash(state);
                self.f_asyncreads.hash(state);
                self.f_namemax.hash(state);
                self.f_owner.hash(state);
                self.f_fsid.hash(state);
                self.f_charspare.hash(state);
                self.f_fstypename.hash(state);
                self.f_mntfromname.hash(state);
                self.f_mntonname.hash(state);
            }
        }

        impl PartialEq for dirent {
            fn eq(&self, other: &dirent) -> bool {
                self.d_fileno == other.d_fileno
                    && self.d_off == other.d_off
                    && self.d_reclen == other.d_reclen
                    && self.d_type == other.d_type
                    && self.d_namlen == other.d_namlen
                    && self
                    .d_name[..self.d_namlen as _]
                    .iter()
                    .zip(other.d_name.iter())
                    .all(|(a,b)| a == b)
            }
        }
        impl Eq for dirent {}
        impl ::fmt::Debug for dirent {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                f.debug_struct("dirent")
                    .field("d_fileno", &self.d_fileno)
                    .field("d_off", &self.d_off)
                    .field("d_reclen", &self.d_reclen)
                    .field("d_type", &self.d_type)
                    .field("d_namlen", &self.d_namlen)
                    .field("d_name", &&self.d_name[..self.d_namlen as _])
                    .finish()
            }
        }
        impl ::hash::Hash for dirent {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                self.d_fileno.hash(state);
                self.d_off.hash(state);
                self.d_reclen.hash(state);
                self.d_type.hash(state);
                self.d_namlen.hash(state);
                self.d_name[..self.d_namlen as _].hash(state);
            }
        }

        impl PartialEq for vnstat {
            fn eq(&self, other: &vnstat) -> bool {
                let self_vn_devname: &[::c_char] = &self.vn_devname;
                let other_vn_devname: &[::c_char] = &other.vn_devname;

                self.vn_fileid == other.vn_fileid &&
                self.vn_size == other.vn_size &&
                self.vn_dev == other.vn_dev &&
                self.vn_fsid == other.vn_fsid &&
                self.vn_mntdir == other.vn_mntdir &&
                self.vn_type == other.vn_type &&
                self.vn_mode == other.vn_mode &&
                self_vn_devname == other_vn_devname
            }
        }
        impl Eq for vnstat {}
        impl ::fmt::Debug for vnstat {
            fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
                let self_vn_devname: &[::c_char] = &self.vn_devname;

                f.debug_struct("vnstat")
                    .field("vn_fileid", &self.vn_fileid)
                    .field("vn_size", &self.vn_size)
                    .field("vn_dev", &self.vn_dev)
                    .field("vn_fsid", &self.vn_fsid)
                    .field("vn_mntdir", &self.vn_mntdir)
                    .field("vn_type", &self.vn_type)
                    .field("vn_mode", &self.vn_mode)
                    .field("vn_devname", &self_vn_devname)
                    .finish()
            }
        }
        impl ::hash::Hash for vnstat {
            fn hash<H: ::hash::Hasher>(&self, state: &mut H) {
                let self_vn_devname: &[::c_char] = &self.vn_devname;

                self.vn_fileid.hash(state);
                self.vn_size.hash(state);
                self.vn_dev.hash(state);
                self.vn_fsid.hash(state);
                self.vn_mntdir.hash(state);
                self.vn_type.hash(state);
                self.vn_mode.hash(state);
                self_vn_devname.hash(state);
            }
        }
    }
}
// Netlink
// sys/socket.h
pub const AF_NETLINK: ::c_int = 38;
pub const PF_NETLINK: ::c_int = AF_NETLINK;

pub const SOL_NETLINK: ::c_int = 270;

// RFC 3549, 2.3.2 standard flag bits (nlmsg_flags)

/// Indicateds request to kernel
pub const NLM_F_REQUEST: ::c_int = 1;
/// Message is part of a group terminated by NLMSG_DONE msg
pub const NLM_F_MULTI: ::c_int = 2;
/// Reply with ack message containing resulting error code
pub const NLM_F_ACK: ::c_int = 4;
/// (not supported) Echo this request back
pub const NLM_F_ECHO: ::c_int = 8;
/// Dump was inconsistent due to sequence change
pub const NLM_F_DUMP_INTR: ::c_int = 16;
/// Dump was filtered as requested
pub const NLM_F_DUMP_FILTERED: ::c_int = 32;

// RFC 3549, 2.3.2 Additional flag bits for GET requests
//
/// Return the complete table
pub const NLM_F_ROOT: ::c_int = 0x100;
/// Return all entries matching criteria
pub const NLM_F_MATCH: ::c_int = 0x200;
/// Return an atomic snapshot (ignored)
pub const NLM_F_ATOMIC: ::c_int = 0x400;
pub const NLM_F_DUMP: ::c_int = NLM_F_ROOT | NLM_F_MATCH;

// RFC 3549, 2.3.2 Additional flag bits for NEW requests

/// Replace existing matching config object
pub const NLM_F_REPLACE: ::c_int = 0x100;
/// Don't replace the object if exists
pub const NLM_F_EXCL: ::c_int = 0x200;
/// Create if it does not exist
pub const NLM_F_CREATE: ::c_int = 0x400;
/// Add to end of list
pub const NLM_F_APPEND: ::c_int = 0x800;

// Modifiers to DELETE requests

/// Do not delete recursively
pub const NLM_F_NONREC: ::c_int = 0x100;

// Flags for ACK Message
/// request was capped
pub const NLM_F_CAPPED: ::c_int = 0x100;
/// extended ACK TVLs were included
pub const NLM_F_ACK_TLVS: ::c_int = 0x200;

// RFC 3549, 2.3.2 standard message types (nlmsg_type)

/// Message is ignored
pub const NLMSG_NOOP: ::c_int = 0x1;
/// reply error code reporting
pub const NLMSG_ERROR: ::c_int = 0x2;
/// Message terminates a multipart message
pub const NLMSG_DONE: ::c_int = 0x3;
/// overrun detected, data is lost
pub const NLMSG_OVERRUN: ::c_int = 0x4;
/// < 0x10: reserved control messages
pub const NLMSG_MIN_TYPE: ::c_int = 0x10;

// Defition of numbers assigned to the netlink subsystems.

/// Routing/device hook
pub const NETLINK_ROUTE: ::c_int = 0;
/// not supported
pub const NETLINK_UNUSED: ::c_int = 1;
/// not supported
pub const NETLINK_USERSOCK: ::c_int = 2;
/// not supported
pub const NETLINK_FIREWALL: ::c_int = 3;
/// not supported
pub const NETLINK_SOCK_DIAG: ::c_int = 4;
/// not supported
pub const NETLINK_NFLOG: ::c_int = 5;
/// not supported
pub const NETLINK_XFRM: ::c_int = 6;
/// not supported
pub const NETLINK_SELINUX: ::c_int = 7;
/// not supported
pub const NETLINK_ISCSI: ::c_int = 8;
/// not supported
pub const NETLINK_AUDIT: ::c_int = 9;
/// not supported
pub const NETLINK_FIB_LOOKUP: ::c_int = 10;
/// not supported
pub const NETLINK_CONNECTOR: ::c_int = 11;
/// not supported
pub const NETLINK_NETFILTER: ::c_int = 12;
/// not supported
pub const NETLINK_IP6_FW: ::c_int = 13;
/// not supported
pub const NETLINK_DNRTMSG: ::c_int = 14;
/// not supported
pub const NETLINK_KOBJECT_UEVENT: ::c_int = 15;
/// Generic netlink (dynamic families)
pub const NETLINK_GENERIC: ::c_int = 16;

// Netlink socket operations

/// Subscribe for the specified group notifications
pub const NETLINK_ADD_MEMBERSHIP: ::c_int = 1;
/// Unsubscribe from the specified group
pub const NETLINK_DROP_MEMBERSHIP: ::c_int = 2;
/// XXX: not supported
pub const NETLINK_PKTINFO: ::c_int = 3;
/// XXX: not supported
pub const NETLINK_BROADCAST_ERROR: ::c_int = 4;
/// XXX: not supported
pub const NETLINK_NO_ENOBUFS: ::c_int = 5;
/// XXX: not supported
pub const NETLINK_RX_RING: ::c_int = 6;
/// XXX: not supported
pub const NETLINK_TX_RING: ::c_int = 7;
/// XXX: not supported
pub const NETLINK_LISTEN_ALL_NSID: ::c_int = 8;

pub const NETLINK_LIST_MEMBERSHIPS: ::c_int = 9;
/// Send only original message header in the reply
pub const NETLINK_CAP_ACK: ::c_int = 10;
/// Ack support for receiving additional TLVs in ack
pub const NETLINK_EXT_ACK: ::c_int = 11;
/// Strick header checking
pub const NETLINK_GET_STRICT_CHK: ::c_int = 12;


/// NETLINK_ROUTE message: creates new interface
pub const NL_RTM_NEWLINK: u16      = 16;
/// NETLINK_ROUTE message: deletes matching interface
pub const NL_RTM_DELLINK: u16      = 17;
/// NETLINK_ROUTE message: lists matching interfaces
pub const NL_RTM_GETLINK: u16      = 18;
/// not supported
pub const NL_RTM_SETLINK: u16      = 19;
/// not supported
pub const NL_RTM_NEWADDR: u16      = 20;
/// not supported
pub const NL_RTM_DELADDR: u16      = 21;
/// NETLINK_ROUTE message: lists matching ifaddrs
pub const NL_RTM_GETADDR: u16      = 22;
/// NETLINK_ROUTE message: adds or changes a route
pub const NL_RTM_NEWROUTE: u16     = 24;
/// NETLINK_ROUTE message: deletes matching route
pub const NL_RTM_DELROUTE: u16     = 25;
/// NETLINK_ROUTE message: lists matching routes
pub const NL_RTM_GETROUTE: u16     = 26;
/// NETLINK_ROUTE message: creates new arp/ndp entry
pub const NL_RTM_NEWNEIGH: u16     = 28;
/// NETLINK_ROUTE message: deletes matching arp/ndp entry
pub const NL_RTM_DELNEIGH: u16     = 29;
/// NETLINK_ROUTE message: lists matching arp/ndp entry
pub const NL_RTM_GETNEIGH: u16     = 30;
/// not supported
pub const NL_RTM_NEWRULE: u16      = 32;
/// not supported
pub const NL_RTM_DELRULE: u16      = 33;
/// not supported
pub const NL_RTM_GETRULE: u16      = 34;
/// not supported
pub const NL_RTM_NEWQDISC: u16     = 36;
/// not supported
pub const NL_RTM_DELQDISC: u16     = 37;
/// not supported
pub const NL_RTM_GETQDISC: u16     = 38;
/// not supported
pub const NL_RTM_NEWTCLASS: u16    = 40;
/// not supported
pub const NL_RTM_DELTCLASS: u16    = 41;
/// not supported
pub const NL_RTM_GETTCLASS: u16    = 42;
/// not supported
pub const NL_RTM_NEWTFILTER: u16   = 44;
/// not supported
pub const NL_RTM_DELTFILTER: u16   = 45;
/// not supported
pub const NL_RTM_GETTFILTER: u16   = 46;
/// not supported
pub const NL_RTM_NEWACTION: u16    = 48;
/// not supported
pub const NL_RTM_DELACTION: u16    = 49;
/// not supported
pub const NL_RTM_GETACTION: u16    = 50;
/// not supported
pub const NL_RTM_NEWPREFIX: u16    = 52;
/// not supported
pub const NL_RTM_GETMULTICAST: u16 = 58;
/// not supported
pub const NL_RTM_GETANYCAST: u16   = 62;
/// not supported
pub const NL_RTM_NEWNEIGHTBL: u16  = 64;
/// not supported
pub const NL_RTM_GETNEIGHTBL: u16  = 66;
/// not supported
pub const NL_RTM_SETNEIGHTBL: u16  = 67;
/// not supported
pub const NL_RTM_NEWNDUSEROPT: u16 = 68;
/// not supported
pub const NL_RTM_NEWADDRLABEL: u16 = 72;
/// not supported
pub const NL_RTM_DELADDRLABEL: u16 = 73;
/// not supported
pub const NL_RTM_GETADDRLABEL: u16 = 74;
/// not supported
pub const NL_RTM_GETDCB: u16       = 78;
/// not supported
pub const NL_RTM_SETDCB: u16       = 79;
/// not supported
pub const NL_RTM_NEWNETCONF: u16   = 80;
/// not supported
pub const NL_RTM_GETNETCONF: u16   = 82;
/// not supported
pub const NL_RTM_NEWMDB: u16       = 84;
/// not supported
pub const NL_RTM_DELMDB: u16       = 85;
/// not supported
pub const NL_RTM_GETMDB: u16       = 86;
/// not supported
pub const NL_RTM_NEWNSID: u16      = 88;
/// not supported
pub const NL_RTM_DELNSID: u16      = 89;
/// not supported
pub const NL_RTM_GETNSID: u16      = 90;
/// not supported
pub const NL_RTM_NEWSTATS: u16	   = 92;
/// not supported
pub const NL_RTM_GETSTATS: u16	   = 94;
/// NETLINK_ROUTE message: creates new user nexhtop
pub const NL_RTM_NEWNEXTHOP: u16   = 104;
/// NETLINK_ROUTE message: deletes matching nexthop
pub const NL_RTM_DELNEXTHOP: u16   = 105;
/// NETLINK_ROUTE message: lists created user nexthops
pub const NL_RTM_GETNEXTHOP: u16   = 106;

// userspace compat definitions for RTNLGRP_*
pub const RTMGRP_LINK: ::c_int = 0x00001;
pub const RTMGRP_NOTIFY: ::c_int = 0x00002;
pub const RTMGRP_NEIGH: ::c_int = 0x00004;
pub const RTMGRP_TC: ::c_int = 0x00008;
pub const RTMGRP_IPV4_IFADDR: ::c_int = 0x00010;
pub const RTMGRP_IPV4_MROUTE: ::c_int = 0x00020;
pub const RTMGRP_IPV4_ROUTE: ::c_int = 0x00040;
pub const RTMGRP_IPV4_RULE: ::c_int = 0x00080;
pub const RTMGRP_IPV6_IFADDR: ::c_int = 0x00100;
pub const RTMGRP_IPV6_MROUTE: ::c_int = 0x00200;
pub const RTMGRP_IPV6_ROUTE: ::c_int = 0x00400;
pub const RTMGRP_IPV6_IFINFO: ::c_int = 0x00800;
pub const RTMGRP_DECnet_IFADDR: ::c_int = 0x01000;
pub const RTMGRP_DECnet_ROUTE: ::c_int = 0x04000;
pub const RTMGRP_IPV6_PREFIX: ::c_int = 0x20000;

// enum rtnetlink_groups
pub const RTNLGRP_NONE: ::c_uint = 0x00;
pub const RTNLGRP_LINK: ::c_uint = 0x01;
pub const RTNLGRP_NOTIFY: ::c_uint = 0x02;
pub const RTNLGRP_NEIGH: ::c_uint = 0x03;
pub const RTNLGRP_TC: ::c_uint = 0x04;
pub const RTNLGRP_IPV4_IFADDR: ::c_uint = 0x05;
pub const RTNLGRP_IPV4_MROUTE: ::c_uint = 0x06;
pub const RTNLGRP_IPV4_ROUTE: ::c_uint = 0x07;
pub const RTNLGRP_IPV4_RULE: ::c_uint = 0x08;
pub const RTNLGRP_IPV6_IFADDR: ::c_uint = 0x09;
pub const RTNLGRP_IPV6_MROUTE: ::c_uint = 0x0a;
pub const RTNLGRP_IPV6_ROUTE: ::c_uint = 0x0b;
pub const RTNLGRP_IPV6_IFINFO: ::c_uint = 0x0c;
pub const RTNLGRP_DECnet_IFADDR: ::c_uint = 0x0d;
pub const RTNLGRP_NOP2: ::c_uint = 0x0e;
pub const RTNLGRP_DECnet_ROUTE: ::c_uint = 0x0f;
pub const RTNLGRP_DECnet_RULE: ::c_uint = 0x10;
pub const RTNLGRP_NOP4: ::c_uint = 0x11;
pub const RTNLGRP_IPV6_PREFIX: ::c_uint = 0x12;
pub const RTNLGRP_IPV6_RULE: ::c_uint = 0x13;
pub const RTNLGRP_ND_USEROPT: ::c_uint = 0x14;
pub const RTNLGRP_PHONET_IFADDR: ::c_uint = 0x15;
pub const RTNLGRP_PHONET_ROUTE: ::c_uint = 0x16;
pub const RTNLGRP_DCB: ::c_uint = 0x17;
pub const RTNLGRP_IPV4_NETCONF: ::c_uint = 0x18;
pub const RTNLGRP_IPV6_NETCONF: ::c_uint = 0x19;
pub const RTNLGRP_MDB: ::c_uint = 0x1a;
pub const RTNLGRP_MPLS_ROUTE: ::c_uint = 0x1b;
pub const RTNLGRP_NSID: ::c_uint = 0x1c;
pub const RTNLGRP_MPLS_NETCONF: ::c_uint = 0x1d;
pub const RTNLGRP_IPV4_MROUTE_R: ::c_uint = 0x1e;
pub const RTNLGRP_IPV6_MROUTE_R: ::c_uint = 0x1f;
pub const RTNLGRP_NEXTHOP: ::c_uint = 0x20;
pub const RTNLGRP_BRVLAN: ::c_uint = 0x21;

pub const RAND_MAX: ::c_int = 0x7fff_ffff;
pub const ELAST: ::c_int = 97;

pub const KF_TYPE_EVENTFD: ::c_int = 13;

/// max length of devicename
pub const SPECNAMELEN: ::c_int = 255;
pub const KI_NSPARE_PTR: usize = 5;

/// domainset policies
pub const DOMAINSET_POLICY_INVALID: ::c_int = 0;
pub const DOMAINSET_POLICY_ROUNDROBIN: ::c_int = 1;
pub const DOMAINSET_POLICY_FIRSTTOUCH: ::c_int = 2;
pub const DOMAINSET_POLICY_PREFER: ::c_int = 3;
pub const DOMAINSET_POLICY_INTERLEAVE: ::c_int = 4;

pub const MINCORE_SUPER: ::c_int = 0x60;

safe_f! {
    pub {const} fn makedev(major: ::c_uint, minor: ::c_uint) -> ::dev_t {
        let major = major as ::dev_t;
        let minor = minor as ::dev_t;
        let mut dev = 0;
        dev |= ((major & 0xffffff00) as dev_t) << 32;
        dev |= ((major & 0x000000ff) as dev_t) << 8;
        dev |= ((minor & 0x0000ff00) as dev_t) << 24;
        dev |= ((minor & 0xffff00ff) as dev_t) << 0;
        dev
    }
}

extern "C" {
    pub fn setgrent();
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;
    pub fn freelocale(loc: ::locale_t);
    pub fn msgrcv(
        msqid: ::c_int,
        msgp: *mut ::c_void,
        msgsz: ::size_t,
        msgtyp: ::c_long,
        msgflg: ::c_int,
    ) -> ::ssize_t;
    pub fn clock_nanosleep(
        clk_id: ::clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;

    pub fn eventfd(init: ::c_uint, flags: ::c_int) -> ::c_int;

    pub fn fdatasync(fd: ::c_int) -> ::c_int;

    pub fn getrandom(buf: *mut ::c_void, buflen: ::size_t, flags: ::c_uint) -> ::ssize_t;
    pub fn getentropy(buf: *mut ::c_void, buflen: ::size_t) -> ::c_int;
    pub fn elf_aux_info(aux: ::c_int, buf: *mut ::c_void, buflen: ::c_int) -> ::c_int;
    pub fn setproctitle_fast(fmt: *const ::c_char, ...);
    pub fn timingsafe_bcmp(a: *const ::c_void, b: *const ::c_void, len: ::size_t) -> ::c_int;
    pub fn timingsafe_memcmp(a: *const ::c_void, b: *const ::c_void, len: ::size_t) -> ::c_int;

    pub fn cpuset_getdomain(
        level: ::cpulevel_t,
        which: ::cpuwhich_t,
        id: ::id_t,
        setsize: ::size_t,
        mask: *mut ::domainset_t,
        policy: *mut ::c_int,
    ) -> ::c_int;
    pub fn cpuset_setdomain(
        level: ::cpulevel_t,
        which: ::cpuwhich_t,
        id: ::id_t,
        setsize: ::size_t,
        mask: *const ::domainset_t,
        policy: ::c_int,
    ) -> ::c_int;

    pub fn dirname(path: *mut ::c_char) -> *mut ::c_char;
    pub fn basename(path: *mut ::c_char) -> *mut ::c_char;
}

#[link(name = "kvm")]
extern "C" {
    pub fn kvm_kerndisp(kd: *mut ::kvm_t) -> ::kssize_t;
}

cfg_if! {
    if #[cfg(any(target_arch = "x86_64",
                 target_arch = "aarch64",
                 target_arch = "riscv64"))] {
        mod b64;
        pub use self::b64::*;
    }
}

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    }
}

