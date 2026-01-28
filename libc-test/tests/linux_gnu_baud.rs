#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
fn baud() {
    use libc::*;
    let controller_fd = unsafe { posix_openpt(O_RDWR | O_NOCTTY) };
    assert!(controller_fd >= 0);
    unsafe {
        grantpt(controller_fd);
        unlockpt(controller_fd);
    }
    let mut buffer = [0; 256];
    let ret = unsafe { ptsname_r(controller_fd, buffer.as_mut_ptr(), 256) };
    assert!(ret >= 0);
    let terminal_fd = unsafe { open(buffer.as_ptr(), O_RDWR | O_NOCTTY) };
    assert!(terminal_fd >= 0);
    #[repr(C)]
    struct Protector(termios, [u8; 512]);
    let mut tio: Protector = unsafe { Protector(std::mem::zeroed(), [0xcc; _]) };
    let ret = unsafe { tcgetattr(terminal_fd, &raw mut tio.0) };
    assert!(ret >= 0);
    assert_eq!(tio.1, [0xcc; _]);
    assert_eq!(unsafe { cfgetispeed(&tio.0) }, B38400);
    assert_eq!(unsafe { cfgetospeed(&tio.0) }, B38400);
    let ret = unsafe { cfsetspeed(&mut tio.0, B1000000) };
    assert!(ret >= 0);
    assert_eq!(unsafe { cfgetispeed(&tio.0) }, B1000000);
    assert_eq!(unsafe { cfgetospeed(&tio.0) }, B1000000);
    let ret = unsafe { cfsetispeed(&mut tio.0, B9600) };
    assert!(ret >= 0);
    assert_eq!(unsafe { cfgetispeed(&tio.0) }, B9600);
    assert!(matches!(unsafe { cfgetospeed(&tio.0) }, B9600 | B1000000));
    let ret = unsafe { cfsetospeed(&mut tio.0, B19200) };
    assert!(ret >= 0);
    assert!(matches!(unsafe { cfgetispeed(&tio.0) }, B9600 | B19200));
    assert_eq!(unsafe { cfgetospeed(&tio.0) }, B19200);
}
