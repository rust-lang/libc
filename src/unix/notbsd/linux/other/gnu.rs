s! {
    /* Header <fcntl.h> */
    pub struct file_handle  {
        pub handle_bytes: ::c_uint,
        pub handle_type: ::c_int,
        pub f_handle: [::c_uchar],
    }
}

extern {
    pub fn name_to_handle_at(dfd: ::c_int,
                             name: *const ::c_char,
			                 handle: *mut ::file_handle,
			                 mnt_id: *mut ::c_int,
			                 flags: ::c_int) -> ::c_int;
    pub fn open_by_handle_at(mountdirfd: ::c_int,
                             handle: *mut ::file_handle,
			                 flags: ::c_int) -> ::c_int;
}
