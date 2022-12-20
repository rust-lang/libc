#include <sys/types.h>
#if defined(__linux__) || defined(__EMSCRIPTEN__)
#include <sys/sysmacros.h>
#endif

// Since makedev is a macro instead of a function, it isn't available to FFI.
// libc must reimplement it, which is error-prone.  This file provides FFI
// access to the actual macro so it can be tested against the Rust
// reimplementation.

dev_t makedev_ffi(unsigned major, unsigned minor) {
	return makedev(major, minor);
}
