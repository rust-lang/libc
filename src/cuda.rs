pub type c_char = u8;
pub type size_t = usize;

cfg_if! {
    if #[cfg(libc_core_cvoid)] {
        pub use ::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        #[allow(missing_copy_implementations)]
        #[allow(missing_debug_implementations)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}

extern "C" {
    /// Syscall to be used whenever the *assert expression produces a `false` value*.
    ///
    /// Syscall arguments:
    /// * `message`: The pointer to the string that should be output.
    /// * `file`: The pointer to the file name string associated with the assert.
    /// * `line`: The line number associated with the assert.
    /// * `function`: The pointer to the function name string associated with the assert.
    /// * `char_size`: The size in bytes of the characters contained in the provided strings. The only supported character size is 1.
    ///
    /// Source:
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    pub fn __assertfail(
        message: *const c_char,
        file: *const c_char,
        line: i32,
        function: *const c_char,
        char_size: size_t,
    );

    /// Print formatted output from a kernel to a host-side output stream.
    ///
    /// Syscall arguments:
    /// * `status`: The status value that is returned by `vprintf`.
    /// * `format`: A pointer to the format specifier input (uses common `printf` format).
    /// * `valist`: A pointer to the valist input.
    ///
    /// ```
    /// #[repr(C)]
    /// struct PrintArgs(f32, f32, f32, i32);
    ///
    /// vprintf(
    ///     "int(%f + %f) = int(%f) = %d\n".as_ptr(),
    ///     transmute(&PrintArgs(a, b, a + b, (a + b) as i32)),
    /// );
    /// ```
    ///
    /// Sources:
    /// [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#formatted-output),
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    pub fn vprintf(format: *const c_char, valist: *const c_void) -> i32;

    /// Allocate memory dynamically from a fixed-size heap in global memory.
    ///
    /// The CUDA in-kernel `malloc()` function allocates at least `size` bytes
    /// from the device heap and returns a pointer to the allocated memory
    /// or `NULL` if insufficient memory exists to fulfill the request.
    ///
    /// The returned pointer is guaranteed to be aligned to a 16-byte boundary.
    ///
    /// The memory allocated by a given CUDA thread via `malloc()` remains allocated
    /// for the lifetime of the CUDA context, or until it is explicitly released
    /// by a call to `free()`. It can be used by any other CUDA threads
    /// even from subsequent kernel launches.
    ///
    /// Sources:
    /// [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#dynamic-global-memory-allocation-and-operations),
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    pub fn malloc(size: size_t) -> *mut c_void;

    /// Free previously dynamically allocated memory.
    ///
    /// The CUDA in-kernel `free()` function deallocates the memory pointed to by `ptr`,
    /// which must have been returned by a previous call to `malloc()`. If `ptr` is NULL,
    /// the call to `free()` is ignored.
    ///
    /// Any CUDA thread may free memory allocated by another thread, but care should be taken
    /// to ensure that the same pointer is not freed more than once. Repeated calls to `free()`
    /// with the same `ptr` has undefined behavior.
    ///
    /// Sources:
    /// [Programming Guide](https://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#dynamic-global-memory-allocation-and-operations),
    /// [PTX Interoperability](https://docs.nvidia.com/cuda/ptx-writers-guide-to-interoperability/index.html#system-calls).
    pub fn free(ptr: *mut c_void);
}
