macro_rules! expand_align {
    () => {
        s! {
            #[repr(align(4))]
            pub struct pthread_mutex_t {
                size: [u8; ::__SIZEOF_PTHREAD_MUTEX_T],
            }

            #[repr(align(4))]
            pub struct pthread_rwlock_t {
                size: [u8; ::__SIZEOF_PTHREAD_RWLOCK_T],
            }

            #[repr(align(4))]
            pub struct pthread_mutexattr_t {
                size: [u8; ::__SIZEOF_PTHREAD_MUTEXATTR_T],
            }

            #[repr(align(4))]
            pub struct pthread_rwlockattr_t {
                size: [u8; ::__SIZEOF_PTHREAD_RWLOCKATTR_T],
            }

            #[repr(align(4))]
            pub struct pthread_condattr_t {
                size: [u8; ::__SIZEOF_PTHREAD_CONDATTR_T],
            }
        }

        s_no_extra_traits! {
            #[cfg_attr(target_pointer_width = "32",
                       repr(align(4)))]
            #[cfg_attr(target_pointer_width = "64",
                       repr(align(8)))]
            #[allow(missing_debug_implementations)]
            pub struct pthread_cond_t {
                size: [u8; ::__SIZEOF_PTHREAD_COND_T],
            }
        }
    }
}
