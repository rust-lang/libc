// Common functions that are unfortunately missing on AIX

use crate::unix::aix::*;
use crate::{c_char, c_int};

#[cfg(target_os = "aix")]
unsafe fn bail(fdm: c_int, fds: c_int) -> c_int {
    let e = *_Errno();
    if fds >= 0 {
        crate::close(fds);
    }
    if fdm >= 0 {
        crate::close(fdm);
    }
    *_Errno() = e;
    -1
}

#[cfg(target_os = "aix")]
pub unsafe fn openpty(
    amain: *mut c_int,
    asubord: *mut c_int,
    name: *mut c_char,
    termp: *const termios,
    winp: *const crate::winsize,
) -> c_int {
    const PTEM: &[u8] = b"ptem\0";
    const LDTERM: &[u8] = b"ldterm\0";

    // Open the main pseudo-terminal device, making sure not to set it as the
    // controlling terminal for this process:
    let fdm = crate::posix_openpt(O_RDWR | O_NOCTTY);
    if fdm < 0 {
        return -1;
    }

    // Set permissions and ownership on the subordinate device and unlock it:
    if crate::grantpt(fdm) < 0 || crate::unlockpt(fdm) < 0 {
        return bail(fdm, -1);
    }

    // Get the path name of the subordinate device:
    let subordpath = crate::ptsname(fdm);
    if subordpath.is_null() {
        return bail(fdm, -1);
    }

    // Open the subordinate device without setting it as the controlling
    // terminal for this process:
    let fds = crate::open(subordpath, O_RDWR | O_NOCTTY);
    if fds < 0 {
        return bail(fdm, -1);
    }

    // Check if the STREAMS modules are already pushed:
    let setup = crate::ioctl(fds, I_FIND, LDTERM.as_ptr());
    if setup < 0 {
        return bail(fdm, fds);
    } else if setup == 0 {
        // The line discipline is not present, so push the appropriate STREAMS
        // modules for the subordinate device:
        if crate::ioctl(fds, I_PUSH, PTEM.as_ptr()) < 0
            || crate::ioctl(fds, I_PUSH, LDTERM.as_ptr()) < 0
        {
            return bail(fdm, fds);
        }
    }

    // If provided, set the terminal parameters:
    if !termp.is_null() && crate::tcsetattr(fds, TCSAFLUSH, termp) != 0 {
        return bail(fdm, fds);
    }

    // If provided, set the window size:
    if !winp.is_null() && crate::ioctl(fds, TIOCSWINSZ, winp) < 0 {
        return bail(fdm, fds);
    }

    // If the caller wants the name of the subordinate device, copy it out.
    //
    // Note that this is a terrible interface: there appears to be no standard
    // upper bound on the copy length for this pointer.  Nobody should pass
    // anything but NULL here, preferring instead to use ptsname(3C) directly.
    if !name.is_null() {
        crate::strcpy(name, subordpath);
    }

    *amain = fdm;
    *asubord = fds;
    0
}

#[cfg(target_os = "aix")]
pub unsafe fn forkpty(
    amain: *mut c_int,
    name: *mut c_char,
    termp: *const termios,
    winp: *const crate::winsize,
) -> crate::pid_t {
    const PTEM: &[u8] = b"ptem\0";
    const LDTERM: &[u8] = b"ldterm\0";
    let fds = -1;

    let fdm = crate::posix_openpt(O_RDWR);
    if fdm < 0 {
        return -1;
    }

    // Set permissions and ownership on the subordinate device and unlock it:
    if crate::grantpt(fdm) < 0 || crate::unlockpt(fdm) < 0 {
        return bail(fdm, -1);
    }

    *amain = fdm;

    let pid = crate::fork();
    if pid < 0 {
        return bail(*amain, fds);
    } else if pid > 0 {
        // In the parent process, we close the subordinate device and return the
        // process ID of the new child:
        crate::close(fds);
        return pid;
    }

    // The rest of this function executes in the child process.

    // Get the path name of the subordinate device:
    let subordpath = crate::ptsname(fdm);
    if subordpath.is_null() {
        return bail(fdm, -1);
    }

    // Open the subordinate device without setting it as the controlling
    // terminal for this process:
    let fds = crate::open(subordpath, O_RDWR);
    if fds < 0 {
        return bail(fdm, -1);
    }

    // Check if the STREAMS modules are already pushed:
    let setup = crate::ioctl(fds, I_FIND, LDTERM.as_ptr());
    if setup < 0 {
        return bail(fdm, fds);
    } else if setup == 0 {
        // The line discipline is not present, so push the appropriate STREAMS
        // modules for the subordinate device:
        if crate::ioctl(fds, I_PUSH, PTEM.as_ptr()) < 0
            || crate::ioctl(fds, I_PUSH, LDTERM.as_ptr()) < 0
        {
            return bail(fdm, fds);
        }
    }

    // If provided, set the terminal parameters:
    if !termp.is_null() && crate::tcsetattr(fds, TCSAFLUSH, termp) != 0 {
        return bail(fdm, fds);
    }

    // If provided, set the window size:
    if !winp.is_null() && crate::ioctl(fds, TIOCSWINSZ, winp) < 0 {
        return bail(fdm, fds);
    }

    // If the caller wants the name of the subordinate device, copy it out.
    //
    // Note that this is a terrible interface: there appears to be no standard
    // upper bound on the copy length for this pointer.  Nobody should pass
    // anything but NULL here, preferring instead to use ptsname(3C) directly.
    if !name.is_null() {
        crate::strcpy(name, subordpath);
    }

    // Close the main side of the pseudo-terminal pair:
    crate::close(*amain);

    // Use TIOCSCTTY to set the subordinate device as our controlling
    // terminal.  This will fail (with ENOTTY) if we are not the leader in
    // our own session, so we call setsid() first.  Finally, arrange for
    // the pseudo-terminal to occupy the standard I/O descriptors.
    if crate::setsid() < 0
        || crate::dup2(fds, 0) < 0
        || crate::dup2(fds, 1) < 0
        || crate::dup2(fds, 2) < 0
    {
        // At this stage there are no particularly good ways to handle failure.
        // Exit as abruptly as possible, using _exit() to avoid messing with any
        // state still shared with the parent process.
        crate::_exit(EXIT_FAILURE);
    }
    // Close the inherited descriptor, taking care to avoid closing the standard
    // descriptors by mistake:
    if fds > 2 {
        crate::close(fds);
    }

    0
}
