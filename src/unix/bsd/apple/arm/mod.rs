use super::*;

// mach/task_info.h
s! {
    #[cfg_attr(libc_packedN, repr(packed(4)))]
    pub struct task_basic_info_64_2 {
        pub suspend_count: integer_t,
        pub virtual_size: mach_vm_size_t,
        pub resident_size: mach_vm_size_t,
        pub user_time: time_value_t,
        pub system_time: time_value_t,
        pub policy: policy_t,
    }
}
