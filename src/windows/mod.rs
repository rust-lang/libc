//! Windows CRT definitions

pub type int8_t = i8;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type ssize_t = isize;

pub type c_char = i8;
pub type c_long = i32;
pub type c_ulong = u32;
pub type wchar_t = u16;

pub type clock_t = i32;

cfg_if! {
    if #[cfg(all(target_arch = "x86", target_env = "gnu"))] {
        pub type time_t = i32;
    } else {
        pub type time_t = i64;
    }
}

pub type off_t = i32;
pub type dev_t = u32;
pub type ino_t = u16;
pub enum timezone {}
pub type time64_t = i64;

s! {
    // note this is the struct called stat64 in Windows. Not stat, nor stati64.
    pub struct stat {
        pub st_dev: dev_t,
        pub st_ino: ino_t,
        pub st_mode: u16,
        pub st_nlink: ::c_short,
        pub st_uid: ::c_short,
        pub st_gid: ::c_short,
        pub st_rdev: dev_t,
        pub st_size: i64,
        pub st_atime: time64_t,
        pub st_mtime: time64_t,
        pub st_ctime: time64_t,
    }

    // note that this is called utimbuf64 in Windows
    pub struct utimbuf {
        pub actime: time64_t,
        pub modtime: time64_t,
    }

    pub struct tm {
        tm_sec: ::c_int,
        tm_min: ::c_int,
        tm_hour: ::c_int,
        tm_mday: ::c_int,
        tm_mon: ::c_int,
        tm_year: ::c_int,
        tm_wday: ::c_int,
        tm_yday: ::c_int,
        tm_isdst: ::c_int,
    }

    pub struct timeval {
        pub tv_sec: c_long,
        pub tv_usec: c_long,
    }

    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: c_long,
    }
}

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;
pub const RAND_MAX: ::c_int = 32767;
pub const EOF: ::c_int = -1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const _IOFBF: ::c_int = 0;
pub const _IONBF: ::c_int = 4;
pub const _IOLBF: ::c_int = 64;
pub const BUFSIZ: ::c_uint = 512;
pub const FOPEN_MAX: ::c_uint = 20;
pub const FILENAME_MAX: ::c_uint = 260;

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_APPEND: ::c_int = 8;
pub const O_CREAT: ::c_int = 256;
pub const O_EXCL: ::c_int = 1024;
pub const O_TEXT: ::c_int = 16384;
pub const O_BINARY: ::c_int = 32768;
pub const O_NOINHERIT: ::c_int = 128;
pub const O_TRUNC: ::c_int = 512;
pub const S_IFCHR: ::c_int = 8192;
pub const S_IFDIR: ::c_int = 16384;
pub const S_IFREG: ::c_int = 32768;
pub const S_IFMT: ::c_int = 61440;
pub const S_IEXEC: ::c_int = 64;
pub const S_IWRITE: ::c_int = 128;
pub const S_IREAD: ::c_int = 256;

pub const LC_ALL: ::c_int = 0;
pub const LC_COLLATE: ::c_int = 1;
pub const LC_CTYPE: ::c_int = 2;
pub const LC_MONETARY: ::c_int = 3;
pub const LC_NUMERIC: ::c_int = 4;
pub const LC_TIME: ::c_int = 5;

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const EDEADLK: ::c_int = 36;
pub const EDEADLOCK: ::c_int = 36;
pub const ENAMETOOLONG: ::c_int = 38;
pub const ENOLCK: ::c_int = 39;
pub const ENOSYS: ::c_int = 40;
pub const ENOTEMPTY: ::c_int = 41;
pub const EILSEQ: ::c_int = 42;
pub const STRUNCATE: ::c_int = 80;

// inline comment below appeases style checker
#[cfg(all(target_env = "msvc", feature = "rustc-dep-of-std"))] // " if "
#[link(name = "msvcrt", cfg(not(target_feature = "crt-static")))]
#[link(name = "libcmt", cfg(target_feature = "crt-static"))]
extern {}

pub enum FILE {}
pub enum fpos_t {} // TODO: fill this out with a struct

extern {
    pub fn isalnum(c: c_int) -> c_int;
    pub fn isalpha(c: c_int) -> c_int;
    pub fn iscntrl(c: c_int) -> c_int;
    pub fn isdigit(c: c_int) -> c_int;
    pub fn isgraph(c: c_int) -> c_int;
    pub fn islower(c: c_int) -> c_int;
    pub fn isprint(c: c_int) -> c_int;
    pub fn ispunct(c: c_int) -> c_int;
    pub fn isspace(c: c_int) -> c_int;
    pub fn isupper(c: c_int) -> c_int;
    pub fn isxdigit(c: c_int) -> c_int;
    pub fn tolower(c: c_int) -> c_int;
    pub fn toupper(c: c_int) -> c_int;
    pub fn fopen(filename: *const c_char, mode: *const c_char) -> *mut FILE;
    pub fn freopen(filename: *const c_char, mode: *const c_char,
                   file: *mut FILE) -> *mut FILE;
    pub fn fflush(file: *mut FILE) -> c_int;
    pub fn fclose(file: *mut FILE) -> c_int;
    pub fn remove(filename: *const c_char) -> c_int;
    pub fn rename(oldname: *const c_char, newname: *const c_char) -> c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn setvbuf(stream: *mut FILE, buffer: *mut c_char, mode: c_int,
                   size: size_t) -> c_int;
    pub fn setbuf(stream: *mut FILE, buf: *mut c_char);
    pub fn getchar() -> c_int;
    pub fn putchar(c: c_int) -> c_int;
    pub fn fgetc(stream: *mut FILE) -> c_int;
    pub fn fgets(buf: *mut c_char, n: c_int, stream: *mut FILE) -> *mut c_char;
    pub fn fputc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fputs(s: *const c_char, stream: *mut FILE) -> c_int;
    pub fn puts(s: *const c_char) -> c_int;
    pub fn ungetc(c: c_int, stream: *mut FILE) -> c_int;
    pub fn fread(ptr: *mut c_void, size: size_t, nobj: size_t,
                 stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const c_void, size: size_t, nobj: size_t,
                  stream: *mut FILE) -> size_t;
    pub fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    pub fn ftell(stream: *mut FILE) -> c_long;
    pub fn rewind(stream: *mut FILE);
    pub fn fgetpos(stream: *mut FILE, ptr: *mut fpos_t) -> c_int;
    pub fn fsetpos(stream: *mut FILE, ptr: *const fpos_t) -> c_int;
    pub fn feof(stream: *mut FILE) -> c_int;
    pub fn ferror(stream: *mut FILE) -> c_int;
    pub fn perror(s: *const c_char);
    pub fn atoi(s: *const c_char) -> c_int;
    pub fn strtod(s: *const c_char, endp: *mut *mut c_char) -> c_double;
    pub fn strtol(s: *const c_char, endp: *mut *mut c_char,
                  base: c_int) -> c_long;
    pub fn strtoul(s: *const c_char, endp: *mut *mut c_char,
                   base: c_int) -> c_ulong;
    pub fn calloc(nobj: size_t, size: size_t) -> *mut c_void;
    pub fn malloc(size: size_t) -> *mut c_void;
    pub fn realloc(p: *mut c_void, size: size_t) -> *mut c_void;
    pub fn free(p: *mut c_void);
    pub fn abort() -> !;
    pub fn exit(status: c_int) -> !;
    pub fn _exit(status: c_int) -> !;
    pub fn atexit(cb: extern fn()) -> c_int;
    pub fn system(s: *const c_char) -> c_int;
    pub fn getenv(s: *const c_char) -> *mut c_char;

    pub fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    pub fn strncpy(dst: *mut c_char, src: *const c_char,
                   n: size_t) -> *mut c_char;
    pub fn strcat(s: *mut c_char, ct: *const c_char) -> *mut c_char;
    pub fn strncat(s: *mut c_char, ct: *const c_char,
                   n: size_t) -> *mut c_char;
    pub fn strcmp(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strncmp(cs: *const c_char, ct: *const c_char, n: size_t) -> c_int;
    pub fn strcoll(cs: *const c_char, ct: *const c_char) -> c_int;
    pub fn strchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strrchr(cs: *const c_char, c: c_int) -> *mut c_char;
    pub fn strspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strcspn(cs: *const c_char, ct: *const c_char) -> size_t;
    pub fn strdup(cs: *const c_char) -> *mut c_char;
    pub fn strpbrk(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strstr(cs: *const c_char, ct: *const c_char) -> *mut c_char;
    pub fn strlen(cs: *const c_char) -> size_t;
    pub fn strnlen(cs: *const c_char, maxlen: size_t) -> size_t;
    pub fn strerror(n: c_int) -> *mut c_char;
    pub fn strtok(s: *mut c_char, t: *const c_char) -> *mut c_char;
    pub fn strxfrm(s: *mut c_char, ct: *const c_char, n: size_t) -> size_t;
    pub fn wcslen(buf: *const wchar_t) -> size_t;
    pub fn wcstombs(dest: *mut c_char, src: *const wchar_t,
                    n: size_t) -> ::size_t;

    pub fn memchr(cx: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    pub fn memcmp(cx: *const c_void, ct: *const c_void, n: size_t) -> c_int;
    pub fn memcpy(dest: *mut c_void, src: *const c_void,
                  n: size_t) -> *mut c_void;
    pub fn memmove(dest: *mut c_void, src: *const c_void,
                   n: size_t) -> *mut c_void;
    pub fn memset(dest: *mut c_void, c: c_int, n: size_t) -> *mut c_void;

    pub fn abs(i: c_int) -> c_int;
    pub fn atof(s: *const c_char) -> c_double;
    pub fn labs(i: c_long) -> c_long;
    pub fn rand() -> c_int;
    pub fn srand(seed: c_uint);

    #[link_name = "_chmod"]
    pub fn chmod(path: *const c_char, mode: ::c_int) -> ::c_int;
    #[link_name = "_wchmod"]
    pub fn wchmod(path: *const wchar_t, mode: ::c_int) -> ::c_int;
    #[link_name = "_mkdir"]
    pub fn mkdir(path: *const c_char) -> ::c_int;
    #[link_name = "_wrmdir"]
    pub fn wrmdir(path: *const wchar_t) -> ::c_int;
    #[link_name = "_fstat64"]
    pub fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int;
    #[link_name = "_stat64"]
    pub fn stat(path: *const c_char, buf: *mut stat) -> ::c_int;
    #[link_name = "_wstat64"]
    pub fn wstat(path: *const wchar_t, buf: *mut stat) -> ::c_int;
    #[link_name = "_wutime64"]
    pub fn wutime(file: *const wchar_t, buf: *mut utimbuf) -> ::c_int;
    #[link_name = "_popen"]
    pub fn popen(command: *const c_char, mode: *const c_char) -> *mut ::FILE;
    #[link_name = "_pclose"]
    pub fn pclose(stream: *mut ::FILE) -> ::c_int;
    #[link_name = "_fdopen"]
    pub fn fdopen(fd: ::c_int, mode: *const c_char) -> *mut ::FILE;
    #[link_name = "_fileno"]
    pub fn fileno(stream: *mut ::FILE) -> ::c_int;
    #[link_name = "_open"]
    pub fn open(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    #[link_name = "_wopen"]
    pub fn wopen(path: *const wchar_t, oflag: ::c_int, ...) -> ::c_int;
    #[link_name = "_creat"]
    pub fn creat(path: *const c_char, mode: ::c_int) -> ::c_int;
    #[link_name = "_access"]
    pub fn access(path: *const c_char, amode: ::c_int) -> ::c_int;
    #[link_name = "_chdir"]
    pub fn chdir(dir: *const c_char) -> ::c_int;
    #[link_name = "_close"]
    pub fn close(fd: ::c_int) -> ::c_int;
    #[link_name = "_dup"]
    pub fn dup(fd: ::c_int) -> ::c_int;
    #[link_name = "_dup2"]
    pub fn dup2(src: ::c_int, dst: ::c_int) -> ::c_int;
    #[link_name = "_execv"]
    pub fn execv(prog: *const c_char, argv: *const *const c_char) -> ::intptr_t;
    #[link_name = "_execve"]
    pub fn execve(prog: *const c_char, argv: *const *const c_char,
                  envp: *const *const c_char) -> ::c_int;
    #[link_name = "_execvp"]
    pub fn execvp(c: *const c_char, argv: *const *const c_char) -> ::c_int;
    #[link_name = "_execvpe"]
    pub fn execvpe(c: *const c_char, argv: *const *const c_char,
                   envp: *const *const c_char) -> ::c_int;
    #[link_name = "_getcwd"]
    pub fn getcwd(buf: *mut c_char, size: ::c_int) -> *mut c_char;
    #[link_name = "_getpid"]
    pub fn getpid() -> ::c_int;
    #[link_name = "_isatty"]
    pub fn isatty(fd: ::c_int) -> ::c_int;
    #[link_name = "_lseek"]
    pub fn lseek(fd: ::c_int, offset: c_long, origin: ::c_int) -> c_long;
    #[link_name = "_pipe"]
    pub fn pipe(fds: *mut ::c_int,
                psize: ::c_uint,
                textmode: ::c_int) -> ::c_int;
    #[link_name = "_read"]
    pub fn read(fd: ::c_int, buf: *mut ::c_void, count: ::c_uint) -> ::c_int;
    #[link_name = "_rmdir"]
    pub fn rmdir(path: *const c_char) -> ::c_int;
    #[link_name = "_unlink"]
    pub fn unlink(c: *const c_char) -> ::c_int;
    #[link_name = "_write"]
    pub fn write(fd: ::c_int, buf: *const ::c_void, count: ::c_uint) -> ::c_int;
    #[link_name = "_commit"]
    pub fn commit(fd: ::c_int) -> ::c_int;
    #[link_name = "_get_osfhandle"]
    pub fn get_osfhandle(fd: ::c_int) -> ::intptr_t;
    #[link_name = "_open_osfhandle"]
    pub fn open_osfhandle(osfhandle: ::intptr_t, flags: ::c_int) -> ::c_int;
    pub fn setlocale(category: ::c_int, locale: *const c_char) -> *mut c_char;
    #[link_name = "_wsetlocale"]
    pub fn wsetlocale(category: ::c_int,
                      locale: *const wchar_t) -> *mut wchar_t;
}

cfg_if! {
    if #[cfg(core_cvoid)] {
        pub use core::ffi::c_void;
    } else {
        // Use repr(u8) as LLVM expects `void*` to be the same as `i8*` to help
        // enable more optimization opportunities around it recognizing things
        // like malloc/free.
        #[repr(u8)]
        pub enum c_void {
            // Two dummy variants so the #[repr] attribute can be used.
            #[doc(hidden)]
            __variant1,
            #[doc(hidden)]
            __variant2,
        }
    }
}

cfg_if! {
    if #[cfg(all(target_env = "gnu"))] {
        mod gnu;
        pub use self::gnu::*;
    } else if #[cfg(all(target_env = "msvc"))] {
        mod msvc;
        pub use self::msvc::*;
    } else {
        // Unknown target_env
    }
}