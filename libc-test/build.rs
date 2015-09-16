extern crate ctest;

use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");
    let mingw = target.contains("windows-gnu");
    let mut cfg = ctest::TestGenerator::new();

    // Pull in extra goodies on linux/mingw
    if target.contains("unknown-linux-gnu") {
        cfg.define("_GNU_SOURCE", None);
    } else if target.contains("windows") {
        cfg.define("_WIN32_WINNT", Some("0x8000"));
    }

    cfg.header("errno.h")
       .header("fcntl.h")
       .header("limits.h")
       .header("stddef.h")
       .header("stdint.h")
       .header("stdio.h")
       .header("stdlib.h")
       .header("sys/stat.h")
       .header("sys/types.h")
       .header("time.h")
       .header("wchar.h");

    if target.contains("apple-darwin") {
        cfg.header("mach-o/dyld.h");
        cfg.header("mach/mach_time.h");
    } else if target.contains("unknown-linux") ||
              target.contains("android") {
        cfg.header("linux/if_packet.h");
        cfg.header("net/ethernet.h");
    }

    if target.contains("windows") {
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
        cfg.header("ctype.h");
        cfg.header("dirent.h");
        cfg.header("net/if.h");
        cfg.header("netdb.h");
        cfg.header("netinet/in.h");
        cfg.header("netinet/ip.h");
        cfg.header("netinet/tcp.h");
        cfg.header("pthread.h");
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

        if target.contains("android") {
            cfg.header("arpa/inet.h");
        } else {
            cfg.header("glob.h");
            cfg.header("ifaddrs.h");
            cfg.header("sys/sysctl.h");
        }
    }

    cfg.type_name(move |ty, is_struct| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "glob_t" |
            "FILE" |
            "DIR" |
            "fpos_t" => ty.to_string(),
            t if t.starts_with("pthread") => t.to_string(),

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

            // Fixup a few types on windows that don't actually exist.
            "time64_t" if windows => "__time64_t".to_string(),
            "ssize_t" if windows => "SSIZE_T".to_string(),

            t => t.to_string(),
        }
    });

    let target2 = target.clone();
    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_ == "stat" => {
                if target2.contains("apple-darwin") {
                    s.replace("_nsec", "spec.tv_nsec")
                } else if target2.contains("android") {
                    s.to_string()
                } else {
                    s.replace("e_nsec", ".tv_nsec")
                }
            }
            s => s.to_string(),
        }
    });

    let target2 = target.clone();
    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            "sighandler_t" => true,

            // Not actually defined on android, but it's not hurting anyone
            "in_port_t" if target2.contains("android") => true,
            _ => false
        }
    });

    cfg.skip_signededness(|c| {
        match c {
            "LARGE_INTEGER" |
            "mach_timebase_info_data_t" |
            "float" |
            "double" => true,
            n if n.starts_with("pthread") => true,

            // windows-isms
            n if n.starts_with("P") => true,
            n if n.starts_with("H") => true,
            n if n.starts_with("LP") => true,
            _ => false,
        }
    });

    // Apparently these don't exist in mingw headers?
    cfg.skip_const(move |name| {
        match name {
            "MEM_RESET_UNDO" |
            "FILE_ATTRIBUTE_NO_SCRUB_DATA" |
            "FILE_ATTRIBUTE_INTEGRITY_STREAM" |
            "ERROR_NOTHING_TO_TERMINATE" if mingw => true,
            "SIG_IGN" => true, // sighandler_t weirdness
            _ => false,
        }
    });

    cfg.skip_fn(|name| {
        match name {
            // manually verified
            "execv" |
            "execve" |
            "execvp" |
            "execvpe" |
            "glob" |
            "getrlimit" |
            "setrlimit" |
            "signal" |
            "getopt" => true,
            _ => false,
        }
    });

    // Windows dllimport oddness?
    cfg.skip_fn_ptrcheck(move |_| windows);

    cfg.generate("../src/lib.rs", "all.rs");
}
