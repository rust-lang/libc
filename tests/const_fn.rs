#[cfg(target_os = "linux")]
const _FOO: libc::size_t = libc::CMSG_SPACE(1);
//^ if CMSG_SPACE is not const, this will fail to compile
