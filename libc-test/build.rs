#![deny(warnings)]

extern crate ctest;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let aarch64 = target.contains("aarch64");
    let x86_64 = target.contains("x86_64");
    let windows = target.contains("windows");
    let mingw = target.contains("windows-gnu");
    let linux = target.contains("unknown-linux");
    let android = target.contains("android");
    let apple = target.contains("apple");
    let musl = target.contains("musl");
    let freebsd = target.contains("freebsd");
    let dragonfly = target.contains("dragonfly");
    let mips = target.contains("mips");
    let netbsd = target.contains("netbsd");
    let openbsd = target.contains("openbsd");
    let rumprun = target.contains("rumprun");
    let bsdlike = freebsd || apple || netbsd || openbsd || dragonfly;
    let mut cfg = ctest::TestGenerator::new();

    // Pull in extra goodies
    if linux || android {
        cfg.define("_GNU_SOURCE", None);
    } else if netbsd {
        cfg.define("_NETBSD_SOURCE", Some("1"));
    } else if windows {
        cfg.define("_WIN32_WINNT", Some("0x8000"));
    }

    // Android doesn't actually have in_port_t but it's much easier if we
    // provide one for us to test against
    if android {
        cfg.define("in_port_t", Some("uint16_t"));
    }

    cfg.header("errno.h")
       .header("fcntl.h")
       .header("limits.h")
       .header("locale.h")
       .header("stddef.h")
       .header("stdint.h")
       .header("stdio.h")
       .header("stdlib.h")
       .header("sys/stat.h")
       .header("sys/types.h")
       .header("time.h")
       .header("wchar.h");

    if windows {
        cfg.header("winsock2.h"); // must be before windows.h

        cfg.header("direct.h");
        cfg.header("io.h");
        cfg.header("sys/utime.h");
        cfg.header("windows.h");
        cfg.header("process.h");
        cfg.header("ws2ipdef.h");

        if target.contains("gnu") {
            cfg.header("ws2tcpip.h");
        }
    } else {
        cfg.flag("-Wno-deprecated-declarations");

        cfg.header("ctype.h");
        cfg.header("dirent.h");
        if openbsd {
            cfg.header("sys/socket.h");
        }
        cfg.header("net/if.h");
        cfg.header("netdb.h");
        cfg.header("netinet/in.h");
        cfg.header("netinet/ip.h");
        cfg.header("netinet/tcp.h");
        cfg.header("resolv.h");
        cfg.header("pthread.h");
        cfg.header("dlfcn.h");
        cfg.header("signal.h");
        cfg.header("string.h");
        cfg.header("sys/file.h");
        cfg.header("sys/ioctl.h");
        cfg.header("sys/mman.h");
        cfg.header("sys/resource.h");
        cfg.header("sys/socket.h");
        cfg.header("sys/time.h");
        cfg.header("sys/un.h");
        cfg.header("sys/wait.h");
        cfg.header("unistd.h");
        cfg.header("utime.h");
        cfg.header("pwd.h");
        cfg.header("grp.h");
        cfg.header("sys/utsname.h");
        cfg.header("sys/ptrace.h");
        cfg.header("sys/mount.h");
        cfg.header("sys/uio.h");
        cfg.header("sched.h");
        cfg.header("termios.h");
        cfg.header("poll.h");
        cfg.header("syslog.h");
        cfg.header("semaphore.h");
        cfg.header("sys/statvfs.h");
    }

    if android {
        if !aarch64 && !x86_64 {
            // time64_t is not define for aarch64 and x86_64
            // If included it will generate the error 'Your time_t is already 64-bit'
            cfg.header("time64.h");
        }
        cfg.header("arpa/inet.h");
        cfg.header("xlocale.h");
        cfg.header("utmp.h");
    } else if !windows {
        cfg.header("glob.h");
        cfg.header("ifaddrs.h");
        cfg.header("langinfo.h");

        if !openbsd && !freebsd && !dragonfly {
            cfg.header("sys/quota.h");
        }

        if !musl {
            cfg.header("sys/sysctl.h");

            if !netbsd && !openbsd {
                cfg.header("execinfo.h");
                cfg.header("xlocale.h");
            }

            if openbsd {
                cfg.header("utmp.h");
            } else {
                cfg.header("utmpx.h");
            }
        }
    }

    if apple {
        cfg.header("mach-o/dyld.h");
        cfg.header("mach/mach_time.h");
        cfg.header("malloc/malloc.h");
        cfg.header("util.h");
        if target.starts_with("x86") {
            cfg.header("crt_externs.h");
        }
    }

    if bsdlike {
        cfg.header("sys/event.h");

        if freebsd {
            cfg.header("libutil.h");
        } else {
            cfg.header("util.h");
        }
    }

    if linux {
        cfg.header("mqueue.h");
        cfg.header("ucontext.h");
        cfg.header("sys/signalfd.h");
        cfg.header("sys/xattr.h");
        cfg.header("sys/ipc.h");
        cfg.header("sys/msg.h");
        cfg.header("sys/shm.h");
        cfg.header("sys/fsuid.h");
        cfg.header("pty.h");
        cfg.header("shadow.h");
    }

    if linux || android {
        cfg.header("malloc.h");
        cfg.header("net/ethernet.h");
        cfg.header("netpacket/packet.h");
        cfg.header("sched.h");
        cfg.header("sys/epoll.h");
        cfg.header("sys/eventfd.h");
        cfg.header("sys/prctl.h");
        cfg.header("sys/sendfile.h");
        cfg.header("sys/vfs.h");
        cfg.header("sys/syscall.h");
        cfg.header("sys/sysinfo.h");
        cfg.header("sys/reboot.h");
        if !musl {
            cfg.header("linux/netlink.h");
            cfg.header("linux/magic.h");
            cfg.header("linux/reboot.h");

            if !mips {
                cfg.header("linux/quota.h");
            }
        }
    }

    if freebsd {
        cfg.header("pthread_np.h");
        cfg.header("sched.h");
        cfg.header("ufs/ufs/quota.h");
        cfg.header("sys/jail.h");
        cfg.header("sys/ipc.h");
        cfg.header("sys/msg.h");
        cfg.header("sys/shm.h");
    }

    if netbsd {
        cfg.header("ufs/ufs/quota.h");
        cfg.header("ufs/ufs/quota1.h");
        cfg.header("sys/ioctl_compat.h");
    }

    if openbsd {
        cfg.header("ufs/ufs/quota.h");
        cfg.header("pthread_np.h");
        cfg.header("sys/syscall.h");
    }

    if dragonfly {
        cfg.header("ufs/ufs/quota.h");
        cfg.header("pthread_np.h");
        cfg.header("sys/ioctl_compat.h");
    }

    if linux || freebsd || dragonfly || netbsd || apple {
        cfg.header("aio.h");
    }

    cfg.type_name(move |ty, is_struct| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" |
            "fd_set" |
            "Dl_info" |
            "DIR" => ty.to_string(),

            // Fixup a few types on windows that don't actually exist.
            "time64_t" if windows => "__time64_t".to_string(),
            "ssize_t" if windows => "SSIZE_T".to_string(),

            // OSX calls this something else
            "sighandler_t" if bsdlike => "sig_t".to_string(),

            t if t.ends_with("_t") => t.to_string(),

            // Windows uppercase structs don't have `struct` in front, there's a
            // few special cases for windows, and then otherwise put `struct` in
            // front of everything.
            t if is_struct => {
                if windows && ty.chars().next().unwrap().is_uppercase() {
                    t.to_string()
                } else if windows && t == "stat" {
                    "struct __stat64".to_string()
                } else if windows && t == "utimbuf" {
                    "struct __utimbuf64".to_string()
                } else {
                    format!("struct {}", t)
                }
            }

            t => t.to_string(),
        }
    });

    let target2 = target.clone();
    cfg.field_name(move |struct_, field| {
        match field {
            "st_birthtime"      if openbsd && struct_ == "stat" => "__st_birthtime".to_string(),
            "st_birthtime_nsec" if openbsd && struct_ == "stat" => "__st_birthtimensec".to_string(),
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                if target2.contains("apple") {
                    s.replace("_nsec", "spec.tv_nsec")
                } else if target2.contains("android") {
                    s.to_string()
                } else {
                    s.replace("e_nsec", ".tv_nsec")
                }
            }
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            "sighandler_t" => true,

            _ => false
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            "sockaddr_nl" => musl,

            // On Linux, the type of `ut_tv` field of `struct utmpx`
            // can be an anonymous struct, so an extra struct,
            // which is absent in glibc, has to be defined.
            "__timeval" if linux => true,

            // The alignment of this is 4 on 64-bit OSX...
            "kevent" if apple && x86_64 => true,

            // This is actually a union, not a struct
            "sigval" => true,

            _ => false
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            "LARGE_INTEGER" |
            "mach_timebase_info_data_t" |
            "float" |
            "double" => true,
            // uuid_t is a struct, not an integer.
            "uuid_t" if dragonfly => true,
            n if n.starts_with("pthread") => true,
            // sem_t is a struct or pointer
            "sem_t" if openbsd || freebsd || dragonfly || rumprun => true,

            // windows-isms
            n if n.starts_with("P") => true,
            n if n.starts_with("H") => true,
            n if n.starts_with("LP") => true,
            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // Apparently these don't exist in mingw headers?
            "MEM_RESET_UNDO" |
            "FILE_ATTRIBUTE_NO_SCRUB_DATA" |
            "FILE_ATTRIBUTE_INTEGRITY_STREAM" |
            "ERROR_NOTHING_TO_TERMINATE" if mingw => true,

            "SIG_IGN" => true, // sighandler_t weirdness

            // types on musl are defined a little differently
            n if musl && n.contains("__SIZEOF_PTHREAD") => true,

            // Skip constants not defined in MUSL but just passed down to the
            // kernel regardless
            "RLIMIT_NLIMITS" |
            "TCP_COOKIE_TRANSACTIONS" |
            "RLIMIT_RTTIME" |
            "MSG_COPY" if musl => true,
            // work around super old mips toolchain
            "SCHED_IDLE" | "SHM_NORESERVE" => mips,

            // weird signed extension or something like that?
            "MS_NOUSER" => true,
            "MS_RMT_MASK" => true, // updated in glibc 2.22 and musl 1.1.13

            // These OSX constants are flagged as deprecated
            "NOTE_EXIT_REPARENTED" |
            "NOTE_REAP" if apple => true,

            // The linux/quota.h header file which defines these can't be
            // included with sys/quota.h currently on MIPS, so we don't include
            // it and just ignore these constants
            "QFMT_VFS_OLD" |
            "QFMT_VFS_V0" if mips && linux => true,

            // These constants were removed in FreeBSD 11 (svn r273250) but will
            // still be accepted and ignored at runtime.
            "MAP_RENAME" |
            "MAP_NORESERVE" if freebsd => true,

            // These constants were removed in FreeBSD 11 (svn r262489),
            // and they've never had any legitimate use outside of the
            // base system anyway.
            "CTL_MAXID" |
            "KERN_MAXID" |
            "HW_MAXID" |
            "USER_MAXID" if freebsd => true,

            // These OSX constants are removed in Sierra.
            // https://developer.apple.com/library/content/releasenotes/General/APIDiffsMacOS10_12/Swift/Darwin.html
            "KERN_KDENABLE_BG_TRACE" if apple => true,
            "KERN_KDDISABLE_BG_TRACE" if apple => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            "execv" |       // crazy stuff with const/mut
            "execve" |
            "execvp" |
            "execvpe" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64" |        // non-int in 2nd arg
            "strerror_r" if linux => true,   // actually xpg-something-or-other

            // int vs uint. Sorry musl, your prototype declarations are "correct" in the sense that
            // they match the interface defined by Linux verbatim, but they conflict with other
            // send*/recv* syscalls
            "sendmmsg" | "recvmmsg" if musl => true,

            // typed 2nd arg on linux and android
            "gettimeofday" if linux || android || freebsd || openbsd || dragonfly => true,

            // not declared in newer android toolchains
            "getdtablesize" if android => true,

            "dlerror" if android => true, // const-ness is added
            "dladdr" if musl => true, // const-ness only added recently

            // OSX has 'struct tm *const' which we can't actually represent in
            // Rust, but is close enough to *mut
            "timegm" if apple => true,

            // OSX's daemon is deprecated in 10.5 so we'll get a warning (which
            // we turn into an error) so just ignore it.
            "daemon" if apple => true,

            // Deprecated on OSX
            "sem_destroy" if apple => true,
            "sem_init" if apple => true,

            // These functions presumably exist on netbsd but don't look like
            // they're implemented on rumprun yet, just let them slide for now.
            // Some of them look like they have headers but then don't have
            // corresponding actual definitions either...
            "shm_open" |
            "shm_unlink" |
            "syscall" |
            "ptrace" |
            "sigaltstack" if rumprun => true,

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
            "eventfd" if linux => true,

            // The `uname` function in freebsd is now an inline wrapper that
            // delegates to another, but the symbol still exists, so don't check
            // the symbol.
            "uname" if freebsd => true,

            // aio_waitcomplete's return type changed between FreeBSD 10 and 11.
            "aio_waitcomplete" if freebsd => true,

            // lio_listio confuses the checker, probably because one of its
            // arguments is an array
            "lio_listio" if freebsd => true,
            "lio_listio" if musl => true,

            // Apparently the NDK doesn't have this defined on android, but
            // it's in a header file?
            "endpwent" if android => true,

            // Apparently res_init exists on Android, but isn't defined in a header:
            // https://mail.gnome.org/archives/commits-list/2013-May/msg01329.html
            "res_init" if android => true,

            // On macOS and iOS, res_init is available, but requires linking with libresolv:
            // http://blog.achernya.com/2013/03/os-x-has-silly-libsystem.html
            // See discussion for skipping here:
            // https://github.com/rust-lang/libc/pull/585#discussion_r114561460
            "res_init" if apple => true,

            _ => false,
        }
    });

    cfg.skip_fn_ptrcheck(move |name| {
        match name {
            // dllimport weirdness?
            _ if windows => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // __timeval type is a patch which doesn't exist in glibc
        (linux && struct_ == "utmpx" && field == "ut_tv") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // aio_buf is "volatile void*" and Rust doesn't understand volatile
        (struct_ == "aiocb" && field == "aio_buf") ||
        // stack_t.ss_sp's type changed from FreeBSD 10 to 11 in svn r294930
        (freebsd && struct_ == "stack_t" && field == "ss_sp")
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
        (struct_ == "sigevent" && field == "sigev_notify_thread_id")
    });

    cfg.fn_cname(move |name, cname| {
        if windows {
            cname.unwrap_or(name).to_string()
        } else {
            name.to_string()
        }
    });

    if env::var("SKIP_COMPILE").is_ok() {
        cfg.generate_files("../src/lib.rs", "all.rs");
    } else {
        cfg.generate("../src/lib.rs", "all.rs");
    }
}
