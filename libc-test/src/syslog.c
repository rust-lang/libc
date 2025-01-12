#include <sys/syslog.h>

// Since the syslog(3) macros are macros instead of functions, they aren't
// available to FFI.  libc must reimplement them, which is error-prone.  This
// file provides FFI access to the actual macros so they can be tested against
// the Rust reimplementations.

int LOG_MASK_ffi(int priority) {
    return LOG_MASK(priority);
}

int LOG_UPTO_ffi(int priority) {
    return LOG_UPTO(priority);
}
