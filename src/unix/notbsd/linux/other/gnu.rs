s! {
    /* Header <fcntl.h> */
    pub struct file_handle  {
        pub handle_bytes: ::c_uint,
        pub handle_type: ::c_int,
        pub f_handle: [::c_uchar],
    }
}
