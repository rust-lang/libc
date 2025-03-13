// Common functions that are unfortunately missing on illumos and
// Solaris, but often needed by other crates.
use core::cmp::min;

use crate::unix::solarish::*;
use crate::{c_char, c_int, size_t};

const PTEM: &[u8] = b"ptem\0";
const LDTERM: &[u8] = b"ldterm\0";

pub unsafe fn cfmakeraw(termios: *mut crate::termios) {
    (*termios).c_iflag &=
        !(IMAXBEL | IGNBRK | BRKINT | PARMRK | ISTRIP | INLCR | IGNCR | ICRNL | IXON);
    (*termios).c_oflag &= !OPOST;
    (*termios).c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    (*termios).c_cflag &= !(CSIZE | PARENB);
    (*termios).c_cflag |= CS8;

    // By default, most software expects a pending read to block until at
    // least one byte becomes available.  As per termio(7I), this requires
    // setting the MIN and TIME parameters appropriately.
    //
    // As a somewhat unfortunate artefact of history, the MIN and TIME slots
    // in the control character array overlap with the EOF and EOL slots used
    // for canonical mode processing.  Because the EOF character needs to be
    // the ASCII EOT value (aka Control-D), it has the byte value 4.  When
    // switching to raw mode, this is interpreted as a MIN value of 4; i.e.,
    // reads will block until at least four bytes have been input.
    //
    // Other platforms with a distinct MIN slot like Linux and FreeBSD appear
    // to default to a MIN value of 1, so we'll force that value here:
    (*termios).c_cc[VMIN] = 1;
    (*termios).c_cc[VTIME] = 0;
}

pub unsafe fn cfsetspeed(termios: *mut crate::termios, speed: crate::speed_t) -> c_int {
    // Neither of these functions on illumos or Solaris actually ever
    // return an error
    crate::cfsetispeed(termios, speed);
    crate::cfsetospeed(termios, speed);
    0
}

unsafe fn bail(fdm: c_int, fds: c_int) -> c_int {
    let e = *___errno();
    if fds >= 0 {
        crate::close(fds);
    }
    if fdm >= 0 {
        crate::close(fdm);
    }
    *___errno() = e;
    return -1;
}

pub unsafe fn getpwent_r(
    pwd: *mut passwd,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut passwd,
) -> c_int {
    let old_errno = *crate::___errno();
    *crate::___errno() = 0;
    *result = native_getpwent_r(pwd, buf, min(buflen, c_int::max_value() as size_t) as c_int);

    let ret = if (*result).is_null() {
        *crate::___errno()
    } else {
        0
    };
    *crate::___errno() = old_errno;

    ret
}

pub unsafe fn getgrent_r(
    grp: *mut crate::group,
    buf: *mut c_char,
    buflen: size_t,
    result: *mut *mut crate::group,
) -> c_int {
    let old_errno = *crate::___errno();
    *crate::___errno() = 0;
    *result = native_getgrent_r(grp, buf, min(buflen, c_int::max_value() as size_t) as c_int);

    let ret = if (*result).is_null() {
        *crate::___errno()
    } else {
        0
    };
    *crate::___errno() = old_errno;

    ret
}
