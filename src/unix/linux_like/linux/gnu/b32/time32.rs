//! 32-bit specific definitions for linux-like values when gnu_time64_abi is not set

s! {
    pub struct timex {
        pub modes: ::c_uint,

        pub offset: ::__syscall_slong_t,
        pub freq: ::__syscall_slong_t,
        pub maxerror: ::__syscall_slong_t,
        pub esterror: ::__syscall_slong_t,
        pub status: ::c_int,
        pub constant: ::__syscall_slong_t,
        pub precision: ::__syscall_slong_t,
        pub tolerance: ::__syscall_slong_t,
        pub time: ::timeval,
        pub tick: ::__syscall_slong_t,
        pub ppsfreq: ::__syscall_slong_t,
        pub jitter: ::__syscall_slong_t,
        pub shift: ::c_int,
        pub stabil: ::__syscall_slong_t,
        pub jitcnt: ::__syscall_slong_t,
        pub calcnt: ::__syscall_slong_t,
        pub errcnt: ::__syscall_slong_t,
        pub stbcnt: ::__syscall_slong_t,
        pub tai: ::c_int,
        pub __unused1: i32,
        pub __unused2: i32,
        pub __unused3: i32,
        pub __unused4: i32,
        pub __unused5: i32,
        pub __unused6: i32,
        pub __unused7: i32,
        pub __unused8: i32,
        pub __unused9: i32,
        pub __unused10: i32,
        pub __unused11: i32,
    }

}
