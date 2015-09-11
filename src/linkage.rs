
// On NaCl, these libraries are static. Thus it would be a Bad Idea to link them
// in when creating a test crate.
#[cfg(not(any(windows, target_env = "musl", all(target_os = "nacl", test))))]
#[link(name = "c")]
#[link(name = "m")]
extern {}

// When compiling rust with musl, statically include libc.a in liblibc.rlib.
// A cargo build of the libc crate will therefore automatically pick up the
// libc.a symbols because liblibc is transitively linked to by the stdlib.
#[cfg(all(target_env = "musl", not(feature = "cargo-build"), not(test)))]
#[link(name = "c", kind = "static")]
extern {}

#[cfg(all(windows, target_env = "msvc"))]
#[link(name = "kernel32")]
#[link(name = "shell32")]
#[link(name = "msvcrt")]
extern {}

// libnacl provides functions that require a trip through the IRT to work.
// ie: _exit, mmap, nanosleep, etc. Anything that would otherwise require a trip
// to the kernel.
#[cfg(all(target_os = "nacl", not(feature = "cargo-build"), not(test)))]
#[link(name = "nacl", kind = "static")]
extern {}

// pnaclmm provides a number of functions that the toolchain's Clang emits calls
// to when codegening atomic ops. All the functions within wrap various atomic
// operations.
// Yes, it could be linked by rustc explicitly, however by linking it here
// instead we save a bit of time where bins are involved (by not running the
// optimizations on the whole pnaclmm foreach binary built).
#[cfg(all(target_os = "nacl", not(feature = "cargo-build"), not(test)))]
#[link(name = "pnaclmm", kind = "static")]
extern {}

