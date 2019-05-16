#![deny(warnings)]

extern crate cc;
extern crate ctest;

use std::env;

fn do_cc() {
    let target = env::var("TARGET").unwrap();
    if cfg!(unix) && !target.contains("wasi") {
        cc::Build::new().file("src/cmsg.c").compile("cmsg");
    }
}

fn do_ctest() {
    match &env::var("TARGET").unwrap() {
        t if t.contains("android") => return test_android(t),
        t if t.contains("apple") => return test_apple(t),
        t if t.contains("cloudabi") => return test_cloudabi(t),
        t if t.contains("dragonfly") => return test_dragonflybsd(t),
        t if t.contains("emscripten") => return test_emscripten(t),
        t if t.contains("freebsd") => return test_freebsd(t),
        t if t.contains("linux") => return test_linux(t),
        t if t.contains("netbsd") => return test_netbsd(t),
        t if t.contains("openbsd") => return test_openbsd(t),
        t if t.contains("redox") => return test_redox(t),
        t if t.contains("solaris") => return test_solaris(t),
        t if t.contains("wasi") => return test_wasi(t),
        t if t.contains("windows") => return test_windows(t),
        t => panic!("unknown target {}", t),
    }
}

fn main() {
    do_cc();
    do_ctest();
}

macro_rules! headers {
    ($cfg:ident: $header:expr) => {
        $cfg.header($header);
    };
    ($cfg:ident: $($header:expr),*) => {
        $(headers!($cfg: $header);)*
    };
    ($cfg:ident: $($header:expr,)*) => {
        $(headers!($cfg: $header);)*
    };
}

fn test_apple(target: &str) {
    assert!(target.contains("apple"));
    let x86_64 = target.contains("x86_64");

    let mut cfg = ctest::TestGenerator::new();
    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("__APPLE_USE_RFC_3542", None);

    headers! { cfg:
        "aio.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "langinfo.h",
        "limits.h",
        "locale.h",
        "mach-o/dyld.h",
        "mach/mach_time.h",
        "malloc/malloc.h",
        "net/bpf.h",
        "net/if.h",
        "net/if_arp.h",
        "net/if_dl.h",
        "net/if_utun.h",
        "net/route.h",
        "net/route.h",
        "netdb.h",
        "netinet/if_ether.h",
        "netinet/in.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "poll.h",
        "pthread.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "spawn.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "sys/event.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/ipc.h",
        "sys/kern_control.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/proc_info.h",
        "sys/ptrace.h",
        "sys/quota.h",
        "sys/resource.h",
        "sys/sem.h",
        "sys/shm.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/sys_domain.h",
        "sys/sysctl.h",
        "sys/time.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "sys/xattr.h",
        "syslog.h",
        "termios.h",
        "time.h",
        "unistd.h",
        "util.h",
        "utime.h",
        "utmpx.h",
        "wchar.h",
        "xlocale.h",
    }

    if x86_64 {
        headers! { cfg: "crt_externs.h" }
    }

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME: actually a union
            "sigval" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // These OSX constants are removed in Sierra.
            // https://developer.apple.com/library/content/releasenotes/General/APIDiffsMacOS10_12/Swift/Darwin.html
            "KERN_KDENABLE_BG_TRACE" | "KERN_KDDISABLE_BG_TRACE" => true,
            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" => true,

            // close calls the close_nocancel system call
            "close" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        match (struct_, field) {
            // FIXME: actually a union
            ("sigevent", "sigev_value") => true,
            _ => false,
        }
    });

    cfg.volatile_item(|i| {
        use ctest::VolatileItemKind::*;
        match i {
            StructField(ref n, ref f) if n == "aiocb" && f == "aio_buf" => {
                true
            }
            _ => false,
        }
    });

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "DIR" | "Dl_info" => ty.to_string(),

            // OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {}", t),
            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", "espec.tv_nsec")
            }
            // FIXME: sigaction actually contains a union with two variants:
            // a sa_sigaction with type: (*)(int, struct __siginfo *, void *)
            // a sa_handler with type sig_t
            "sa_sigaction" if struct_ == "sigaction" => {
                "sa_handler".to_string()
            }
            s => s.to_string(),
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_openbsd(target: &str) {
    assert!(target.contains("openbsd"));

    let mut cfg = ctest::TestGenerator::new();
    cfg.flag("-Wno-deprecated-declarations");

    headers! { cfg:
        "errno.h",
        "fcntl.h",
        "limits.h",
        "locale.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "sys/stat.h",
        "sys/types.h",
        "time.h",
        "wchar.h",
        "ctype.h",
        "dirent.h",
        "sys/socket.h",
        "net/if.h",
        "net/route.h",
        "net/if_arp.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "resolv.h",
        "pthread.h",
        "dlfcn.h",
        "signal.h",
        "string.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/mman.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/time.h",
        "sys/un.h",
        "sys/wait.h",
        "unistd.h",
        "utime.h",
        "pwd.h",
        "grp.h",
        "sys/utsname.h",
        "sys/ptrace.h",
        "sys/mount.h",
        "sys/uio.h",
        "sched.h",
        "termios.h",
        "poll.h",
        "syslog.h",
        "semaphore.h",
        "sys/statvfs.h",
        "sys/times.h",
        "glob.h",
        "ifaddrs.h",
        "langinfo.h",
        "sys/sysctl.h",
        "utmp.h",
        "sys/event.h",
        "net/if_dl.h",
        "util.h",
        "ufs/ufs/quota.h",
        "pthread_np.h",
        "sys/syscall.h",
    }

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME: actually a union
            "sigval" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // Removed in OpenBSD 6.0
            "KERN_USERMOUNT" | "KERN_ARND" => true,
            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        match name {
            "execv" | "execve" | "execvp" | "execvpe" => true,

            // typed 2nd arg
            "gettimeofday" => true,

            // Removed in OpenBSD 6.5
            // https://marc.info/?l=openbsd-cvs&m=154723400730318
            "mincore" => true,

            _ => false,
        }
    });

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "DIR" | "Dl_info" => ty.to_string(),

            // OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {}", t),
            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| match field {
        "st_birthtime" if struct_.starts_with("stat") => {
            "__st_birthtime".to_string()
        }
        "st_birthtime_nsec" if struct_.starts_with("stat") => {
            "__st_birthtimensec".to_string()
        }
        s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
            s.replace("e_nsec", ".tv_nsec")
        }
        "sa_sigaction" if struct_ == "sigaction" => "sa_handler".to_string(),
        s => s.to_string(),
    });

    cfg.skip_field_type(move |struct_, field| {
        // type siginfo_t.si_addr changed from OpenBSD 6.0 to 6.1
        (struct_ == "siginfo_t" && field == "si_addr")
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_windows(target: &str) {
    assert!(target.contains("windows"));
    let gnu = target.contains("gnu");

    let mut cfg = ctest::TestGenerator::new();
    cfg.define("_WIN32_WINNT", Some("0x8000"));

    headers! { cfg:
        "direct.h",
        "errno.h",
        "fcntl.h",
        "io.h",
        "limits.h",
        "locale.h",
        "process.h",
        "signal.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "sys/stat.h",
        "sys/types.h",
        "sys/utime.h",
        "time.h",
        "wchar.h",
    }

    if gnu {
        headers! { cfg: "ws2tcpip.h" }
    } else {
        headers! { cfg: "Winsock2.h" };
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "DIR" | "Dl_info" => ty.to_string(),

            // FIXME: these don't exist:
            "time64_t" => "__time64_t".to_string(),
            "ssize_t" => "SSIZE_T".to_string(),

            "sighandler_t" if !gnu => "_crt_signal_t".to_string(),
            "sighandler_t" if gnu => "__p_sig_fn_t".to_string(),

            t if is_union => format!("union {}", t),
            t if t.ends_with("_t") => t.to_string(),

            // Windows uppercase structs don't have `struct` in front:
            t if is_struct => {
                if ty.clone().chars().next().unwrap().is_uppercase() {
                    t.to_string()
                } else if t == "stat" {
                    "struct __stat64".to_string()
                } else if t == "utimbuf" {
                    "struct __utimbuf64".to_string()
                } else {
                    // put `struct` in front of all structs:
                    format!("struct {}", t)
                }
            }
            t => t.to_string(),
        }
    });

    cfg.fn_cname(move |name, cname| cname.unwrap_or(name).to_string());

    cfg.skip_type(move |name| match name {
        "SSIZE_T" if !gnu => true,
        "ssize_t" if !gnu => true,
        _ => false,
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: API error:
            // SIG_ERR type is "void (*)(int)", not "int"
            "SIG_ERR" => true,
            _ => false,
        }
    });

    // FIXME: All functions point to the wrong addresses?
    cfg.skip_fn_ptrcheck(|_| true);

    cfg.skip_signededness(move |c| {
        match c {
            // windows-isms
            n if n.starts_with("P") => true,
            n if n.starts_with("H") => true,
            n if n.starts_with("LP") => true,
            "sighandler_t" if gnu => true,
            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        match name {
            // FIXME: API error:
            "execv" | "execve" | "execvp" | "execvpe" => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_redox(target: &str) {
    assert!(target.contains("redox"));

    let mut cfg = ctest::TestGenerator::new();
    cfg.flag("-Wno-deprecated-declarations");

    headers! {
        cfg:
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "langinfo.h",
        "limits.h",
        "locale.h",
        "net/if.h",
        "net/if_arp.h",
        "net/route.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "poll.h",
        "pthread.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "string.h",
        "strings.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/ptrace.h",
        "sys/quota.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/sysctl.h",
        "sys/time.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "syslog.h",
        "termios.h",
        "time.h",
        "unistd.h",
        "utime.h",
        "utmpx.h",
        "wchar.h",
    }

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_cloudabi(target: &str) {
    assert!(target.contains("cloudabi"));

    let mut cfg = ctest::TestGenerator::new();
    cfg.flag("-Wno-deprecated-declarations");

    headers! {
        cfg:
        "execinfo.h",
        "glob.h",
        "ifaddrs.h",
        "langinfo.h",
        "sys/ptrace.h",
        "sys/quota.h",
        "sys/sysctl.h",
        "utmpx.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "fcntl.h",
        "grp.h",
        "limits.h",
        "locale.h",
        "net/if.h",
        "net/if_arp.h",
        "net/route.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "poll.h",
        "pthread.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "strings.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/time.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "syslog.h",
        "termios.h",
        "time.h",
        "unistd.h",
        "utime.h",
        "wchar.h",
    }

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_solaris(target: &str) {
    assert!(target.contains("solaris"));

    let mut cfg = ctest::TestGenerator::new();
    cfg.flag("-Wno-deprecated-declarations");

    cfg.define("_XOPEN_SOURCE", Some("700"));
    cfg.define("__EXTENSIONS__", None);
    cfg.define("_LCONV_C99", None);

    headers! {
        cfg:
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "langinfo.h",
        "limits.h",
        "locale.h",
        "net/if.h",
        "net/if_arp.h",
        "net/route.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "poll.h",
        "port.h",
        "pthread.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "sys/epoll.h",
        "sys/file.h",
        "sys/filio.h",
        "sys/ioctl.h",
        "sys/loadavg.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/time.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "syslog.h",
        "termios.h",
        "time.h",
        "ucontext.h",
        "unistd.h",
        "utime.h",
        "utmpx.h",
        "wchar.h",
    }

    cfg.skip_const(move |name| match name {
        "DT_FIFO" | "DT_CHR" | "DT_DIR" | "DT_BLK" | "DT_REG" | "DT_LNK"
        | "DT_SOCK" | "USRQUOTA" | "GRPQUOTA" | "PRIO_MIN" | "PRIO_MAX" => {
            true
        }

        _ => false,
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // const-ness only added recently
            "dladdr" => true,

            // Definition of those functions as changed since unified headers
            // from NDK r14b These changes imply some API breaking changes but
            // are still ABI compatible. We can wait for the next major release
            // to be compliant with the new API.
            //
            // FIXME: unskip these for next major release
            "setpriority" | "personality" => true,

            // signal is defined with sighandler_t, so ignore
            "signal" => true,

            "cfmakeraw" | "cfsetspeed" => true,

            // FIXME: mincore is defined with caddr_t on Solaris.
            "mincore" => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_netbsd(target: &str) {
    assert!(target.contains("netbsd"));
    let rumprun = target.contains("rumprun");
    let mut cfg = ctest::TestGenerator::new();

    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("_NETBSD_SOURCE", Some("1"));

    headers! {
        cfg:
        "errno.h",
        "fcntl.h",
        "limits.h",
        "locale.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "sys/stat.h",
        "sys/types.h",
        "time.h",
        "wchar.h",
        "aio.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "langinfo.h",
        "net/if.h",
        "net/if_arp.h",
        "net/if_dl.h",
        "net/route.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "poll.h",
        "pthread.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "string.h",
        "sys/extattr.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/ioctl_compat.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/ptrace.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/statvfs.h",
        "sys/sysctl.h",
        "sys/time.h",
        "sys/times.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "syslog.h",
        "termios.h",
        "ufs/ufs/quota.h",
        "ufs/ufs/quota1.h",
        "unistd.h",
        "util.h",
        "utime.h",
        "mqueue.h",
        "netinet/dccp.h",
        "sys/event.h",
        "sys/quota.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr"
            | "Elf64_Phdr" | "Elf32_Shdr" | "Elf64_Shdr" | "Elf32_Sym"
            | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr" | "Elf32_Chdr"
            | "Elf64_Chdr" => ty.to_string(),

            // OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", ".tv_nsec")
            }
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // FIXME: sighandler_t is crazy across platforms
            "sighandler_t" => true,
            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // This is actually a union, not a struct
            "sigval" => true,
            // These are tested as part of the linux_fcntl tests since there are
            // header conflicts when including them with all the other structs.
            "termios2" => true,
            _ => false,
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            "LARGE_INTEGER" | "float" | "double" => true,
            // uuid_t is a struct, not an integer.
            n if n.starts_with("pthread") => true,
            // sem_t is a struct or pointer
            "sem_t" => true,
            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            "SIGUNUSED" => true,                       // removed in glibc 2.26

            // weird signed extension or something like that?
            "MS_NOUSER" => true,
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13
            "BOTHER" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        match name {
            // FIXME: incorrect API
            "execv" |
            "execve" |
            "execvp" |
            "execvpe" |
            "fexecve" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64" |        // non-int in 2nd arg

            // These functions presumably exist on netbsd but don't look like
            // they're implemented on rumprun yet, just let them slide for now.
            // Some of them look like they have headers but then don't have
            // corresponding actual definitions either...
            "shm_open" |
            "shm_unlink" |
            "syscall" |
            "mq_open" |
            "mq_close" |
            "mq_getattr" |
            "mq_notify" |
            "mq_receive" |
            "mq_send" |
            "mq_setattr" |
            "mq_timedreceive" |
            "mq_timedsend" |
            "mq_unlink" |
            "ptrace" |
            "sigaltstack" if rumprun => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        (struct_ == "aiocb" && field == "aio_buf")
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_dragonflybsd(target: &str) {
    assert!(target.contains("dragonfly"));
    let mut cfg = ctest::TestGenerator::new();
    cfg.flag("-Wno-deprecated-declarations");

    headers! {
        cfg:
        "aio.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "langinfo.h",
        "limits.h",
        "locale.h",
        "mqueue.h",
        "net/if.h",
        "net/if_arp.h",
        "net/if_dl.h",
        "net/route.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "poll.h",
        "pthread.h",
        "pthread_np.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "sys/event.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/ptrace.h",
        "sys/resource.h",
        "sys/rtprio.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/sysctl.h",
        "sys/time.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "syslog.h",
        "termios.h",
        "time.h",
        "ufs/ufs/quota.h",
        "unistd.h",
        "util.h",
        "utime.h",
        "utmpx.h",
        "wchar.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr"
            | "Elf64_Phdr" | "Elf32_Shdr" | "Elf64_Shdr" | "Elf32_Sym"
            | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr" | "Elf32_Chdr"
            | "Elf64_Chdr" => ty.to_string(),

            // FIXME: OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", ".tv_nsec")
            }
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            "type_"
                if struct_ == "input_event"
                    || struct_ == "input_mask"
                    || struct_ == "ff_effect"
                    || struct_ == "rtprio" =>
            {
                "type".to_string()
            }
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            "sighandler_t" => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // This is actually a union, not a struct
            "sigval" => true,

            // FIXME: These are tested as part of the linux_fcntl tests since
            // there are header conflicts when including them with all the other
            // structs.
            "termios2" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            "LARGE_INTEGER" | "float" | "double" => true,
            // uuid_t is a struct, not an integer.
            "uuid_t" => true,
            n if n.starts_with("pthread") => true,
            // sem_t is a struct or pointer
            "sem_t" => true,
            // mqd_t is a pointer on DragonFly
            "mqd_t" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness

            // weird signed extension or something like that?
            "MS_NOUSER" => true,
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // These are defined for Solaris 11, but the crate is tested on
            // illumos, where they are currently not defined
            "EADI"
            | "PORT_SOURCE_POSTWAIT"
            | "PORT_SOURCE_SIGNAL"
            | "PTHREAD_STACK_MIN" => true,

            // These change all the time from release to release of linux
            // distros, let's just not bother trying to verify them. They
            // shouldn't be used in code anyway...
            "AF_MAX" | "PF_MAX" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" |
            "fexecve" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64" |        // non-int in 2nd arg
            // typed 2nd arg on linux
            "gettimeofday" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        (struct_ == "aiocb" && field == "aio_buf")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        (struct_ == "siginfo_t" && field == "_pad") ||
        // sigev_notify_thread_id is actually part of a sigev_un union
        (struct_ == "sigevent" && field == "sigev_notify_thread_id")
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_wasi(target: &str) {
    assert!(target.contains("wasi"));

    let mut cfg = ctest::TestGenerator::new();
    cfg.define("_GNU_SOURCE", None);

    headers! { cfg:
        "ctype.h",
        "dirent.h",
        "errno.h",
        "fcntl.h",
        "limits.h",
        "locale.h",
        "malloc.h",
        "poll.h",
        "sched.h",
        "stdbool.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "sys/resource.h",
        "sys/select.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/utsname.h",
        "time.h",
        "unistd.h",
        "wasi/core.h",
        "wasi/libc.h",
        "wasi/libc-find-relpath.h",
        "wchar.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "FILE" | "fd_set" | "DIR" => ty.to_string(),
        t if is_union => format!("union {}", t),
        t if t.starts_with("__wasi") && t.ends_with("_u") => {
            format!("union {}", t)
        }
        t if t.starts_with("__wasi") && is_struct => format!("struct {}", t),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {}", t),
        t => t.to_string(),
    });

    cfg.field_name(move |_struct, field| {
        match field {
            // deal with fields as rust keywords
            "type_" => "type".to_string(),
            s => s.to_string(),
        }
    });

    // Looks like LLD doesn't merge duplicate imports, so if the Rust
    // code imports from a module and the C code also imports from a
    // module we end up with two imports of function pointers which
    // import the same thing but have different function pointers
    cfg.skip_fn_ptrcheck(|f| f.starts_with("__wasi"));

    // d_name is declared as a flexible array in WASI libc, so it
    // doesn't support sizeof.
    cfg.skip_field(|s, field| s == "dirent" && field == "d_name");

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_android(target: &str) {
    assert!(target.contains("android"));
    let target_pointer_width = match target {
        t if t.contains("aarch64") || t.contains("x86_64") => 64,
        t if t.contains("i686") || t.contains("arm") => 32,
        t => panic!("unsupported target: {}", t),
    };
    let x86 = target.contains("i686") || target.contains("x86_64");

    let mut cfg = ctest::TestGenerator::new();
    cfg.define("_GNU_SOURCE", None);

    // FIXME: still necessary?
    cfg.flag("-Wno-deprecated-declarations");

    // Android doesn't actually have in_port_t but it's much easier if we
    // provide one for us to test against
    // FIXME: still necessary?
    cfg.define("in_port_t", Some("uint16_t"));

    headers! { cfg:
               "arpa/inet.h",
               "asm/mman.h",
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "errno.h",
               "fcntl.h",
               "grp.h",
               "ifaddrs.h",
               "limits.h",
               "linux/dccp.h",
               "linux/futex.h",
               "linux/fs.h",
               "linux/genetlink.h",
               "linux/if_alg.h",
               "linux/if_ether.h",
               "linux/if_tun.h",
               "linux/magic.h",
               "linux/memfd.h",
               "linux/module.h",
               "linux/net_tstamp.h",
               "linux/netfilter/nf_tables.h",
               "linux/netfilter_ipv4.h",
               "linux/netfilter_ipv6.h",
               "linux/netlink.h",
               "linux/quota.h",
               "linux/reboot.h",
               "linux/seccomp.h",
               "linux/sockios.h",
               "locale.h",
               "malloc.h",
               "net/ethernet.h",
               "net/if.h",
               "net/if_arp.h",
               "net/route.h",
               "netdb.h",
               "netinet/in.h",
               "netinet/ip.h",
               "netinet/tcp.h",
               "netinet/udp.h",
               "netpacket/packet.h",
               "poll.h",
               "pthread.h",
               "pty.h",
               "pwd.h",
               "resolv.h",
               "sched.h",
               "semaphore.h",
               "signal.h",
               "stddef.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "sys/epoll.h",
               "sys/eventfd.h",
               "sys/file.h",
               "sys/fsuid.h",
               "sys/inotify.h",
               "sys/ioctl.h",
               "sys/mman.h",
               "sys/mount.h",
               "sys/personality.h",
               "sys/prctl.h",
               "sys/ptrace.h",
               "sys/reboot.h",
               "sys/resource.h",
               "sys/sendfile.h",
               "sys/signalfd.h",
               "sys/socket.h",
               "sys/stat.h",
               "sys/statvfs.h",
               "sys/swap.h",
               "sys/syscall.h",
               "sys/sysinfo.h",
               "sys/time.h",
               "sys/times.h",
               "sys/types.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/utsname.h",
               "sys/vfs.h",
               "sys/wait.h",
               "syslog.h",
               "termios.h",
               "time.h",
               "unistd.h",
               "utime.h",
               "utmp.h",
               "wchar.h",
               "xlocale.h",
    }

    if target_pointer_width == 32 {
        // time64_t is not defined for 64-bit targets If included it will
        // generate the error 'Your time_t is already 64-bit'
        cfg.header("time64.h");
    }
    if x86 {
        cfg.header("sys/reg.h");
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            // FIXME: still required ?
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr"
            | "Elf64_Phdr" | "Elf32_Shdr" | "Elf64_Shdr" | "Elf32_Sym"
            | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr" | "Elf32_Chdr"
            | "Elf64_Chdr" => ty.to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.to_string()
            }
            // FIXME: still necessary?
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            // FIXME: still necessary?
            "sighandler_t" => true,
            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // This is actually a union, not a struct
            // FIXME: still necessary
            "sigval" => true,

            // These structs have changed since unified headers in NDK r14b.
            // `st_atime` and `st_atime_nsec` have changed sign.
            // FIXME: unskip it for next major release
            "stat" | "stat64" => true,

            // These are tested as part of the linux_fcntl tests since there are
            // header conflicts when including them with all the other structs.
            // FIXME: still necessary
            "termios2" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            // FIXME: still necessary?
            "LARGE_INTEGER" | "float" | "double" => true,
            // FIXME: still necessary?
            n if n.starts_with("pthread") => true,
            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: still necessary?
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            // FIXME: still necessary?
            "SIGUNUSED" => true, // removed in glibc 2.26

            // weird signed extension or something like that?
            // FIXME: still necessary?
            "MS_NOUSER" => true,
            // FIXME: still necessary?
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // Android uses old kernel headers
            // These are constants used in getrandom syscall
            // FIXME: still necessary?
            "GRND_NONBLOCK" | "GRND_RANDOM" => true,

            // Defined by libattr not libc on linux (hard to test).
            // See constant definition for more details.
            // FIXME: still necessary?
            "ENOATTR" => true,

            // FIXME: still necessary?
            "BOTHER" => true,

            // MFD_HUGETLB is not available in some older libc versions on the CI builders. On the
            // x86_64 and i686 builders it seems to be available for all targets, so at least test
            // it there.
            // FIXME: still necessary?
            "MFD_HUGETLB" => true,

            // These change all the time from release to release of linux
            // distros, let's just not bother trying to verify them. They
            // shouldn't be used in code anyway...
            // FIXME: still necessary?
            "AF_MAX" | "PF_MAX" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: still necessary?
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" |
            "fexecve" => true,

            // typed 2nd arg on android
            // FIXME: still necessary?
            "gettimeofday" => true,

            // not declared in newer android toolchains
            // FIXME: still necessary?
            "getdtablesize" => true,

            // FIXME: still necessary?
            "dlerror" => true, // const-ness is added

            // Apparently the NDK doesn't have this defined on android, but
            // it's in a header file?
            // FIXME: still necessary?
            "endpwent" => true,

            // Apparently res_init exists on Android, but isn't defined in a header:
            // https://mail.gnome.org/archives/commits-list/2013-May/msg01329.html
            // FIXME: still necessary?
            "res_init" => true,

            // Definition of those functions as changed since unified headers from NDK r14b
            // These changes imply some API breaking changes but are still ABI compatible.
            // We can wait for the next major release to be compliant with the new API.
            // FIXME: unskip these for next major release
            "strerror_r" | "madvise" | "msync" | "mprotect" | "recvfrom" | "getpriority" |
            "setpriority" | "personality"  => true,
            // In Android 64 bits, these functions have been fixed since unified headers.
            // Ignore these until next major version.
            "bind" | "writev" | "readv" | "sendmsg" | "recvmsg"
                if target_pointer_width == 64 => true,

            _ => false,
        }
    });

    cfg.skip_static(move |name| {
        match name {
            // Internal constant, not declared in any headers.
            // FIXME: still necessary
            "__progname" => true,
            _ => false,
        }
    });

    // FIXME: still necessary?
    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        (struct_ == "aiocb" && field == "aio_buf")
    });

    // FIXME: still necessary?
    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        (struct_ == "siginfo_t" && field == "_pad") ||
        // sigev_notify_thread_id is actually part of a sigev_un union
        (struct_ == "sigevent" && field == "sigev_notify_thread_id") ||
        // signalfd had SIGSYS fields added in Linux 4.18, but no libc release has them yet.
        (struct_ == "signalfd_siginfo" && (field == "ssi_addr_lsb" ||
                                           field == "_pad2" ||
                                           field == "ssi_syscall" ||
                                           field == "ssi_call_addr" ||
                                           field == "ssi_arch"))
    });

    // FIXME: remove
    cfg.fn_cname(move |name, _cname| name.to_string());

    cfg.generate("../src/lib.rs", "main.rs");

    // On Android also generate another script for testing linux/fcntl
    // declarations. These cannot be tested normally because including both
    // `linux/fcntl.h` and `fcntl.h` fails.
    //
    // FIXME: is still necessary?
    let mut cfg = ctest::TestGenerator::new();
    cfg.skip_type(|_| true)
        .skip_fn(|_| true)
        .skip_static(|_| true);
    cfg.header("linux/fcntl.h");
    cfg.header("net/if.h");
    cfg.header("linux/if.h");
    cfg.header("linux/quota.h");
    cfg.header("asm/termbits.h");
    cfg.skip_const(move |name| match name {
        "F_CANCELLK" | "F_ADD_SEALS" | "F_GET_SEALS" => false,
        "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW" | "F_SEAL_WRITE" => {
            false
        }
        "BOTHER" => false,
        _ => true,
    });
    cfg.skip_struct(|s| s != "termios2");
    cfg.type_name(move |ty, is_struct, is_union| match ty {
        t if is_struct => format!("struct {}", t),
        t if is_union => format!("union {}", t),
        t => t.to_string(),
    });
    cfg.generate("../src/lib.rs", "linux_fcntl.rs");
}

fn test_freebsd(target: &str) {
    assert!(target.contains("freebsd"));
    let x86 = target.contains("i686") || target.contains("x86_64");

    let mut cfg = ctest::TestGenerator::new();
    // Required for `getline`:
    cfg.define("_WITH_GETLINE", None);

    headers! { cfg:
                "aio.h",
                "arpa/inet.h",
                "ctype.h",
                "dirent.h",
                "dlfcn.h",
                "errno.h",
                "fcntl.h",
                "glob.h",
                "grp.h",
                "ifaddrs.h",
                "langinfo.h",
                "libutil.h",
                "limits.h",
                "locale.h",
                "mqueue.h",
                "net/bpf.h",
                "net/if.h",
                "net/if_arp.h",
                "net/if_dl.h",
                "net/route.h",
                "netdb.h",
                "netinet/ip.h",
                "netinet/in.h",
                "netinet/tcp.h",
                "netinet/udp.h",
                "poll.h",
                "pthread.h",
                "pthread_np.h",
                "pwd.h",
                "resolv.h",
                "sched.h",
                "semaphore.h",
                "signal.h",
                "spawn.h",
                "stddef.h",
                "stdint.h",
                "stdio.h",
                "stdlib.h",
                "string.h",
                "sys/event.h",
                "sys/extattr.h",
                "sys/file.h",
                "sys/ioctl.h",
                "sys/ipc.h",
                "sys/jail.h",
                "sys/mman.h",
                "sys/mount.h",
                "sys/msg.h",
                "sys/procdesc.h",
                "sys/ptrace.h",
                "sys/resource.h",
                "sys/rtprio.h",
                "sys/shm.h",
                "sys/socket.h",
                "sys/stat.h",
                "sys/statvfs.h",
                "sys/sysctl.h",
                "sys/time.h",
                "sys/times.h",
                "sys/types.h",
                "sys/uio.h",
                "sys/un.h",
                "sys/utsname.h",
                "sys/wait.h",
                "syslog.h",
                "termios.h",
                "time.h",
                "ufs/ufs/quota.h",
                "unistd.h",
                "utime.h",
                "utmpx.h",
                "wchar.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            // FIXME: still required?
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr"
            | "Elf64_Phdr" | "Elf32_Shdr" | "Elf64_Shdr" | "Elf32_Sym"
            | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr" | "Elf32_Chdr"
            | "Elf64_Chdr" => ty.to_string(),

            // FIXME: still required?
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", ".tv_nsec")
            }
            // FIXME: still required?
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            // FIXME: still required?
            "type_"
                if struct_ == "input_event"
                    || struct_ == "input_mask"
                    || struct_ == "ff_effect"
                    || struct_ == "rtprio" =>
            {
                "type".to_string()
            }
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            // FIXME: still required?
            "sighandler_t" => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // This is actually a union, not a struct
            // FIXME: still required?
            "sigval" => true,

            // These are tested as part of the linux_fcntl tests since there are
            // header conflicts when including them with all the other structs.
            // FIXME: still required?
            "termios2" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            // FIXME: still required?
            "LARGE_INTEGER" | "float" | "double" => true,
            // FIXME: still required?
            n if n.starts_with("pthread") => true,
            // sem_t is a struct or pointer
            // FIXME: still required?
            "sem_t" => true,
            // mqd_t is a pointer on FreeBSD
            // FIXME: still required?
            "mqd_t" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: still required?
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            // FIXME: still required?
            "SIGUNUSED" => true, // removed in glibc 2.26

            // weird signed extension or something like that?
            // FIXME: still required?
            "MS_NOUSER" => true,
            // FIXME: still required?
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // These constants were removed in FreeBSD 11 (svn r273250) but will
            // still be accepted and ignored at runtime.
            "MAP_RENAME" | "MAP_NORESERVE" => true,

            // These constants were removed in FreeBSD 11 (svn r262489),
            // and they've never had any legitimate use outside of the
            // base system anyway.
            "CTL_MAXID" | "KERN_MAXID" | "HW_MAXID" | "NET_MAXID"
            | "USER_MAXID" => true,

            // These constants were added in FreeBSD 11
            // FIXME: still required?
            "EVFILT_PROCDESC" | "EVFILT_SENDFILE" | "EVFILT_EMPTY"
            | "PD_CLOEXEC" | "PD_ALLOWED_AT_FORK" => true,

            // These constants were added in FreeBSD 12
            // FIXME: still required?
            "SF_USER_READAHEAD" | "SO_REUSEPORT_LB" => true,

            // These constants are tested in a separate test program generated
            // below because there are header conflicts if we try to include the
            // headers that define them here.
            // FIXME: still required?
            "F_CANCELLK" | "F_ADD_SEALS" | "F_GET_SEALS" => true,
            // FIXME: still required?
            "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW"
            | "F_SEAL_WRITE" => true,
            // FIXME: still required?
            "BOTHER" => true,

            // MFD_HUGETLB is not available in some older libc versions on the
            // CI builders. On the x86_64 and i686 builders it seems to be
            // available for all targets, so at least test it there.
            // FIXME: still required?
            "MFD_HUGETLB" if !x86 => true,

            // These change all the time from release to release of linux
            // distros, let's just not bother trying to verify them. They
            // shouldn't be used in code anyway...
            // FIXME: still required?
            "AF_MAX" | "PF_MAX" => true,

            // FreeBSD 12 required, but CI has FreeBSD 11.
            // FIXME: still required?
            "IP_ORIGDSTADDR"
            | "IP_RECVORIGDSTADDR"
            | "IPV6_ORIGDSTADDR"
            | "IPV6_RECVORIGDSTADDR" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: still required?
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" |
            "fexecve" => true,

            // FIXME: for some reason, our signature is wrong
            "gettimeofday" => true,

            // The `uname` function in freebsd is now an inline wrapper that
            // delegates to another, but the symbol still exists, so don't check
            // the symbol.
            // FIXME: still required?
            "uname" => true,

            // FIXME: need to upgrade FreeBSD version; see https://github.com/rust-lang/libc/issues/938
            // FIXME: still required?
            "setgrent" => true,

            // aio_waitcomplete's return type changed between FreeBSD 10 and 11.
            // FIXME: still required?
            "aio_waitcomplete"  => true,

            // lio_listio confuses the checker, probably because one of its
            // arguments is an array
            // FIXME: still required?
            "lio_listio" => true,

            // Definition of those functions as changed since unified headers from NDK r14b
            // These changes imply some API breaking changes but are still ABI compatible.
            // We can wait for the next major release to be compliant with the new API.
            // FIXME: unskip these for next major release
            // FIXME: still required ?
            "strerror_r" | "madvise" | "msync" | "mprotect" | "recvfrom" | "getpriority" |

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        // FIXME: still required?
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // FIXME: still required?
        // sighandler_t type is super weird
            (struct_ == "sigaction" && field == "sa_sigaction") ||
        // FIXME: still required?
        // sigval is actually a union, but we pretend it's a struct
            (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        // FIXME: still required?
            (struct_ == "aiocb" && field == "aio_buf") ||
        // stack_t.ss_sp's type changed from FreeBSD 10 to 11 in svn r294930
        // FIXME: still required?
            (struct_ == "stack_t" && field == "ss_sp")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        // FIXME: still required?
        (struct_ == "siginfo_t" && field == "_pad") ||
        // sigev_notify_thread_id is actually part of a sigev_un union
        // FIXME: still required?
        (struct_ == "sigevent" && field == "sigev_notify_thread_id") ||
        // signalfd had SIGSYS fields added in Linux 4.18, but no libc release has them yet.
        // FIXME: still required?
        (struct_ == "signalfd_siginfo" && (field == "ssi_addr_lsb" ||
                                           field == "_pad2" ||
                                           field == "ssi_syscall" ||
                                           field == "ssi_call_addr" ||
                                           field == "ssi_arch"))
    });

    // FIXME: remove
    cfg.fn_cname(move |name, _cname| name.to_string());

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_emscripten(target: &str) {
    assert!(target.contains("emscripten"));

    let mut cfg = ctest::TestGenerator::new();
    // FIXME: still necessary?
    cfg.define("_GNU_SOURCE", None);

    // FIXME: still necessary?
    cfg.flag("-Wno-deprecated-declarations");

    headers! { cfg:
               "aio.h",
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "errno.h",
               "fcntl.h",
               "glob.h",
               "grp.h",
               "ifaddrs.h",
               "langinfo.h",
               "limits.h",
               "locale.h",
               "malloc.h",
               "mntent.h",
               "mqueue.h",
               "net/ethernet.h",
               "net/if.h",
               "net/if_arp.h",
               "net/route.h",
               "netdb.h",
               "netinet/in.h",
               "netinet/ip.h",
               "netinet/tcp.h",
               "netinet/udp.h",
               "netpacket/packet.h",
               "poll.h",
               "pthread.h",
               "pty.h",
               "pwd.h",
               "resolv.h",
               "sched.h",
               "sched.h",
               "semaphore.h",
               "shadow.h",
               "signal.h",
               "stddef.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "sys/epoll.h",
               "sys/eventfd.h",
               "sys/file.h",
               "sys/ioctl.h",
               "sys/ipc.h",
               "sys/mman.h",
               "sys/mount.h",
               "sys/msg.h",
               "sys/personality.h",
               "sys/prctl.h",
               "sys/ptrace.h",
               "sys/quota.h",
               "sys/reboot.h",
               "sys/resource.h",
               "sys/sem.h",
               "sys/sendfile.h",
               "sys/shm.h",
               "sys/signalfd.h",
               "sys/socket.h",
               "sys/stat.h",
               "sys/statvfs.h",
               "sys/swap.h",
               "sys/syscall.h",
               "sys/sysctl.h",
               "sys/sysinfo.h",
               "sys/time.h",
               "sys/timerfd.h",
               "sys/times.h",
               "sys/types.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/user.h",
               "sys/utsname.h",
               "sys/vfs.h",
               "sys/wait.h",
               "sys/xattr.h",
               "syslog.h",
               "termios.h",
               "time.h",
               "ucontext.h",
               "unistd.h",
               "utime.h",
               "utmp.h",
               "utmpx.h",
               "wchar.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            // FIXME: is this necessary?
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr"
            | "Elf64_Phdr" | "Elf32_Shdr" | "Elf64_Shdr" | "Elf32_Sym"
            | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr" | "Elf32_Chdr"
            | "Elf64_Chdr" => ty.to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", ".tv_nsec")
            }
            // FIXME: is this necessary?
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            // FIXME: is this necessary?
            "sighandler_t" => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME: is this necessary?
            "sockaddr_nl" => true,

            // This is actually a union, not a struct
            // FIXME: is this necessary?
            "sigval" => true,

            // Linux kernel headers used on musl are too old to have this
            // definition. Because it's tested on other Linux targets, skip it.
            // FIXME: is this necessary?
            "input_mask" => true,

            // These are tested as part of the linux_fcntl tests since there are
            // header conflicts when including them with all the other structs.
            // FIXME: is this necessary?
            "termios2" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| match c {
        // FIXME: is this necessary?
        "LARGE_INTEGER" | "float" | "double" => true,
        // FIXME: is this necessary?
        n if n.starts_with("pthread") => true,
        _ => false,
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: is this necessary?
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            // FIXME: is this necessary?
            "SIGUNUSED" => true,                       // removed in glibc 2.26

            // types on musl are defined a little differently
            // FIXME: is this necessary?
            n if n.contains("__SIZEOF_PTHREAD") => true,

            // Skip constants not defined in MUSL but just passed down to the
            // kernel regardless
            // FIXME: is this necessary?
            "RLIMIT_NLIMITS"
            | "TCP_COOKIE_TRANSACTIONS"
            | "RLIMIT_RTTIME"
            | "MSG_COPY"
                 =>
            {
                true
            }

            // weird signed extension or something like that?
            // FIXME: is this necessary?
            "MS_NOUSER" => true,
            // FIXME: is this necessary?
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // Musl uses old, patched kernel headers
            // FIXME: is this necessary?
            "FALLOC_FL_COLLAPSE_RANGE"
            | "FALLOC_FL_ZERO_RANGE"
            | "FALLOC_FL_INSERT_RANGE"
            | "FALLOC_FL_UNSHARE_RANGE"
            | "RENAME_NOREPLACE"
            | "RENAME_EXCHANGE"
            | "RENAME_WHITEOUT"
            // ALG_SET_AEAD_* constants are available starting from kernel 3.19
            | "ALG_SET_AEAD_ASSOCLEN"
            | "ALG_SET_AEAD_AUTHSIZE"
                 =>
            {
                true
            }

            // musl uses old kernel headers
            // These are constants used in getrandom syscall
            // FIXME: is this necessary?
            "GRND_NONBLOCK" | "GRND_RANDOM" => true,


            // These constants are tested in a separate test program generated below because there
            // are header conflicts if we try to include the headers that define them here.
            // FIXME: is this necessary?
            "F_CANCELLK" | "F_ADD_SEALS" | "F_GET_SEALS" => true,
            // FIXME: is this necessary?
            "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW"
                | "F_SEAL_WRITE" => true,
            // FIXME: is this necessary?
            "BOTHER" => true,

            // FIXME: is this necessary?
            "MFD_CLOEXEC" | "MFD_ALLOW_SEALING"  => true,
            // MFD_HUGETLB is not available in some older libc versions on the CI builders. On the
            // x86_64 and i686 builders it seems to be available for all targets, so at least test
            // it there.
            // FIXME: is this necessary?
            "MFD_HUGETLB"  =>
            {
                true
            }

            // These are defined for Solaris 11, but the crate is tested on
            // illumos, where they are currently not defined
            // FIXME: is this necessary?
            "EADI"
            | "PORT_SOURCE_POSTWAIT"
            | "PORT_SOURCE_SIGNAL"
            | "PTHREAD_STACK_MIN" => true,

            // These change all the time from release to release of linux
            // distros, let's just not bother trying to verify them. They
            // shouldn't be used in code anyway...
            // FIXME: is this necessary?
            "AF_MAX" | "PF_MAX" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: is this necessary?
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" |
            "fexecve" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64" |        // non-int in 2nd arg

            // int vs uint. Sorry musl, your prototype declarations are "correct" in the sense that
            // they match the interface defined by Linux verbatim, but they conflict with other
            // send*/recv* syscalls
            // FIXME: is this necessary?
            "sendmmsg" | "recvmmsg" => true,

            // FIXME: is this necessary?
            "dladdr" => true, // const-ness only added recently

            // FIXME: is this necessary?
            "lio_listio" => true,

            // Definition of those functions as changed since unified headers from NDK r14b
            // These changes imply some API breaking changes but are still ABI compatible.
            // We can wait for the next major release to be compliant with the new API.
            // FIXME: unskip these for next major release
            "strerror_r" | "madvise" | "msync" | "mprotect" | "recvfrom" | "getpriority" |

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        // FIXME: is this necessary?
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        // FIXME: is this necessary?
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // sigval is actually a union, but we pretend it's a struct
        // FIXME: is this necessary?
        (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        // FIXME: is this necessary?
        (struct_ == "aiocb" && field == "aio_buf")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        // FIXME: is this necessary?
        (struct_ == "siginfo_t" && field == "_pad") ||
        // musl names this __dummy1 but it's still there
        // FIXME: is this necessary?
        (struct_ == "glob_t" && field == "gl_flags") ||
        // musl seems to define this as an *anonymous* bitfield
        // FIXME: is this necessary?
        (struct_ == "statvfs" && field == "__f_unused") ||
        // sigev_notify_thread_id is actually part of a sigev_un union
        (struct_ == "sigevent" && field == "sigev_notify_thread_id") ||
        // signalfd had SIGSYS fields added in Linux 4.18, but no libc release has them yet.
        (struct_ == "signalfd_siginfo" && (field == "ssi_addr_lsb" ||
                                           field == "_pad2" ||
                                           field == "ssi_syscall" ||
                                           field == "ssi_call_addr" ||
                                           field == "ssi_arch"))
    });

    // FIXME: remove
    cfg.fn_cname(move |name, _cname| name.to_string());

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_linux(target: &str) {
    assert!(target.contains("linux"));

    // target_env
    let gnu = target.contains("gnu");
    let musl = target.contains("musl");
    let uclibc = target.contains("uclibc");

    match (gnu, musl, uclibc) {
        (true, false, false) => (),
        (false, true, false) => (),
        (false, false, true) => (),
        (_, _, _) => panic!(
            "linux target lib is gnu: {}, musl: {}, uclibc: {}",
            gnu, musl, uclibc
        ),
    }

    let mips = target.contains("mips");
    let i686 = target.contains("i686");
    let x86_64 = target.contains("x86_64");
    let x32 = target.ends_with("gnux32");

    let mut cfg = ctest::TestGenerator::new();
    // FIXME: still necessary?
    cfg.define("_GNU_SOURCE", None);
    // This macro re-deifnes fscanf,scanf,sscanf to link to the symbols that are
    // deprecated since glibc >= 2.29. This allows Rust binaries to link against
    // glibc versions older than 2.29.
    cfg.define("__GLIBC_USE_DEPRECATED_SCANF", None);

    // FIXME: still necessary?
    cfg.flag("-Wno-deprecated-declarations");

    headers! { cfg:
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "elf.h",
               "fcntl.h",
               "glob.h",
               "grp.h",
               "ifaddrs.h",
               "langinfo.h",
               "limits.h",
               "link.h",
               "locale.h",
               "malloc.h",
               "mntent.h",
               "mqueue.h",
               "net/ethernet.h",
               "net/if.h",
               "net/if_arp.h",
               "net/route.h",
               "netdb.h",
               "netinet/in.h",
               "netinet/ip.h",
               "netinet/tcp.h",
               "netinet/udp.h",
               "netpacket/packet.h",
               "poll.h",
               "pthread.h",
               "pty.h",
               "pwd.h",
               "resolv.h",
               "sched.h",
               "semaphore.h",
               "shadow.h",
               "signal.h",
               "spawn.h",
               "stddef.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "sys/epoll.h",
               "sys/eventfd.h",
               "sys/file.h",
               "sys/fsuid.h",
               "sys/inotify.h",
               "sys/ioctl.h",
               "sys/ipc.h",
               "sys/mman.h",
               "sys/mount.h",
               "sys/msg.h",
               "sys/personality.h",
               "sys/prctl.h",
               "sys/ptrace.h",
               "sys/quota.h",
               "sys/reboot.h",
               "sys/resource.h",
               "sys/sem.h",
               "sys/sendfile.h",
               "sys/shm.h",
               "sys/signalfd.h",
               "sys/socket.h",
               "sys/stat.h",
               "sys/statvfs.h",
               "sys/swap.h",
               "sys/syscall.h",
               "sys/time.h",
               "sys/timerfd.h",
               "sys/times.h",
               "sys/types.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/user.h",
               "sys/utsname.h",
               "sys/vfs.h",
               "sys/wait.h",
               "syslog.h",
               "termios.h",
               "time.h",
               "ucontext.h",
               "unistd.h",
               "utime.h",
               "utmp.h",
               "wchar.h",
               "errno.h",
    }

    // Include linux headers at the end:
    headers! {
        cfg:
        "linux/falloc.h",
        "linux/futex.h",
        "linux/fs.h",
        "linux/genetlink.h",
        "linux/if_alg.h",
        "linux/if_ether.h",
        "linux/if_tun.h",
        "linux/input.h",
        "linux/module.h",
        "linux/net_tstamp.h",
        "linux/netfilter_ipv4.h",
        "linux/netfilter_ipv6.h",
        "linux/netlink.h",
        "linux/random.h",
        "linux/seccomp.h",
        "linux/sockios.h",
    }

    if x86_64 {
        headers! { cfg: "sys/io.h" };
    }
    if i686 || x86_64 {
        headers! { cfg: "sys/reg.h" };
    }

    if !musl {
        assert!(uclibc || gnu);
        headers! { cfg:
                   "asm/mman.h",
                   "linux/if.h",
                   "linux/magic.h",
                   "linux/netfilter/nf_tables.h",
                   "linux/reboot.h",
                   "sys/auxv.h",
        };

        if !x32 {
            assert!((gnu || uclibc) && !x32);
            headers! { cfg: "sys/sysctl.h", }
        }
        if !uclibc {
            assert!(gnu);
            headers! { cfg:
                       "execinfo.h",
                       "utmpx.h",
            }
        }
        if !mips {
            assert!((gnu || uclibc) && !mips);
            headers! { cfg: "linux/quota.h" };
        }
    }

    // DCCP support
    if !uclibc && !musl {
        assert!(gnu);
        headers! { cfg: "linux/dccp.h" };
    }

    if !musl || mips {
        assert!(gnu || uclibc || (mips && musl));
        headers! { cfg:  "linux/memfd.h" };
    }

    // note: aio.h must be included before sys/mount.h
    if !uclibc {
        assert!(gnu || musl);
        // optionally included in uclibc
        headers! { cfg:
                   "sys/xattr.h",
                   "sys/sysinfo.h",
                   "aio.h",
        }
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr"
            | "Elf64_Phdr" | "Elf32_Shdr" | "Elf64_Shdr" | "Elf32_Sym"
            | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr" | "Elf32_Chdr"
            | "Elf64_Chdr" => ty.to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", ".tv_nsec")
            }
            // FIXME: is this necessary?
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            // FIXME: is this necessary?
            "type_"
                if struct_ == "input_event"
                    || struct_ == "input_mask"
                    || struct_ == "ff_effect"
                    || struct_ == "rtprio" =>
            {
                "type".to_string()
            }
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            // FIXME: is this necessary?
            "sighandler_t" => true,

            // These cannot be tested when "resolv.h" is included and are tested
            // below.
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME: is this necessary?
            "sockaddr_nl" if musl => true,

            // These cannot be tested when "resolv.h" is included and are tested
            // below.
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            // On Linux, the type of `ut_tv` field of `struct utmpx`
            // can be an anonymous struct, so an extra struct,
            // which is absent in glibc, has to be defined.
            "__timeval" => true,

            // This is actually a union, not a struct
            "sigval" => true,

            // Linux kernel headers used on musl are too old to have this
            // definition. Because it's tested on other Linux targets, skip it.
            // FIXME: is this necessary?
            "input_mask" if musl => true,

            // These are tested as part of the linux_fcntl tests since there are
            // header conflicts when including them with all the other structs.
            // FIXME: is this necessary?
            "termios2" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| match c {
        // FIXME: is this necessary?
        "LARGE_INTEGER" | "float" | "double" => true,
        // FIXME: is this necessary?
        n if n.starts_with("pthread") => true,
        _ => false,
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: is this necessary?
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            // FIXME: is this necessary?
            "SIGUNUSED" => true,                       // removed in glibc 2.26

            // types on musl are defined a little differently
            // FIXME: is this necessary?
            n if musl && n.contains("__SIZEOF_PTHREAD") => true,

            // Skip constants not defined in MUSL but just passed down to the
            // kernel regardless
            // FIXME: is this necessary?
            "RLIMIT_NLIMITS"
            | "TCP_COOKIE_TRANSACTIONS"
            | "RLIMIT_RTTIME"
            | "MSG_COPY"
                if musl =>
            {
                true
            }
            // work around super old mips toolchain
            // FIXME: is this necessary?
            "SCHED_IDLE" | "SHM_NORESERVE" => mips,

            // weird signed extension or something like that?
            // FIXME: is this necessary?
            "MS_NOUSER" => true,
            // FIXME: is this necessary?
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // These are either unimplemented or optionally built into uClibc
            // FIXME: is this necessary?
            "LC_CTYPE_MASK"
            | "LC_NUMERIC_MASK"
            | "LC_TIME_MASK"
            | "LC_COLLATE_MASK"
            | "LC_MONETARY_MASK"
            | "LC_MESSAGES_MASK"
            | "MADV_MERGEABLE"
            | "MADV_UNMERGEABLE"
            | "MADV_HWPOISON"
            | "IPV6_ADD_MEMBERSHIP"
            | "IPV6_DROP_MEMBERSHIP"
            | "IPV6_MULTICAST_LOOP"
            | "IPV6_V6ONLY"
            | "MAP_STACK"
            | "RTLD_DEEPBIND"
            | "SOL_IPV6"
            | "SOL_ICMPV6"
                if uclibc =>
            {
                true
            }

            // Musl uses old, patched kernel headers
            // FIXME: is this necessary?
            "FALLOC_FL_COLLAPSE_RANGE"
            | "FALLOC_FL_ZERO_RANGE"
            | "FALLOC_FL_INSERT_RANGE"
            | "FALLOC_FL_UNSHARE_RANGE"
            | "RENAME_NOREPLACE"
            | "RENAME_EXCHANGE"
            | "RENAME_WHITEOUT"
            // ALG_SET_AEAD_* constants are available starting from kernel 3.19
            | "ALG_SET_AEAD_ASSOCLEN"
            | "ALG_SET_AEAD_AUTHSIZE"
                if musl =>
            {
                true
            }

            // musl uses old kernel headers
            // These are constants used in getrandom syscall
            // FIXME: is this necessary?
            "GRND_NONBLOCK" | "GRND_RANDOM" if musl => true,

            // Defined by libattr not libc on linux (hard to test).
            // See constant definition for more details.
            // FIXME: is this necessary?
            "ENOATTR" => true,

            // On mips*-unknown-linux-gnu* CMSPAR cannot be included with the set of headers we
            // want to use here for testing. It's originally defined in asm/termbits.h, which is
            // also included by asm/termios.h, but not the standard termios.h. There's no way to
            // include both asm/termbits.h and termios.h and there's no way to include both
            // asm/termios.h and ioctl.h (+ some other headers) because of redeclared types.
            // FIXME: is this necessary?
            "CMSPAR" if mips && !musl => true,

            // On mips Linux targets, MADV_SOFT_OFFLINE is currently missing, though it's been added but CI has too old
            // of a Linux version. Since it exists on all other Linux targets, just ignore this for now and remove once
            // it's been fixed in CI.
            // FIXME: is this necessary?
            "MADV_SOFT_OFFLINE" if mips => true,

            // These constants are tested in a separate test program generated below because there
            // are header conflicts if we try to include the headers that define them here.
            // FIXME: is this necessary?
            "F_CANCELLK" | "F_ADD_SEALS" | "F_GET_SEALS" => true,
            // FIXME: is this necessary?
            "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW"
                | "F_SEAL_WRITE" => true,
            // FIXME: is this necessary?
            "QFMT_VFS_OLD" | "QFMT_VFS_V0" | "QFMT_VFS_V1"
                if mips =>
            {
                true
            } // Only on MIPS
            // FIXME: is this necessary?
            "BOTHER" => true,

            // FIXME: is this necessary?
            "MFD_CLOEXEC" | "MFD_ALLOW_SEALING" if !mips && musl => true,
            // MFD_HUGETLB is not available in some older libc versions on the CI builders. On the
            // x86_64 and i686 builders it seems to be available for all targets, so at least test
            // it there.
            // FIXME: is this necessary?
            "MFD_HUGETLB"
                if !(x86_64 || i686) || musl =>
            {
                true
            }

            // These are defined for Solaris 11, but the crate is tested on
            // illumos, where they are currently not defined
            // FIXME: is this necessary?
            "EADI"
            | "PORT_SOURCE_POSTWAIT"
            | "PORT_SOURCE_SIGNAL"
            | "PTHREAD_STACK_MIN" => true,

            // These change all the time from release to release of linux
            // distros, let's just not bother trying to verify them. They
            // shouldn't be used in code anyway...
            // FIXME: is this necessary?
            "AF_MAX" | "PF_MAX" => true,

            // These are not in a glibc release yet, only in kernel headers.
            // FIXME: is this necessary?
            "AF_XDP"
            | "PF_XDP"
            | "SOL_XDP"
            | "IPV6_FLOWINFO"
            | "IPV6_FLOWLABEL_MGR"
            | "IPV6_FLOWINFO_SEND"
            | "IPV6_FLOWINFO_FLOWLABEL"
            | "IPV6_FLOWINFO_PRIORITY"
                 =>
            {
                true
            }

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" |
            "fexecve" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64"          // non-int in 2nd arg
                => true,

            // int vs uint. Sorry musl, your prototype declarations are "correct" in the sense that
            // they match the interface defined by Linux verbatim, but they conflict with other
            // send*/recv* syscalls
            // FIXME: is this necessary?
            "sendmmsg" | "recvmmsg" if musl => true,

            // typed 2nd arg on linux
            // FIXME: is this necessary?
            "gettimeofday" => true,

            // FIXME: is this necessary?
            "dladdr" if musl => true, // const-ness only added recently

            // There seems to be a small error in EGLIBC's eventfd.h header. The
            // [underlying system call][1] always takes its first `count`
            // argument as an `unsigned int`, but [EGLIBC's <sys/eventfd.h>
            // header][2] declares it to take an `int`. [GLIBC's header][3]
            // matches the kernel.
            //
            // EGLIBC is no longer actively developed, and Debian, the largest
            // distribution that had been using it, switched back to GLIBC in
            // April 2015. So effectively all Linux <sys/eventfd.h> headers will
            // be using `unsigned int` soon.
            //
            // [1]: https://git.kernel.org/cgit/linux/kernel/git/stable/linux-stable.git/tree/fs/eventfd.c?id=refs/tags/v3.12.51#n397
            // [2]: http://bazaar.launchpad.net/~ubuntu-branches/ubuntu/trusty/eglibc/trusty/view/head:/sysdeps/unix/sysv/linux/sys/eventfd.h
            // [3]: https://sourceware.org/git/?p=glibc.git;a=blob;f=sysdeps/unix/sysv/linux/sys/eventfd.h;h=6295f32e937e779e74318eb9d3bdbe76aef8a8f3;hb=4e42b5b8f89f0e288e68be7ad70f9525aebc2cff#l34
            // FIXME: is this necessary?
            "eventfd" => true,

            "lio_listio" if musl => true,

            // These are either unimplemented or optionally built into uClibc
            // or "sysinfo", where it's defined but the structs in linux/sysinfo.h and sys/sysinfo.h
            // clash so it can't be tested
            "getxattr" | "lgetxattr" | "fgetxattr" | "setxattr" | "lsetxattr" | "fsetxattr" |
            "listxattr" | "llistxattr" | "flistxattr" | "removexattr" | "lremovexattr" |
            "fremovexattr" |
            "backtrace" |
            "sysinfo" | "newlocale" | "duplocale" | "freelocale" | "uselocale" |
            "nl_langinfo_l" | "wcslen" | "wcstombs" if uclibc => true,

            // Definition of those functions as changed since unified headers from NDK r14b
            // These changes imply some API breaking changes but are still ABI compatible.
            // We can wait for the next major release to be compliant with the new API.
            // FIXME: unskip these for next major release
            "strerror_r" | "madvise" | "msync" | "mprotect" | "recvfrom" | "getpriority"  => true,

            _ => false,
        }
    });

    // FIXME: is this necessary?
    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // __timeval type is a patch which doesn't exist in glibc
        (struct_ == "utmpx" && field == "ut_tv") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        (struct_ == "aiocb" && field == "aio_buf") ||
        // this one is an anonymous union
        (struct_ == "ff_effect" && field == "u")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        (struct_ == "siginfo_t" && field == "_pad") ||
        // musl names this __dummy1 but it's still there
        (musl && struct_ == "glob_t" && field == "gl_flags") ||
        // musl seems to define this as an *anonymous* bitfield
        (musl && struct_ == "statvfs" && field == "__f_unused") ||
        // sigev_notify_thread_id is actually part of a sigev_un union
        (struct_ == "sigevent" && field == "sigev_notify_thread_id") ||
        // signalfd had SIGSYS fields added in Linux 4.18, but no libc release has them yet.
        (struct_ == "signalfd_siginfo" && (field == "ssi_addr_lsb" ||
                                           field == "_pad2" ||
                                           field == "ssi_syscall" ||
                                           field == "ssi_call_addr" ||
                                           field == "ssi_arch"))
    });

    // FIXME: remove
    cfg.fn_cname(move |name, _cname| name.to_string());

    cfg.generate("../src/lib.rs", "main.rs");

    // On Linux also generate another script for testing linux/fcntl declarations.
    // These cannot be tested normally because including both `linux/fcntl.h` and `fcntl.h`
    // fails on a lot of platforms.
    let mut cfg = ctest::TestGenerator::new();
    cfg.skip_type(|_| true)
        .skip_fn(|_| true)
        .skip_static(|_| true);
    // musl defines these directly in `fcntl.h`
    if musl {
        cfg.header("fcntl.h");
    } else {
        cfg.header("linux/fcntl.h");
    }
    if !musl {
        cfg.header("net/if.h");
        cfg.header("linux/if.h");
    }
    cfg.header("linux/quota.h");
    cfg.header("asm/termbits.h");
    cfg.skip_const(move |name| match name {
        "F_CANCELLK" | "F_ADD_SEALS" | "F_GET_SEALS" => false,
        "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW" | "F_SEAL_WRITE" => {
            false
        }
        "QFMT_VFS_OLD" | "QFMT_VFS_V0" | "QFMT_VFS_V1" if mips => false,
        "BOTHER" => false,
        _ => true,
    });
    cfg.skip_struct(|s| s != "termios2");
    cfg.type_name(move |ty, is_struct, is_union| match ty {
        t if is_struct => format!("struct {}", t),
        t if is_union => format!("union {}", t),
        t => t.to_string(),
    });
    cfg.generate("../src/lib.rs", "linux_fcntl.rs");

    // Test Elf64_Phdr and Elf32_Phdr
    // These types have a field called `p_type`, but including
    // "resolve.h" defines a `p_type` macro that expands to `__p_type`
    // making the tests for these fails when both are included.
    let mut cfg = ctest::TestGenerator::new();
    cfg.skip_fn(|_| true)
        .skip_const(|_| true)
        .skip_static(|_| true)
        .type_name(move |ty, _is_struct, _is_union| {
            ty.to_string()
        });
    cfg.skip_struct(move |ty| {
        match ty {
            "Elf64_Phdr" | "Elf32_Phdr" => false,
            _ => true,
        }
    });
    cfg.skip_type(move |ty| {
        match ty {
            "Elf64_Phdr" | "Elf32_Phdr" => false,
            _ => true,
        }
    });
    cfg.header("elf.h");
    cfg.generate("../src/lib.rs", "linux_elf.rs");
}
