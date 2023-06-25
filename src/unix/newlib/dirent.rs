s! {
    pub struct dirent {
        pub d_ino: ::ino_t,
        pub d_type: ::c_uchar,
        pub d_name: [::c_char; 256usize],
    }
}
