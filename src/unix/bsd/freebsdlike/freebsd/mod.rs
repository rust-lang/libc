use unix::iovec;
use unix::bsd::sa_family_t;

pub type fflags_t = u32;
pub type clock_t = i32;
pub type ino_t = u32;
pub type lwpid_t = i32;
pub type nlink_t = u16;
pub type blksize_t = u32;
pub type clockid_t = ::c_int;
pub type sem_t = _sem;

pub type fsblkcnt_t = ::uint64_t;
pub type fsfilcnt_t = ::uint64_t;
pub type idtype_t = ::c_uint;

pub type key_t = ::c_long;
pub type msglen_t = ::c_ulong;
pub type msgqnum_t = ::c_ulong;

pub type sctp_assoc_t = ::uint32_t;

// Defined in netinet/sctp_structs.h
// TODO: Define this struct properly, but we'll need to figure out
//       how to properly represent the `TAILQ_ENTRY(sctp_nets)` field
//       in the struct (which is some kind of linked-list pointer).
pub type sctp_nets = ::c_void;

s! {
    pub struct utmpx {
        pub ut_type: ::c_short,
        pub ut_tv: ::timeval,
        pub ut_id: [::c_char; 8],
        pub ut_pid: ::pid_t,
        pub ut_user: [::c_char; 32],
        pub ut_line: [::c_char; 16],
        pub ut_host: [::c_char; 128],
        pub __ut_spare: [::c_char; 64],
    }

    pub struct aiocb {
        pub aio_fildes: ::c_int,
        pub aio_offset: ::off_t,
        pub aio_buf: *mut ::c_void,
        pub aio_nbytes: ::size_t,
        __unused1: [::c_int; 2],
        __unused2: *mut ::c_void,
        pub aio_lio_opcode: ::c_int,
        pub aio_reqprio: ::c_int,
        // unused 3 through 5 are the __aiocb_private structure
        __unused3: ::c_long,
        __unused4: ::c_long,
        __unused5: *mut ::c_void,
        pub aio_sigevent: sigevent
    }

    pub struct dirent {
        pub d_fileno: u32,
        pub d_reclen: u16,
        pub d_type: u8,
        pub d_namlen: u8,
        pub d_name: [::c_char; 256],
    }

    pub struct jail {
        pub version: u32,
        pub path: *mut ::c_char,
        pub hostname: *mut ::c_char,
        pub jailname: *mut ::c_char,
        pub ip4s: ::c_uint,
        pub ip6s: ::c_uint,
        pub ip4: *mut ::in_addr,
        pub ip6: *mut ::in6_addr,
    }

    pub struct sigevent {
        pub sigev_notify: ::c_int,
        pub sigev_signo: ::c_int,
        pub sigev_value: ::sigval,
        //The rest of the structure is actually a union.  We expose only
        //sigev_notify_thread_id because it's the most useful union member.
        pub sigev_notify_thread_id: ::lwpid_t,
        #[cfg(target_pointer_width = "64")]
        __unused1: ::c_int,
        __unused2: [::c_long; 7]
    }

    pub struct statvfs {
        pub f_bavail: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_blocks: ::fsblkcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_files: ::fsfilcnt_t,
        pub f_bsize: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_fsid: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    // internal structure has changed over time
    pub struct _sem {
        data: [u32; 4],
    }

    pub struct ipc_perm {
        pub cuid: ::uid_t,
        pub cgid: ::gid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
        pub mode: ::mode_t,
        pub seq: ::c_ushort,
        pub key: ::key_t,
    }

    pub struct msqid_ds {
        pub msg_perm: ::ipc_perm,
        __unused1: *mut ::c_void,
        __unused2: *mut ::c_void,
        pub msg_cbytes: ::msglen_t,
        pub msg_qnum: ::msgqnum_t,
        pub msg_qbytes: ::msglen_t,
        pub msg_lspid: ::pid_t,
        pub msg_lrpid: ::pid_t,
        pub msg_stime: ::time_t,
        pub msg_rtime: ::time_t,
        pub msg_ctime: ::time_t,
    }

    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        pub shm_segsz: ::size_t,
        pub shm_lpid: ::pid_t,
        pub shm_cpid: ::pid_t,
        pub shm_nattch: ::c_int,
        pub shm_atime: ::time_t,
        pub shm_dtime: ::time_t,
        pub shm_ctime: ::time_t,
    }

    pub struct xucred {
        pub cr_version: ::c_uint,
        pub cr_uid: ::uid_t,
        pub cr_ngroups: ::c_short,
        pub cr_groups: [::gid_t;16],
        __cr_unused1: *mut ::c_void,
    }

    // SCTP support
    // netinet/sctp.h

    #[repr(packed)]
    pub struct sctphdr {
        /// source port
        pub src_port: ::uint16_t,
        /// destination port
        pub dest_port: ::uint16_t,
        /// verification tag of packet
        pub v_tag: ::uint32_t,
        /// Adler32 checksum
        pub checksum: ::uint32_t,
    }

    /// SCTP chunk header
    #[repr(packed)]
    pub struct sctp_chunkhdr {
        /// chunk type
        pub chunk_type: u8,
        /// chunk flags
        pub chunk_flags: u8,
        /// chunk length
        pub chunk_length: ::uint16_t,
    }

    /// SCTP chunk parameters
    #[repr(packed)]
    pub struct sctp_paramhdr {
        /// parameter type
        pub param_type: ::uint16_t,
        /// parameter length
        pub param_length: ::uint16_t,
    }

    #[repr(packed)]
    pub struct sctp_gen_error_cause {
        pub code: ::uint16_t,
        pub length: ::uint16_t,
        pub info: [::uint8_t; 0]
    }

    #[repr(packed)]
    pub struct sctp_error_cause {
        pub code: ::uint16_t,
        pub length: ::uint16_t,
    }

    #[repr(packed)]
    pub struct sctp_error_invalid_stream {
        /// code=SCTP_ERROR_INVALID_STREAM
        pub cause: sctp_error_cause,
        /// stream id of the DATA in error
        pub stream_id: ::uint16_t,
        pub reserved: ::uint16_t,
    }

    #[repr(packed)]
    pub struct sctp_error_missing_param {
        /// code=SCTP_ERROR_MISSING_PARAM
        pub cause: sctp_error_cause,
        /// number of missing parameters
        pub num_missing_params: ::uint32_t,
        pub type_: [::uint16_t; 0]
    }

    #[repr(packed)]
    pub struct sctp_error_stale_cookie {
        /// code=SCTP_ERROR_STALE_COOKIE
        pub cause: sctp_error_cause,
        /// time in usec of staleness
        pub stale_time: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_error_out_of_resource {
        /// code=SCTP_ERROR_OUT_OF_RESOURCES
        pub cause: sctp_error_cause,
    }

    #[repr(packed)]
    pub struct sctp_error_unresolv_addr {
        /// code=SCTP_ERROR_UNRESOLVABLE_ADDR
        pub cause: sctp_error_cause,
    }

    #[repr(packed)]
    pub struct sctp_error_unrecognized_chunk {
        /// code=SCTP_ERROR_UNRECOG_CHUNK
        pub cause: sctp_error_cause,
        /// header from chunk in error
        pub ch: sctp_chunkhdr,
    }

    #[repr(packed)]
    pub struct sctp_error_no_user_data {
        /// code=SCTP_CAUSE_NO_USER_DATA
        pub cause: sctp_error_cause,
        /// TSN of the empty data chunk
        pub tsn: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_error_auth_invalid_hmac {
        /// code=SCTP_CAUSE_UNSUPPORTED_HMACID
        pub cause: sctp_error_cause,
        pub hmac_id: ::uint16_t,
    }

    pub struct sctp_event {
        pub se_assoc_id: sctp_assoc_t,
        pub se_type: ::uint16_t,
        pub se_on: ::uint8_t,
    }

    /// On/Off setup for subscription to events
    pub struct sctp_event_subscribe {
        pub sctp_data_io_event: ::uint8_t,
        pub sctp_association_event: ::uint8_t,
        pub sctp_address_event: ::uint8_t,
        pub sctp_send_failure_event: ::uint8_t,
        pub sctp_peer_error_event: ::uint8_t,
        pub sctp_shutdown_event: ::uint8_t,
        pub sctp_partial_delivery_event: ::uint8_t,
        pub sctp_adaptation_layer_event: ::uint8_t,
        pub sctp_authentication_event: ::uint8_t,
        pub sctp_sender_dry_event: ::uint8_t,
        // TODO: Use an anonymous union here? This field used to be called
        // 'sctp_stream_reset_events' and the header has a macro to define
        // the old name as the new name for backwards-compatibility.
        // NetBSD uses the old name. Maybe if we use a union here we can
        // provide the ability to use either name.
        pub sctp_stream_reset_event: ::uint8_t,
    }

    pub struct sctp_initmsg {
        pub sinit_num_ostreams: ::uint16_t,
        pub sinit_max_instreams: ::uint16_t,
        pub sinit_max_attempts: ::uint16_t,
        pub sinit_max_init_timeo: ::uint16_t,
    }

    pub struct sctp_sndrcvinfo {
        pub sinfo_stream: ::uint16_t,
        pub sinfo_ssn: ::uint16_t,
        pub sinfo_flags: ::uint16_t,
        pub sinfo_ppid: ::uint32_t,
        pub sinfo_context: ::uint32_t,
        pub sinfo_timetolive: ::uint32_t,
        pub sinfo_tsn: ::uint32_t,
        pub sinfo_cumtsn: ::uint32_t,
        pub sinfo_assoc_id: sctp_assoc_t,
        pub sinfo_keynumber: ::uint16_t,
        pub sinfo_keynumber_valid: ::uint16_t,
        __reserve_pad: [::uint8_t; SCTP_ALIGN_RESV_PAD]
    }

    pub struct sctp_extrcvinfo {
        pub sinfo_stream: ::uint16_t,
        pub sinfo_ssn: ::uint16_t,
        pub sinfo_flags: ::uint16_t,
        pub sinfo_ppid: ::uint32_t,
        pub sinfo_context: ::uint32_t,
        // TODO: Can we use a union here? The header has a define for
        // this field so it can be accessed by an alternative name.
        pub sinfo_timetolive: ::uint32_t,   // AKA sinfo_pr_value
        pub sinfo_tsn: ::uint32_t,
        pub sinfo_cumtsn: ::uint32_t,
        pub sinfo_assoc_id: sctp_assoc_t,
        // TODO: For the next 5 'serinfo_' fields, the header has
        // defines to allow them to be accessed by another name;
        // can we provide both names if we use a union here?
        pub sreinfo_next_flags: ::uint16_t, // AKA serinfo_next_flags
        pub sreinfo_next_stream: ::uint16_t, // AKA serinfo_next_stream
        pub sreinfo_next_aid: ::uint32_t, // AKA serinfo_next_aid
        pub sreinfo_next_length: ::uint32_t, // AKA serinfo_next_length
        pub sreinfo_next_ppid: ::uint32_t, // AKA serinfo_next_ppid
        pub sinfo_keynumber: ::uint16_t,
        pub sinfo_keynumber_valid: ::uint16_t,
        __reserve_pad: [::uint8_t; SCTP_ALIGN_RESV_PAD_SHORT]
    }

    pub struct sctp_sndinfo {
        pub snd_sid: ::uint16_t,
        pub snd_flags: ::uint16_t,
        pub snd_ppid: ::uint32_t,
        pub snd_context: ::uint32_t,
        pub snd_assoc_id: sctp_assoc_t
    }

    pub struct sctp_prinfo {
        pub pr_policy: ::uint16_t,
        pub pr_value: ::uint32_t
    }

    pub struct sctp_default_prinfo {
        pub pr_policy: ::uint16_t,
        pub pr_value: ::uint32_t,
        pub pr_assoc_id: sctp_assoc_t
    }

    pub struct sctp_authinfo {
        pub auth_keynumber: ::uint16_t
    }

    pub struct sctp_rcvinfo {
        pub rcv_sid: ::uint16_t,
        pub rcv_ssn: ::uint16_t,
        pub rcv_flags: ::uint16_t,
        pub rcv_ppid: ::uint32_t,
        pub rcv_tsn: ::uint32_t,
        pub rcv_cumtsn: ::uint32_t,
        pub rcv_context: ::uint32_t,
        pub rcv_assoc_id: sctp_assoc_t
    }

    pub struct sctp_nxtinfo {
        pub nxt_sid: ::uint16_t,
        pub nxt_flags: ::uint16_t,
        pub nxt_ppid: ::uint32_t,
        pub nxt_length: ::uint32_t,
        pub nxt_assoc_id: sctp_assoc_t
    }

    pub struct sctp_recvv_rn {
        pub recvv_rcvinfo: sctp_rcvinfo,
        pub recvv_nxtinfo: sctp_nxtinfo
    }

    pub struct sctp_sendv_spa {
        pub sendv_flags: ::uint32_t,
        pub sendv_sndinfo: sctp_sndinfo,
        pub sendv_prinfo: sctp_prinfo,
        pub sendv_authinfo: sctp_authinfo
    }

    pub struct sctp_snd_all_completes {
        pub sall_stream: ::uint16_t,
        pub sall_flags: ::uint16_t,
        pub sall_ppid: ::uint32_t,
        pub sall_context: ::uint32_t,
        pub sall_num_sent: ::uint32_t,
        pub sall_num_failed: ::uint32_t
    }

    pub struct sctp_pcbinfo {
        pub ep_count: ::uint32_t,
        pub asoc_count: ::uint32_t,
        pub laddr_count: ::uint32_t,
        pub raddr_count: ::uint32_t,
        pub chk_count: ::uint32_t,
        pub readq_count: ::uint32_t,
        pub free_chunks: ::uint32_t,
        pub stream_oque: ::uint32_t
    }

    pub struct sctp_sockstat {
        pub ss_assoc_id: sctp_assoc_t,
        pub ss_total_sndbuf: ::uint32_t,
        pub ss_total_recv_buf: ::uint32_t,
    }

    /// association change event
    pub struct sctp_assoc_change {
        pub sac_type: ::uint16_t,
        pub sac_flags: ::uint16_t,
        pub sac_length: ::uint32_t,
        pub sac_state: ::uint16_t,
        pub sac_error: ::uint16_t,
        pub sac_outbound_streams: ::uint16_t,
        pub sac_inbound_streams: ::uint16_t,
        pub sac_assoc_id: sctp_assoc_t,
        pub sac_info: [::uint8_t; 0]
    }

    /// Address event
    pub struct sctp_paddr_change {
        pub spc_type: ::uint16_t,
        pub spc_flags: ::uint16_t,
        pub spc_length: ::uint32_t,
        pub spc_aaddr: ::sockaddr_storage,
        pub spc_state: ::uint32_t,
        pub spc_error: ::uint32_t,
        pub spc_assoc_id: sctp_assoc_t,
    }

    /// remote error event
    pub struct sctp_remote_error {
        pub sre_type: ::uint16_t,
        pub sre_flags: ::uint16_t,
        pub sre_length: ::uint32_t,
        pub sre_error: ::uint16_t,
        pub sre_assoc_id: sctp_assoc_t,
        pub sre_data: [::uint8_t; 0]
    }

    /// data send failure event (deprecated)
    pub struct sctp_send_failed {
        pub ssf_type: ::uint16_t,
        pub ssf_flags: ::uint16_t,
        pub ssf_length: ::uint32_t,
        pub ssf_error: ::uint32_t,
        pub ssf_info: sctp_sndrcvinfo,
        pub ssf_assoc_id: sctp_assoc_t,
        pub ssf_data: [::uint8_t; 0]
    }

    /// data send failure event (not deprecated)
    pub struct sctp_send_failed_event {
        pub ssfe_type: ::uint16_t,
        pub ssfe_flags: ::uint16_t,
        pub ssfe_length: ::uint32_t,
        pub ssfe_error: ::uint32_t,
        pub ssfe_info: sctp_sndinfo,
        pub ssfe_assoc_id: sctp_assoc_t,
        pub ssfe_data: [::uint8_t; 0]
    }

    /// shutdown event
    pub struct sctp_shutdown_event {
        pub sse_type: ::uint16_t,
        pub sse_flags: ::uint16_t,
        pub sse_length: ::uint32_t,
        pub sse_assoc_id: sctp_assoc_t,
    }

    /// Adaptation layer indication stuff
    pub struct sctp_adaptation_event {
        pub sai_type: ::uint16_t,
        pub sai_flags: ::uint16_t,
        pub sai_length: ::uint32_t,
        pub sai_adaptation_ind: ::uint32_t,
        pub sai_assoc_id: sctp_assoc_t,
    }

    pub struct sctp_setadaptation {
        pub ssb_adaptation_ind: ::uint32_t
    }

    /// compatible old spelling for sctp_adaptation_event
    pub struct sctp_adaption_event {
        pub sai_type: ::uint16_t,
        pub sai_flags: ::uint16_t,
        pub sai_length: ::uint32_t,
        pub sai_adaption_ind: ::uint32_t,
        pub sai_assoc_id: sctp_assoc_t,
    }

    /// compatible old spelling for sctp_setadaptation
    pub struct sctp_setadaption {
        pub ssb_adaption_ind: ::uint32_t
    }

    /// Partial Delivery API event
    pub struct sctp_pdapi_event {
        pub pdapi_type: ::uint16_t,
        pub pdapi_flags: ::uint16_t,
        pub pdapi_length: ::uint32_t,
        pub pdapi_indication: ::uint32_t,
        pub pdapi_stream: ::uint16_t,
        pub pdapi_seq: ::uint16_t,
        pub pdapi_assoc_id: sctp_assoc_t,
    }

    /// authentication key event
    pub struct sctp_authkey_event {
        pub auth_type: ::uint16_t,
        pub auth_flags: ::uint16_t,
        pub auth_length: ::uint32_t,
        pub auth_keynumber: ::uint16_t,
        pub auth_altkeynumber: ::uint16_t,
        pub auth_indication: ::uint32_t,
        pub auth_assoc_id: sctp_assoc_t
    }

    pub struct sctp_sender_dry_event {
        pub sender_dry_type: ::uint16_t,
        pub sender_dry_flags: ::uint16_t,
        pub sender_dry_length: ::uint32_t,
        pub sender_dry_assoc_id: sctp_assoc_t
    }

    /// Stream reset event - subscribe to SCTP_STREAM_RESET_EVENT
    pub struct sctp_stream_reset_event {
        pub strreset_type: ::uint16_t,
        pub strreset_flags: ::uint16_t,
        pub strreset_length: ::uint32_t,
        pub strreset_assoc_id: sctp_assoc_t,
        pub strreset_stream_list: [::uint16_t; 0]
    }

    /// Assoc reset event - subscribe to SCTP_ASSOC_RESET_EVENT
    pub struct sctp_assoc_reset_event {
        pub assocreset_type: ::uint16_t,
        pub assocreset_flags: ::uint16_t,
        pub assocreset_length: ::uint32_t,
        pub assocreset_assoc_id: sctp_assoc_t,
        pub assocreset_local_tsn: ::uint32_t,
        pub assocreset_remote_tsn: ::uint32_t
    }

    /// Stream change event - subscribe to SCTP_STREAM_CHANGE_EVENT
    pub struct sctp_stream_change_event {
        pub strchange_type: ::uint16_t,
        pub strchange_flags: ::uint16_t,
        pub strchange_length: ::uint32_t,
        pub strchange_assoc_id: sctp_assoc_t,
        pub strchange_instrms: ::uint16_t,
        pub strchange_outstrms: ::uint16_t
    }

    /// SCTP notification event
    pub struct sctp_tlv {
        pub sn_type: ::uint16_t,
        pub sn_flags: ::uint16_t,
        pub sn_length: ::uint32_t,
    }

    // TODO: Change this to use `union` when libc targets an appropriate
    //       minimum version of the Rust language spec, then uncomment
    //       the other fields in the union below and remove the private
    //       padding bytes.
    /// notification event
    //pub union sctp_notification {
    pub struct sctp_notification {
        pub sn_header: sctp_tlv,
        // TEMP: padding bytes; sctp_paddr_change is the largest field
        //       in this union due to holding a sockaddr_storage. The
        //       padding bytes here should make this struct the same
        //       size as the union would be.
        __notification_data: [u8; 140]
        // pub sn_assoc_change: sctp_assoc_change,
        // pub sn_paddr_change: sctp_paddr_change,
        // pub sn_remote_error: sctp_remote_error,
        // pub sn_send_failed: sctp_send_failed,
        // pub sn_shutdown_event: sctp_shutdown_event,
        // pub sn_adaptation_event: sctp_adaptation_event,
        // pub sn_adaption_event: sctp_adaption_event,
        // pub sn_pdapi_event: sctp_pdapi_event,
        // pub sn_auth_event: sctp_authkey_event,
        // pub sn_sender_dry_event: sctp_sender_dry_event,
        // pub sn_send_failed_event: sctp_send_failed_event,
        // pub sn_strreset_event: sctp_strreset_event,
        // pub sn_assocreset_event: sctp_assoc_reset_event,
        // pub sn_strchange_event: sctp_stream_change_event
    }

    pub struct sctp_paddrparams {
        pub spp_address: ::sockaddr_storage,
        pub spp_assoc_id: sctp_assoc_t,
        pub spp_hbinterval: ::uint32_t,
        pub spp_pathmtu: ::uint32_t,
        pub spp_flags: ::uint32_t,
        pub spp_ipv6_flowlabel: ::uint32_t,
        pub spp_pathmaxrxt: ::uint16_t,
        pub spp_dscp: ::uint8_t // AKA spp_ipv4_tos
    }

    pub struct sctp_paddrthlds {
        pub spt_address: ::sockaddr_storage,
        pub spt_assoc_id: sctp_assoc_t,
        pub spt_pathmaxrxt: ::uint16_t,
        pub spt_pathpfthld: ::uint16_t,
        pub spt_pathcpthld: ::uint16_t
    }

    pub struct sctp_paddrinfo {
        pub spinfo_address: ::sockaddr_storage,
        pub spinfo_assoc_id: sctp_assoc_t,
        pub spinfo_state: ::int32_t,
        pub spinfo_cwnd: ::uint32_t,
        pub spinfo_srtt: ::uint32_t,
        pub spinfo_rto: ::uint32_t,
        pub spinfo_mtu: ::uint32_t,
    }

    pub struct sctp_rtoinfo {
        pub srto_assoc_id: sctp_assoc_t,
        pub srto_initial: ::uint32_t,
        pub srto_max: ::uint32_t,
        pub srto_min: ::uint32_t,
    }

    pub struct sctp_assocparams {
        pub sasoc_assoc_id: sctp_assoc_t,
        pub sasoc_peer_rwnd: ::uint32_t,
        pub sasoc_local_rwnd: ::uint32_t,
        pub sasoc_cookie_life: ::uint32_t,
        pub sasoc_asocmaxrxt: ::uint16_t,
        pub sasoc_number_peer_destinations: ::uint16_t
    }

    pub struct sctp_setprim {
        pub ssp_addr: ::sockaddr_storage,
        pub ssp_assoc_id: sctp_assoc_t,
        ssp_padding: [::uint8_t; 4]
    }

    pub struct sctp_setpeerprim {
        pub sspp_addr: ::sockaddr_storage,
        pub sspp_assoc_id: sctp_assoc_t,
        sspp_padding: [::uint8_t; 4]
    }

    pub struct sctp_getaddresses {
        pub sget_assoc_id: sctp_assoc_t,
        // addr is filled in for N * sockaddr_storage
        pub addr: [::sockaddr; 1],
    }

    pub struct sctp_status {
        pub sstat_assoc_id: sctp_assoc_t,
        pub sstat_state: ::int32_t,
        pub sstat_rwnd: ::uint32_t,
        pub sstat_unackdata: ::uint16_t,
        pub sstat_penddata: ::uint16_t,
        pub sstat_instrms: ::uint16_t,
        pub sstat_outstrms: ::uint16_t,
        pub sstat_fragmentation_point: ::uint32_t,
        pub sstat_primary: sctp_paddrinfo,
    }

    /// SCTP_AUTH_CHUNK
    pub struct sctp_authchunk {
        pub sauth_chunk: ::uint8_t
    }

    /// SCTP_AUTH_KEY
    pub struct sctp_authkey {
        pub sca_assoc_id: sctp_assoc_t,
        pub sca_keynumber: ::uint16_t,
        pub sca_keylength: ::uint16_t,
        pub sca_key: [::uint8_t; 0]
    }

    /// SCTP_HMAC_IDENT
    pub struct sctp_hmacalgo {
        pub shmac_number_of_idents: ::uint32_t,
        pub shmac_idents: [::uint16_t; 0]
    }

    /// SCTP_AUTH_ACTIVE_KEY / SCTP_AUTH_DELETE_KEY
    pub struct sctp_authkeyid {
        pub scact_assoc_id: sctp_assoc_t,
        pub scact_keynumber: ::uint16_t
    }

    /// SCTP_PEER_AUTH_CHUNKS / SCTP_LOCAL_AUTH_CHUNKS
    pub struct sctp_authchunks {
        pub gauth_assoc_id: sctp_assoc_t,
        pub gauth_number_of_chunks: ::uint32_t,
        pub gauth_chunks: [::uint8_t; 0]
    }

    pub struct sctp_assoc_value {
        pub assoc_id: sctp_assoc_t,
        pub assoc_value: ::uint32_t
    }

    pub struct sctp_cc_option {
        pub option: ::c_int,
        pub aid_value: sctp_assoc_value
    }

    pub struct sctp_stream_value {
        pub assoc_id: sctp_assoc_t,
        pub stream_id: ::uint16_t,
        pub stream_value: ::uint16_t
    }

    pub struct sctp_assoc_ids {
        pub gaids_number_of_ids: ::uint32_t,
        pub gaids_assoc_id: [sctp_assoc_t; MAX_ASOC_IDS_RET as usize],
    }

    pub struct sctp_sack_info {
        pub sack_assoc_id: sctp_assoc_t,
        pub sack_delay: ::uint32_t,
        pub sack_freq: ::uint32_t
    }

    pub struct sctp_timeouts {
        pub stimo_assoc_id: sctp_assoc_t,
        pub stimo_init: ::uint32_t,
        pub stimo_data: ::uint32_t,
        pub stimo_sack: ::uint32_t,
        pub stimo_shutdown: ::uint32_t,
        pub stimo_heartbeat: ::uint32_t,
        pub stimo_cookie: ::uint32_t,
        pub stimo_shutdownack: ::uint32_t
    }

    pub struct sctp_udpencaps {
        pub sue_address: ::sockaddr_storage,
        pub sue_assoc_id: sctp_assoc_t,
        pub sue_port: ::uint16_t
    }

    pub struct sctp_prstatus {
        pub sprstat_assoc_id: sctp_assoc_t,
        pub sprstat_sid: ::uint16_t,
        pub sprstat_policy: ::uint16_t,
        pub sprstat_abandoned_unsent: ::uint64_t,
        pub sprstat_abandoned_sent: ::uint64_t
    }

    pub struct sctp_cwnd_args {
        /// network to
        pub net: *mut sctp_nets,
        /// cwnd in k
        pub cwnd_new_value: ::uint32_t,
        /// flightsize in k
        pub inflight: ::uint16_t,
        /// increment to it
        pub cwnd_augment: ::uint16_t,
        pub meets_pseudo_cumack: ::uint8_t,
        pub need_new_pseudo_cumack: ::uint8_t,
        pub cnt_in_send: ::uint8_t,
        pub cnt_in_str: ::uint8_t
    }

    pub struct sctp_blk_args {
        /// in 1k bytes
        pub onsb: ::uint32_t,
        /// len of send being attempted
        pub sndlen: ::uint32_t,
        /// rwnd of peer
        pub peer_rwnd: ::uint32_t,
        /// chnk cnt
        pub send_sent_qcnt: ::uint16_t,
        /// chnk cnt
        pub stream_qcnt: ::uint16_t,
        /// chunks out
        pub chunks_on_oque: ::uint16_t,
        /// flight size in k
        pub flight_size: ::uint16_t
    }

    pub struct sctp_reset_streams {
        pub srs_assoc_id: sctp_assoc_t,
        pub srs_flags: ::uint16_t,
        /// 0 == ALL
        pub srs_number_streams: ::uint16_t,
        /// list if strrst_num_streams is not 0
        pub srs_stream_list: [::uint16_t; 0]
    }

    pub struct sctp_add_streams {
        pub sas_assoc_id: sctp_assoc_t,
        pub sas_instrms: ::uint16_t,
        pub sas_outstrms: ::uint16_t
    }

    pub struct sctp_get_nonce_values {
        pub gn_assoc_id: sctp_assoc_t,
        pub gn_peers_tag: ::uint32_t,
        pub gn_local_tag: ::uint32_t,
    }

    /* SCTP debugging logs */
    pub struct sctp_log_entry {
        pub timestamp: ::uint64_t,
        pub subsys: ::uint32_t,
        pub padding: ::uint32_t,
        pub params: [::uint32_t; SCTP_TRACE_PARAMS as usize]
    }

    pub struct sctp_log {
        pub entry: [sctp_log_entry; SCTP_MAX_LOGGING_SIZE as usize],
        pub index: ::uint32_t,
        pub padding: ::uint32_t
    }
}

pub const SIGEV_THREAD_ID: ::c_int = 4;

pub const RAND_MAX: ::c_int = 0x7fff_fffd;
pub const PTHREAD_STACK_MIN: ::size_t = 2048;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: ::c_int = 4;
pub const SIGSTKSZ: ::size_t = 34816;
pub const SF_NODISKIO: ::c_int = 0x00000001;
pub const SF_MNOWAIT: ::c_int = 0x00000002;
pub const SF_SYNC: ::c_int = 0x00000004;
pub const O_CLOEXEC: ::c_int = 0x00100000;
pub const O_DIRECTORY: ::c_int = 0x00020000;
pub const O_EXEC: ::c_int = 0x00040000;
pub const O_TTY_INIT: ::c_int = 0x00080000;
pub const F_GETLK: ::c_int = 11;
pub const F_SETLK: ::c_int = 12;
pub const F_SETLKW: ::c_int = 13;
pub const ELAST: ::c_int = 96;
pub const RLIMIT_NPTS: ::c_int = 11;
pub const RLIMIT_SWAP: ::c_int = 12;
pub const RLIM_NLIMITS: ::rlim_t = 13;

pub const Q_GETQUOTA: ::c_int = 0x700;
pub const Q_SETQUOTA: ::c_int = 0x800;

pub const POSIX_FADV_NORMAL: ::c_int = 0;
pub const POSIX_FADV_RANDOM: ::c_int = 1;
pub const POSIX_FADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_FADV_WILLNEED: ::c_int = 3;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;

pub const POLLINIGNEOF: ::c_short = 0x2000;

pub const EVFILT_READ: ::int16_t = -1;
pub const EVFILT_WRITE: ::int16_t = -2;
pub const EVFILT_AIO: ::int16_t = -3;
pub const EVFILT_VNODE: ::int16_t = -4;
pub const EVFILT_PROC: ::int16_t = -5;
pub const EVFILT_SIGNAL: ::int16_t = -6;
pub const EVFILT_TIMER: ::int16_t = -7;
pub const EVFILT_FS: ::int16_t = -9;
pub const EVFILT_LIO: ::int16_t = -10;
pub const EVFILT_USER: ::int16_t = -11;

pub const EV_ADD: ::uint16_t = 0x1;
pub const EV_DELETE: ::uint16_t = 0x2;
pub const EV_ENABLE: ::uint16_t = 0x4;
pub const EV_DISABLE: ::uint16_t = 0x8;
pub const EV_ONESHOT: ::uint16_t = 0x10;
pub const EV_CLEAR: ::uint16_t = 0x20;
pub const EV_RECEIPT: ::uint16_t = 0x40;
pub const EV_DISPATCH: ::uint16_t = 0x80;
pub const EV_DROP: ::uint16_t = 0x1000;
pub const EV_FLAG1: ::uint16_t = 0x2000;
pub const EV_ERROR: ::uint16_t = 0x4000;
pub const EV_EOF: ::uint16_t = 0x8000;
pub const EV_SYSFLAGS: ::uint16_t = 0xf000;

pub const NOTE_TRIGGER: ::uint32_t = 0x01000000;
pub const NOTE_FFNOP: ::uint32_t = 0x00000000;
pub const NOTE_FFAND: ::uint32_t = 0x40000000;
pub const NOTE_FFOR: ::uint32_t = 0x80000000;
pub const NOTE_FFCOPY: ::uint32_t = 0xc0000000;
pub const NOTE_FFCTRLMASK: ::uint32_t = 0xc0000000;
pub const NOTE_FFLAGSMASK: ::uint32_t = 0x00ffffff;
pub const NOTE_LOWAT: ::uint32_t = 0x00000001;
pub const NOTE_DELETE: ::uint32_t = 0x00000001;
pub const NOTE_WRITE: ::uint32_t = 0x00000002;
pub const NOTE_EXTEND: ::uint32_t = 0x00000004;
pub const NOTE_ATTRIB: ::uint32_t = 0x00000008;
pub const NOTE_LINK: ::uint32_t = 0x00000010;
pub const NOTE_RENAME: ::uint32_t = 0x00000020;
pub const NOTE_REVOKE: ::uint32_t = 0x00000040;
pub const NOTE_EXIT: ::uint32_t = 0x80000000;
pub const NOTE_FORK: ::uint32_t = 0x40000000;
pub const NOTE_EXEC: ::uint32_t = 0x20000000;
pub const NOTE_PDATAMASK: ::uint32_t = 0x000fffff;
pub const NOTE_PCTRLMASK: ::uint32_t = 0xf0000000;
pub const NOTE_TRACK: ::uint32_t = 0x00000001;
pub const NOTE_TRACKERR: ::uint32_t = 0x00000002;
pub const NOTE_CHILD: ::uint32_t = 0x00000004;
pub const NOTE_SECONDS: ::uint32_t = 0x00000001;
pub const NOTE_MSECONDS: ::uint32_t = 0x00000002;
pub const NOTE_USECONDS: ::uint32_t = 0x00000004;
pub const NOTE_NSECONDS: ::uint32_t = 0x00000008;

pub const MADV_PROTECT: ::c_int = 10;
pub const RUSAGE_THREAD: ::c_int = 1;

pub const CLOCK_REALTIME: ::clockid_t = 0;
pub const CLOCK_VIRTUAL: ::clockid_t = 1;
pub const CLOCK_PROF: ::clockid_t = 2;
pub const CLOCK_MONOTONIC: ::clockid_t = 4;
pub const CLOCK_UPTIME: ::clockid_t = 5;
pub const CLOCK_UPTIME_PRECISE: ::clockid_t = 7;
pub const CLOCK_UPTIME_FAST: ::clockid_t = 8;
pub const CLOCK_REALTIME_PRECISE: ::clockid_t = 9;
pub const CLOCK_REALTIME_FAST: ::clockid_t = 10;
pub const CLOCK_MONOTONIC_PRECISE: ::clockid_t = 11;
pub const CLOCK_MONOTONIC_FAST: ::clockid_t = 12;
pub const CLOCK_SECOND: ::clockid_t = 13;
pub const CLOCK_THREAD_CPUTIME_ID: ::clockid_t = 14;
pub const CLOCK_PROCESS_CPUTIME_ID: ::clockid_t = 15;

pub const CTL_UNSPEC: ::c_int = 0;
pub const CTL_KERN: ::c_int = 1;
pub const CTL_VM: ::c_int = 2;
pub const CTL_VFS: ::c_int = 3;
pub const CTL_NET: ::c_int = 4;
pub const CTL_DEBUG: ::c_int = 5;
pub const CTL_HW: ::c_int = 6;
pub const CTL_MACHDEP: ::c_int = 7;
pub const CTL_USER: ::c_int = 8;
pub const CTL_P1003_1B: ::c_int = 9;
pub const KERN_OSTYPE: ::c_int = 1;
pub const KERN_OSRELEASE: ::c_int = 2;
pub const KERN_OSREV: ::c_int = 3;
pub const KERN_VERSION: ::c_int = 4;
pub const KERN_MAXVNODES: ::c_int = 5;
pub const KERN_MAXPROC: ::c_int = 6;
pub const KERN_MAXFILES: ::c_int = 7;
pub const KERN_ARGMAX: ::c_int = 8;
pub const KERN_SECURELVL: ::c_int = 9;
pub const KERN_HOSTNAME: ::c_int = 10;
pub const KERN_HOSTID: ::c_int = 11;
pub const KERN_CLOCKRATE: ::c_int = 12;
pub const KERN_VNODE: ::c_int = 13;
pub const KERN_PROC: ::c_int = 14;
pub const KERN_FILE: ::c_int = 15;
pub const KERN_PROF: ::c_int = 16;
pub const KERN_POSIX1: ::c_int = 17;
pub const KERN_NGROUPS: ::c_int = 18;
pub const KERN_JOB_CONTROL: ::c_int = 19;
pub const KERN_SAVED_IDS: ::c_int = 20;
pub const KERN_BOOTTIME: ::c_int = 21;
pub const KERN_NISDOMAINNAME: ::c_int = 22;
pub const KERN_UPDATEINTERVAL: ::c_int = 23;
pub const KERN_OSRELDATE: ::c_int = 24;
pub const KERN_NTP_PLL: ::c_int = 25;
pub const KERN_BOOTFILE: ::c_int = 26;
pub const KERN_MAXFILESPERPROC: ::c_int = 27;
pub const KERN_MAXPROCPERUID: ::c_int = 28;
pub const KERN_DUMPDEV: ::c_int = 29;
pub const KERN_IPC: ::c_int = 30;
pub const KERN_DUMMY: ::c_int = 31;
pub const KERN_PS_STRINGS: ::c_int = 32;
pub const KERN_USRSTACK: ::c_int = 33;
pub const KERN_LOGSIGEXIT: ::c_int = 34;
pub const KERN_IOV_MAX: ::c_int = 35;
pub const KERN_HOSTUUID: ::c_int = 36;
pub const KERN_ARND: ::c_int = 37;
pub const KERN_PROC_ALL: ::c_int = 0;
pub const KERN_PROC_PID: ::c_int = 1;
pub const KERN_PROC_PGRP: ::c_int = 2;
pub const KERN_PROC_SESSION: ::c_int = 3;
pub const KERN_PROC_TTY: ::c_int = 4;
pub const KERN_PROC_UID: ::c_int = 5;
pub const KERN_PROC_RUID: ::c_int = 6;
pub const KERN_PROC_ARGS: ::c_int = 7;
pub const KERN_PROC_PROC: ::c_int = 8;
pub const KERN_PROC_SV_NAME: ::c_int = 9;
pub const KERN_PROC_RGID: ::c_int = 10;
pub const KERN_PROC_GID: ::c_int = 11;
pub const KERN_PROC_PATHNAME: ::c_int = 12;
pub const KERN_PROC_OVMMAP: ::c_int = 13;
pub const KERN_PROC_OFILEDESC: ::c_int = 14;
pub const KERN_PROC_KSTACK: ::c_int = 15;
pub const KERN_PROC_INC_THREAD: ::c_int = 0x10;
pub const KERN_PROC_VMMAP: ::c_int = 32;
pub const KERN_PROC_FILEDESC: ::c_int = 33;
pub const KERN_PROC_GROUPS: ::c_int = 34;
pub const KERN_PROC_ENV: ::c_int = 35;
pub const KERN_PROC_AUXV: ::c_int = 36;
pub const KERN_PROC_RLIMIT: ::c_int = 37;
pub const KERN_PROC_PS_STRINGS: ::c_int = 38;
pub const KERN_PROC_UMASK: ::c_int = 39;
pub const KERN_PROC_OSREL: ::c_int = 40;
pub const KERN_PROC_SIGTRAMP: ::c_int = 41;
pub const KIPC_MAXSOCKBUF: ::c_int = 1;
pub const KIPC_SOCKBUF_WASTE: ::c_int = 2;
pub const KIPC_SOMAXCONN: ::c_int = 3;
pub const KIPC_MAX_LINKHDR: ::c_int = 4;
pub const KIPC_MAX_PROTOHDR: ::c_int = 5;
pub const KIPC_MAX_HDR: ::c_int = 6;
pub const KIPC_MAX_DATALEN: ::c_int = 7;
pub const HW_MACHINE: ::c_int = 1;
pub const HW_MODEL: ::c_int = 2;
pub const HW_NCPU: ::c_int = 3;
pub const HW_BYTEORDER: ::c_int = 4;
pub const HW_PHYSMEM: ::c_int = 5;
pub const HW_USERMEM: ::c_int = 6;
pub const HW_PAGESIZE: ::c_int = 7;
pub const HW_DISKNAMES: ::c_int = 8;
pub const HW_DISKSTATS: ::c_int = 9;
pub const HW_FLOATINGPT: ::c_int = 10;
pub const HW_MACHINE_ARCH: ::c_int = 11;
pub const HW_REALMEM: ::c_int = 12;
pub const USER_CS_PATH: ::c_int = 1;
pub const USER_BC_BASE_MAX: ::c_int = 2;
pub const USER_BC_DIM_MAX: ::c_int = 3;
pub const USER_BC_SCALE_MAX: ::c_int = 4;
pub const USER_BC_STRING_MAX: ::c_int = 5;
pub const USER_COLL_WEIGHTS_MAX: ::c_int = 6;
pub const USER_EXPR_NEST_MAX: ::c_int = 7;
pub const USER_LINE_MAX: ::c_int = 8;
pub const USER_RE_DUP_MAX: ::c_int = 9;
pub const USER_POSIX2_VERSION: ::c_int = 10;
pub const USER_POSIX2_C_BIND: ::c_int = 11;
pub const USER_POSIX2_C_DEV: ::c_int = 12;
pub const USER_POSIX2_CHAR_TERM: ::c_int = 13;
pub const USER_POSIX2_FORT_DEV: ::c_int = 14;
pub const USER_POSIX2_FORT_RUN: ::c_int = 15;
pub const USER_POSIX2_LOCALEDEF: ::c_int = 16;
pub const USER_POSIX2_SW_DEV: ::c_int = 17;
pub const USER_POSIX2_UPE: ::c_int = 18;
pub const USER_STREAM_MAX: ::c_int = 19;
pub const USER_TZNAME_MAX: ::c_int = 20;
pub const CTL_P1003_1B_ASYNCHRONOUS_IO: ::c_int = 1;
pub const CTL_P1003_1B_MAPPED_FILES: ::c_int = 2;
pub const CTL_P1003_1B_MEMLOCK: ::c_int = 3;
pub const CTL_P1003_1B_MEMLOCK_RANGE: ::c_int = 4;
pub const CTL_P1003_1B_MEMORY_PROTECTION: ::c_int = 5;
pub const CTL_P1003_1B_MESSAGE_PASSING: ::c_int = 6;
pub const CTL_P1003_1B_PRIORITIZED_IO: ::c_int = 7;
pub const CTL_P1003_1B_PRIORITY_SCHEDULING: ::c_int = 8;
pub const CTL_P1003_1B_REALTIME_SIGNALS: ::c_int = 9;
pub const CTL_P1003_1B_SEMAPHORES: ::c_int = 10;
pub const CTL_P1003_1B_FSYNC: ::c_int = 11;
pub const CTL_P1003_1B_SHARED_MEMORY_OBJECTS: ::c_int = 12;
pub const CTL_P1003_1B_SYNCHRONIZED_IO: ::c_int = 13;
pub const CTL_P1003_1B_TIMERS: ::c_int = 14;
pub const CTL_P1003_1B_AIO_LISTIO_MAX: ::c_int = 15;
pub const CTL_P1003_1B_AIO_MAX: ::c_int = 16;
pub const CTL_P1003_1B_AIO_PRIO_DELTA_MAX: ::c_int = 17;
pub const CTL_P1003_1B_DELAYTIMER_MAX: ::c_int = 18;
pub const CTL_P1003_1B_MQ_OPEN_MAX: ::c_int = 19;
pub const CTL_P1003_1B_PAGESIZE: ::c_int = 20;
pub const CTL_P1003_1B_RTSIG_MAX: ::c_int = 21;
pub const CTL_P1003_1B_SEM_NSEMS_MAX: ::c_int = 22;
pub const CTL_P1003_1B_SEM_VALUE_MAX: ::c_int = 23;
pub const CTL_P1003_1B_SIGQUEUE_MAX: ::c_int = 24;
pub const CTL_P1003_1B_TIMER_MAX: ::c_int = 25;
pub const TIOCGPTN: ::c_uint = 0x4004740f;
pub const TIOCPTMASTER: ::c_uint = 0x2000741c;
pub const TIOCSIG: ::c_uint = 0x2004745f;
pub const TIOCM_DCD: ::c_int = 0x40;
pub const H4DISC: ::c_int = 0x7;

pub const JAIL_API_VERSION: u32 = 2;
pub const JAIL_CREATE: ::c_int = 0x01;
pub const JAIL_UPDATE: ::c_int = 0x02;
pub const JAIL_ATTACH: ::c_int = 0x04;
pub const JAIL_DYING: ::c_int = 0x08;
pub const JAIL_SET_MASK: ::c_int = 0x0f;
pub const JAIL_GET_MASK: ::c_int = 0x08;
pub const JAIL_SYS_DISABLE: ::c_int = 0;
pub const JAIL_SYS_NEW: ::c_int = 1;
pub const JAIL_SYS_INHERIT: ::c_int = 2;

pub const SO_BINTIME: ::c_int = 0x2000;
pub const SO_NO_OFFLOAD: ::c_int = 0x4000;
pub const SO_NO_DDP: ::c_int = 0x8000;
pub const SO_LABEL: ::c_int = 0x1009;
pub const SO_PEERLABEL: ::c_int = 0x1010;
pub const SO_LISTENQLIMIT: ::c_int = 0x1011;
pub const SO_LISTENQLEN: ::c_int = 0x1012;
pub const SO_LISTENINCQLEN: ::c_int = 0x1013;
pub const SO_SETFIB: ::c_int = 0x1014;
pub const SO_USER_COOKIE: ::c_int = 0x1015;
pub const SO_PROTOCOL: ::c_int = 0x1016;
pub const SO_PROTOTYPE: ::c_int = SO_PROTOCOL;
pub const SO_VENDOR: ::c_int = 0x80000000;

pub const LOCAL_PEERCRED: ::c_int = 1;
pub const LOCAL_CREDS: ::c_int = 2;
pub const LOCAL_CONNWAIT: ::c_int = 4;
pub const LOCAL_VENDOR: ::c_int = SO_VENDOR;

pub const AF_SLOW: ::c_int = 33;
pub const AF_SCLUSTER: ::c_int = 34;
pub const AF_ARP: ::c_int = 35;
pub const AF_BLUETOOTH: ::c_int = 36;
pub const AF_IEEE80211: ::c_int = 37;
pub const AF_INET_SDP: ::c_int = 40;
pub const AF_INET6_SDP: ::c_int = 42;
#[doc(hidden)]
pub const AF_MAX: ::c_int = 42;

// https://github.com/freebsd/freebsd/blob/master/sys/net/if.h#L140
pub const IFF_UP: ::c_int = 0x1; // (n) interface is up
pub const IFF_BROADCAST: ::c_int = 0x2; // (i) broadcast address valid
pub const IFF_DEBUG: ::c_int = 0x4; // (n) turn on debugging
pub const IFF_LOOPBACK: ::c_int = 0x8; // (i) is a loopback net
pub const IFF_POINTOPOINT: ::c_int = 0x10; // (i) is a point-to-point link
// 0x20           was IFF_SMART
pub const IFF_RUNNING: ::c_int = 0x40; // (d) resources allocated
#[deprecated(since="0.2.34",
             note="please use the portable `IFF_RUNNING` constant instead")]
pub const IFF_DRV_RUNNING: ::c_int = 0x40;
pub const IFF_NOARP: ::c_int = 0x80; // (n) no address resolution protocol
pub const IFF_PROMISC: ::c_int = 0x100; // (n) receive all packets
pub const IFF_ALLMULTI: ::c_int = 0x200; // (n) receive all multicast packets
pub const IFF_OACTIVE: ::c_int = 0x400; // (d) tx hardware queue is full
#[deprecated(since="0.2.34",
             note="please use the portable `IFF_OACTIVE` constant instead")]
pub const IFF_DRV_OACTIVE: ::c_int = 0x400;
pub const IFF_SIMPLEX: ::c_int = 0x800; // (i) can't hear own transmissions
pub const IFF_LINK0: ::c_int = 0x1000; // per link layer defined bit
pub const IFF_LINK1: ::c_int = 0x2000; // per link layer defined bit
pub const IFF_LINK2: ::c_int = 0x4000; // per link layer defined bit
pub const IFF_ALTPHYS: ::c_int = IFF_LINK2; // use alternate physical connection
pub const IFF_MULTICAST: ::c_int = 0x8000; // (i) supports multicast
// (i) unconfigurable using ioctl(2)
pub const IFF_CANTCONFIG: ::c_int = 0x10000;
pub const IFF_PPROMISC: ::c_int = 0x20000; // (n) user-requested promisc mode
pub const IFF_MONITOR: ::c_int = 0x40000; // (n) user-requested monitor mode
pub const IFF_STATICARP: ::c_int = 0x80000; // (n) static ARP
pub const IFF_DYING: ::c_int = 0x200000; // (n) interface is winding down
pub const IFF_RENAMING: ::c_int = 0x400000; // (n) interface is being renamed

// sys/netinet/in.h
// Protocols (RFC 1700)
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

// IPPROTO_IP defined in src/unix/mod.rs
/// IP6 hop-by-hop options
pub const IPPROTO_HOPOPTS: ::c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: ::c_int = 2;
/// gateway^2 (deprecated)
pub const IPPROTO_GGP: ::c_int = 3;
/// for compatibility
pub const IPPROTO_IPIP: ::c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// Stream protocol II.
pub const IPPROTO_ST: ::c_int = 7;
/// exterior gateway protocol
pub const IPPROTO_EGP: ::c_int = 8;
/// private interior gateway
pub const IPPROTO_PIGP: ::c_int = 9;
/// BBN RCC Monitoring
pub const IPPROTO_RCCMON: ::c_int = 10;
/// network voice protocol
pub const IPPROTO_NVPII: ::c_int = 11;
/// pup
pub const IPPROTO_PUP: ::c_int = 12;
/// Argus
pub const IPPROTO_ARGUS: ::c_int = 13;
/// EMCON
pub const IPPROTO_EMCON: ::c_int = 14;
/// Cross Net Debugger
pub const IPPROTO_XNET: ::c_int = 15;
/// Chaos
pub const IPPROTO_CHAOS: ::c_int = 16;
// IPPROTO_UDP defined in src/unix/mod.rs
/// Multiplexing
pub const IPPROTO_MUX: ::c_int = 18;
/// DCN Measurement Subsystems
pub const IPPROTO_MEAS: ::c_int = 19;
/// Host Monitoring
pub const IPPROTO_HMP: ::c_int = 20;
/// Packet Radio Measurement
pub const IPPROTO_PRM: ::c_int = 21;
/// xns idp
pub const IPPROTO_IDP: ::c_int = 22;
/// Trunk-1
pub const IPPROTO_TRUNK1: ::c_int = 23;
/// Trunk-2
pub const IPPROTO_TRUNK2: ::c_int = 24;
/// Leaf-1
pub const IPPROTO_LEAF1: ::c_int = 25;
/// Leaf-2
pub const IPPROTO_LEAF2: ::c_int = 26;
/// Reliable Data
pub const IPPROTO_RDP: ::c_int = 27;
/// Reliable Transaction
pub const IPPROTO_IRTP: ::c_int = 28;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: ::c_int = 29;
/// Bulk Data Transfer
pub const IPPROTO_BLT: ::c_int = 30;
/// Network Services
pub const IPPROTO_NSP: ::c_int = 31;
/// Merit Internodal
pub const IPPROTO_INP: ::c_int = 32;
/// Sequential Exchange
pub const IPPROTO_SEP: ::c_int = 33;
/// Third Party Connect
pub const IPPROTO_3PC: ::c_int = 34;
/// InterDomain Policy Routing
pub const IPPROTO_IDPR: ::c_int = 35;
/// XTP
pub const IPPROTO_XTP: ::c_int = 36;
/// Datagram Delivery
pub const IPPROTO_DDP: ::c_int = 37;
/// Control Message Transport
pub const IPPROTO_CMTP: ::c_int = 38;
/// TP++ Transport
pub const IPPROTO_TPXX: ::c_int = 39;
/// IL transport protocol
pub const IPPROTO_IL: ::c_int = 40;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// Source Demand Routing
pub const IPPROTO_SDRP: ::c_int = 42;
/// IP6 routing header
pub const IPPROTO_ROUTING: ::c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: ::c_int = 44;
/// InterDomain Routing
pub const IPPROTO_IDRP: ::c_int = 45;
/// resource reservation
pub const IPPROTO_RSVP: ::c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: ::c_int = 47;
/// Mobile Host Routing
pub const IPPROTO_MHRP: ::c_int = 48;
/// BHA
pub const IPPROTO_BHA: ::c_int = 49;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: ::c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: ::c_int = 51;
/// Integ. Net Layer Security
pub const IPPROTO_INLSP: ::c_int = 52;
/// IP with encryption
pub const IPPROTO_SWIPE: ::c_int = 53;
/// Next Hop Resolution
pub const IPPROTO_NHRP: ::c_int = 54;
/// IP Mobility
pub const IPPROTO_MOBILE: ::c_int = 55;
/// Transport Layer Security
pub const IPPROTO_TLSP: ::c_int = 56;
/// SKIP
pub const IPPROTO_SKIP: ::c_int = 57;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: ::c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: ::c_int = 60;
/// any host internal protocol
pub const IPPROTO_AHIP: ::c_int = 61;
/// CFTP
pub const IPPROTO_CFTP: ::c_int = 62;
/// "hello" routing protocol
pub const IPPROTO_HELLO: ::c_int = 63;
/// SATNET/Backroom EXPAK
pub const IPPROTO_SATEXPAK: ::c_int = 64;
/// Kryptolan
pub const IPPROTO_KRYPTOLAN: ::c_int = 65;
/// Remote Virtual Disk
pub const IPPROTO_RVD: ::c_int = 66;
/// Pluribus Packet Core
pub const IPPROTO_IPPC: ::c_int = 67;
/// Any distributed FS
pub const IPPROTO_ADFS: ::c_int = 68;
/// Satnet Monitoring
pub const IPPROTO_SATMON: ::c_int = 69;
/// VISA Protocol
pub const IPPROTO_VISA: ::c_int = 70;
/// Packet Core Utility
pub const IPPROTO_IPCV: ::c_int = 71;
/// Comp. Prot. Net. Executive
pub const IPPROTO_CPNX: ::c_int = 72;
/// Comp. Prot. HeartBeat
pub const IPPROTO_CPHB: ::c_int = 73;
/// Wang Span Network
pub const IPPROTO_WSN: ::c_int = 74;
/// Packet Video Protocol
pub const IPPROTO_PVP: ::c_int = 75;
/// BackRoom SATNET Monitoring
pub const IPPROTO_BRSATMON: ::c_int = 76;
/// Sun net disk proto (temp.)
pub const IPPROTO_ND: ::c_int = 77;
/// WIDEBAND Monitoring
pub const IPPROTO_WBMON: ::c_int = 78;
/// WIDEBAND EXPAK
pub const IPPROTO_WBEXPAK: ::c_int = 79;
/// ISO cnlp
pub const IPPROTO_EON: ::c_int = 80;
/// VMTP
pub const IPPROTO_VMTP: ::c_int = 81;
/// Secure VMTP
pub const IPPROTO_SVMTP: ::c_int = 82;
/// Banyon VINES
pub const IPPROTO_VINES: ::c_int = 83;
/// TTP
pub const IPPROTO_TTP: ::c_int = 84;
/// NSFNET-IGP
pub const IPPROTO_IGP: ::c_int = 85;
/// dissimilar gateway prot.
pub const IPPROTO_DGP: ::c_int = 86;
/// TCF
pub const IPPROTO_TCF: ::c_int = 87;
/// Cisco/GXS IGRP
pub const IPPROTO_IGRP: ::c_int = 88;
/// OSPFIGP
pub const IPPROTO_OSPFIGP: ::c_int = 89;
/// Strite RPC protocol
pub const IPPROTO_SRPC: ::c_int = 90;
/// Locus Address Resoloution
pub const IPPROTO_LARP: ::c_int = 91;
/// Multicast Transport
pub const IPPROTO_MTP: ::c_int = 92;
/// AX.25 Frames
pub const IPPROTO_AX25: ::c_int = 93;
/// IP encapsulated in IP
pub const IPPROTO_IPEIP: ::c_int = 94;
/// Mobile Int.ing control
pub const IPPROTO_MICP: ::c_int = 95;
/// Semaphore Comm. security
pub const IPPROTO_SCCSP: ::c_int = 96;
/// Ethernet IP encapsulation
pub const IPPROTO_ETHERIP: ::c_int = 97;
/// encapsulation header
pub const IPPROTO_ENCAP: ::c_int = 98;
/// any private encr. scheme
pub const IPPROTO_APES: ::c_int = 99;
/// GMTP
pub const IPPROTO_GMTP: ::c_int = 100;
/// payload compression (IPComp)
pub const IPPROTO_IPCOMP: ::c_int = 108;
/// SCTP
pub const IPPROTO_SCTP: ::c_int = 132;
/// IPv6 Mobility Header
pub const IPPROTO_MH: ::c_int = 135;
/// UDP-Lite
pub const IPPROTO_UDPLITE: ::c_int = 136;
/// IP6 Host Identity Protocol
pub const IPPROTO_HIP: ::c_int = 139;
/// IP6 Shim6 Protocol
pub const IPPROTO_SHIM6: ::c_int = 140;

/* 101-254: Partly Unassigned */
/// Protocol Independent Mcast
pub const IPPROTO_PIM: ::c_int = 103;
/// CARP
pub const IPPROTO_CARP: ::c_int = 112;
/// PGM
pub const IPPROTO_PGM: ::c_int = 113;
/// MPLS-in-IP
pub const IPPROTO_MPLS: ::c_int = 137;
/// PFSYNC
pub const IPPROTO_PFSYNC: ::c_int = 240;

/* 255: Reserved */
/* BSD Private, local use, namespace incursion, no longer used */
/// OLD divert pseudo-proto
pub const IPPROTO_OLD_DIVERT: ::c_int = 254;
pub const IPPROTO_MAX: ::c_int = 256;
/// last return value of *_input(), meaning "all job for this pkt is done".
pub const IPPROTO_DONE: ::c_int = 257;

/* Only used internally, so can be outside the range of valid IP protocols. */
/// divert pseudo-protocol
pub const IPPROTO_DIVERT: ::c_int = 258;
/// SeND pseudo-protocol
pub const IPPROTO_SEND: ::c_int = 259;

pub const IP_BINDANY: ::c_int = 24;

pub const PF_SLOW: ::c_int = AF_SLOW;
pub const PF_SCLUSTER: ::c_int = AF_SCLUSTER;
pub const PF_ARP: ::c_int = AF_ARP;
pub const PF_BLUETOOTH: ::c_int = AF_BLUETOOTH;
pub const PF_IEEE80211: ::c_int = AF_IEEE80211;
pub const PF_INET_SDP: ::c_int = AF_INET_SDP;
pub const PF_INET6_SDP: ::c_int = AF_INET6_SDP;
#[doc(hidden)]
pub const PF_MAX: ::c_int = AF_MAX;

pub const NET_RT_DUMP: ::c_int = 1;
pub const NET_RT_FLAGS: ::c_int = 2;
pub const NET_RT_IFLIST: ::c_int = 3;
pub const NET_RT_IFMALIST: ::c_int = 4;
pub const NET_RT_IFLISTL: ::c_int = 5;

pub const MAX_ASOC_IDS_RET: ::c_uint = 255;

#[doc(hidden)]
pub const SCTP_ALIGN_RESV_PAD: usize = 92;
#[doc(hidden)]
pub const SCTP_ALIGN_RESV_PAD_SHORT: usize = 76;

pub const SCTP_MAX_LOGGING_SIZE: ::c_uint = 30000;
pub const SCTP_TRACE_PARAMS: ::c_uint = 6;

pub const SCTP_RTOINFO: ::c_int = 0x01;
pub const SCTP_ASSOCINFO: ::c_int = 0x02;
pub const SCTP_INITMSG: ::c_int = 0x03;
pub const SCTP_NODELAY: ::c_int = 0x04;
pub const SCTP_AUTOCLOSE: ::c_int = 0x05;
pub const SCTP_SET_PEER_PRIMARY_ADDR: ::c_int = 0x06;
pub const SCTP_PRIMARY_ADDR: ::c_int = 0x07;
pub const SCTP_ADAPTATION_LAYER: ::c_int = 0x08;
pub const SCTP_ADAPTION_LAYER: ::c_int = 0x08;
pub const SCTP_DISABLE_FRAGMENTS: ::c_int = 0x09;
pub const SCTP_PEER_ADDR_PARAMS: ::c_int = 0x0a;
pub const SCTP_DEFAULT_SEND_PARAM: ::c_int = 0x0b;
/// deprecated
pub const SCTP_EVENTS: ::c_int = 0x0c;
pub const SCTP_I_WANT_MAPPED_V4_ADDR: ::c_int = 0x0d;
pub const SCTP_MAXSEG: ::c_int = 0x0e;
pub const SCTP_DELAYED_SACK: ::c_int = 0x0f;
pub const SCTP_FRAGMENT_INTERLEAVE: ::c_int = 0x10;
pub const SCTP_PARTIAL_DELIVERY_POINT: ::c_int = 0x11;
pub const SCTP_AUTH_CHUNK: ::c_int = 0x12;
pub const SCTP_AUTH_KEY: ::c_int = 0x13;
pub const SCTP_HMAC_IDENT: ::c_int = 0x14;
pub const SCTP_AUTH_ACTIVE_KEY: ::c_int = 0x15;
pub const SCTP_AUTH_DELETE_KEY: ::c_int = 0x16;
pub const SCTP_USE_EXT_RCVINFO: ::c_int = 0x17;
pub const SCTP_AUTO_ASCONF: ::c_int = 0x18;
pub const SCTP_MAXBURST: ::c_int = 0x19;
pub const SCTP_MAX_BURST: ::c_int = 0x1a;
pub const SCTP_EXPLICIT_EOR: ::c_int = 0x1b;
pub const SCTP_REUSE_PORT: ::c_int = 0x1c;
pub const SCTP_AUTH_DEACTIVATE_KEY: ::c_int = 0x1d;
pub const SCTP_EVENT: ::c_int = 0x1e;
pub const SCTP_RECVRCVINFO: ::c_int = 0x1f;
pub const SCTP_RECVNXTINFO: ::c_int = 0x20;
pub const SCTP_DEFAULT_SNDINFO: ::c_int = 0x21;
pub const SCTP_DEFAULT_PRINFO: ::c_int = 0x22;
pub const SCTP_PEER_ADDR_THLDS: ::c_int = 0x23;
pub const SCTP_REMOTE_UDP_ENCAPS_PORT: ::c_int = 0x24;
pub const SCTP_ECN_SUPPORTED: ::c_int = 0x25;
pub const SCTP_PR_SUPPORTED: ::c_int = 0x26;
pub const SCTP_AUTH_SUPPORTED: ::c_int = 0x27;
pub const SCTP_ASCONF_SUPPORTED: ::c_int = 0x28;
pub const SCTP_RECONFIG_SUPPORTED: ::c_int = 0x29;
pub const SCTP_NRSACK_SUPPORTED: ::c_int = 0x30;
pub const SCTP_PKTDROP_SUPPORTED: ::c_int = 0x31;
pub const SCTP_MAX_CWND: ::c_int = 0x32;

pub const SCTP_STATUS: ::c_int = 0x100;
pub const SCTP_GET_PEER_ADDR_INFO: ::c_int = 0x101;
pub const SCTP_PEER_AUTH_CHUNKS: ::c_int = 0x102;
pub const SCTP_LOCAL_AUTH_CHUNKS: ::c_int = 0x103;
pub const SCTP_GET_ASSOC_NUMBER: ::c_int = 0x104;
pub const SCTP_GET_ASSOC_ID_LIST: ::c_int = 0x105;
pub const SCTP_TIMEOUTS: ::c_int = 0x106;
pub const SCTP_PR_STREAM_STATUS: ::c_int = 0x107;
pub const SCTP_PR_ASSOC_STATUS: ::c_int = 0x108;

pub const SCTP_ENABLE_STREAM_RESET: ::c_int = 0x0900;
pub const SCTP_RESET_STREAMS: ::c_int = 0x0901;
pub const SCTP_RESET_ASSOC: ::c_int = 0x0902;
pub const SCTP_ADD_STREAMS: ::c_int = 0x0903;

pub const SCTP_ENABLE_RESET_STREAM_REQ: ::c_int = 0x0001;
pub const SCTP_ENABLE_RESET_ASSOC_REQ: ::c_int = 0x0002;
pub const SCTP_ENABLE_CHANGE_ASSOC_REQ: ::c_int = 0x0004;
pub const SCTP_ENABLE_VALUE_MASK: ::c_int = 0x0007;
pub const SCTP_STREAM_RESET_INCOMING: ::c_int = 0x0001;
pub const SCTP_STREAM_RESET_OUTGOING: ::c_int = 0x0002;

pub const SCTP_SET_DEBUG_LEVEL: ::c_int = 0x1005;
pub const SCTP_CLR_STAT_LOG: ::c_int = 0x1007;
pub const SCTP_CMT_ON_OFF: ::c_int = 0x1200;
pub const SCTP_CMT_USE_DAC: ::c_int = 0x1201;
pub const SCTP_PLUGGABLE_CC: ::c_int = 0x1202;
pub const SCTP_PLUGGABLE_SS: ::c_int = 0x1203;
pub const SCTP_SS_VALUE: ::c_int = 0x1204;
pub const SCTP_CC_OPTION: ::c_int = 0x1205;
pub const SCTP_INTERLEAVING_SUPPORTED: ::c_int = 0x1206;
pub const SCTP_GET_SNDBUF_USE: ::c_int = 0x1101;
pub const SCTP_GET_STAT_LOG: ::c_int = 0x1103;
pub const SCTP_PCB_STATUS: ::c_int = 0x1104;
pub const SCTP_GET_NONCE_VALUES: ::c_int = 0x1105;
pub const SCTP_SET_DYNAMIC_PRIMARY: ::c_int = 0x2001;

pub const SCTP_VRF_ID: ::c_int = 0x3001;
pub const SCTP_ADD_VRF_ID: ::c_int = 0x3002;
pub const SCTP_GET_VRF_IDS: ::c_int = 0x3003;
pub const SCTP_GET_ASOC_VRF: ::c_int = 0x3004;
pub const SCTP_DEL_VRF_ID: ::c_int = 0x3005;

pub const SCTP_GET_PACKET_LOG: ::c_int = 0x4001;

/// Standard TCP Congestion Control
pub const SCTP_CC_RFC2581: ::c_int = 0x00;
/// High Speed TCP Congestion Control (Floyd)
pub const SCTP_CC_HSTCP: ::c_int = 0x01;
/// HTCP Congestion Control
pub const SCTP_CC_HTCP: ::c_int = 0x02;
/// RTCC Congestion Control - RFC2581 plus
pub const SCTP_CC_RTCC: ::c_int = 0x03;

pub const SCTP_CC_OPT_RTCC_SETMODE: ::c_int = 0x2000;
pub const SCTP_CC_OPT_USE_DCCC_ECN: ::c_int = 0x2001;
pub const SCTP_CC_OPT_STEADY_STEP: ::c_int = 0x2002;

pub const SCTP_CMT_OFF: ::c_int = 0;
pub const SCTP_CMT_BASE: ::c_int = 1;
pub const SCTP_CMT_RPV1: ::c_int = 2;
pub const SCTP_CMT_RPV2: ::c_int = 3;
pub const SCTP_CMT_MPTCP: ::c_int = 4;
pub const SCTP_CMT_MAX: ::c_int = SCTP_CMT_MPTCP;

/// Default simple round-robin
pub const SCTP_SS_DEFAULT: ::c_int = 0;
/// Real round-robin
pub const SCTP_SS_ROUND_ROBIN: ::c_int = 1;
/// Real round-robin per packet
pub const SCTP_SS_ROUND_ROBIN_PACKET: ::c_int = 2;
/// Priority
pub const SCTP_SS_PRIORITY: ::c_int = 3;
/// Fair Bandwidth
pub const SCTP_SS_FAIR_BANDWITH: ::c_int = 4;
/// First-come, first-serve
pub const SCTP_SS_FIRST_COME: ::c_int = 5;

pub const SCTP_FRAG_LEVEL_0: ::c_int = 0;
pub const SCTP_FRAG_LEVEL_1: ::c_int = 1;
pub const SCTP_FRAG_LEVEL_2: ::c_int = 2;

pub const SCTP_CLOSED: ::c_int = 0x0000;
pub const SCTP_BOUND: ::c_int = 0x1000;
pub const SCTP_LISTEN: ::c_int = 0x2000;
pub const SCTP_COOKIE_WAIT: ::c_int = 0x0002;
pub const SCTP_COOKIE_ECHOED: ::c_int = 0x0004;
pub const SCTP_ESTABLISHED: ::c_int = 0x0008;
pub const SCTP_SHUTDOWN_SENT: ::c_int = 0x0010;
pub const SCTP_SHUTDOWN_RECEIVED: ::c_int = 0x0020;
pub const SCTP_SHUTDOWN_ACK_SENT: ::c_int = 0x0040;
pub const SCTP_SHUTDOWN_PENDING: ::c_int = 0x0080;

pub const SCTP_CAUSE_NO_ERROR: ::c_int = 0x0000;
pub const SCTP_CAUSE_INVALID_STREAM: ::c_int = 0x0001;
pub const SCTP_CAUSE_MISSING_PARAM: ::c_int = 0x0002;
pub const SCTP_CAUSE_STALE_COOKIE: ::c_int = 0x0003;
pub const SCTP_CAUSE_OUT_OF_RESC: ::c_int = 0x0004;
pub const SCTP_CAUSE_UNRESOLVABLE_ADDR: ::c_int = 0x0005;
pub const SCTP_CAUSE_UNRECOG_CHUNK: ::c_int = 0x0006;
pub const SCTP_CAUSE_INVALID_PARAM: ::c_int = 0x0007;
pub const SCTP_CAUSE_UNRECOG_PARAM: ::c_int = 0x0008;
pub const SCTP_CAUSE_NO_USER_DATA: ::c_int = 0x0009;
pub const SCTP_CAUSE_COOKIE_IN_SHUTDOWN: ::c_int = 0x000a;
pub const SCTP_CAUSE_RESTART_W_NEWADDR: ::c_int = 0x000b;
pub const SCTP_CAUSE_USER_INITIATED_ABT: ::c_int = 0x000c;
pub const SCTP_CAUSE_PROTOCOL_VIOLATION: ::c_int = 0x000d;

pub const SCTP_CAUSE_DELETING_LAST_ADDR: ::c_int = 0x00a0;
pub const SCTP_CAUSE_RESOURCE_SHORTAGE: ::c_int = 0x00a1;
pub const SCTP_CAUSE_DELETING_SRC_ADDR: ::c_int = 0x00a2;
pub const SCTP_CAUSE_ILLEGAL_ASCONF_ACK: ::c_int = 0x00a3;
pub const SCTP_CAUSE_REQUEST_REFUSED: ::c_int = 0x00a4;

pub const SCTP_CAUSE_NAT_COLLIDING_STATE: ::c_int = 0x00b0;
pub const SCTP_CAUSE_NAT_MISSING_STATE: ::c_int = 0x00b1;
pub const SCTP_CAUSE_UNSUPPORTED_HMACID: ::c_int = 0x0105;

pub const SCTP_DATA: ::c_int = 0x00;
pub const SCTP_INITIATION: ::c_int = 0x01;
pub const SCTP_INITIATION_ACK: ::c_int = 0x02;
pub const SCTP_SELECTIVE_ACK: ::c_int = 0x03;
pub const SCTP_HEARTBEAT_REQUEST: ::c_int = 0x04;
pub const SCTP_HEARTBEAT_ACK: ::c_int = 0x05;
pub const SCTP_ABORT_ASSOCIATION: ::c_int = 0x06;
pub const SCTP_SHUTDOWN: ::c_int = 0x07;
pub const SCTP_SHUTDOWN_ACK: ::c_int = 0x08;
pub const SCTP_OPERATION_ERROR: ::c_int = 0x09;
pub const SCTP_COOKIE_ECHO: ::c_int = 0x0a;
pub const SCTP_COOKIE_ACK: ::c_int = 0x0b;
pub const SCTP_ECN_ECHO: ::c_int = 0x0c;
pub const SCTP_ECN_CWR: ::c_int = 0x0d;
pub const SCTP_SHUTDOWN_COMPLETE: ::c_int = 0x0e;

/// RFC4895
pub const SCTP_AUTHENTICATION: ::c_int = 0x0f;
/// EY nr_sack chunk id
pub const SCTP_NR_SELECTIVE_ACK: ::c_int = 0x10;
pub const SCTP_IDATA: ::c_int = 0x40;
/// RFC5061
pub const SCTP_ASCONF_ACK: ::c_int = 0x80;
/// draft-ietf-stewart-pktdrpsctp
pub const SCTP_PACKET_DROPPED: ::c_int = 0x81;
/// draft-ietf-stewart-strreset-xxx
pub const SCTP_STREAM_RESET: ::c_int = 0x82;
/// RFC4820
pub const SCTP_PAD_CHUNK: ::c_int = 0x84;
/// RFC3758
pub const SCTP_FORWARD_CUM_TSN: ::c_int = 0xc0;
/// RFC5061
pub const SCTP_ASCONF: ::c_int = 0xc1;
/// RFC5061
pub const SCTP_IFORWARD_CUM_TSN: ::c_int = 0xc2;

/// ABORT and SHUTDOWN COMPLETE FLAG
pub const SCTP_HAD_NO_TCB: ::c_int = 0x01;
pub const SCTP_FROM_MIDDLE_BOX: ::c_int = SCTP_HAD_NO_TCB;
pub const SCTP_BADCRC: ::c_int = 0x02;
pub const SCTP_PACKET_TRUNCATED: ::c_int = 0x04;

pub const SCTP_CWR_REDUCE_OVERRIDE: ::c_int = 0x01;
pub const SCTP_CWR_IN_SAME_WINDOW: ::c_int = 0x02;

pub const SCTP_SAT_NETWORK_MIN: ::c_int = 400;
pub const SCTP_SAT_NETWORK_BURST_INCR: ::c_int = 2;

pub const SCTP_DATA_FRAG_MASK: ::c_int = 0x03;
pub const SCTP_DATA_MIDDLE_FRAG: ::c_int = 0x00;
pub const SCTP_DATA_LAST_FRAG: ::c_int = 0x01;
pub const SCTP_DATA_FIRST_FRAG: ::c_int = 0x02;
pub const SCTP_DATA_NOT_FRAG: ::c_int = 0x03;
pub const SCTP_DATA_UNORDERED: ::c_int = 0x04;
pub const SCTP_DATA_SACK_IMMEDIATELY: ::c_int = 0x08;

pub const SCTP_SACK_NONCE_SUM: ::c_int = 0x01;

/// CMT DAC algorithm SACK flag
pub const SCTP_SACK_CMT_DAC: ::c_int = 0x80;

pub const SCTP_PCB_FLAGS_UDPTYPE: ::c_int = 0x00000001;
pub const SCTP_PCB_FLAGS_TCPTYPE: ::c_int = 0x00000002;
pub const SCTP_PCB_FLAGS_BOUNDALL: ::c_int = 0x00000004;
pub const SCTP_PCB_FLAGS_ACCEPTING: ::c_int = 0x00000008;
pub const SCTP_PCB_FLAGS_UNBOUND: ::c_int = 0x00000010;
pub const SCTP_PCB_FLAGS_CLOSE_IP: ::c_int = 0x00040000;
pub const SCTP_PCB_FLAGS_WAS_CONNECTED: ::c_int = 0x00080000;
pub const SCTP_PCB_FLAGS_WAS_ABORTED: ::c_int = 0x00100000;

pub const SCTP_PCB_FLAGS_CONNECTED: ::c_int = 0x00200000;
pub const SCTP_PCB_FLAGS_IN_TCPPOOL: ::c_int = 0x00400000;
pub const SCTP_PCB_FLAGS_DONT_WAKE: ::c_int = 0x00800000;
pub const SCTP_PCB_FLAGS_WAKEOUTPUT: ::c_int = 0x01000000;
pub const SCTP_PCB_FLAGS_WAKEINPUT: ::c_int = 0x02000000;
pub const SCTP_PCB_FLAGS_BOUND_V6: ::c_int = 0x04000000;
pub const SCTP_PCB_FLAGS_BLOCKING_IO: ::c_int = 0x08000000;
pub const SCTP_PCB_FLAGS_SOCKET_GONE: ::c_int = 0x10000000;
pub const SCTP_PCB_FLAGS_SOCKET_ALLGONE: ::c_int = 0x20000000;
pub const SCTP_PCB_FLAGS_SOCKET_CANT_READ: ::c_int = 0x40000000;
pub const SCTP_PCB_COPY_FLAGS: ::c_int =
    SCTP_PCB_FLAGS_BOUNDALL
    | SCTP_PCB_FLAGS_WAKEINPUT
    | SCTP_PCB_FLAGS_BOUND_V6;

pub const SCTP_PCB_FLAGS_DO_NOT_PMTUD: ::c_long = 0x0000000000000001;
pub const SCTP_PCB_FLAGS_EXT_RCVINFO: ::c_long = 0x0000000000000002;
pub const SCTP_PCB_FLAGS_DONOT_HEARTBEAT: ::c_long =0x0000000000000004;
pub const SCTP_PCB_FLAGS_FRAG_INTERLEAVE: ::c_long = 0x0000000000000008;
pub const SCTP_PCB_FLAGS_INTERLEAVE_STRMS: ::c_long = 0x0000000000000010;
pub const SCTP_PCB_FLAGS_DO_ASCONF: ::c_long = 0x0000000000000020;
pub const SCTP_PCB_FLAGS_AUTO_ASCONF: ::c_long = 0x0000000000000040;

pub const SCTP_PCB_FLAGS_NODELAY: ::c_long = 0x0000000000000100;
pub const SCTP_PCB_FLAGS_AUTOCLOSE: ::c_long = 0x0000000000000200;
pub const SCTP_PCB_FLAGS_RECVDATAIOEVNT: ::c_long = 0x0000000000000400;
pub const SCTP_PCB_FLAGS_RECVASSOCEVNT: ::c_long = 0x0000000000000800;
pub const SCTP_PCB_FLAGS_RECVPADDREVNT: ::c_long = 0x0000000000001000;
pub const SCTP_PCB_FLAGS_RECVPEERERR: ::c_long = 0x0000000000002000;
pub const SCTP_PCB_FLAGS_RECVSENDFAILEVNT: ::c_long = 0x0000000000004000;
pub const SCTP_PCB_FLAGS_RECVSHUTDOWNEVNT: ::c_long = 0x0000000000008000;
pub const SCTP_PCB_FLAGS_ADAPTATIONEVNT: ::c_long = 0x0000000000010000;
pub const SCTP_PCB_FLAGS_PDAPIEVNT: ::c_long = 0x0000000000020000;
pub const SCTP_PCB_FLAGS_AUTHEVNT: ::c_long = 0x0000000000040000;
pub const SCTP_PCB_FLAGS_STREAM_RESETEVNT: ::c_long = 0x0000000000080000;
pub const SCTP_PCB_FLAGS_NO_FRAGMENT: ::c_long = 0x0000000000100000;
pub const SCTP_PCB_FLAGS_EXPLICIT_EOR: ::c_long = 0x0000000000400000;
pub const SCTP_PCB_FLAGS_NEEDS_MAPPED_V4: ::c_long = 0x0000000000800000;
pub const SCTP_PCB_FLAGS_MULTIPLE_ASCONFS: ::c_long = 0x0000000001000000;
pub const SCTP_PCB_FLAGS_PORTREUSE: ::c_long = 0x0000000002000000;
pub const SCTP_PCB_FLAGS_DRYEVNT: ::c_long = 0x0000000004000000;
pub const SCTP_PCB_FLAGS_RECVRCVINFO: ::c_long = 0x0000000008000000;
pub const SCTP_PCB_FLAGS_RECVNXTINFO: ::c_long = 0x0000000010000000;
pub const SCTP_PCB_FLAGS_ASSOC_RESETEVNT: ::c_long = 0x0000000020000000;
pub const SCTP_PCB_FLAGS_STREAM_CHANGEEVNT: ::c_long = 0x0000000040000000;
pub const SCTP_PCB_FLAGS_RECVNSENDFAILEVNT: ::c_long = 0x0000000080000000;

pub const SCTP_MOBILITY_BASE: ::c_int = 0x00000001;
pub const SCTP_MOBILITY_FASTHANDOFF: ::c_int = 0x00000002;
pub const SCTP_MOBILITY_PRIM_DELETED: ::c_int = 0x00000004;

/// smallest pmtu allowed when disabling PMTU discovery
pub const SCTP_SMALLEST_PMTU: ::c_int = 512;

pub const SCTP_FUTURE_ASSOC: ::c_int = 0;
pub const SCTP_CURRENT_ASSOC: ::c_int = 1;
pub const SCTP_ALL_ASSOC: ::c_int = 2;

pub const SCTP_INIT: ::c_int = 0x0001;
pub const SCTP_SNDRCV: ::c_int = 0x0002;
pub const SCTP_EXTRCV: ::c_int = 0x0003;
pub const SCTP_SNDINFO: ::c_int = 0x0004;
pub const SCTP_RCVINFO: ::c_int = 0x0005;
pub const SCTP_NXTINFO: ::c_int = 0x0006;
pub const SCTP_PRINFO: ::c_int = 0x0007;
pub const SCTP_AUTHINFO: ::c_int = 0x0008;
pub const SCTP_DSTADDRV4: ::c_int = 0x0009;
pub const SCTP_DSTADDRV6: ::c_int = 0x000a;

pub const SCTP_NO_NEXT_MSG: ::c_int = 0x0000;
pub const SCTP_NEXT_MSG_AVAIL: ::c_int = 0x0001;
pub const SCTP_NEXT_MSG_ISCOMPLETE: ::c_int = 0x0002;
pub const SCTP_NEXT_MSG_IS_UNORDERED: ::c_int = 0x0004;
pub const SCTP_NEXT_MSG_IS_NOTIFICATION: ::c_int = 0x0008;

pub const SCTP_RECVV_NOINFO: ::c_int = 0;
pub const SCTP_RECVV_RCVINFO: ::c_int = 1;
pub const SCTP_RECVV_NXTINFO: ::c_int = 2;
pub const SCTP_RECVV_RN: ::c_int = 3;

pub const SCTP_SENDV_NOINFO: ::c_int = 0;
pub const SCTP_SENDV_SNDINFO: ::c_int = 1;
pub const SCTP_SENDV_PRINFO: ::c_int = 2;
pub const SCTP_SENDV_AUTHINFO: ::c_int = 3;
pub const SCTP_SENDV_SPA: ::c_int = 4;

pub const SCTP_SEND_SNDINFO_VALID: ::c_int = 0x00000001;
pub const SCTP_SEND_PRINFO_VALID: ::c_int = 0x00000002;
pub const SCTP_SEND_AUTHINFO_VALID: ::c_int = 0x00000004;

/// next message is a notification
pub const SCTP_NOTIFICATION: ::c_int = 0x0010;
/// next message is complete
pub const SCTP_COMPLETE: ::c_int = 0x0020;
/// Start shutdown procedures
pub const SCTP_EOF: ::c_int = 0x0100;
/// Send an ABORT to peer
pub const SCTP_ABORT: ::c_int = 0x0200;
/// Message is un-ordered
pub const SCTP_UNORDERED: ::c_int = 0x0400;
/// Override the primary-address
pub const SCTP_ADDR_OVER: ::c_int = 0x0800;
/// Send this on all associations
pub const SCTP_SENDALL: ::c_int = 0x1000;
/// end of message signal
pub const SCTP_EOR: ::c_int = 0x2000;
/// Set I-Bit
pub const SCTP_SACK_IMMEDIATELY: ::c_int = 0x4000;

/// Reliable transfer
pub const SCTP_PR_SCTP_NONE: ::c_int = 0x0000;
/// Time based PR-SCTP
pub const SCTP_PR_SCTP_TTL: ::c_int = 0x0001;
/// Buffer based PR-SCTP
pub const SCTP_PR_SCTP_PRIO: ::c_int = 0x0002;
/// For backwards compatibility
pub const SCTP_PR_SCTP_BUF: ::c_int = SCTP_PR_SCTP_PRIO;
/// Number of retransmissions based PR-SCTP
pub const SCTP_PR_SCTP_RTX: ::c_int = 0x0003;
pub const SCTP_PR_SCTP_MAX: ::c_int = SCTP_PR_SCTP_RTX;
/// Used for aggregated stats
pub const SCTP_PR_SCTP_ALL: ::c_int = 0x000f;

/* sac_state values */
pub const SCTP_COMM_UP: ::c_int = 0x0001;
pub const SCTP_COMM_LOST: ::c_int = 0x0002;
pub const SCTP_RESTART: ::c_int = 0x0003;
pub const SCTP_SHUTDOWN_COMP: ::c_int = 0x0004;
pub const SCTP_CANT_STR_ASSOC: ::c_int = 0x0005;

/* sac_info values */
pub const SCTP_ASSOC_SUPPORTS_PR: ::c_int = 0x01;
pub const SCTP_ASSOC_SUPPORTS_AUTH: ::c_int = 0x02;
pub const SCTP_ASSOC_SUPPORTS_ASCONF: ::c_int = 0x03;
pub const SCTP_ASSOC_SUPPORTS_MULTIBUF: ::c_int = 0x04;
pub const SCTP_ASSOC_SUPPORTS_RE_CONFIG: ::c_int = 0x05;
pub const SCTP_ASSOC_SUPPORTS_INTERLEAVING: ::c_int = 0x06;
pub const SCTP_ASSOC_SUPPORTS_MAX: ::c_int = 0x06;

/* paddr state values */
pub const SCTP_ADDR_AVAILABLE: ::c_int = 0x0001;
pub const SCTP_ADDR_UNREACHABLE: ::c_int = 0x0002;
pub const SCTP_ADDR_REMOVED: ::c_int = 0x0003;
pub const SCTP_ADDR_ADDED: ::c_int = 0x0004;
pub const SCTP_ADDR_MADE_PRIM: ::c_int = 0x0005;
pub const SCTP_ADDR_CONFIRMED: ::c_int = 0x0006;

pub const SCTP_ACTIVE: ::c_int = 0x0001;
pub const SCTP_INACTIVE: ::c_int = 0x0002;
pub const SCTP_UNCONFIRMED: ::c_int = 0x0200;
pub const SCTP_NOHEARTBEAT: ::c_int = 0x0040;

/// inqueue never on wire
pub const SCTP_DATA_UNSENT: ::c_int = 0x0001;
/// on wire at failure
pub const SCTP_DATA_SENT: ::c_int = 0x0002;

pub const SCTP_PARTIAL_DELIVERY_ABORTED: ::c_int = 0x0001;

pub const SCTP_AUTH_NEW_KEY: ::c_int = 0x0001;
pub const SCTP_AUTH_NEWKEY: ::c_int = SCTP_AUTH_NEW_KEY;
pub const SCTP_AUTH_NO_AUTH: ::c_int = 0x0002;
pub const SCTP_AUTH_FREE_KEY: ::c_int = 0x0003;

/* flags in stream_reset_event (strreset_flags) */
pub const SCTP_STREAM_RESET_INCOMING_SSN: ::c_int = 0x0001;
pub const SCTP_STREAM_RESET_OUTGOING_SSN: ::c_int = 0x0002;
pub const SCTP_STREAM_RESET_DENIED: ::c_int = 0x0004;
pub const SCTP_STREAM_RESET_FAILED: ::c_int = 0x0008;

pub const SCTP_ASSOC_RESET_DENIED: ::c_int = 0x0004;
pub const SCTP_ASSOC_RESET_FAILED: ::c_int = 0x0008;

pub const SCTP_STREAM_CHANGE_DENIED: ::c_int = 0x0004;
pub const SCTP_STREAM_CHANGE_FAILED: ::c_int = 0x0008;

/* notification types */
pub const SCTP_ASSOC_CHANGE: ::c_int = 0x0001;
pub const SCTP_PEER_ADDR_CHANGE: ::c_int = 0x0002;
pub const SCTP_REMOTE_ERROR: ::c_int = 0x0003;
pub const SCTP_SEND_FAILED: ::c_int = 0x0004;
pub const SCTP_SHUTDOWN_EVENT: ::c_int = 0x0005;
pub const SCTP_ADAPTATION_INDICATION: ::c_int = 0x0006;
/// For backwards-compatibility with SCTP_ADAPTATION_INDICATION.
pub const SCTP_ADAPTION_INDICATION: ::c_int = 0x0006;
pub const SCTP_PARTIAL_DELIVERY_EVENT: ::c_int = 0x0007;
pub const SCTP_AUTHENTICATION_EVENT: ::c_int = 0x0008;
pub const SCTP_STREAM_RESET_EVENT: ::c_int = 0x0009;
pub const SCTP_SENDER_DRY_EVENT: ::c_int = 0x000a;
pub const SCTP_NOTIFICATIONS_STOPPED_EVENT: ::c_int = 0x000b;
pub const SCTP_ASSOC_RESET_EVENT: ::c_int = 0x000c;
pub const SCTP_STREAM_CHANGE_EVENT: ::c_int = 0x000d;
pub const SCTP_SEND_FAILED_EVENT: ::c_int = 0x000e;

pub const SPP_HB_ENABLE: ::c_int = 0x00000001;
pub const SPP_HB_DISABLE: ::c_int = 0x00000002;
pub const SPP_HB_DEMAND: ::c_int = 0x00000004;
pub const SPP_PMTUD_ENABLE: ::c_int = 0x00000008;
pub const SPP_PMTUD_DISABLE: ::c_int = 0x00000010;
pub const SPP_HB_TIME_IS_ZERO: ::c_int = 0x00000080;
pub const SPP_IPV6_FLOWLABEL: ::c_int = 0x00000100;
pub const SPP_DSCP: ::c_int = 0x00000200;
pub const SPP_IPV4_TOS: ::c_int = SPP_DSCP;

/* AUTH hmac_id */
pub const SCTP_AUTH_HMAC_ID_RSVD: ::c_int = 0x0000;
pub const SCTP_AUTH_HMAC_ID_SHA1: ::c_int = 0x0001;
pub const SCTP_AUTH_HMAC_ID_SHA256: ::c_int = 0x0003;

pub const SCTP_MAX_EXPLICT_STR_RESET: ::c_int = 1000;

/// the size of the packet collection buffer.
pub const SCTP_PACKET_LOG_SIZE: ::c_int = 65536;

pub const SCTP_MAX_SACK_DELAY: ::c_int = 500;
pub const SCTP_MAX_HB_INTERVAL: ::c_int = 14400000;
pub const SCTP_MAX_COOKIE_LIFE: ::c_int = 3600000;

/* Types of logging/KTR tracing that can be enabled. */
pub const SCTP_BLK_LOGGING_ENABLE: ::c_int = 0x00000001;
pub const SCTP_CWND_MONITOR_ENABLE: ::c_int = 0x00000002;
pub const SCTP_CWND_LOGGING_ENABLE: ::c_int = 0x00000004;
pub const SCTP_FLIGHT_LOGGING_ENABLE: ::c_int = 0x00000020;
pub const SCTP_FR_LOGGING_ENABLE: ::c_int = 0x00000040;
pub const SCTP_LOCK_LOGGING_ENABLE: ::c_int = 0x00000080;
pub const SCTP_MAP_LOGGING_ENABLE: ::c_int = 0x00000100;
pub const SCTP_MBCNT_LOGGING_ENABLE: ::c_int = 0x00000200;
pub const SCTP_MBUF_LOGGING_ENABLE: ::c_int = 0x00000400;
pub const SCTP_NAGLE_LOGGING_ENABLE: ::c_int = 0x00000800;
pub const SCTP_RECV_RWND_LOGGING_ENABLE: ::c_int = 0x00001000;
pub const SCTP_RTTVAR_LOGGING_ENABLE: ::c_int = 0x00002000;
pub const SCTP_SACK_LOGGING_ENABLE: ::c_int = 0x00004000;
pub const SCTP_SACK_RWND_LOGGING_ENABLE: ::c_int = 0x00008000;
pub const SCTP_SB_LOGGING_ENABLE: ::c_int = 0x00010000;
pub const SCTP_STR_LOGGING_ENABLE: ::c_int = 0x00020000;
pub const SCTP_WAKE_LOGGING_ENABLE: ::c_int = 0x00040000;
pub const SCTP_LOG_MAXBURST_ENABLE: ::c_int = 0x00080000;
pub const SCTP_LOG_RWND_ENABLE: ::c_int = 0x00100000;
pub const SCTP_LOG_SACK_ARRIVALS_ENABLE: ::c_int = 0x00200000;
pub const SCTP_LTRACE_CHUNK_ENABLE: ::c_int = 0x00400000;
pub const SCTP_LTRACE_ERROR_ENABLE: ::c_int = 0x00800000;
pub const SCTP_LAST_PACKET_TRACING: ::c_int = 0x01000000;
pub const SCTP_THRESHOLD_LOGGING: ::c_int = 0x02000000;
pub const SCTP_LOG_AT_SEND_2_SCTP: ::c_int = 0x04000000;
pub const SCTP_LOG_AT_SEND_2_OUTQ: ::c_int = 0x08000000;
pub const SCTP_LOG_TRY_ADVANCE: ::c_int = 0x10000000;

// System V IPC
pub const IPC_PRIVATE: ::key_t = 0;
pub const IPC_CREAT: ::c_int = 0o1000;
pub const IPC_EXCL: ::c_int = 0o2000;
pub const IPC_NOWAIT: ::c_int = 0o4000;
pub const IPC_RMID: ::c_int = 0;
pub const IPC_SET: ::c_int = 1;
pub const IPC_STAT: ::c_int = 2;
pub const IPC_INFO: ::c_int = 3;
pub const IPC_R : ::c_int = 0o400;
pub const IPC_W : ::c_int = 0o200;
pub const IPC_M : ::c_int = 0o10000;
pub const MSG_NOERROR: ::c_int = 0o10000;
pub const SHM_RDONLY: ::c_int = 0o10000;
pub const SHM_RND: ::c_int = 0o20000;
pub const SHM_R: ::c_int = 0o400;
pub const SHM_W: ::c_int = 0o200;
pub const SHM_LOCK: ::c_int = 11;
pub const SHM_UNLOCK: ::c_int = 12;
pub const SHM_STAT: ::c_int = 13;
pub const SHM_INFO: ::c_int = 14;
pub const SHM_ANON: *mut ::c_char = 1 as *mut ::c_char;

// The *_MAXID constants never should've been used outside of the
// FreeBSD base system.  And with the exception of CTL_P1003_1B_MAXID,
// they were all removed in svn r262489.  They remain here for backwards
// compatibility only, and are scheduled to be removed in libc 1.0.0.
#[doc(hidden)]
pub const NET_MAXID: ::c_int = AF_MAX;
#[doc(hidden)]
pub const CTL_MAXID: ::c_int = 10;
#[doc(hidden)]
pub const KERN_MAXID: ::c_int = 38;
#[doc(hidden)]
pub const HW_MAXID: ::c_int = 13;
#[doc(hidden)]
pub const USER_MAXID: ::c_int = 21;
#[doc(hidden)]
pub const CTL_P1003_1B_MAXID: ::c_int = 26;

pub const MSG_NOTIFICATION: ::c_int = 0x00002000;
pub const MSG_NBIO: ::c_int = 0x00004000;
pub const MSG_COMPAT: ::c_int = 0x00008000;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x00040000;
pub const MSG_NOSIGNAL: ::c_int = 0x20000;

pub const EMPTY: ::c_short = 0;
pub const BOOT_TIME: ::c_short = 1;
pub const OLD_TIME: ::c_short = 2;
pub const NEW_TIME: ::c_short = 3;
pub const USER_PROCESS: ::c_short = 4;
pub const INIT_PROCESS: ::c_short = 5;
pub const LOGIN_PROCESS: ::c_short = 6;
pub const DEAD_PROCESS: ::c_short = 7;
pub const SHUTDOWN_TIME: ::c_short = 8;

pub const LC_COLLATE_MASK: ::c_int = (1 << 0);
pub const LC_CTYPE_MASK: ::c_int = (1 << 1);
pub const LC_MESSAGES_MASK: ::c_int = (1 << 2);
pub const LC_MONETARY_MASK: ::c_int = (1 << 3);
pub const LC_NUMERIC_MASK: ::c_int = (1 << 4);
pub const LC_TIME_MASK: ::c_int = (1 << 5);
pub const LC_ALL_MASK: ::c_int = LC_COLLATE_MASK
                               | LC_CTYPE_MASK
                               | LC_MESSAGES_MASK
                               | LC_MONETARY_MASK
                               | LC_NUMERIC_MASK
                               | LC_TIME_MASK;

pub const WSTOPPED: ::c_int = 2; // same as WUNTRACED
pub const WCONTINUED: ::c_int = 4;
pub const WNOWAIT: ::c_int = 8;
pub const WEXITED: ::c_int = 16;
pub const WTRAPPED: ::c_int = 32;

// FreeBSD defines a great many more of these, we only expose the
// standardized ones.
pub const P_PID: idtype_t = 0;
pub const P_PGID: idtype_t = 2;
pub const P_ALL: idtype_t = 7;

pub const B460800: ::speed_t = 460800;
pub const B921600: ::speed_t = 921600;

pub const AT_FDCWD: ::c_int = -100;
pub const AT_EACCESS: ::c_int = 0x100;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: ::c_int = 0x400;
pub const AT_REMOVEDIR: ::c_int = 0x800;

pub const TABDLY: ::tcflag_t = 0x00000004;
pub const TAB0: ::tcflag_t = 0x00000000;
pub const TAB3: ::tcflag_t = 0x00000004;

pub const _PC_ACL_NFS4: ::c_int = 64;

pub const _SC_CPUSET_SIZE: ::c_int = 122;

pub const XU_NGROUPS: ::c_int = 16;
pub const XUCRED_VERSION: ::c_uint = 0;

f! {
    pub fn INVALID_SINFO_FLAG(x: ::c_int) -> bool {
        ((x & 0xfffffff0) &
        !(SCTP_EOF | SCTP_ABORT | SCTP_UNORDERED | SCTP_ADDR_OVER |
          SCTP_SENDALL | SCTP_EOR | SCTP_SACK_IMMEDIATELY)) != 0
    }

    pub fn PR_SCTP_POLICY(x: ::c_int) -> ::c_int {
        x & 0x0f
    }

    pub fn PR_SCTP_ENABLED(x: ::c_int) -> bool {
        PR_SCTP_POLICY(x) != SCTP_PR_SCTP_NONE &&
        PR_SCTP_POLICY(x) != SCTP_PR_SCTP_ALL
    }

    pub fn PR_SCTP_TTL_ENABLED(x: ::c_int) -> bool {
        PR_SCTP_POLICY(x) == SCTP_PR_SCTP_TTL
    }

    pub fn PR_SCTP_BUF_ENABLED(x: ::c_int) -> bool {
        PR_SCTP_POLICY(x) == SCTP_PR_SCTP_BUF
    }

    pub fn PR_SCTP_RTX_ENABLED(x: ::c_int) -> bool {
        PR_SCTP_POLICY(x) == SCTP_PR_SCTP_RTX
    }

    pub fn PR_SCTP_INVALID_POLICY(x: ::c_int) -> bool {
        PR_SCTP_POLICY(x) > SCTP_PR_SCTP_MAX
    }

    pub fn PR_SCTP_VALID_POLICY(x: ::c_int) -> bool {
        PR_SCTP_POLICY(x) <= SCTP_PR_SCTP_MAX
    }
}

extern {
    pub fn __error() -> *mut ::c_int;

    pub fn mprotect(addr: *const ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;

    pub fn clock_getres(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_settime(clk_id: ::clockid_t, tp: *const ::timespec) -> ::c_int;

    pub fn jail(jail: *mut ::jail) -> ::c_int;
    pub fn jail_attach(jid: ::c_int) -> ::c_int;
    pub fn jail_remove(jid: ::c_int) -> ::c_int;
    pub fn jail_get(iov: *mut ::iovec, niov: ::c_uint, flags: ::c_int)
                    -> ::c_int;
    pub fn jail_set(iov: *mut ::iovec, niov: ::c_uint, flags: ::c_int)
                    -> ::c_int;

    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t,
                           len: ::off_t) -> ::c_int;
    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t,
                         advise: ::c_int) -> ::c_int;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char,
                     suffixlen: ::c_int,
                     flags: ::c_int) -> ::c_int;

    pub fn getutxuser(user: *const ::c_char) -> *mut utmpx;
    pub fn setutxdb(_type: ::c_int, file: *const ::c_char) -> ::c_int;

    pub fn aio_waitcomplete(iocbp: *mut *mut aiocb,
                            timeout: *mut ::timespec) -> ::ssize_t;

    pub fn freelocale(loc: ::locale_t) -> ::c_int;
    pub fn waitid(idtype: idtype_t, id: ::id_t, infop: *mut ::siginfo_t,
                  options: ::c_int) -> ::c_int;

    pub fn ftok(pathname: *const ::c_char, proj_id: ::c_int) -> ::key_t;
    pub fn shmget(key: ::key_t, size: ::size_t, shmflg: ::c_int) -> ::c_int;
    pub fn shmat(shmid: ::c_int, shmaddr: *const ::c_void,
        shmflg: ::c_int) -> *mut ::c_void;
    pub fn shmdt(shmaddr: *const ::c_void) -> ::c_int;
    pub fn shmctl(shmid: ::c_int, cmd: ::c_int,
        buf: *mut ::shmid_ds) -> ::c_int;
    pub fn msgctl(msqid: ::c_int, cmd: ::c_int,
        buf: *mut ::msqid_ds) -> ::c_int;
    pub fn msgget(key: ::key_t, msgflg: ::c_int) -> ::c_int;
    pub fn msgrcv(msqid: ::c_int, msgp: *mut ::c_void, msgsz: ::size_t,
        msgtyp: ::c_long, msgflg: ::c_int) -> ::c_int;
    pub fn msgsnd(msqid: ::c_int, msgp: *const ::c_void, msgsz: ::size_t,
        msgflg: ::c_int) -> ::c_int;
    pub fn cfmakesane(termios: *mut ::termios);
    pub fn fexecve(fd: ::c_int, argv: *const *const ::c_char,
                   envp: *const *const ::c_char)
                   -> ::c_int;

    // SCTP support
    pub fn sctp_peeloff(sd: ::c_int,
                        assoc_id: sctp_assoc_t) -> ::c_int;
    pub fn sctp_bindx(sd: ::c_int,
                      addrs: *const ::sockaddr,
                      addrcnt: ::c_int,
                      flags: ::c_int) -> ::c_int;
    pub fn sctp_connectx(sd: ::c_int,
                         addrs: *const ::sockaddr,
                         addrcnt: ::c_int,
                         id: *mut sctp_assoc_t) -> ::c_int;
    pub fn sctp_getaddrlen(family: sa_family_t) -> ::c_int;
    pub fn sctp_getpaddrs(sd: ::c_int,
                          id: sctp_assoc_t,
                          raddrs: *mut *mut ::sockaddr) -> ::c_int;
    pub fn sctp_freepaddrs(addrs: *const ::sockaddr) -> ::c_void;
    pub fn sctp_getladdrs(sd: ::c_int,
                          id: sctp_assoc_t,
                          raddrs: *mut *mut ::sockaddr) -> ::c_int;
    pub fn sctp_freeladdrs(addrs: *const ::sockaddr) -> ::c_void;
    pub fn sctp_opt_info(sd: ::c_int,
                         id: sctp_assoc_t,
                         opt: ::c_int,
                         arg: *mut ::c_void,
                         size: *mut ::socklen_t) -> ::c_int;
    pub fn sctp_sendmsg(s: ::c_int,
                        data: *const ::c_void,
                        len: ::size_t,
                        to: *const ::sockaddr,
                        tolen: ::socklen_t,
                        ppid: ::uint32_t,
                        flags: ::uint32_t,
                        stream_no: ::uint16_t,
                        timetolive: ::uint32_t,
                        context: ::uint32_t) -> ::ssize_t;
    pub fn sctp_send(sd: ::c_int,
                     data: *const ::c_void,
                     len: ::size_t,
                     sinfo: *const sctp_sndrcvinfo,
                     flags: ::c_int) -> ::ssize_t;
    pub fn sctp_sendx(sd: ::c_int,
                      msg: *const ::c_void,
                      msg_len: ::size_t,
                      addrs: *mut ::sockaddr,
                      addrcnt: ::c_int,
                      sinfo: *mut sctp_sndrcvinfo,
                      flags: ::c_int) -> ::ssize_t;
    pub fn sctp_sendmsgx(sd: ::c_int,
                         msg: *const ::c_void,
                         len: ::size_t,
                         addrs: *mut ::sockaddr,
                         addrcnt: ::c_int,
                         ppid: ::uint32_t,
                         flags: ::uint32_t,
                         stream_no: ::uint16_t,
                         timetolive: ::uint32_t,
                         context: ::uint32_t) -> ::ssize_t;
    pub fn sctp_getassocid(sd: ::c_int,
                           sa: *mut ::sockaddr) -> sctp_assoc_t;
    pub fn sctp_recvmsg(s: ::c_int,
                        dbuf: *mut ::c_void,
                        len: ::size_t,
                        from: *mut ::sockaddr,
                        fromlen: *mut ::socklen_t,
                        sinfo: *mut sctp_sndrcvinfo,
                        msg_flags: *mut ::c_int) -> ::ssize_t;
    pub fn sctp_sendv(sd: ::c_int,
                      iov: *const iovec,
                      iovcnt: ::c_int,
                      addrs: *mut ::sockaddr,
                      addrcnt: ::c_int,
                      info: *mut ::c_void,
                      infolen: ::socklen_t,
                      infotype: ::c_uint,
                      flags: ::c_int) -> ::ssize_t;
    pub fn sctp_recvv(sd: ::c_int,
                      iov: *const iovec,
                      iovlen: ::c_int,
                      from: *mut ::sockaddr,
                      fromlen: *mut ::socklen_t,
                      info: *mut ::c_void,
                      infolen: *mut ::socklen_t,
                      infotype: *mut ::c_uint,
                      flags: *mut ::c_int) -> ::ssize_t;
}

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        mod x86;
        pub use self::x86::*;
    } else if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use self::x86_64::*;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::*;
    } else {
        // Unknown target_arch
    }
}
