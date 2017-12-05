pub type clock_t = ::c_uint;
pub type suseconds_t = ::c_int;
pub type dev_t = u64;
pub type blksize_t = ::int32_t;
pub type fsblkcnt_t = ::uint64_t;
pub type fsfilcnt_t = ::uint64_t;
pub type idtype_t = ::c_int;

pub type sctp_assoc_t = ::uint32_t;

// Defined in netinet/sctp_structs.h
// TODO: Define this struct properly, but we'll need to figure out
//       how to properly represent the `TAILQ_ENTRY(sctp_nets)` field
//       in the struct (which is some kind of linked-list pointer).
pub type sctp_nets = ::c_void;

s! {
    pub struct aiocb {
        pub aio_offset: ::off_t,
        pub aio_buf: *mut ::c_void,
        pub aio_nbytes: ::size_t,
        pub aio_fildes: ::c_int,
        pub aio_lio_opcode: ::c_int,
        pub aio_reqprio: ::c_int,
        pub aio_sigevent: ::sigevent,
        _state: ::c_int,
        _errno: ::c_int,
        _retval: ::ssize_t
    }

    pub struct dirent {
        pub d_fileno: ::ino_t,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [::c_char; 512],
    }

    pub struct glob_t {
        pub gl_pathc:   ::size_t,
        pub gl_matchc:  ::size_t,
        pub gl_offs:    ::size_t,
        pub gl_flags:   ::c_int,
        pub gl_pathv:   *mut *mut ::c_char,

        __unused3: *mut ::c_void,

        __unused4: *mut ::c_void,
        __unused5: *mut ::c_void,
        __unused6: *mut ::c_void,
        __unused7: *mut ::c_void,
        __unused8: *mut ::c_void,
    }

    pub struct sigevent {
        pub sigev_notify: ::c_int,
        pub sigev_signo: ::c_int,
        pub sigev_value: ::sigval,
        __unused1: *mut ::c_void,       //actually a function pointer
        pub sigev_notify_attributes: *mut ::c_void
    }

    pub struct sigset_t {
        __bits: [u32; 4],
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_mode: ::mode_t,
        pub st_ino: ::ino_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_atime: ::time_t,
        pub st_atimensec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtimensec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctimensec: ::c_long,
        pub st_birthtime: ::time_t,
        pub st_birthtimensec: ::c_long,
        pub st_size: ::off_t,
        pub st_blocks: ::blkcnt_t,
        pub st_blksize: ::blksize_t,
        pub st_flags: ::uint32_t,
        pub st_gen: ::uint32_t,
        pub st_spare: [::uint32_t; 2],
    }

    pub struct statvfs {
        pub f_flag: ::c_ulong,
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_iosize: ::c_ulong,

        pub f_blocks: ::fsblkcnt_t,
        pub f_bfree: ::fsblkcnt_t,
        pub f_bavail: ::fsblkcnt_t,
        pub f_bresvd: ::fsblkcnt_t,

        pub f_files: ::fsfilcnt_t,
        pub f_ffree: ::fsfilcnt_t,
        pub f_favail: ::fsfilcnt_t,
        pub f_fresvd: ::fsfilcnt_t,

        pub f_syncreads: ::uint64_t,
        pub f_syncwrites: ::uint64_t,

        pub f_asyncreads: ::uint64_t,
        pub f_asyncwrites: ::uint64_t,

        pub f_fsidx: ::fsid_t,
        pub f_fsid: ::c_ulong,
        pub f_namemax: ::c_ulong,
        pub f_owner: ::uid_t,

        pub f_spare: [::uint32_t; 4],

        pub f_fstypename: [::c_char; 32],
        pub f_mntonname: [::c_char; 1024],
        pub f_mntfromname: [::c_char; 1024],
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: ::socklen_t,
        pub ai_canonname: *mut ::c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut ::addrinfo,
    }

    pub struct sockaddr_storage {
        pub ss_len: u8,
        pub ss_family: ::sa_family_t,
        __ss_pad1: [u8; 6],
        __ss_pad2: i64,
        __ss_pad3: [u8; 112],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_errno: ::c_int,
        __pad1: ::c_int,
        pub si_addr: *mut ::c_void,
        __pad2: [u64; 13],
    }

    pub struct pthread_attr_t {
        pta_magic: ::c_uint,
        pta_flags: ::c_int,
        pta_private: *mut ::c_void,
    }

    pub struct pthread_mutex_t {
        ptm_magic: ::c_uint,
        ptm_errorcheck: ::c_uchar,
        ptm_pad1: [u8; 3],
        ptm_interlock: ::c_uchar,
        ptm_pad2: [u8; 3],
        ptm_owner: ::pthread_t,
        ptm_waiters: *mut u8,
        ptm_recursed: ::c_uint,
        ptm_spare2: *mut ::c_void,
    }

    pub struct pthread_mutexattr_t {
        ptma_magic: ::c_uint,
        ptma_private: *mut ::c_void,
    }

    pub struct pthread_rwlockattr_t {
        ptra_magic: ::c_uint,
        ptra_private: *mut ::c_void,
    }

    pub struct pthread_cond_t {
        ptc_magic: ::c_uint,
        ptc_lock: ::c_uchar,
        ptc_waiters_first: *mut u8,
        ptc_waiters_last: *mut u8,
        ptc_mutex: *mut ::pthread_mutex_t,
        ptc_private: *mut ::c_void,
    }

    pub struct pthread_condattr_t {
        ptca_magic: ::c_uint,
        ptca_private: *mut ::c_void,
    }

    pub struct pthread_rwlock_t {
        ptr_magic: ::c_uint,
        ptr_interlock: ::c_uchar,
        ptr_rblocked_first: *mut u8,
        ptr_rblocked_last: *mut u8,
        ptr_wblocked_first: *mut u8,
        ptr_wblocked_last: *mut u8,
        ptr_nreaders: ::c_uint,
        ptr_owner: ::pthread_t,
        ptr_private: *mut ::c_void,
    }

    pub struct kevent {
        pub ident: ::uintptr_t,
        pub filter: ::uint32_t,
        pub flags: ::uint32_t,
        pub fflags: ::uint32_t,
        pub data: ::int64_t,
        pub udata: ::intptr_t,
    }

    pub struct dqblk {
        pub dqb_bhardlimit: ::uint32_t,
        pub dqb_bsoftlimit: ::uint32_t,
        pub dqb_curblocks: ::uint32_t,
        pub dqb_ihardlimit: ::uint32_t,
        pub dqb_isoftlimit: ::uint32_t,
        pub dqb_curinodes: ::uint32_t,
        pub dqb_btime: ::int32_t,
        pub dqb_itime: ::int32_t,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *const ::c_void,
    }

    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
        pub int_n_sign_posn: ::c_char,
    }

    pub struct if_data {
        pub ifi_type: ::c_uchar,
        pub ifi_addrlen: ::c_uchar,
        pub ifi_hdrlen: ::c_uchar,
        pub ifi_link_state: ::c_int,
        pub ifi_mtu: u64,
        pub ifi_metric: u64,
        pub ifi_baudrate: u64,
        pub ifi_ipackets: u64,
        pub ifi_ierrors: u64,
        pub ifi_opackets: u64,
        pub ifi_oerrors: u64,
        pub ifi_collisions: u64,
        pub ifi_ibytes: u64,
        pub ifi_obytes: u64,
        pub ifi_imcasts: u64,
        pub ifi_omcasts: u64,
        pub ifi_iqdrops: u64,
        pub ifi_noproto: u64,
        pub ifi_lastchange: ::timespec,
    }

    pub struct if_msghdr {
        pub ifm_msglen: ::c_ushort,
        pub ifm_version: ::c_uchar,
        pub ifm_type: ::c_uchar,
        pub ifm_addrs: ::c_int,
        pub ifm_flags: ::c_int,
        pub ifm_index: ::c_ushort,
        pub ifm_data: if_data,
    }

    // SCTP support
    // netinet/sctp.h
    // netinet/sctp_uio.h

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
        pub chunk_type: ::uint8_t,
        /// chunk flags
        pub chunk_flags: ::uint8_t,
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

    /// On/Off setup for subscription to events
    #[repr(packed)]
    pub struct sctp_event_subscribe {
        pub sctp_data_io_event: ::uint8_t,
        pub sctp_association_event: ::uint8_t,
        pub sctp_address_event: ::uint8_t,
        pub sctp_send_failure_event: ::uint8_t,
        pub sctp_peer_error_event: ::uint8_t,
        pub sctp_shutdown_event: ::uint8_t,
        pub sctp_partial_delivery_event: ::uint8_t,
        pub sctp_adaption_layer_event: ::uint8_t,
        pub sctp_stream_reset_events: ::uint8_t,
    }

    #[repr(packed)]
    pub struct sctp_initmsg {
        pub sinit_num_ostreams: ::uint32_t,
        pub sinit_max_instreams: ::uint32_t,
        pub sinit_max_attempts: ::uint16_t,
        pub sinit_max_init_timeo: ::uint16_t,
    }

    #[repr(packed)]
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
    }

    #[repr(packed)]
    pub struct sctp_snd_all_completes {
        pub sall_stream: ::uint16_t,
        pub sall_flags: ::uint16_t,
        pub sall_ppid: ::uint32_t,
        pub sall_context: ::uint32_t,
        pub sall_num_sent: ::uint32_t,
        pub sall_num_failed: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_pcbinfo {
        pub ep_count: ::uint32_t,
        pub asoc_count: ::uint32_t,
        pub laddr_count: ::uint32_t,
        pub raddr_count: ::uint32_t,
        pub chk_count: ::uint32_t,
        pub sockq_count: ::uint32_t,
        pub mbuf_track: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_sockstat {
        pub ss_assoc_id: sctp_assoc_t,
        pub ss_total_sndbuf: ::uint32_t,
        pub ss_total_mbuf_sndbuf: ::uint32_t,
        pub ss_total_recv_buf: ::uint32_t,
    }

    /// association change events
    #[repr(packed)]
    pub struct sctp_assoc_change {
        pub sac_type: ::uint16_t,
        pub sac_flags: ::uint16_t,
        pub sac_length: ::uint32_t,
        pub sac_state: ::uint16_t,
        pub sac_error: ::uint16_t,
        pub sac_outbound_streams: ::uint16_t,
        pub sac_inbound_streams: ::uint16_t,
        pub sac_assoc_id: sctp_assoc_t,
    }

    /// Address events
    #[repr(packed)]
    pub struct sctp_paddr_change {
        pub spc_type: ::uint16_t,
        pub spc_flags: ::uint16_t,
        pub spc_length: ::uint32_t,
        pub spc_aaddr: sockaddr_storage,
        pub spc_state: ::uint32_t,
        pub spc_error: ::uint32_t,
        pub spc_assoc_id: sctp_assoc_t,
    }

    /// remote error events
    #[repr(packed)]
    pub struct sctp_remote_error {
        pub sre_type: ::uint16_t,
        pub sre_flags: ::uint16_t,
        pub sre_length: ::uint32_t,
        pub sre_error: ::uint16_t,
        pub sre_assoc_id: sctp_assoc_t,
        pub sre_data: [::uint8_t; 4],
    }

    /// data send failure event
    #[repr(packed)]
    pub struct sctp_send_failed {
        pub ssf_type: ::uint16_t,
        pub ssf_flags: ::uint16_t,
        pub ssf_length: ::uint32_t,
        pub ssf_error: ::uint32_t,
        pub ssf_info: sctp_sndrcvinfo,
        pub ssf_assoc_id: sctp_assoc_t,
        pub ssf_data: [::uint8_t; 4],
    }

    /// shutdown event
    #[repr(packed)]
    pub struct sctp_shutdown_event {
        pub sse_type: ::uint16_t,
        pub sse_flags: ::uint16_t,
        pub sse_length: ::uint32_t,
        pub sse_assoc_id: sctp_assoc_t,
    }

    /// Adaption layer indication stuff
    #[repr(packed)]
    pub struct sctp_adaption_event {
        pub sai_type: ::uint16_t,
        pub sai_flags: ::uint16_t,
        pub sai_length: ::uint32_t,
        pub sai_adaption_ind: ::uint32_t,
        pub sai_assoc_id: sctp_assoc_t,
    }

    #[repr(packed)]
    pub struct sctp_setadaption {
        pub ssb_adaption_ind: ::uint32_t,
    }

    /// Partial Delivery API event
    #[repr(packed)]
    pub struct sctp_pdapi_event {
        pub pdapi_type: ::uint16_t,
        pub pdapi_flags: ::uint16_t,
        pub pdapi_length: ::uint32_t,
        pub pdapi_indication: ::uint32_t,
        pub pdapi_assoc_id: sctp_assoc_t,
    }

    /// stream reset stuff
    #[repr(packed)]
    pub struct sctp_stream_reset_event {
        pub strreset_type: ::uint16_t,
        pub strreset_flags: ::uint16_t,
        pub strreset_length: ::uint32_t,
        pub strreset_assoc_id: sctp_assoc_t,
        pub strreset_list: [::uint16_t; 0]
    }

    #[repr(packed)]
    pub struct sctp_assoc_ids {
        // array of index's start at 0
        pub asls_assoc_start: ::uint16_t,
        pub asls_numb_present: ::uint8_t,
        pub asls_more_to_get: ::uint8_t,
        pub asls_assoc_id: [sctp_assoc_t; MAX_ASOC_IDS_RET as usize],
    }

    #[repr(packed)]
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
    #[repr(packed)]
    pub struct sctp_notification {
    //pub union sctp_notification {
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
        // pub sn_adaption_event: sctp_adaption_event,
        // pub sn_pdapi_event: sctp_pdapi_event,
        // pub sn_strreset_event: sctp_strreset_event,
    }

    #[repr(packed)]
    pub struct sctp_paddrparams {
        pub spp_assoc_id: sctp_assoc_t,
        pub spp_address: sockaddr_storage,
        pub spp_hbinterval: ::uint32_t,
        pub spp_pathmaxrxt: ::uint16_t,
    }

    #[repr(packed)]
    pub struct sctp_paddrinfo {
        pub spinfo_assoc_id: sctp_assoc_t,
        pub spinfo_address: sockaddr_storage,
        pub spinfo_state: ::int32_t,
        pub spinfo_cwnd: ::uint32_t,
        pub spinfo_srtt: ::uint32_t,
        pub spinfo_rto: ::uint32_t,
        pub spinfo_mtu: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_rtoinfo {
        pub srto_assoc_id: sctp_assoc_t,
        pub srto_initial: ::uint32_t,
        pub srto_max: ::uint32_t,
        pub srto_min: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_assocparams {
        pub sasoc_assoc_id: sctp_assoc_t,
        pub sasoc_asocmaxrxt: ::uint16_t,
        pub sasoc_number_peer_destinations: ::uint16_t,
        pub sasoc_peer_rwnd: ::uint32_t,
        pub sasoc_local_rwnd: ::uint32_t,
        pub sasoc_cookie_life: ::uint32_t,
    }

    #[repr(packed)]
    pub struct sctp_setprim {
        pub ssp_assoc_id: sctp_assoc_t,
        pub ssp_addr: sockaddr_storage,
    }

    #[repr(packed)]
    pub struct sctp_setpeerprim {
        pub sspp_assoc_id: sctp_assoc_t,
        pub sspp_addr: sockaddr_storage,
    }

    #[repr(packed)]
    pub struct sctp_getaddresses {
        pub sget_assoc_id: sctp_assoc_t,
        // addr is filled in for N * sockaddr_storage
        pub addr: [::sockaddr; 1],
    }

    #[repr(packed)]
    pub struct sctp_setstrm_timeout {
        pub ssto_assoc_id: sctp_assoc_t,
        pub ssto_timeout: ::uint32_t,
        pub ssto_streamid_start: ::uint32_t,
        pub ssto_streamid_end: ::uint32_t,
    }

    #[repr(packed)]
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

    #[repr(packed)]
    pub struct sctp_cwnd_args {
        /// network to
        pub net: *mut sctp_nets,
        /// cwnd in k
        pub cwnd_new_value: ::uint32_t,
        /// flightsize in k
        pub inflight: ::uint32_t,
        /// increment to it
        pub cwnd_augment: ::c_int,
    }

    #[repr(packed)]
    pub struct sctp_blk_args {
        /// in 1k bytes
        pub onmb: ::uint32_t,
        /// in 1k bytes
        pub onsb: ::uint32_t,
        /// in 1k bytes
        pub maxmb: ::uint16_t,
        /// in 1k bytes
        pub maxsb: ::uint16_t,
        /// chnk cnt
        pub send_sent_qcnt: ::uint16_t,
        /// chnk cnt
        pub stream_qcnt: ::uint16_t,
    }

    #[repr(packed)]
    pub struct sctp_stream_reset {
        pub strrst_assoc_id: sctp_assoc_t,
        pub strrst_flags: ::uint16_t,
        /// 0 == ALL
        pub strrst_num_streams: ::uint16_t,
        /// list if strrst_num_streams is not 0
        pub strrst_list: [::uint16_t; 0]
    }

    #[repr(packed)]
    pub struct sctp_get_nonce_values {
        pub gn_assoc_id: sctp_assoc_t,
        pub gn_peers_tag: ::uint32_t,
        pub gn_local_tag: ::uint32_t,
    }

    // TODO: Uncomment the structs and union below once libc targets
    //       a version of Rust which includes `union` support.

    // #[repr(packed)]
    // pub struct sctp_str_log {
    //     pub n_tsn: ::uint32_t,
    //     pub e_tsn: ::uint32_t,
    //     pub n_sseq: ::uint16_t,
    //     pub e_sseq: ::uint16_t,
    // }

    // #[repr(packed)]
    // pub struct sctp_fr_log {
    //     pub largest_tsn: ::uint32_t,
    //     pub largest_new_tsn: ::uint32_t,
    //     pub tsn: ::uint32_t,
    // }

    // #[repr(packed)]
    // pub struct sctp_fr_map {
    //     pub base: ::uint32_t,
    //     pub cum: ::uint32_t,
    //     pub high: ::uint32_t,
    // }

    // #[repr(packed)]
    // pub struct sctp_rwnd_log {
    //     pub rwnd: ::uint32_t,
    //     pub send_size: ::uint32_t,
    //     pub overhead: ::uint32_t,
    //     pub new_rwnd: ::uint32_t,
    // }

    // #[repr(packed)]
    // pub struct sctp_mbcnt_log {
    //     pub total_queue_size: ::uint32_t,
    //     pub size_change: ::uint32_t,
    //     pub total_queue_mb_size: ::uint32_t,
    //     pub mbcnt_change: ::uint32_t,
    // }

    // #[repr(packed)]
    // pub struct sctp_cwnd_log {
    //     pub x: union {
    //         pub blk: sctp_blk_args,
    //         pub cwnd: sctp_cwnd_args,
    //         pub strlog: sctp_str_log,
    //         pub fr: sctp_fr_log,
    //         pub map: sctp_fr_map,
    //         pub rwnd: sctp_rwnd_log,
    //         pub mbcnt: sctp_mbcnt_log,
    //     }
    //     pub from: ::uint8_t,
    //     pub event_type: ::uint8_t,
    // }

    // #[repr(packed)]
    // pub struct sctp_cwnd_log_req {
    //     /// Number in log
    //     pub num_in_log: ::c_int,
    //     /// Number returned
    //     pub num_ret: ::c_int,
    //     /// start at this one
    //     pub start_at: ::c_int,
    //     /// end at this one
    //     pub end_at: ::c_int,
    //     pub log: [sctp_cwnd_log; 0]
    // }
}

pub const AT_FDCWD: ::c_int = -100;
pub const AT_EACCESS: ::c_int = 0x100;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: ::c_int = 0x400;
pub const AT_REMOVEDIR: ::c_int = 0x800;

pub const LC_COLLATE_MASK: ::c_int = (1 << ::LC_COLLATE);
pub const LC_CTYPE_MASK: ::c_int = (1 << ::LC_CTYPE);
pub const LC_MONETARY_MASK: ::c_int = (1 << ::LC_MONETARY);
pub const LC_NUMERIC_MASK: ::c_int = (1 << ::LC_NUMERIC);
pub const LC_TIME_MASK: ::c_int = (1 << ::LC_TIME);
pub const LC_MESSAGES_MASK: ::c_int = (1 << ::LC_MESSAGES);
pub const LC_ALL_MASK: ::c_int = !0;

pub const ERA: ::nl_item = 52;
pub const ERA_D_FMT: ::nl_item = 53;
pub const ERA_D_T_FMT: ::nl_item = 54;
pub const ERA_T_FMT: ::nl_item = 55;
pub const ALT_DIGITS: ::nl_item = 56;

pub const O_CLOEXEC: ::c_int = 0x400000;
pub const O_ALT_IO: ::c_int = 0x40000;
pub const O_NOSIGPIPE: ::c_int = 0x1000000;
pub const O_SEARCH: ::c_int = 0x800000;
pub const O_DIRECTORY: ::c_int = 0x200000;
pub const O_DIRECT : ::c_int = 0x00080000;
pub const O_RSYNC : ::c_int = 0x00020000;

pub const MS_SYNC : ::c_int = 0x4;
pub const MS_INVALIDATE : ::c_int = 0x2;

pub const RLIM_NLIMITS: ::c_int = 12;

pub const EIDRM: ::c_int = 82;
pub const ENOMSG: ::c_int = 83;
pub const EOVERFLOW: ::c_int = 84;
pub const EILSEQ: ::c_int = 85;
pub const ENOTSUP: ::c_int = 86;
pub const ECANCELED: ::c_int = 87;
pub const EBADMSG: ::c_int = 88;
pub const ENODATA: ::c_int = 89;
pub const ENOSR: ::c_int = 90;
pub const ENOSTR: ::c_int = 91;
pub const ETIME: ::c_int = 92;
pub const ENOATTR: ::c_int = 93;
pub const EMULTIHOP: ::c_int = 94;
pub const ENOLINK: ::c_int = 95;
pub const EPROTO: ::c_int = 96;
pub const ELAST: ::c_int = 96;

pub const F_DUPFD_CLOEXEC : ::c_int = 12;
pub const F_CLOSEM: ::c_int = 10;
pub const F_GETNOSIGPIPE: ::c_int = 13;
pub const F_SETNOSIGPIPE: ::c_int = 14;
pub const F_MAXFD: ::c_int = 11;

pub const IPV6_JOIN_GROUP: ::c_int = 12;
pub const IPV6_LEAVE_GROUP: ::c_int = 13;

pub const SOCK_CONN_DGRAM: ::c_int = 6;
pub const SOCK_DCCP: ::c_int = SOCK_CONN_DGRAM;
pub const SOCK_NOSIGPIPE: ::c_int = 0x40000000;
pub const SOCK_FLAGS_MASK: ::c_int = 0xf0000000;

pub const SO_SNDTIMEO: ::c_int = 0x100b;
pub const SO_RCVTIMEO: ::c_int = 0x100c;
pub const SO_ACCEPTFILTER: ::c_int = 0x1000;
pub const SO_TIMESTAMP: ::c_int = 0x2000;
pub const SO_OVERFLOWED: ::c_int = 0x1009;
pub const SO_NOHEADER: ::c_int = 0x100a;

// https://github.com/NetBSD/src/blob/trunk/sys/net/if.h#L373
pub const IFF_UP: ::c_int = 0x0001; // interface is up
pub const IFF_BROADCAST: ::c_int = 0x0002; // broadcast address valid
pub const IFF_DEBUG: ::c_int = 0x0004; // turn on debugging
pub const IFF_LOOPBACK: ::c_int = 0x0008; // is a loopback net
pub const IFF_POINTOPOINT: ::c_int = 0x0010; // interface is point-to-point link
pub const IFF_NOTRAILERS: ::c_int = 0x0020; // avoid use of trailers
pub const IFF_RUNNING: ::c_int = 0x0040; // resources allocated
pub const IFF_NOARP: ::c_int = 0x0080; // no address resolution protocol
pub const IFF_PROMISC: ::c_int = 0x0100; // receive all packets
pub const IFF_ALLMULTI: ::c_int = 0x0200; // receive all multicast packets
pub const IFF_OACTIVE: ::c_int = 0x0400; // transmission in progress
pub const IFF_SIMPLEX: ::c_int = 0x0800; // can't hear own transmissions
pub const IFF_LINK0: ::c_int = 0x1000; // per link layer defined bit
pub const IFF_LINK1: ::c_int = 0x2000; // per link layer defined bit
pub const IFF_LINK2: ::c_int = 0x4000; // per link layer defined bit
pub const IFF_MULTICAST: ::c_int = 0x8000; // supports multicast

// sys/netinet/in.h
// netinet/in.h
// Protocols (RFC 1700)
// NOTE: These are in addition to the constants defined in src/unix/mod.rs

// IPPROTO_IP defined in src/unix/mod.rs
/// Hop-by-hop option header
pub const IPPROTO_HOPOPTS: ::c_int = 0;
// IPPROTO_ICMP defined in src/unix/mod.rs
/// group mgmt protocol
pub const IPPROTO_IGMP: ::c_int = 2;
/// gateway^2 (deprecated)
pub const IPPROTO_GGP: ::c_int = 3;
/// for compatibility
pub const IPPROTO_IPIP: ::c_int = 4;
// IPPROTO_TCP defined in src/unix/mod.rs
/// exterior gateway protocol
pub const IPPROTO_EGP: ::c_int = 8;
/// pup
pub const IPPROTO_PUP: ::c_int = 12;
// IPPROTO_UDP defined in src/unix/mod.rs
/// xns idp
pub const IPPROTO_IDP: ::c_int = 22;
/// tp-4 w/ class negotiation
pub const IPPROTO_TP: ::c_int = 29;
/// DCCP
pub const IPPROTO_DCCP: ::c_int = 33;
// IPPROTO_IPV6 defined in src/unix/mod.rs
/// IP6 routing header
pub const IPPROTO_ROUTING: ::c_int = 43;
/// IP6 fragmentation header
pub const IPPROTO_FRAGMENT: ::c_int = 44;
/// resource reservation
pub const IPPROTO_RSVP: ::c_int = 46;
/// General Routing Encap.
pub const IPPROTO_GRE: ::c_int = 47;
/// IP6 Encap Sec. Payload
pub const IPPROTO_ESP: ::c_int = 50;
/// IP6 Auth Header
pub const IPPROTO_AH: ::c_int = 51;
/// IP Mobility RFC 2004
pub const IPPROTO_MOBILE: ::c_int = 55;
/// IPv6 ICMP
pub const IPPROTO_IPV6_ICMP: ::c_int = 58;
// IPPROTO_ICMPV6 defined in src/unix/mod.rs
/// IP6 no next header
pub const IPPROTO_NONE: ::c_int = 59;
/// IP6 destination option
pub const IPPROTO_DSTOPTS: ::c_int = 60;
/// ISO cnlp
pub const IPPROTO_EON: ::c_int = 80;
/// Ethernet-in-IP
pub const IPPROTO_ETHERIP: ::c_int = 97;
/// encapsulation header
pub const IPPROTO_ENCAP: ::c_int = 98;
/// Protocol indep. multicast
pub const IPPROTO_PIM: ::c_int = 103;
/// IP Payload Comp. Protocol
pub const IPPROTO_IPCOMP: ::c_int = 108;
/// VRRP RFC 2338
pub const IPPROTO_VRRP: ::c_int = 112;
/// Common Address Resolution Protocol
pub const IPPROTO_CARP: ::c_int = 112;
/// L2TPv3
// TEMP: Disabled for now; this constant was added to NetBSD on 2017-02-16,
// but isn't yet supported by the NetBSD rumprun kernel image used for
// libc testing.
//pub const IPPROTO_L2TP: ::c_int = 115;
/// SCTP
pub const IPPROTO_SCTP: ::c_int = 132;
/// PFSYNC
pub const IPPROTO_PFSYNC: ::c_int = 240;
pub const IPPROTO_MAX: ::c_int = 256;

/// last return value of *_input(), meaning "all job for this pkt is done".
pub const IPPROTO_DONE: ::c_int = 257;

/// sysctl placeholder for (FAST_)IPSEC
pub const CTL_IPPROTO_IPSEC: ::c_int = 258;

pub const AF_OROUTE: ::c_int = 17;
pub const AF_ARP: ::c_int = 28;
pub const pseudo_AF_KEY: ::c_int = 29;
pub const pseudo_AF_HDRCMPLT: ::c_int = 30;
pub const AF_BLUETOOTH: ::c_int = 31;
pub const AF_IEEE80211: ::c_int = 32;
pub const AF_MPLS: ::c_int = 33;
pub const AF_ROUTE: ::c_int = 34;
pub const AF_MAX: ::c_int = 35;

pub const NET_MAXID: ::c_int = AF_MAX;
pub const NET_RT_DUMP: ::c_int = 1;
pub const NET_RT_FLAGS: ::c_int = 2;
pub const NET_RT_OOIFLIST: ::c_int = 3;
pub const NET_RT_OIFLIST: ::c_int = 4;
pub const NET_RT_IFLIST: ::c_int = 5;
pub const NET_RT_MAXID: ::c_int = 6;

pub const PF_OROUTE: ::c_int = AF_OROUTE;
pub const PF_ARP: ::c_int = AF_ARP;
pub const PF_KEY: ::c_int = pseudo_AF_KEY;
pub const PF_BLUETOOTH: ::c_int = AF_BLUETOOTH;
pub const PF_MPLS: ::c_int = AF_MPLS;
pub const PF_ROUTE: ::c_int = AF_ROUTE;
pub const PF_MAX: ::c_int = AF_MAX;

pub const MSG_NBIO: ::c_int = 0x1000;
pub const MSG_WAITFORONE: ::c_int = 0x2000;
pub const MSG_NOTIFICATION: ::c_int = 0x4000;

pub const SCM_TIMESTAMP: ::c_int = 0x08;
pub const SCM_CREDS: ::c_int = 0x10;

pub const O_DSYNC : ::c_int = 0x10000;

pub const MAP_RENAME : ::c_int = 0x20;
pub const MAP_NORESERVE : ::c_int = 0x40;
pub const MAP_HASSEMAPHORE : ::c_int = 0x200;
pub const MAP_WIRED: ::c_int = 0x800;

pub const DCCP_TYPE_REQUEST: ::c_int = 0;
pub const DCCP_TYPE_RESPONSE: ::c_int = 1;
pub const DCCP_TYPE_DATA: ::c_int = 2;
pub const DCCP_TYPE_ACK: ::c_int = 3;
pub const DCCP_TYPE_DATAACK: ::c_int =  4;
pub const DCCP_TYPE_CLOSEREQ: ::c_int = 5;
pub const DCCP_TYPE_CLOSE: ::c_int = 6;
pub const DCCP_TYPE_RESET: ::c_int = 7;
pub const DCCP_TYPE_MOVE: ::c_int = 8;

pub const DCCP_FEATURE_CC: ::c_int = 1;
pub const DCCP_FEATURE_ECN: ::c_int = 2;
pub const DCCP_FEATURE_ACKRATIO: ::c_int =  3;
pub const DCCP_FEATURE_ACKVECTOR: ::c_int = 4;
pub const DCCP_FEATURE_MOBILITY: ::c_int =  5;
pub const DCCP_FEATURE_LOSSWINDOW: ::c_int = 6;
pub const DCCP_FEATURE_CONN_NONCE: ::c_int = 8;
pub const DCCP_FEATURE_IDENTREG: ::c_int =  7;

pub const DCCP_OPT_PADDING: ::c_int = 0;
pub const DCCP_OPT_DATA_DISCARD: ::c_int = 1;
pub const DCCP_OPT_SLOW_RECV: ::c_int = 2;
pub const DCCP_OPT_BUF_CLOSED: ::c_int = 3;
pub const DCCP_OPT_CHANGE_L: ::c_int = 32;
pub const DCCP_OPT_CONFIRM_L: ::c_int = 33;
pub const DCCP_OPT_CHANGE_R: ::c_int = 34;
pub const DCCP_OPT_CONFIRM_R: ::c_int = 35;
pub const DCCP_OPT_INIT_COOKIE: ::c_int = 36;
pub const DCCP_OPT_NDP_COUNT: ::c_int = 37;
pub const DCCP_OPT_ACK_VECTOR0: ::c_int = 38;
pub const DCCP_OPT_ACK_VECTOR1: ::c_int = 39;
pub const DCCP_OPT_RECV_BUF_DROPS: ::c_int = 40;
pub const DCCP_OPT_TIMESTAMP: ::c_int = 41;
pub const DCCP_OPT_TIMESTAMP_ECHO: ::c_int = 42;
pub const DCCP_OPT_ELAPSEDTIME: ::c_int = 43;
pub const DCCP_OPT_DATACHECKSUM: ::c_int = 44;

pub const DCCP_REASON_UNSPEC: ::c_int = 0;
pub const DCCP_REASON_CLOSED: ::c_int = 1;
pub const DCCP_REASON_INVALID: ::c_int = 2;
pub const DCCP_REASON_OPTION_ERR: ::c_int = 3;
pub const DCCP_REASON_FEA_ERR: ::c_int = 4;
pub const DCCP_REASON_CONN_REF: ::c_int = 5;
pub const DCCP_REASON_BAD_SNAME: ::c_int = 6;
pub const DCCP_REASON_BAD_COOKIE: ::c_int = 7;
pub const DCCP_REASON_INV_MOVE: ::c_int = 8;
pub const DCCP_REASON_UNANSW_CH: ::c_int = 10;
pub const DCCP_REASON_FRUITLESS_NEG: ::c_int = 11;

pub const DCCP_CCID: ::c_int = 1;
pub const DCCP_CSLEN: ::c_int = 2;
pub const DCCP_MAXSEG: ::c_int = 4;
pub const DCCP_SERVICE: ::c_int = 8;

pub const DCCP_NDP_LIMIT: ::c_int = 16;
pub const DCCP_SEQ_NUM_LIMIT: ::c_int = 16777216;
pub const DCCP_MAX_OPTIONS: ::c_int = 32;
pub const DCCP_MAX_PKTS: ::c_int = 100;

pub const MAX_ASOC_IDS_RET: ::c_uint = 255;

pub const SCTP_NODELAY: ::c_int = 0x01;
pub const SCTP_MAXSEG: ::c_int = 0x02;
pub const SCTP_ASSOCINFO: ::c_int = 0x03;
pub const SCTP_INITMSG: ::c_int = 0x04;
pub const SCTP_AUTOCLOSE: ::c_int = 0x05;
pub const SCTP_SET_PEER_PRIMARY_ADDR: ::c_int = 0x06;
pub const SCTP_PRIMARY_ADDR: ::c_int = 0x07;
pub const SCTP_STATUS: ::c_int = 0x08;
pub const SCTP_PCB_STATUS: ::c_int = 0x09;
pub const SCTP_EVENTS: ::c_int = 0x0a;
pub const SCTP_PEER_ADDR_PARAMS: ::c_int = 0x0b;
pub const SCTP_GET_PEER_ADDR_INFO: ::c_int = 0x0c;
pub const SCTP_GET_PEER_ADDRESSES: ::c_int = 0x0d;
pub const SCTP_GET_LOCAL_ADDRESSES: ::c_int = 0x0e;
pub const SCTP_GET_SNDBUF_USE: ::c_int = 0x0f;
pub const SCTP_ADAPTION_LAYER: ::c_int = 0x10;
pub const SCTP_DISABLE_FRAGMENTS: ::c_int = 0x11;
pub const SCTP_BINDX_ADD_ADDR: ::c_int = 0x12;
pub const SCTP_BINDX_REM_ADDR: ::c_int = 0x13;
pub const SCTP_GET_LOCAL_ADDR_SIZE: ::c_int = 0x14;
pub const SCTP_I_WANT_MAPPED_V4_ADDR: ::c_int = 0x15;
pub const SCTP_GET_REMOTE_ADDR_SIZE: ::c_int = 0x16;
pub const SCTP_GET_PEGS: ::c_int = 0x17;
pub const SCTP_DEFAULT_SEND_PARAM: ::c_int = 0x18;
pub const SCTP_SET_DEBUG_LEVEL: ::c_int = 0x19;
pub const SCTP_RTOINFO: ::c_int = 0x1a;
pub const SCTP_AUTO_ASCONF: ::c_int = 0x1b;
pub const SCTP_MAXBURST: ::c_int = 0x1c;
pub const SCTP_GET_STAT_LOG: ::c_int = 0x1d;
pub const SCTP_CONNECT_X: ::c_int = 0x1e;
pub const SCTP_RESET_STREAMS: ::c_int = 0x1f;
pub const SCTP_CONNECT_X_DELAYED: ::c_int = 0x20;
pub const SCTP_CONNECT_X_COMPLETE: ::c_int = 0x21;
pub const SCTP_GET_ASOC_ID_LIST: ::c_int = 0x22;
pub const SCTP_GET_NONCE_VALUES: ::c_int = 0x23;
pub const SCTP_DELAYED_ACK_TIME: ::c_int = 0x24;
pub const SCTP_PEER_PUBLIC_KEY: ::c_int = 0x100;
pub const SCTP_MY_PUBLIC_KEY: ::c_int = 0x101;
pub const SCTP_SET_AUTH_SECRET: ::c_int = 0x102;
pub const SCTP_SET_AUTH_CHUNKS: ::c_int = 0x103;

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

pub const SCTP_ERROR_NO_ERROR: ::c_int = 0x0000;
pub const SCTP_ERROR_INVALID_STREAM: ::c_int = 0x0001;
pub const SCTP_ERROR_MISSING_PARAM: ::c_int = 0x0002;
pub const SCTP_ERROR_STALE_COOKIE: ::c_int = 0x0003;
pub const SCTP_ERROR_OUT_OF_RESOURCES: ::c_int = 0x0004;
pub const SCTP_ERROR_UNRESOLVABLE_ADDR: ::c_int = 0x0005;
pub const SCTP_ERROR_UNRECOG_CHUNK: ::c_int = 0x0006;
pub const SCTP_ERROR_INVALID_PARAM: ::c_int = 0x0007;
pub const SCTP_ERROR_UNRECOG_PARAM: ::c_int = 0x0008;
pub const SCTP_ERROR_NO_USER_DATA: ::c_int = 0x0009;
pub const SCTP_ERROR_COOKIE_IN_SHUTDOWN: ::c_int = 0x000a;

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

pub const SCTP_HAD_NO_TCB: ::c_int = 0x01;
pub const SCTP_FROM_MIDDLE_BOX: ::c_int = SCTP_HAD_NO_TCB;
pub const SCTP_BADCRC: ::c_int = 0x02;
pub const SCTP_PACKET_TRUNCATED: ::c_int = 0x04;

pub const SCTP_DATA_FRAG_MASK: ::c_int = 0x03;
pub const SCTP_DATA_MIDDLE_FRAG: ::c_int = 0x00;
pub const SCTP_DATA_LAST_FRAG: ::c_int = 0x01;
pub const SCTP_DATA_FIRST_FRAG: ::c_int = 0x02;
pub const SCTP_DATA_NOT_FRAG: ::c_int = 0x03;
pub const SCTP_DATA_UNORDERED: ::c_int = 0x04;

pub const SCTP_SACK_NONCE_SUM: ::c_int = 0x01;

pub const SCTP_INIT: ::c_int = 0x0001;
pub const SCTP_SNDRCV: ::c_int = 0x0002;

pub const MSG_SENDALL: ::c_int = 0x0200;
pub const MSG_PR_SCTP_TTL: ::c_int = 0x0400;
pub const MSG_PR_SCTP_BUF: ::c_int = 0x0800;
pub const MSG_EOF: ::c_int = 0x1000;
pub const MSG_UNORDERED: ::c_int = 0x2000;
pub const MSG_ADDR_OVER: ::c_int = 0x4000;
pub const MSG_ABORT: ::c_int = 0x8000;

pub const SCTP_COMM_UP: ::c_int = 0x0001;
pub const SCTP_COMM_LOST: ::c_int = 0x0002;
pub const SCTP_RESTART: ::c_int = 0x0003;
pub const SCTP_SHUTDOWN_COMP: ::c_int = 0x0004;
pub const SCTP_CANT_STR_ASSOC: ::c_int = 0x0005;

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
pub const SCTP_DATA_UNSENT: ::c_int = 0x0001;
pub const SCTP_DATA_SENT: ::c_int = 0x0002;
pub const SCTP_PARTIAL_DELIVERY_ABORTED: ::c_int = 0x0001;
pub const SCTP_STRRESET_INBOUND_STR: ::c_int = 0x0001;
pub const SCTP_STRRESET_OUTBOUND_STR: ::c_int = 0x0002;
pub const SCTP_STRRESET_ALL_STREAMS: ::c_int = 0x0004;
pub const SCTP_STRRESET_STREAM_LIST: ::c_int = 0x0008;

pub const SCTP_ASSOC_CHANGE: ::c_int = 0x0001;
pub const SCTP_PEER_ADDR_CHANGE: ::c_int = 0x0002;
pub const SCTP_REMOTE_ERROR: ::c_int = 0x0003;
pub const SCTP_SEND_FAILED: ::c_int = 0x0004;
pub const SCTP_SHUTDOWN_EVENT: ::c_int = 0x0005;
pub const SCTP_ADAPTION_INDICATION: ::c_int = 0x0006;
pub const SCTP_PARTIAL_DELIVERY_EVENT: ::c_int = 0x0007;
pub const SCTP_STREAM_RESET_EVENT: ::c_int = 0x0008;
pub const SCTP_ISSUE_HB: ::c_int = 0xffffffff;
pub const SCTP_NO_HB: ::c_int = 0x0;
pub const SCTP_MAX_EXPLICT_STR_RESET: ::c_int = 1000;
pub const SCTP_RESET_LOCAL_RECV: ::c_int = 0x0001;
pub const SCTP_RESET_LOCAL_SEND: ::c_int = 0x0002;
pub const SCTP_RESET_BOTH: ::c_int = 0x0003;

pub const _PC_LINK_MAX : ::c_int = 1;
pub const _PC_MAX_CANON : ::c_int = 2;
pub const _PC_MAX_INPUT : ::c_int = 3;
pub const _PC_NAME_MAX : ::c_int = 4;
pub const _PC_PATH_MAX : ::c_int = 5;
pub const _PC_PIPE_BUF : ::c_int = 6;
pub const _PC_CHOWN_RESTRICTED : ::c_int = 7;
pub const _PC_NO_TRUNC : ::c_int = 8;
pub const _PC_VDISABLE : ::c_int = 9;
pub const _PC_SYNC_IO : ::c_int = 10;
pub const _PC_FILESIZEBITS : ::c_int = 11;
pub const _PC_SYMLINK_MAX : ::c_int = 12;
pub const _PC_2_SYMLINKS : ::c_int = 13;
pub const _PC_ACL_EXTENDED : ::c_int = 14;
pub const _PC_MIN_HOLE_SIZE : ::c_int = 15;

pub const _SC_SYNCHRONIZED_IO : ::c_int = 31;
pub const _SC_IOV_MAX : ::c_int = 32;
pub const _SC_MAPPED_FILES : ::c_int = 33;
pub const _SC_MEMLOCK : ::c_int = 34;
pub const _SC_MEMLOCK_RANGE : ::c_int = 35;
pub const _SC_MEMORY_PROTECTION : ::c_int = 36;
pub const _SC_LOGIN_NAME_MAX : ::c_int = 37;
pub const _SC_MONOTONIC_CLOCK : ::c_int = 38;
pub const _SC_CLK_TCK : ::c_int = 39;
pub const _SC_ATEXIT_MAX : ::c_int = 40;
pub const _SC_THREADS : ::c_int = 41;
pub const _SC_SEMAPHORES : ::c_int = 42;
pub const _SC_BARRIERS : ::c_int = 43;
pub const _SC_TIMERS : ::c_int = 44;
pub const _SC_SPIN_LOCKS : ::c_int = 45;
pub const _SC_READER_WRITER_LOCKS : ::c_int = 46;
pub const _SC_GETGR_R_SIZE_MAX : ::c_int = 47;
pub const _SC_GETPW_R_SIZE_MAX : ::c_int = 48;
pub const _SC_CLOCK_SELECTION : ::c_int = 49;
pub const _SC_ASYNCHRONOUS_IO : ::c_int = 50;
pub const _SC_AIO_LISTIO_MAX : ::c_int = 51;
pub const _SC_AIO_MAX : ::c_int = 52;
pub const _SC_MESSAGE_PASSING : ::c_int = 53;
pub const _SC_MQ_OPEN_MAX : ::c_int = 54;
pub const _SC_MQ_PRIO_MAX : ::c_int = 55;
pub const _SC_PRIORITY_SCHEDULING : ::c_int = 56;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS : ::c_int = 57;
pub const _SC_THREAD_KEYS_MAX : ::c_int = 58;
pub const _SC_THREAD_STACK_MIN : ::c_int = 59;
pub const _SC_THREAD_THREADS_MAX : ::c_int = 60;
pub const _SC_THREAD_ATTR_STACKADDR : ::c_int = 61;
pub const _SC_THREAD_ATTR_STACKSIZE : ::c_int = 62;
pub const _SC_THREAD_PRIORITY_SCHEDULING : ::c_int = 63;
pub const _SC_THREAD_PRIO_INHERIT : ::c_int = 64;
pub const _SC_THREAD_PRIO_PROTECT : ::c_int = 65;
pub const _SC_THREAD_PROCESS_SHARED : ::c_int = 66;
pub const _SC_THREAD_SAFE_FUNCTIONS : ::c_int = 67;
pub const _SC_TTY_NAME_MAX : ::c_int = 68;
pub const _SC_HOST_NAME_MAX : ::c_int = 69;
pub const _SC_PASS_MAX : ::c_int = 70;
pub const _SC_REGEXP : ::c_int = 71;
pub const _SC_SHELL : ::c_int = 72;
pub const _SC_SYMLOOP_MAX : ::c_int = 73;
pub const _SC_V6_ILP32_OFF32 : ::c_int = 74;
pub const _SC_V6_ILP32_OFFBIG : ::c_int = 75;
pub const _SC_V6_LP64_OFF64 : ::c_int = 76;
pub const _SC_V6_LPBIG_OFFBIG : ::c_int = 77;
pub const _SC_2_PBS : ::c_int = 80;
pub const _SC_2_PBS_ACCOUNTING : ::c_int = 81;
pub const _SC_2_PBS_CHECKPOINT : ::c_int = 82;
pub const _SC_2_PBS_LOCATE : ::c_int = 83;
pub const _SC_2_PBS_MESSAGE : ::c_int = 84;
pub const _SC_2_PBS_TRACK : ::c_int = 85;
pub const _SC_SPAWN : ::c_int = 86;
pub const _SC_SHARED_MEMORY_OBJECTS : ::c_int = 87;
pub const _SC_TIMER_MAX : ::c_int = 88;
pub const _SC_SEM_NSEMS_MAX : ::c_int = 89;
pub const _SC_CPUTIME : ::c_int = 90;
pub const _SC_THREAD_CPUTIME : ::c_int = 91;
pub const _SC_DELAYTIMER_MAX : ::c_int = 92;
// These two variables will be supported in NetBSD 8.0
// pub const _SC_SIGQUEUE_MAX : ::c_int = 93;
// pub const _SC_REALTIME_SIGNALS : ::c_int = 94;
pub const _SC_PHYS_PAGES : ::c_int = 121;
pub const _SC_NPROCESSORS_CONF : ::c_int = 1001;
pub const _SC_NPROCESSORS_ONLN : ::c_int = 1002;
pub const _SC_SCHED_RT_TS : ::c_int = 2001;
pub const _SC_SCHED_PRI_MIN : ::c_int = 2002;
pub const _SC_SCHED_PRI_MAX : ::c_int = 2003;

pub const FD_SETSIZE: usize = 0x100;

pub const ST_NOSUID: ::c_ulong = 8;

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    ptm_magic: 0x33330003,
    ptm_errorcheck: 0,
    ptm_interlock: 0,
    ptm_waiters: 0 as *mut _,
    ptm_owner: 0,
    ptm_pad1: [0; 3],
    ptm_pad2: [0; 3],
    ptm_recursed: 0,
    ptm_spare2: 0 as *mut _,
};
pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    ptc_magic: 0x55550005,
    ptc_lock: 0,
    ptc_waiters_first: 0 as *mut _,
    ptc_waiters_last: 0 as *mut _,
    ptc_mutex: 0 as *mut _,
    ptc_private: 0 as *mut _,
};
pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    ptr_magic: 0x99990009,
    ptr_interlock: 0,
    ptr_rblocked_first: 0 as *mut _,
    ptr_rblocked_last: 0 as *mut _,
    ptr_wblocked_first: 0 as *mut _,
    ptr_wblocked_last: 0 as *mut _,
    ptr_nreaders: 0,
    ptr_owner: 0,
    ptr_private: 0 as *mut _,
};
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 2;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;

pub const EVFILT_AIO: ::uint32_t = 2;
pub const EVFILT_PROC: ::uint32_t = 4;
pub const EVFILT_READ: ::uint32_t = 0;
pub const EVFILT_SIGNAL: ::uint32_t = 5;
pub const EVFILT_TIMER: ::uint32_t = 6;
pub const EVFILT_VNODE: ::uint32_t = 3;
pub const EVFILT_WRITE: ::uint32_t = 1;

pub const EV_ADD: ::uint32_t = 0x1;
pub const EV_DELETE: ::uint32_t = 0x2;
pub const EV_ENABLE: ::uint32_t = 0x4;
pub const EV_DISABLE: ::uint32_t = 0x8;
pub const EV_ONESHOT: ::uint32_t = 0x10;
pub const EV_CLEAR: ::uint32_t = 0x20;
pub const EV_RECEIPT: ::uint32_t = 0x40;
pub const EV_DISPATCH: ::uint32_t = 0x80;
pub const EV_FLAG1: ::uint32_t = 0x2000;
pub const EV_ERROR: ::uint32_t = 0x4000;
pub const EV_EOF: ::uint32_t = 0x8000;
pub const EV_SYSFLAGS: ::uint32_t = 0xf000;

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

pub const TMP_MAX : ::c_uint = 308915776;

pub const NI_MAXHOST: ::socklen_t = 1025;

pub const RTLD_NOLOAD: ::c_int = 0x2000;
pub const RTLD_LOCAL: ::c_int = 0x200;

pub const CTL_MAXNAME: ::c_int = 12;
pub const SYSCTL_NAMELEN: ::c_int = 32;
pub const SYSCTL_DEFSIZE: ::c_int = 8;
pub const CTLTYPE_NODE: ::c_int = 1;
pub const CTLTYPE_INT: ::c_int = 2;
pub const CTLTYPE_STRING: ::c_int = 3;
pub const CTLTYPE_QUAD: ::c_int = 4;
pub const CTLTYPE_STRUCT: ::c_int = 5;
pub const CTLTYPE_BOOL: ::c_int = 6;
pub const CTLFLAG_READONLY: ::c_int = 0x00000000;
pub const CTLFLAG_READWRITE: ::c_int = 0x00000070;
pub const CTLFLAG_ANYWRITE: ::c_int = 0x00000080;
pub const CTLFLAG_PRIVATE: ::c_int = 0x00000100;
pub const CTLFLAG_PERMANENT: ::c_int = 0x00000200;
pub const CTLFLAG_OWNDATA: ::c_int = 0x00000400;
pub const CTLFLAG_IMMEDIATE: ::c_int = 0x00000800;
pub const CTLFLAG_HEX: ::c_int = 0x00001000;
pub const CTLFLAG_ROOT: ::c_int = 0x00002000;
pub const CTLFLAG_ANYNUMBER: ::c_int = 0x00004000;
pub const CTLFLAG_HIDDEN: ::c_int = 0x00008000;
pub const CTLFLAG_ALIAS: ::c_int = 0x00010000;
pub const CTLFLAG_MMAP: ::c_int = 0x00020000;
pub const CTLFLAG_OWNDESC: ::c_int = 0x00040000;
pub const CTLFLAG_UNSIGNED: ::c_int = 0x00080000;
pub const SYSCTL_VERS_MASK: ::c_int = 0xff000000;
pub const SYSCTL_VERS_0: ::c_int = 0x00000000;
pub const SYSCTL_VERS_1: ::c_int = 0x01000000;
pub const SYSCTL_VERSION: ::c_int = SYSCTL_VERS_1;
pub const CTL_EOL: ::c_int = -1;
pub const CTL_QUERY: ::c_int = -2;
pub const CTL_CREATE: ::c_int = -3;
pub const CTL_CREATESYM: ::c_int = -4;
pub const CTL_DESTROY: ::c_int = -5;
pub const CTL_MMAP: ::c_int = -6;
pub const CTL_DESCRIBE: ::c_int = -7;
pub const CTL_UNSPEC: ::c_int = 0;
pub const CTL_KERN: ::c_int = 1;
pub const CTL_VM: ::c_int = 2;
pub const CTL_VFS: ::c_int = 3;
pub const CTL_NET: ::c_int = 4;
pub const CTL_DEBUG: ::c_int = 5;
pub const CTL_HW: ::c_int = 6;
pub const CTL_MACHDEP: ::c_int = 7;
pub const CTL_USER: ::c_int = 8;
pub const CTL_DDB: ::c_int = 9;
pub const CTL_PROC: ::c_int = 10;
pub const CTL_VENDOR: ::c_int = 11;
pub const CTL_EMUL: ::c_int = 12;
pub const CTL_SECURITY: ::c_int = 13;
pub const CTL_MAXID: ::c_int = 14;
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
pub const KERN_OBOOTTIME: ::c_int = 21;
pub const KERN_DOMAINNAME: ::c_int = 22;
pub const KERN_MAXPARTITIONS: ::c_int = 23;
pub const KERN_RAWPARTITION: ::c_int = 24;
pub const KERN_NTPTIME: ::c_int = 25;
pub const KERN_TIMEX: ::c_int = 26;
pub const KERN_AUTONICETIME: ::c_int = 27;
pub const KERN_AUTONICEVAL: ::c_int = 28;
pub const KERN_RTC_OFFSET: ::c_int = 29;
pub const KERN_ROOT_DEVICE: ::c_int = 30;
pub const KERN_MSGBUFSIZE: ::c_int = 31;
pub const KERN_FSYNC: ::c_int = 32;
pub const KERN_OLDSYSVMSG: ::c_int = 33;
pub const KERN_OLDSYSVSEM: ::c_int = 34;
pub const KERN_OLDSYSVSHM: ::c_int = 35;
pub const KERN_OLDSHORTCORENAME: ::c_int = 36;
pub const KERN_SYNCHRONIZED_IO: ::c_int = 37;
pub const KERN_IOV_MAX: ::c_int = 38;
pub const KERN_MBUF: ::c_int = 39;
pub const KERN_MAPPED_FILES: ::c_int = 40;
pub const KERN_MEMLOCK: ::c_int = 41;
pub const KERN_MEMLOCK_RANGE: ::c_int = 42;
pub const KERN_MEMORY_PROTECTION: ::c_int = 43;
pub const KERN_LOGIN_NAME_MAX: ::c_int = 44;
pub const KERN_DEFCORENAME: ::c_int = 45;
pub const KERN_LOGSIGEXIT: ::c_int = 46;
pub const KERN_PROC2: ::c_int = 47;
pub const KERN_PROC_ARGS: ::c_int = 48;
pub const KERN_FSCALE: ::c_int = 49;
pub const KERN_CCPU: ::c_int = 50;
pub const KERN_CP_TIME: ::c_int = 51;
pub const KERN_OLDSYSVIPC_INFO: ::c_int = 52;
pub const KERN_MSGBUF: ::c_int = 53;
pub const KERN_CONSDEV: ::c_int = 54;
pub const KERN_MAXPTYS: ::c_int = 55;
pub const KERN_PIPE: ::c_int = 56;
pub const KERN_MAXPHYS: ::c_int = 57;
pub const KERN_SBMAX: ::c_int = 58;
pub const KERN_TKSTAT: ::c_int = 59;
pub const KERN_MONOTONIC_CLOCK: ::c_int = 60;
pub const KERN_URND: ::c_int = 61;
pub const KERN_LABELSECTOR: ::c_int = 62;
pub const KERN_LABELOFFSET: ::c_int = 63;
pub const KERN_LWP: ::c_int = 64;
pub const KERN_FORKFSLEEP: ::c_int = 65;
pub const KERN_POSIX_THREADS: ::c_int = 66;
pub const KERN_POSIX_SEMAPHORES: ::c_int = 67;
pub const KERN_POSIX_BARRIERS: ::c_int = 68;
pub const KERN_POSIX_TIMERS: ::c_int = 69;
pub const KERN_POSIX_SPIN_LOCKS: ::c_int = 70;
pub const KERN_POSIX_READER_WRITER_LOCKS: ::c_int = 71;
pub const KERN_DUMP_ON_PANIC: ::c_int = 72;
pub const KERN_SOMAXKVA: ::c_int = 73;
pub const KERN_ROOT_PARTITION: ::c_int = 74;
pub const KERN_DRIVERS: ::c_int = 75;
pub const KERN_BUF: ::c_int = 76;
pub const KERN_FILE2: ::c_int = 77;
pub const KERN_VERIEXEC: ::c_int = 78;
pub const KERN_CP_ID: ::c_int = 79;
pub const KERN_HARDCLOCK_TICKS: ::c_int = 80;
pub const KERN_ARND: ::c_int = 81;
pub const KERN_SYSVIPC: ::c_int = 82;
pub const KERN_BOOTTIME: ::c_int = 83;
pub const KERN_EVCNT: ::c_int = 84;
pub const KERN_MAXID: ::c_int = 85;
pub const KERN_PROC_ALL: ::c_int = 0;
pub const KERN_PROC_PID: ::c_int = 1;
pub const KERN_PROC_PGRP: ::c_int = 2;
pub const KERN_PROC_SESSION: ::c_int = 3;
pub const KERN_PROC_TTY: ::c_int = 4;
pub const KERN_PROC_UID: ::c_int = 5;
pub const KERN_PROC_RUID: ::c_int = 6;
pub const KERN_PROC_GID: ::c_int = 7;
pub const KERN_PROC_RGID: ::c_int = 8;
pub const KERN_PROC_ARGV: ::c_int = 1;
pub const KERN_PROC_NARGV: ::c_int = 2;
pub const KERN_PROC_ENV: ::c_int = 3;
pub const KERN_PROC_NENV: ::c_int = 4;
pub const KERN_PROC_PATHNAME: ::c_int = 5;

pub const EAI_SYSTEM: ::c_int = 11;

pub const AIO_CANCELED: ::c_int = 1;
pub const AIO_NOTCANCELED: ::c_int = 2;
pub const AIO_ALLDONE: ::c_int = 3;
pub const LIO_NOP: ::c_int = 0;
pub const LIO_WRITE: ::c_int = 1;
pub const LIO_READ: ::c_int = 2;
pub const LIO_WAIT: ::c_int = 1;
pub const LIO_NOWAIT: ::c_int = 0;

pub const SIGEV_NONE: ::c_int = 0;
pub const SIGEV_SIGNAL: ::c_int = 1;
pub const SIGEV_THREAD: ::c_int = 2;

pub const WSTOPPED: ::c_int = 0x00000002; // same as WUNTRACED
pub const WCONTINUED: ::c_int = 0x00000010;
pub const WEXITED: ::c_int = 0x000000020;
pub const WNOWAIT: ::c_int = 0x00010000;

pub const P_ALL: idtype_t = 0;
pub const P_PID: idtype_t = 1;
pub const P_PGID: idtype_t = 4;

pub const B460800: ::speed_t = 460800;
pub const B921600: ::speed_t = 921600;

pub const ONOCR: ::tcflag_t = 0x20;
pub const ONLRET: ::tcflag_t = 0x40;
pub const CDTRCTS: ::tcflag_t = 0x00020000;
pub const CHWFLOW: ::tcflag_t = ::MDMBUF | ::CRTSCTS | ::CDTRCTS;

pub const SOCK_CLOEXEC: ::c_int = 0x10000000;
pub const SOCK_NONBLOCK: ::c_int = 0x20000000;

// dirfd() is a macro on netbsd to access
// the first field of the struct where dirp points to:
// http://cvsweb.netbsd.org/bsdweb.cgi/src/include/dirent.h?rev=1.36
f! {
    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int {
        unsafe { *(dirp as *const ::c_int) }
    }

    pub fn WIFCONTINUED(status: ::c_int) -> bool {
        status == 0xffff
    }
}

extern {
    pub fn aio_read(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_write(aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_fsync(op: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn aio_error(aiocbp: *const aiocb) -> ::c_int;
    pub fn aio_return(aiocbp: *mut aiocb) -> ::ssize_t;
    #[link_name = "__aio_suspend50"]
    pub fn aio_suspend(aiocb_list: *const *const aiocb, nitems: ::c_int,
                       timeout: *const ::timespec) -> ::c_int;
    pub fn aio_cancel(fd: ::c_int, aiocbp: *mut aiocb) -> ::c_int;
    pub fn lio_listio(mode: ::c_int, aiocb_list: *const *mut aiocb,
                      nitems: ::c_int, sevp: *mut sigevent) -> ::c_int;

    pub fn lutimes(file: *const ::c_char, times: *const ::timeval) -> ::c_int;
    pub fn getnameinfo(sa: *const ::sockaddr,
                       salen: ::socklen_t,
                       host: *mut ::c_char,
                       hostlen: ::socklen_t,
                       serv: *mut ::c_char,
                       sevlen: ::socklen_t,
                       flags: ::c_int) -> ::c_int;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int)
                    -> ::c_int;
    pub fn sysctl(name: *const ::c_int,
                  namelen: ::c_uint,
                  oldp: *mut ::c_void,
                  oldlenp: *mut ::size_t,
                  newp: *const ::c_void,
                  newlen: ::size_t)
                  -> ::c_int;
    pub fn sysctlbyname(name: *const ::c_char,
                        oldp: *mut ::c_void,
                        oldlenp: *mut ::size_t,
                        newp: *const ::c_void,
                        newlen: ::size_t)
                        -> ::c_int;
    #[link_name = "__kevent50"]
    pub fn kevent(kq: ::c_int,
                  changelist: *const ::kevent,
                  nchanges: ::size_t,
                  eventlist: *mut ::kevent,
                  nevents: ::size_t,
                  timeout: *const ::timespec) -> ::c_int;
    #[link_name = "__mount50"]
    pub fn mount(src: *const ::c_char,
                 target: *const ::c_char,
                 flags: ::c_int,
                 data: *mut ::c_void,
                 size: ::size_t) -> ::c_int;
    pub fn ptrace(request: ::c_int,
                  pid: ::pid_t,
                  addr: *mut ::c_void,
                  data: ::c_int) -> ::c_int;
    pub fn pthread_setname_np(t: ::pthread_t,
                              name: *const ::c_char,
                              arg: *mut ::c_void) -> ::c_int;
    pub fn pthread_getattr_np(native: ::pthread_t,
                              attr: *mut ::pthread_attr_t) -> ::c_int;
    pub fn pthread_attr_getguardsize(attr: *const ::pthread_attr_t,
                                     guardsize: *mut ::size_t) -> ::c_int;
    pub fn pthread_attr_getstack(attr: *const ::pthread_attr_t,
                                 stackaddr: *mut *mut ::c_void,
                                 stacksize: *mut ::size_t) -> ::c_int;
    #[link_name = "__sigtimedwait50"]
    pub fn sigtimedwait(set: *const sigset_t,
                        info: *mut siginfo_t,
                        timeout: *const ::timespec) -> ::c_int;
    pub fn sigwaitinfo(set: *const sigset_t,
                       info: *mut siginfo_t) -> ::c_int;
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t);
    pub fn localeconv_l(loc: ::locale_t) -> *mut lconv;
    pub fn newlocale(mask: ::c_int,
                     locale: *const ::c_char,
                     base: ::locale_t) -> ::locale_t;
    #[link_name = "__settimeofday50"]
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::c_void) -> ::c_int;

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
}

mod other;
pub use self::other::*;
