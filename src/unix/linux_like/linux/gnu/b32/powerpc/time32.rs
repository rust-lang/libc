s! {
    pub struct shmid_ds {
        pub shm_perm: ::ipc_perm,
        __glibc_reserved1: ::c_uint,
        pub shm_atime: ::time_t,
        __glibc_reserved2: ::c_uint,
        pub shm_dtime: ::time_t,
        __glibc_reserved3: ::c_uint,
        pub shm_ctime: ::time_t,
        __glibc_reserved4: ::c_uint,
        pub shm_segsz: ::size_t,
        pub shm_cpid: ::pid_t,
        pub shm_lpid: ::pid_t,
        pub shm_nattch: ::shmatt_t,
        __glibc_reserved5: ::c_ulong,
        __glibc_reserved6: ::c_ulong,
    }

}
