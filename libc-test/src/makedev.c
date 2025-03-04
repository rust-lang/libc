#include <sys/types.h>
#if defined(__linux__) || defined(__EMSCRIPTEN__) || defined(__CYGWIN__)
#include <sys/sysmacros.h>
#endif

// Since makedev, major, minor are macros instead of functions, they aren't
// available to FFI. libc must reimplement them, which is error-prone. This
// file provides FFI access to the actual macros so they can be tested against
// the Rust reimplementation.

dev_t makedev_ffi(unsigned major, unsigned minor) {
	return makedev(major, minor);
}

unsigned int major_ffi(dev_t dev) {
    return major(dev);
}

unsigned int minor_ffi(dev_t dev) {
    return minor(dev);
}
