macro_rules! expand_align {
    () => {
        s! {
            pub struct pthread_mutex_t {
                __align: [::c_long; 0],
                size: [u8; ::__SIZEOF_PTHREAD_MUTEX_T],
            }

            pub struct pthread_rwlock_t {
                __align: [::c_long; 0],
                size: [u8; ::__SIZEOF_PTHREAD_RWLOCK_T],
            }

            pub struct pthread_mutexattr_t {
                __align: [::c_int; 0],
                size: [u8; ::__SIZEOF_PTHREAD_MUTEXATTR_T],
            }

            pub struct pthread_rwlockattr_t {
                __align: [::c_int; 0],
                size: [u8; ::__SIZEOF_PTHREAD_RWLOCKATTR_T],
            }

            pub struct pthread_condattr_t {
                __align: [::c_int; 0],
                size: [u8; ::__SIZEOF_PTHREAD_CONDATTR_T],
            }
        }

        s_no_extra_traits! {
            #[allow(missing_debug_implementations)]
            pub struct pthread_cond_t {
                __align: [*const ::c_void; 0],
                size: [u8; ::__SIZEOF_PTHREAD_COND_T],
            }
        }
    }
}
