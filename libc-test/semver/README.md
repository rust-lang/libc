# Supported API by libc

These files are read by [`build.rs`](../build.rs) and turned into tests to
ensure that APIs aren't removed between libc releases.

## File order

Files are including in the following order:
 * Family, e.g. `unix.txt`. NOTE: Windows is skipped here and includes as OS
   name below.
 * Vendor, e.g. `apple.txt`. This allows us to have a single file with system
   calls shared between multiple OSs, e.g. `ios.txt`, `macos.txt` share the same
   kernel.
 * OS, e.g `linux.txt`, `macos.txt`, `windows.txt`.
 * Architecture specific system calls, e.g. `linux-x86_64.txt` or
   `linux-aarch64.txt`.
 * Target environment, e.g. `windows-mscv.txt` or `windows-gnu.txt`.
