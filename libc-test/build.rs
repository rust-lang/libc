#![deny(warnings)]

extern crate cc;
extern crate ctest2 as ctest;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, io};

fn do_cc() {
    let target = env::var("TARGET").unwrap();
    if cfg!(unix) {
        let exclude = ["redox", "wasi"];
        if !exclude.iter().any(|x| target.contains(x)) {
            let mut cmsg = cc::Build::new();

            cmsg.file("src/cmsg.c");

            if target.contains("solaris") || target.contains("illumos") {
                cmsg.define("_XOPEN_SOURCE", "700");
            }
            cmsg.compile("cmsg");
        }
    }
    if target.contains("android") || target.contains("linux") {
        cc::Build::new().file("src/errqueue.c").compile("errqueue");
    }
}

fn do_ctest() {
    match &env::var("TARGET").unwrap() {
        t if t.contains("android") => return test_android(t),
        t if t.contains("apple") => return test_apple(t),
        t if t.contains("dragonfly") => return test_dragonflybsd(t),
        t if t.contains("emscripten") => return test_emscripten(t),
        t if t.contains("freebsd") => return test_freebsd(t),
        t if t.contains("haiku") => return test_haiku(t),
        t if t.contains("linux") => return test_linux(t),
        t if t.contains("netbsd") => return test_netbsd(t),
        t if t.contains("openbsd") => return test_openbsd(t),
        t if t.contains("redox") => return test_redox(t),
        t if t.contains("solaris") => return test_solarish(t),
        t if t.contains("illumos") => return test_solarish(t),
        t if t.contains("wasi") => return test_wasi(t),
        t if t.contains("windows") => return test_windows(t),
        t if t.contains("vxworks") => return test_vxworks(t),
        t => panic!("unknown target {}", t),
    }
}

fn ctest_cfg() -> ctest::TestGenerator {
    let mut cfg = ctest::TestGenerator::new();
    let libc_cfgs = [
        "libc_priv_mod_use",
        "libc_union",
        "libc_const_size_of",
        "libc_align",
        "libc_core_cvoid",
        "libc_packedN",
        "libc_thread_local",
    ];
    for f in &libc_cfgs {
        cfg.cfg(f, None);
    }
    cfg
}

fn do_semver() {
    let mut out = PathBuf::from(env::var("OUT_DIR").unwrap());
    out.push("semver.rs");
    let mut output = BufWriter::new(File::create(&out).unwrap());

    let family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap();
    let vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    // `libc-test/semver` dir.
    let mut semver_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    semver_root.push("semver");

    // NOTE: Windows has the same `family` as `os`, no point in including it
    // twice.
    // NOTE: Android doesn't include the unix file (or the Linux file) because
    // there are some many definitions missing it's actually easier just to
    // maintain a file for Android.
    if family != os && os != "android" {
        process_semver_file(&mut output, &mut semver_root, &family);
    }
    process_semver_file(&mut output, &mut semver_root, &vendor);
    process_semver_file(&mut output, &mut semver_root, &os);
    let os_arch = format!("{}-{}", os, arch);
    process_semver_file(&mut output, &mut semver_root, &os_arch);
    if target_env != "" {
        let os_env = format!("{}-{}", os, target_env);
        process_semver_file(&mut output, &mut semver_root, &os_env);

        let os_env_arch = format!("{}-{}-{}", os, target_env, arch);
        process_semver_file(&mut output, &mut semver_root, &os_env_arch);
    }
}

fn process_semver_file<W: Write, P: AsRef<Path>>(output: &mut W, path: &mut PathBuf, file: P) {
    // NOTE: `path` is reused between calls, so always remove the file again.
    path.push(file);
    path.set_extension("txt");

    println!("cargo:rerun-if-changed={}", path.display());
    let input_file = match File::open(&*path) {
        Ok(file) => file,
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
            path.pop();
            return;
        }
        Err(err) => panic!("unexpected error opening file: {}", err),
    };
    let input = BufReader::new(input_file);

    write!(output, "// Source: {}.\n", path.display()).unwrap();
    output.write(b"use libc::{\n").unwrap();
    for line in input.lines() {
        let line = line.unwrap().into_bytes();
        match line.first() {
            // Ignore comments and empty lines.
            Some(b'#') | None => continue,
            _ => {
                output.write(b"    ").unwrap();
                output.write(&line).unwrap();
                output.write(b",\n").unwrap();
            }
        }
    }
    output.write(b"};\n\n").unwrap();
    path.pop();
}

fn main() {
    do_cc();
    do_ctest();
    do_semver();
}

macro_rules! headers {
    ($cfg:ident: [$m:expr]: $header:literal) => {
        if $m {
            $cfg.header($header);
        }
    };
    ($cfg:ident: $header:literal) => {
        $cfg.header($header);
    };
    ($($cfg:ident: $([$c:expr]:)* $header:literal,)*) => {
        $(headers!($cfg: $([$c]:)* $header);)*
    };
    ($cfg:ident: $( $([$c:expr]:)* $header:literal,)*) => {
        headers!($($cfg: $([$c]:)* $header,)*);
    };
    ($cfg:ident: $( $([$c:expr]:)* $header:literal),*) => {
        headers!($($cfg: $([$c]:)* $header,)*);
    };
}

fn test_apple(target: &str) {
    assert!(target.contains("apple"));
    let x86_64 = target.contains("x86_64");
    let i686 = target.contains("i686");

    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("__APPLE_USE_RFC_3542", None);

    headers! { cfg:
        "aio.h",
        "CommonCrypto/CommonCrypto.h",
        "CommonCrypto/CommonRandom.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "glob.h",
        "grp.h",
        "iconv.h",
        "ifaddrs.h",
        "langinfo.h",
        "libproc.h",
        "limits.h",
        "locale.h",
        "mach-o/dyld.h",
        "mach/mach_init.h",
        "mach/mach_time.h",
        "mach/mach_types.h",
        "mach/mach_vm.h",
        "mach/thread_act.h",
        "mach/thread_policy.h",
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
        "regex.h",
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
        "sys/attr.h",
        "sys/clonefile.h",
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
        "sys/timex.h",
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
        [x86_64]: "crt_externs.h",
    }

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME: actually a union
            "sigval" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        // They're declared via `deprecated_mach` and we don't support it anymore.
        if name.starts_with("VM_FLAGS_") {
            return true;
        }
        match name {
            // These OSX constants are removed in Sierra.
            // https://developer.apple.com/library/content/releasenotes/General/APIDiffsMacOS10_12/Swift/Darwin.html
            "KERN_KDENABLE_BG_TRACE" | "KERN_KDDISABLE_BG_TRACE" => true,
            // FIXME: the value has been changed since Catalina (0xffff0000 -> 0x3fff0000).
            "SF_SETTABLE" => true,
            // FIXME: the values have been changed since Big Sur
            "HW_TARGET" | "HW_PRODUCT" | "HW_MAXID" => true,
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

            // these calls require macOS 11.0 or higher
            "preadv" | "pwritev" => true,

            _ => false,
        }
    });

    cfg.skip_field(move |struct_, field| {
        match (struct_, field) {
            // FIXME: the array size has been changed since macOS 10.15 ([8] -> [7]).
            ("statfs", "f_reserved") => true,
            ("__darwin_arm_neon_state64", "__v") => true,
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
            StructField(ref n, ref f) if n == "aiocb" && f == "aio_buf" => true,
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
            "sa_sigaction" if struct_ == "sigaction" => "sa_handler".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME: this type has the wrong ABI
        "max_align_t" if i686 => true,
        // Can't return an array from a C function.
        "uuid_t" => true,
        _ => false,
    });
    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_openbsd(target: &str) {
    assert!(target.contains("openbsd"));

    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");

    headers! { cfg:
        "elf.h",
        "errno.h",
        "fcntl.h",
        "limits.h",
        "link.h",
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
        "net/bpf.h",
        "regex.h",
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
        "sys/shm.h",
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
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" => true,

            // Removed in OpenBSD 6.5
            // https://marc.info/?l=openbsd-cvs&m=154723400730318
            "mincore" => true,

            _ => false,
        }
    });

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "DIR" | "Dl_info" | "Elf32_Phdr" | "Elf64_Phdr" => ty.to_string(),

            // OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {}", t),
            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| match field {
        "st_birthtime" if struct_.starts_with("stat") => "__st_birthtime".to_string(),
        "st_birthtime_nsec" if struct_.starts_with("stat") => "__st_birthtimensec".to_string(),
        s if s.ends_with("_nsec") && struct_.starts_with("stat") => s.replace("e_nsec", ".tv_nsec"),
        "sa_sigaction" if struct_ == "sigaction" => "sa_handler".to_string(),
        s => s.to_string(),
    });

    cfg.skip_field_type(move |struct_, field| {
        // type siginfo_t.si_addr changed from OpenBSD 6.0 to 6.1
        struct_ == "siginfo_t" && field == "si_addr"
    });

    cfg.skip_field(|struct_, field| {
        match (struct_, field) {
            // conflicting with `p_type` macro from <resolve.h>.
            ("Elf32_Phdr", "p_type") => true,
            ("Elf64_Phdr", "p_type") => true,
            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_windows(target: &str) {
    assert!(target.contains("windows"));
    let gnu = target.contains("gnu");

    let mut cfg = ctest_cfg();
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
        [gnu]: "ws2tcpip.h",
        [!gnu]: "Winsock2.h",
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
            "SIG_ERR" |
            // Similar for SIG_DFL/IGN/GET/SGE/ACK
            "SIG_DFL" | "SIG_IGN" | "SIG_GET" | "SIG_SGE" | "SIG_ACK" => true,
            // FIXME: newer windows-gnu environment on CI?
            "_O_OBTAIN_DIR" if gnu => true,
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
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_redox(target: &str) {
    assert!(target.contains("redox"));

    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");

    headers! {
        cfg:
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "fcntl.h",
        "grp.h",
        "limits.h",
        "locale.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "poll.h",
        "pwd.h",
        "semaphore.h",
        "string.h",
        "strings.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/mman.h",
        "sys/ptrace.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/time.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "termios.h",
        "time.h",
        "unistd.h",
        "utime.h",
        "wchar.h",
    }

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_solarish(target: &str) {
    let is_solaris = target.contains("solaris");
    let is_illumos = target.contains("illumos");
    assert!(is_solaris || is_illumos);

    // ctest generates arguments supported only by clang, so make sure to run with CC=clang.
    // While debugging, "CFLAGS=-ferror-limit=<large num>" is useful to get more error output.
    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");

    cfg.define("_XOPEN_SOURCE", Some("700"));
    cfg.define("__EXTENSIONS__", None);
    cfg.define("_LCONV_C99", None);

    headers! {
        cfg:
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "door.h",
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
        "sys/eventfd.h",
        "sys/file.h",
        "sys/filio.h",
        "sys/ioctl.h",
        "sys/loadavg.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/pset.h",
        "sys/resource.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/stropts.h",
        "sys/shm.h",
        "sys/time.h",
        "sys/times.h",
        "sys/timex.h",
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

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is not present here
            "sighandler_t" => true,
            _ => false,
        }
    });

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "FILE" => "__FILE".to_string(),
        "DIR" | "Dl_info" => ty.to_string(),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {}", t),
        t if is_union => format!("union {}", t),
        t => t.to_string(),
    });

    cfg.field_name(move |struct_, field| {
        match struct_ {
            // rust struct uses raw u64, rather than union
            "epoll_event" if field == "u64" => "data.u64".to_string(),
            // rust struct was committed with typo for Solaris
            "door_arg_t" if field == "dec_num" => "desc_num".to_string(),
            "stat" if field.ends_with("_nsec") => {
                // expose stat.Xtim.tv_nsec fields
                field.trim_end_matches("e_nsec").to_string() + ".tv_nsec"
            }
            _ => field.to_string(),
        }
    });

    cfg.skip_const(move |name| match name {
        "DT_FIFO" | "DT_CHR" | "DT_DIR" | "DT_BLK" | "DT_REG" | "DT_LNK" | "DT_SOCK"
        | "USRQUOTA" | "GRPQUOTA" | "PRIO_MIN" | "PRIO_MAX" => true,

        // skip sighandler_t assignments
        "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true,

        "DT_UNKNOWN" => true,

        "_UTX_LINESIZE" | "_UTX_USERSIZE" | "_UTX_PADSIZE" | "_UTX_IDSIZE" | "_UTX_HOSTSIZE" => {
            true
        }

        "EADI" | "EXTPROC" | "IPC_SEAT" => true,

        // This evaluates to a sysconf() call rather than a constant
        "PTHREAD_STACK_MIN" => true,

        // EPOLLEXCLUSIVE is a relatively recent addition to the epoll interface and may not be
        // defined on older systems.  It is, however, safe to use on systems which do not
        // explicitly support it. (A no-op is an acceptable implementation of EPOLLEXCLUSIVE.)
        "EPOLLEXCLUSIVE" => true,

        _ => false,
    });

    cfg.skip_struct(move |ty| {
        // the union handling is a mess
        if ty.contains("door_desc_t_") {
            return true;
        }
        match ty {
            // union, not a struct
            "sigval" => true,
            // a bunch of solaris-only fields
            "utmpx" if is_illumos => true,
            _ => false,
        }
    });

    cfg.skip_field(move |s, field| {
        match s {
            // C99 sizing on this is tough
            "dirent" if field == "d_name" => true,
            // the union/macro makes this rough
            "sigaction" if field == "sa_sigaction" => true,
            // Missing in illumos
            "sigevent" if field == "ss_sp" => true,
            // Avoid sigval union issues
            "sigevent" if field == "sigev_value" => true,
            // const issues
            "sigevent" if field == "sigev_notify_attributes" => true,

            // Avoid const and union issues
            "door_arg" if field == "desc_ptr" => true,
            "door_desc_t" if field == "d_data" => true,
            "door_arg_t" if field.ends_with("_ptr") => true,
            "door_arg_t" if field.ends_with("rbuf") => true,

            _ => false,
        }
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

            // signal is defined in terms of sighandler_t, so ignore
            "signal" => true,

            // Currently missing
            "cfmakeraw" | "cfsetspeed" => true,

            // const-ness issues
            "execv" | "execve" | "execvp" | "settimeofday" | "sethostname" => true,

            // Solaris-different
            "getpwent_r" | "getgrent_r" | "updwtmpx" if is_illumos => true,
            "madvise" | "mprotect" if is_illumos => true,
            "door_call" | "door_return" | "door_create" if is_illumos => true,

            // These functions may return int or void depending on the exact
            // configuration of the compilation environment, but the return
            // value is not useful (always 0) so we can ignore it:
            "setservent" | "endservent" if is_illumos => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_netbsd(target: &str) {
    assert!(target.contains("netbsd"));
    let rumprun = target.contains("rumprun");
    let mut cfg = ctest_cfg();

    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("_NETBSD_SOURCE", Some("1"));

    headers! {
        cfg:
        "elf.h",
        "errno.h",
        "fcntl.h",
        "limits.h",
        "link.h",
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
        "net/bpf.h",
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
        "regex.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "string.h",
        "sys/endian.h",
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
        "sys/timex.h",
        "sys/ucontext.h",
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
        "sys/shm.h",
        "iconv.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr" | "Elf64_Phdr" | "Elf32_Shdr"
            | "Elf64_Shdr" | "Elf32_Sym" | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr"
            | "Elf32_Chdr" | "Elf64_Chdr" => ty.to_string(),

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
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" => true,

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

    cfg.skip_field(|struct_, field| {
        match (struct_, field) {
            // conflicting with `p_type` macro from <resolve.h>.
            ("Elf32_Phdr", "p_type") => true,
            ("Elf64_Phdr", "p_type") => true,
            // pthread_spin_t is a volatile uchar
            ("pthread_spinlock_t", "pts_spin") => true,
            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_dragonflybsd(target: &str) {
    assert!(target.contains("dragonfly"));
    let mut cfg = ctest_cfg();
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
        "regex.h",
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
        "sys/sched.h",
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
        "ucontext.h",
        "ufs/ufs/quota.h",
        "unistd.h",
        "util.h",
        "utime.h",
        "utmpx.h",
        "wchar.h",
        "iconv.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr" | "Elf64_Phdr" | "Elf32_Shdr"
            | "Elf64_Shdr" | "Elf32_Sym" | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr"
            | "Elf32_Chdr" | "Elf64_Chdr" => ty.to_string(),

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
            // Field is named `type` in C but that is a Rust keyword,
            // so these fields are translated to `type_` in the bindings.
            "type_" if struct_ == "rtprio" => "type".to_string(),
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
            "EADI" | "PORT_SOURCE_POSTWAIT" | "PORT_SOURCE_SIGNAL" | "PTHREAD_STACK_MIN" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64"        // non-int in 2nd arg
             => true,

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

    let mut cfg = ctest_cfg();
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
        "sys/ioctl.h",
        "time.h",
        "unistd.h",
        "wasi/api.h",
        "wasi/libc.h",
        "wasi/libc-find-relpath.h",
        "wasi/libc-nocwd.h",
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

    // Currently Rust/clang disagree on function argument ABI, so skip these
    // tests. For more info see WebAssembly/tool-conventions#88
    cfg.skip_roundtrip(|_| true);

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

    let mut cfg = ctest_cfg();
    cfg.define("_GNU_SOURCE", None);

    headers! { cfg:
               "arpa/inet.h",
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "elf.h",
               "errno.h",
               "fcntl.h",
               "grp.h",
               "ifaddrs.h",
               "limits.h",
               "link.h",
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
               "regex.h",
               "resolv.h",
               "sched.h",
               "semaphore.h",
               "signal.h",
               "stddef.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "sys/auxv.h",
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
               "sys/random.h",
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
               "sys/system_properties.h",
               "sys/time.h",
               "sys/timerfd.h",
               "sys/times.h",
               "sys/types.h",
               "sys/ucontext.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/utsname.h",
               "sys/vfs.h",
               "sys/xattr.h",
               "sys/wait.h",
               "syslog.h",
               "termios.h",
               "time.h",
               "unistd.h",
               "utime.h",
               "utmp.h",
               "wchar.h",
               "xlocale.h",
               // time64_t is not defined for 64-bit targets If included it will
               // generate the error 'Your time_t is already 64-bit'
               [target_pointer_width == 32]: "time64.h",
               [x86]: "sys/reg.h",
    }

    // Include linux headers at the end:
    headers! { cfg:
                "asm/mman.h",
                "linux/auxvec.h",
                "linux/dccp.h",
                "linux/elf.h",
                "linux/errqueue.h",
                "linux/falloc.h",
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
                "linux/netfilter/nfnetlink.h",
                "linux/netfilter/nfnetlink_log.h",
                "linux/netfilter/nfnetlink_queue.h",
                "linux/netfilter/nf_tables.h",
                "linux/netfilter_ipv4.h",
                "linux/netfilter_ipv6.h",
                "linux/netfilter_ipv6/ip6_tables.h",
                "linux/netlink.h",
                "linux/quota.h",
                "linux/reboot.h",
                "linux/seccomp.h",
                "linux/sched.h",
                "linux/sockios.h",
                "linux/vm_sockets.h",
                "linux/wait.h",

    }

    // Include Android-specific headers:
    headers! { cfg:
                "android/set_abort_message.h"
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "Elf32_Phdr" | "Elf64_Phdr" => ty.to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // sigval is a struct in Rust, but a union in C:
            "sigval" => format!("union sigval"),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {}", t),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => s.to_string(),
            // FIXME: appears that `epoll_event.data` is an union
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // FIXME: `sighandler_t` type is incorrect, see:
            // https://github.com/rust-lang/libc/issues/1359
            "sighandler_t" => true,
            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // These are tested as part of the linux_fcntl tests since there are
            // header conflicts when including them with all the other structs.
            "termios2" => true,
            // uc_sigmask and uc_sigmask64 of ucontext_t are an anonymous union
            "ucontext_t" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: deprecated: not available in any header
            // See: https://github.com/rust-lang/libc/issues/1356
            "ENOATTR" => true,

            // FIXME: still necessary?
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            // FIXME: deprecated - removed in glibc 2.26
            "SIGUNUSED" => true,

            // Needs a newer Android SDK for the definition
            "P_PIDFD" => true,

            // Requires Linux kernel 5.6
            "VMADDR_CID_LOCAL" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" | "fexecve" => true,

            // There are two versions of the sterror_r function, see
            //
            // https://linux.die.net/man/3/strerror_r
            //
            // An XSI-compliant version provided if:
            //
            // (_POSIX_C_SOURCE >= 200112L || _XOPEN_SOURCE >= 600) && ! _GNU_SOURCE
            //
            // and a GNU specific version provided if _GNU_SOURCE is defined.
            //
            // libc provides bindings for the XSI-compliant version, which is
            // preferred for portable applications.
            //
            // We skip the test here since here _GNU_SOURCE is defined, and
            // test the XSI version below.
            "strerror_r" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // FIXME: `sa_sigaction` has type `sighandler_t` but that type is
        // incorrect, see: https://github.com/rust-lang/libc/issues/1359
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // signalfd had SIGSYS fields added in Android 4.19, but CI does not have that version yet.
        (struct_ == "signalfd_siginfo" && field == "ssi_call_addr")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        (struct_ == "siginfo_t" && field == "_pad") ||
        // FIXME: `sa_sigaction` has type `sighandler_t` but that type is
        // incorrect, see: https://github.com/rust-lang/libc/issues/1359
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // sigev_notify_thread_id is actually part of a sigev_un union
        (struct_ == "sigevent" && field == "sigev_notify_thread_id") ||
        // signalfd had SIGSYS fields added in Android 4.19, but CI does not have that version yet.
        (struct_ == "signalfd_siginfo" && (field == "ssi_syscall" ||
                                           field == "ssi_call_addr" ||
                                           field == "ssi_arch"))
    });

    cfg.skip_field(|struct_, field| {
        match (struct_, field) {
            // conflicting with `p_type` macro from <resolve.h>.
            ("Elf32_Phdr", "p_type") => true,
            ("Elf64_Phdr", "p_type") => true,

            // this is actually a union on linux, so we can't represent it well and
            // just insert some padding.
            ("siginfo_t", "_pad") => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");

    test_linux_like_apis(target);
}

fn test_freebsd(target: &str) {
    assert!(target.contains("freebsd"));
    let mut cfg = ctest_cfg();

    let freebsd_ver = which_freebsd();

    match freebsd_ver {
        Some(10) => cfg.cfg("freebsd10", None),
        Some(11) => cfg.cfg("freebsd11", None),
        Some(12) => cfg.cfg("freebsd12", None),
        Some(13) => cfg.cfg("freebsd13", None),
        _ => &mut cfg,
    };

    // Required for `getline`:
    cfg.define("_WITH_GETLINE", None);
    // Required for making freebsd11_stat available in the headers
    match freebsd_ver {
        Some(10) => &mut cfg,
        _ => cfg.define("_WANT_FREEBSD11_STAT", None),
    };

    let freebsdlast = match freebsd_ver {
        Some(12) | Some(13) => true,
        _ => false,
    };

    headers! { cfg:
                "aio.h",
                "arpa/inet.h",
                "ctype.h",
                "dirent.h",
                "dlfcn.h",
                "elf.h",
                "errno.h",
                "execinfo.h",
                "fcntl.h",
                "glob.h",
                "grp.h",
                "iconv.h",
                "ifaddrs.h",
                "langinfo.h",
                "libutil.h",
                "limits.h",
                "link.h",
                "locale.h",
                "machine/reg.h",
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
                "regex.h",
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
                "sys/capsicum.h",
                [freebsdlast]:"sys/auxv.h",
                "sys/cpuset.h",
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
                "sys/random.h",
                "sys/resource.h",
                "sys/rtprio.h",
                "sys/shm.h",
                "sys/socket.h",
                "sys/stat.h",
                "sys/statvfs.h",
                "sys/sysctl.h",
                "sys/time.h",
                "sys/times.h",
                "sys/timex.h",
                "sys/types.h",
                "sys/ucontext.h",
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
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr" | "Elf64_Phdr" => ty.to_string(),

            // FIXME: https://github.com/rust-lang/libc/issues/1273
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // sigval is a struct in Rust, but a union in C:
            "sigval" => format!("union sigval"),

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
            // Field is named `type` in C but that is a Rust keyword,
            // so these fields are translated to `type_` in the bindings.
            "type_" if struct_ == "rtprio" => "type".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // These constants are to be introduced in yet-unreleased FreeBSD 12.2.
            "F_ADD_SEALS" | "F_GET_SEALS" | "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW"
            | "F_SEAL_WRITE"
                if Some(12) <= freebsd_ver =>
            {
                true
            }

            // These constants were introduced in FreeBSD 12:
            "SF_USER_READAHEAD"
            | "EVFILT_EMPTY"
            | "SO_REUSEPORT_LB"
            | "IP_ORIGDSTADDR"
            | "IP_RECVORIGDSTADDR"
            | "IPV6_ORIGDSTADDR"
            | "IPV6_RECVORIGDSTADDR"
            | "NI_NUMERICSCOPE"
                if Some(11) == freebsd_ver =>
            {
                true
            }

            // These constants were introduced in FreeBSD 11:
            "SF_USER_READAHEAD"
            | "SF_NOCACHE"
            | "RLIMIT_KQUEUES"
            | "RLIMIT_UMTXP"
            | "EVFILT_PROCDESC"
            | "EVFILT_SENDFILE"
            | "EVFILT_EMPTY"
            | "SO_REUSEPORT_LB"
            | "TCP_CCALGOOPT"
            | "TCP_PCAP_OUT"
            | "TCP_PCAP_IN"
            | "IP_BINDMULTI"
            | "IP_ORIGDSTADDR"
            | "IP_RECVORIGDSTADDR"
            | "IPV6_ORIGDSTADDR"
            | "IPV6_RECVORIGDSTADDR"
            | "PD_CLOEXEC"
            | "PD_ALLOWED_AT_FORK"
            | "IP_RSS_LISTEN_BUCKET"
            | "NI_NUMERICSCOPE"
                if Some(10) == freebsd_ver =>
            {
                true
            }

            // FIXME: This constant has a different value in FreeBSD 10:
            "RLIM_NLIMITS" if Some(10) == freebsd_ver => true,

            // FIXME: There are deprecated - remove in a couple of releases.
            // These constants were removed in FreeBSD 11 (svn r273250) but will
            // still be accepted and ignored at runtime.
            "MAP_RENAME" | "MAP_NORESERVE" if Some(10) != freebsd_ver => true,

            // FIXME: There are deprecated - remove in a couple of releases.
            // These constants were removed in FreeBSD 11 (svn r262489),
            // and they've never had any legitimate use outside of the
            // base system anyway.
            "CTL_MAXID" | "KERN_MAXID" | "HW_MAXID" | "USER_MAXID" => true,

            // This constant was removed in FreeBSD 13 (svn r363622), and never
            // had any legitimate use outside of the base system anyway.
            "CTL_P1003_1B_MAXID" => true,

            // This was renamed in FreeBSD 12.2 and 13 (r352486).
            "CTL_UNSPEC" | "CTL_SYSCTL" => true,

            // These were added in FreeBSD 12.2 and 13 (r352486),
            // but they are just names for magic numbers that existed for ages.
            "CTL_SYSCTL_DEBUG"
            | "CTL_SYSCTL_NAME"
            | "CTL_SYSCTL_NEXT"
            | "CTL_SYSCTL_NAME2OID"
            | "CTL_SYSCTL_OIDFMT"
            | "CTL_SYSCTL_OIDDESCR"
            | "CTL_SYSCTL_OIDLABEL" => true,

            // This was renamed in FreeBSD 12.2 and 13 (r350749).
            "IPPROTO_SEP" | "IPPROTO_DCCP" => true,

            // This was changed to 96(0x60) in FreeBSD 13:
            // https://github.com/freebsd/freebsd/
            // commit/06b00ceaa914a3907e4e27bad924f44612bae1d7
            "MINCORE_SUPER" if Some(13) == freebsd_ver => true,

            // This was increased to 97 in FreeBSD 12.2 and 13.
            // https://github.com/freebsd/freebsd/
            // commit/72a21ba0f62da5e86a1c0b462aeb3f5ff849a1b7
            "ELAST" if Some(12) == freebsd_ver => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // `mmsghdr` is not available in FreeBSD 10
            "mmsghdr" if Some(10) == freebsd_ver => true,

            // `max_align_t` is not available in FreeBSD 10
            "max_align_t" if Some(10) == freebsd_ver => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" | "fexecve" => true,

            // These functions were added in FreeBSD 11:
            "fdatasync" | "mq_getfd_np" | "sendmmsg" | "recvmmsg" if Some(10) == freebsd_ver => {
                true
            }

            // This function changed its return type from `int` in FreeBSD10 to
            // `ssize_t` in FreeBSD11:
            "aio_waitcomplete" if Some(10) == freebsd_ver => true,

            // The `uname` function in the `utsname.h` FreeBSD header is a C
            // inline function (has no symbol) that calls the `__xuname` symbol.
            // Therefore the function pointer comparison does not make sense for it.
            "uname" => true,

            // FIXME: Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using `&`/`&mut` here, see:
            // https://github.com/gnzlbg/ctest/issues/68
            "lio_listio" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| {
        match c {
            // FIXME: has a different sign in FreeBSD10
            "blksize_t" if Some(10) == freebsd_ver => true,
            _ => false,
        }
    });

    cfg.volatile_item(|i| {
        use ctest::VolatileItemKind::*;
        match i {
            // aio_buf is a volatile void** but since we cannot express that in
            // Rust types, we have to explicitly tell the checker about it here:
            StructField(ref n, ref f) if n == "aiocb" && f == "aio_buf" => true,
            _ => false,
        }
    });

    cfg.skip_field(move |struct_, field| {
        match (struct_, field) {
            // FIXME: `sa_sigaction` has type `sighandler_t` but that type is
            // incorrect, see: https://github.com/rust-lang/libc/issues/1359
            ("sigaction", "sa_sigaction") => true,

            // FIXME: in FreeBSD10 this field has type `char*` instead of
            // `void*`:
            ("stack_t", "ss_sp") if Some(10) == freebsd_ver => true,

            // conflicting with `p_type` macro from <resolve.h>.
            ("Elf32_Phdr", "p_type") => true,
            ("Elf64_Phdr", "p_type") => true,

            // not available until FreeBSD 12, and is an anonymous union there.
            ("xucred", "cr_pid__c_anonymous_union") => true,

            // m_owner field is a volatile __lwpid_t
            ("umutex", "m_owner") => true,
            // c_has_waiters field is a volatile int32_t
            ("ucond", "c_has_waiters") => true,

            _ => false,
        }
    });

    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_emscripten(target: &str) {
    assert!(target.contains("emscripten"));

    let mut cfg = ctest_cfg();
    cfg.define("_GNU_SOURCE", None); // FIXME: ??

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
            "FILE" | "fd_set" | "Dl_info" | "DIR" => ty.to_string(),

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
            // FIXME: appears that `epoll_event.data` is an union
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
            // This is actually a union, not a struct
            // FIXME: is this necessary?
            "sigval" => true,

            // FIXME: It was removed in
            // emscripten-core/emscripten@953e414
            "pthread_mutexattr_t" => true,

            // FIXME: Investigate why the test fails.
            // Skip for now to unblock CI.
            "pthread_condattr_t" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" | "fexecve" => true,

            // FIXME: Investigate why CI is missing it.
            "clearenv" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: deprecated - SIGNUNUSED was removed in glibc 2.26
            // users should use SIGSYS instead
            "SIGUNUSED" => true,

            // FIXME: emscripten uses different constants to constructs these
            n if n.contains("__SIZEOF_PTHREAD") => true,

            // FIXME: `SYS_gettid` was removed in
            // emscripten-core/emscripten@6d6474e
            "SYS_gettid" => true,

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

    // FIXME: test linux like
    cfg.generate("../src/lib.rs", "main.rs");
}

fn test_vxworks(target: &str) {
    assert!(target.contains("vxworks"));

    let mut cfg = ctest::TestGenerator::new();
    headers! { cfg:
               "vxWorks.h",
               "yvals.h",
               "nfs/nfsCommon.h",
               "rtpLibCommon.h",
               "randomNumGen.h",
               "taskLib.h",
               "sysLib.h",
               "ioLib.h",
               "inetLib.h",
               "socket.h",
               "errnoLib.h",
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "elf.h",
               "fcntl.h",
               "grp.h",
               "sys/poll.h",
               "ifaddrs.h",
               "langinfo.h",
               "limits.h",
               "link.h",
               "locale.h",
               "sys/stat.h",
               "netdb.h",
               "pthread.h",
               "pwd.h",
               "sched.h",
               "semaphore.h",
               "signal.h",
               "stddef.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "sys/file.h",
               "sys/ioctl.h",
               "sys/socket.h",
               "sys/time.h",
               "sys/times.h",
               "sys/types.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/utsname.h",
               "sys/wait.h",
               "netinet/tcp.h",
               "syslog.h",
               "termios.h",
               "time.h",
               "ucontext.h",
               "unistd.h",
               "utime.h",
               "wchar.h",
               "errno.h",
               "sys/mman.h",
               "pathLib.h",
               "mqueue.h",
    }
    // FIXME
    cfg.skip_const(move |name| match name {
        // sighandler_t weirdness
        "SIG_DFL" | "SIG_ERR" | "SIG_IGN"
        // This is not defined in vxWorks
        | "RTLD_DEFAULT"   => true,
        _ => false,
    });
    // FIXME
    cfg.skip_type(move |ty| match ty {
        "stat64" | "sighandler_t" | "off64_t" => true,
        _ => false,
    });

    cfg.skip_field_type(move |struct_, field| match (struct_, field) {
        ("siginfo_t", "si_value") | ("stat", "st_size") | ("sigaction", "sa_u") => true,
        _ => false,
    });

    cfg.skip_roundtrip(move |s| match s {
        _ => false,
    });

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "DIR" | "FILE" | "Dl_info" | "RTP_DESC" => ty.to_string(),
        t if is_union => format!("union {}", t),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {}", t),
        t => t.to_string(),
    });

    // FIXME
    cfg.skip_fn(move |name| match name {
        // sigval
        "sigqueue" | "_sigqueue"
        // sighandler_t
        | "signal"
        // not used in static linking by default
        | "dlerror" => true,
        _ => false,
    });

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

    let arm = target.contains("arm");
    let i686 = target.contains("i686");
    let mips = target.contains("mips");
    let mips32 = mips && !target.contains("64");
    let mips64 = mips && target.contains("64");
    let ppc64 = target.contains("powerpc64");
    let s390x = target.contains("s390x");
    let sparc64 = target.contains("sparc64");
    let x32 = target.contains("x32");
    let x86_32 = target.contains("i686");
    let x86_64 = target.contains("x86_64");
    let aarch64_musl = target.contains("aarch64") && musl;
    let gnuabihf = target.contains("gnueabihf");
    let x86_64_gnux32 = target.contains("gnux32") && x86_64;
    let riscv64 = target.contains("riscv64");
    let uclibc = target.contains("uclibc");

    let mut cfg = ctest_cfg();
    cfg.define("_GNU_SOURCE", None);
    // This macro re-deifnes fscanf,scanf,sscanf to link to the symbols that are
    // deprecated since glibc >= 2.29. This allows Rust binaries to link against
    // glibc versions older than 2.29.
    cfg.define("__GLIBC_USE_DEPRECATED_SCANF", None);

    headers! { cfg:
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "elf.h",
               "fcntl.h",
               "glob.h",
               "grp.h",
               "iconv.h",
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
               "regex.h",
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
               "sys/random.h",
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
               "sys/timex.h",
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
               "utmpx.h",
               "wchar.h",
               "errno.h",
               // `sys/io.h` is only available on x86*, Alpha, IA64, and 32-bit
               // ARM: https://bugzilla.redhat.com/show_bug.cgi?id=1116162
               // Also unavailable on gnuabihf with glibc 2.30.
               // https://sourceware.org/git/?p=glibc.git;a=commitdiff;h=6b33f373c7b9199e00ba5fbafd94ac9bfb4337b1
               [(x86_64 || x86_32 || arm) && !gnuabihf]: "sys/io.h",
               // `sys/reg.h` is only available on x86 and x86_64
               [x86_64 || x86_32]: "sys/reg.h",
               // sysctl system call is deprecated and not available on musl
               // It is also unsupported in x32, deprecated since glibc 2.30:
               [!(x32 || musl || gnu)]: "sys/sysctl.h",
               // <execinfo.h> is not supported by musl:
               // https://www.openwall.com/lists/musl/2015/04/09/3
               // <execinfo.h> is not present on uclibc.
               [!(musl || uclibc)]: "execinfo.h",
    }

    // Include linux headers at the end:
    headers! {
        cfg:
        "asm/mman.h",
        "linux/can.h",
        "linux/dccp.h",
        "linux/errqueue.h",
        "linux/falloc.h",
        "linux/fs.h",
        "linux/futex.h",
        "linux/genetlink.h",
        "linux/if.h",
        "linux/if_addr.h",
        "linux/if_alg.h",
        "linux/if_ether.h",
        "linux/if_tun.h",
        "linux/input.h",
        "linux/keyctl.h",
        "linux/magic.h",
        "linux/memfd.h",
        "linux/mman.h",
        "linux/module.h",
        "linux/net_tstamp.h",
        "linux/netfilter/nfnetlink.h",
        "linux/netfilter/nfnetlink_log.h",
        "linux/netfilter/nfnetlink_queue.h",
        "linux/netfilter/nf_tables.h",
        "linux/netfilter_ipv4.h",
        "linux/netfilter_ipv6.h",
        "linux/netfilter_ipv6/ip6_tables.h",
        "linux/netlink.h",
        "linux/quota.h",
        "linux/random.h",
        "linux/reboot.h",
        "linux/rtnetlink.h",
        "linux/seccomp.h",
        "linux/sockios.h",
        "linux/uinput.h",
        "linux/vm_sockets.h",
        "linux/wait.h",
        "sys/fanotify.h",
        // <sys/auxv.h> is not present on uclibc
        [!uclibc]: "sys/auxv.h",
    }

    // note: aio.h must be included before sys/mount.h
    headers! {
        cfg:
        "sys/xattr.h",
        "sys/sysinfo.h",
        // AIO is not supported by uclibc:
        [!uclibc]: "aio.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr" | "Elf64_Phdr" | "Elf32_Shdr"
            | "Elf64_Shdr" | "Elf32_Sym" | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr"
            | "Elf32_Chdr" | "Elf64_Chdr" => ty.to_string(),

            t if is_union => format!("union {}", t),

            t if t.ends_with("_t") => t.to_string(),

            // In MUSL `flock64` is a typedef to `flock`.
            "flock64" if musl => format!("struct {}", ty),

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
            // FIXME: epoll_event.data is actually a union in C, but in Rust
            // it is only a u64 because we only expose one field
            // http://man7.org/linux/man-pages/man2/epoll_wait.2.html
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            // The following structs have a field called `type` in C,
            // but `type` is a Rust keyword, so these fields are translated
            // to `type_` in Rust.
            "type_"
                if struct_ == "input_event"
                    || struct_ == "input_mask"
                    || struct_ == "ff_effect" =>
            {
                "type".to_string()
            }

            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // FIXME: `sighandler_t` type is incorrect, see:
            // https://github.com/rust-lang/libc/issues/1359
            "sighandler_t" => true,

            // These cannot be tested when "resolv.h" is included and are tested
            // in the `linux_elf.rs` file.
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            // This type is private on Linux. It is implemented as a C `enum`
            // (`c_uint`) and this clashes with the type of the `rlimit` APIs
            // which expect a `c_int` even though both are ABI compatible.
            "__rlimit_resource_t" => true,
            // on Linux, this is a volatile int
            "pthread_spinlock_t" => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        // FIXME: musl CI has old headers
        if (musl || sparc64) && ty.starts_with("uinput_") {
            return true;
        }
        match ty {
            // These cannot be tested when "resolv.h" is included and are tested
            // in the `linux_elf.rs` file.
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            // On Linux, the type of `ut_tv` field of `struct utmpx`
            // can be an anonymous struct, so an extra struct,
            // which is absent in glibc, has to be defined.
            "__timeval" => true,

            // FIXME: This is actually a union, not a struct
            "sigval" => true,

            // This type is tested in the `linux_termios.rs` file since there
            // are header conflicts when including them with all the other
            // structs.
            "termios2" => true,

            // FIXME: remove once we set minimum supported glibc version.
            // ucontext_t added a new field as of glibc 2.28; our struct definition is
            // conservative and omits the field, but that means the size doesn't match for newer
            // glibcs (see https://github.com/rust-lang/libc/issues/1410)
            "ucontext_t" if gnu => true,

            // FIXME: Somehow we cannot include headers correctly in glibc 2.30.
            // So let's ignore for now and re-visit later.
            // Probably related: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=91085
            "statx" => true,
            "statx_timestamp" => true,

            // On Linux, the type of `ut_exit` field of struct `utmpx`
            // can be an anonymous struct, so an extra struct,
            // which is absent in musl, has to be defined.
            "__exit_status" if musl => true,

            // FIXME: CI's kernel header version is old.
            "sockaddr_can" => true,

            // Requires glibc 2.33 or newer.
            "mallinfo2" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        if !gnu {
            // Skip definitions from the kernel on non-glibc Linux targets.
            // They're libc-independent, so we only need to check them on one
            // libc. We don't want to break CI if musl or another libc doesn't
            // have the definitions yet. (We do still want to check them on
            // every glibc target, though, as some of them can vary by
            // architecture.)
            //
            // This is not an exhaustive list of kernel constants, just a list
            // of prefixes of all those that have appeared here or that get
            // updated regularly and seem likely to cause breakage.
            if name.starts_with("AF_")
                || name.starts_with("ARPHRD_")
                || name.starts_with("EPOLL")
                || name.starts_with("F_")
                || name.starts_with("FALLOC_FL_")
                || name.starts_with("IFLA_")
                || name.starts_with("MS_")
                || name.starts_with("MSG_")
                || name.starts_with("P_")
                || name.starts_with("PF_")
                || name.starts_with("RLIMIT_")
                || name.starts_with("SOL_")
                || name.starts_with("STATX_")
                || name.starts_with("SW_")
                || name.starts_with("SYS_")
                || name.starts_with("TCP_")
                || name.starts_with("UINPUT_")
                || name.starts_with("VMADDR_")
            {
                return true;
            }
        }
        match name {
            // These constants are not available if gnu headers have been included
            // and can therefore not be tested here
            //
            // The IPV6 constants are tested in the `linux_ipv6.rs` tests:
            | "IPV6_FLOWINFO"
            | "IPV6_FLOWLABEL_MGR"
            | "IPV6_FLOWINFO_SEND"
            | "IPV6_FLOWINFO_FLOWLABEL"
            | "IPV6_FLOWINFO_PRIORITY"
            // The F_ fnctl constants are tested in the `linux_fnctl.rs` tests:
            | "F_CANCELLK"
            | "F_ADD_SEALS"
            | "F_GET_SEALS"
            | "F_SEAL_SEAL"
            | "F_SEAL_SHRINK"
            | "F_SEAL_GROW"
            | "F_SEAL_WRITE" => true,
            // The `ARPHRD_CAN` is tested in the `linux_if_arp.rs` tests
            // because including `linux/if_arp.h` causes some conflicts:
            "ARPHRD_CAN" => true,

            // Require Linux kernel 5.1:
            "F_SEAL_FUTURE_WRITE" => true,

            // FIXME: deprecated: not available in any header
            // See: https://github.com/rust-lang/libc/issues/1356
            "ENOATTR" => true,

            // FIXME: SIGUNUSED was removed in glibc 2.26
            // Users should use SIGSYS instead.
            "SIGUNUSED" => true,

            // FIXME: conflicts with glibc headers and is tested in
            // `linux_termios.rs` below:
            "BOTHER" => true,

            // FIXME: on musl the pthread types are defined a little differently
            // - these constants are used by the glibc implementation.
            n if musl && n.contains("__SIZEOF_PTHREAD") => true,

            // FIXME: It was extended to 4096 since glibc 2.31 (Linux 5.4).
            // We should do so after a while.
            "SOMAXCONN" if gnu => true,

            // deprecated: not available from Linux kernel 5.6:
            "VMADDR_CID_RESERVED" => true,

            // Require Linux kernel 5.6:
            "VMADDR_CID_LOCAL" => true,

            // IPPROTO_MAX was increased in 5.6 for IPPROTO_MPTCP:
            | "IPPROTO_MAX"
            | "IPPROTO_MPTCP" => true,

            // FIXME: Not currently available in headers
            "P_PIDFD" if mips => true,
            "SYS_pidfd_open" if mips => true,

            // FIXME: Not currently available in headers on MIPS
            // Not yet implemented on sparc64
            "SYS_clone3" if mips | sparc64 => true,

            // FIXME: these syscalls were added in Linux 5.9 or later
            // and are currently not included in the glibc headers.
            | "SYS_close_range"
            | "SYS_openat2"
            | "SYS_pidfd_getfd"
            | "SYS_faccessat2"
            | "SYS_process_madvise"
            | "SYS_epoll_pwait2"
            | "SYS_mount_setattr" => true,

            // Requires more recent kernel headers:
            | "IFLA_PROP_LIST"
            | "IFLA_ALT_IFNAME"
            | "IFLA_PERM_ADDRESS"
            | "IFLA_PROTO_DOWN_REASON" => true,

            // FIXME: They require recent kernel header:
            | "CAN_J1939"
            | "CAN_RAW_FILTER_MAX"
            | "CAN_NPROTO" => true,

            // FIXME: Requires recent kernel headers (5.8):
            "STATX_MNT_ID" => true,

            // FIXME: requires more recent kernel headers on CI
            | "UINPUT_VERSION"
            | "SW_MAX"
            | "SW_CNT"
                if mips || ppc64 || riscv64 || sparc64 => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" | "fexecve" => true,

            // There are two versions of the sterror_r function, see
            //
            // https://linux.die.net/man/3/strerror_r
            //
            // An XSI-compliant version provided if:
            //
            // (_POSIX_C_SOURCE >= 200112L || _XOPEN_SOURCE >= 600)
            //  && ! _GNU_SOURCE
            //
            // and a GNU specific version provided if _GNU_SOURCE is defined.
            //
            // libc provides bindings for the XSI-compliant version, which is
            // preferred for portable applications.
            //
            // We skip the test here since here _GNU_SOURCE is defined, and
            // test the XSI version below.
            "strerror_r" => true,

            // FIXME: Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using `&`/`&mut` here, see:
            // https://github.com/gnzlbg/ctest/issues/68
            "lio_listio" if musl => true,

            // FIXME: the glibc version used by the Sparc64 build jobs
            // which use Debian 10.0 is too old.
            "statx" if sparc64 => true,

            // FIXME: Deprecated since glibc 2.30. Remove fn once upstream does.
            "sysctl" if gnu => true,

            // FIXME: It now takes c_void instead of timezone since glibc 2.31.
            "gettimeofday" if gnu => true,

            // These are all implemented as static inline functions in uclibc, so
            // they cannot be linked against.
            // If implementations are required, they might need to be implemented
            // in this crate.
            "posix_spawnattr_init" if uclibc => true,
            "posix_spawnattr_destroy" if uclibc => true,
            "posix_spawnattr_getsigdefault" if uclibc => true,
            "posix_spawnattr_setsigdefault" if uclibc => true,
            "posix_spawnattr_getsigmask" if uclibc => true,
            "posix_spawnattr_setsigmask" if uclibc => true,
            "posix_spawnattr_getflags" if uclibc => true,
            "posix_spawnattr_setflags" if uclibc => true,
            "posix_spawnattr_getpgroup" if uclibc => true,
            "posix_spawnattr_setpgroup" if uclibc => true,
            "posix_spawnattr_getschedpolicy" if uclibc => true,
            "posix_spawnattr_setschedpolicy" if uclibc => true,
            "posix_spawnattr_getschedparam" if uclibc => true,
            "posix_spawnattr_setschedparam" if uclibc => true,
            "posix_spawn_file_actions_init" if uclibc => true,
            "posix_spawn_file_actions_destroy" if uclibc => true,

            // uclibc defines the flags type as a uint, but dependent crates
            // assume it's a int instead.
            "getnameinfo" if uclibc => true,

            // FIXME: This needs musl 1.2.2 or later.
            "gettid" if musl => true,

            // Needs glibc 2.33 or later.
            "mallinfo2" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // __timeval type is a patch which doesn't exist in glibc
        (struct_ == "utmpx" && field == "ut_tv") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // this one is an anonymous union
        (struct_ == "ff_effect" && field == "u") ||
        // `__exit_status` type is a patch which is absent in musl
        (struct_ == "utmpx" && field == "ut_exit" && musl) ||
        // `can_addr` is an anonymous union
        (struct_ == "sockaddr_can" && field == "can_addr")
    });

    cfg.volatile_item(|i| {
        use ctest::VolatileItemKind::*;
        match i {
            // aio_buf is a volatile void** but since we cannot express that in
            // Rust types, we have to explicitly tell the checker about it here:
            StructField(ref n, ref f) if n == "aiocb" && f == "aio_buf" => true,
            _ => false,
        }
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
        // signalfd had SIGSYS fields added in Linux 4.18, but no libc release
        // has them yet.
        (struct_ == "signalfd_siginfo" && (field == "ssi_addr_lsb" ||
                                           field == "_pad2" ||
                                           field == "ssi_syscall" ||
                                           field == "ssi_call_addr" ||
                                           field == "ssi_arch")) ||
        // FIXME: After musl 1.1.24, it have only one field `sched_priority`,
        // while other fields become reserved.
        (struct_ == "sched_param" && [
            "sched_ss_low_priority",
            "sched_ss_repl_period",
            "sched_ss_init_budget",
            "sched_ss_max_repl",
        ].contains(&field) && musl) ||
        // FIXME: After musl 1.1.24, the type becomes `int` instead of `unsigned short`.
        (struct_ == "ipc_perm" && field == "__seq" && aarch64_musl) ||
        // glibc uses unnamed fields here and Rust doesn't support that yet
        (struct_ == "timex" && field.starts_with("__unused")) ||
        // FIXME: It now takes mode_t since glibc 2.31 on some targets.
        (struct_ == "ipc_perm" && field == "mode"
            && ((x86_64 || i686 || arm || riscv64) && gnu || x86_64_gnux32)
        )
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME:
        "utsname" if mips32 || mips64 => true,
        // FIXME:
        "mcontext_t" if s390x => true,
        // FIXME: This is actually a union.
        "fpreg_t" if s390x => true,

        "sockaddr_un" | "sembuf" | "ff_constant_effect" if mips32 && (gnu || musl) => true,
        "ipv6_mreq"
        | "ip_mreq_source"
        | "sockaddr_in6"
        | "sockaddr_ll"
        | "in_pktinfo"
        | "arpreq"
        | "arpreq_old"
        | "sockaddr_un"
        | "ff_constant_effect"
        | "ff_ramp_effect"
        | "ff_condition_effect"
        | "Elf32_Ehdr"
        | "Elf32_Chdr"
        | "ucred"
        | "in6_pktinfo"
        | "sockaddr_nl"
        | "termios"
        | "nlmsgerr"
            if (mips64 || sparc64) && gnu =>
        {
            true
        }

        // FIXME: the call ABI of max_align_t is incorrect on these platforms:
        "max_align_t" if i686 || mips64 || ppc64 => true,

        _ => false,
    });

    cfg.generate("../src/lib.rs", "main.rs");

    test_linux_like_apis(target);
}

// This function tests APIs that are incompatible to test when other APIs
// are included (e.g. because including both sets of headers clashes)
fn test_linux_like_apis(target: &str) {
    let musl = target.contains("musl");
    let linux = target.contains("linux");
    let emscripten = target.contains("emscripten");
    let android = target.contains("android");
    assert!(linux || android || emscripten);

    if linux || android || emscripten {
        // test strerror_r from the `string.h` header
        let mut cfg = ctest_cfg();
        cfg.skip_type(|_| true).skip_static(|_| true);

        headers! { cfg: "string.h" }
        cfg.skip_fn(|f| match f {
            "strerror_r" => false,
            _ => true,
        })
        .skip_const(|_| true)
        .skip_struct(|_| true);
        cfg.generate("../src/lib.rs", "linux_strerror_r.rs");
    }

    if linux || android || emscripten {
        // test fcntl - see:
        // http://man7.org/linux/man-pages/man2/fcntl.2.html
        let mut cfg = ctest_cfg();

        if musl {
            cfg.header("fcntl.h");
        } else {
            cfg.header("linux/fcntl.h");
        }

        cfg.skip_type(|_| true)
            .skip_static(|_| true)
            .skip_struct(|_| true)
            .skip_fn(|_| true)
            .skip_const(move |name| match name {
                // test fcntl constants:
                "F_CANCELLK" | "F_ADD_SEALS" | "F_GET_SEALS" | "F_SEAL_SEAL" | "F_SEAL_SHRINK"
                | "F_SEAL_GROW" | "F_SEAL_WRITE" => false,
                _ => true,
            })
            .type_name(move |ty, is_struct, is_union| match ty {
                t if is_struct => format!("struct {}", t),
                t if is_union => format!("union {}", t),
                t => t.to_string(),
            });

        cfg.generate("../src/lib.rs", "linux_fcntl.rs");
    }

    if linux || android {
        // test termios
        let mut cfg = ctest_cfg();
        cfg.header("asm/termbits.h");
        cfg.skip_type(|_| true)
            .skip_static(|_| true)
            .skip_fn(|_| true)
            .skip_const(|c| c != "BOTHER")
            .skip_struct(|s| s != "termios2")
            .type_name(move |ty, is_struct, is_union| match ty {
                t if is_struct => format!("struct {}", t),
                t if is_union => format!("union {}", t),
                t => t.to_string(),
            });
        cfg.generate("../src/lib.rs", "linux_termios.rs");
    }

    if linux || android {
        // test IPV6_ constants:
        let mut cfg = ctest_cfg();
        headers! {
            cfg:
            "linux/in6.h"
        }
        cfg.skip_type(|_| true)
            .skip_static(|_| true)
            .skip_fn(|_| true)
            .skip_const(|_| true)
            .skip_struct(|_| true)
            .skip_const(move |name| match name {
                "IPV6_FLOWINFO"
                | "IPV6_FLOWLABEL_MGR"
                | "IPV6_FLOWINFO_SEND"
                | "IPV6_FLOWINFO_FLOWLABEL"
                | "IPV6_FLOWINFO_PRIORITY" => false,
                _ => true,
            })
            .type_name(move |ty, is_struct, is_union| match ty {
                t if is_struct => format!("struct {}", t),
                t if is_union => format!("union {}", t),
                t => t.to_string(),
            });
        cfg.generate("../src/lib.rs", "linux_ipv6.rs");
    }

    if linux || android {
        // Test Elf64_Phdr and Elf32_Phdr
        // These types have a field called `p_type`, but including
        // "resolve.h" defines a `p_type` macro that expands to `__p_type`
        // making the tests for these fails when both are included.
        let mut cfg = ctest_cfg();
        cfg.header("elf.h");
        cfg.skip_fn(|_| true)
            .skip_static(|_| true)
            .skip_const(|_| true)
            .type_name(move |ty, _is_struct, _is_union| ty.to_string())
            .skip_struct(move |ty| match ty {
                "Elf64_Phdr" | "Elf32_Phdr" => false,
                _ => true,
            })
            .skip_type(move |ty| match ty {
                "Elf64_Phdr" | "Elf32_Phdr" => false,
                _ => true,
            });
        cfg.generate("../src/lib.rs", "linux_elf.rs");
    }

    if linux || android {
        // Test `ARPHRD_CAN`.
        let mut cfg = ctest_cfg();
        cfg.header("linux/if_arp.h");
        cfg.skip_fn(|_| true)
            .skip_static(|_| true)
            .skip_const(move |name| match name {
                "ARPHRD_CAN" => false,
                _ => true,
            })
            .skip_struct(|_| true)
            .skip_type(|_| true);
        cfg.generate("../src/lib.rs", "linux_if_arp.rs");
    }
}

fn which_freebsd() -> Option<i32> {
    let output = std::process::Command::new("freebsd-version")
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;

    match &stdout {
        s if s.starts_with("10") => Some(10),
        s if s.starts_with("11") => Some(11),
        s if s.starts_with("12") => Some(12),
        s if s.starts_with("13") => Some(13),
        _ => None,
    }
}

fn test_haiku(target: &str) {
    assert!(target.contains("haiku"));

    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("__USE_GNU", Some("1"));

    // POSIX API
    headers! { cfg:
               "alloca.h",
               "arpa/inet.h",
               "arpa/nameser.h",
               "arpa/nameser_compat.h",
               "assert.h",
               "bsd_mem.h",
               "complex.h",
               "ctype.h",
               "dirent.h",
               "div_t.h",
               "dlfcn.h",
               "endian.h",
               "errno.h",
               "fcntl.h",
               "fenv.h",
               "fnmatch.h",
               "fts.h",
               "ftw.h",
               "getopt.h",
               "glob.h",
               "grp.h",
               "inttypes.h",
               "iovec.h",
               "langinfo.h",
               "libgen.h",
               "libio.h",
               "limits.h",
               "locale.h",
               "malloc.h",
               "malloc_debug.h",
               "math.h",
               "memory.h",
               "monetary.h",
               "net/if.h",
               "net/if_dl.h",
               "net/if_media.h",
               "net/if_tun.h",
               "net/if_types.h",
               "net/route.h",
               "netdb.h",
               "netinet/icmp6.h",
               "netinet/in.h",
               "netinet/ip.h",
               "netinet/ip6.h",
               "netinet/ip_icmp.h",
               "netinet/ip_var.h",
               "netinet/tcp.h",
               "netinet/udp.h",
               "netinet6/in6.h",
               "nl_types.h",
               "null.h",
               "poll.h",
               "pthread.h",
               "pwd.h",
               "regex.h",
               "resolv.h",
               "sched.h",
               "search.h",
               "semaphore.h",
               "setjmp.h",
               "shadow.h",
               "signal.h",
               "size_t.h",
               "spawn.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "strings.h",
               "sys/cdefs.h",
               "sys/file.h",
               "sys/ioctl.h",
               "sys/ipc.h",
               "sys/mman.h",
               "sys/msg.h",
               "sys/param.h",
               "sys/poll.h",
               "sys/resource.h",
               "sys/select.h",
               "sys/sem.h",
               "sys/socket.h",
               "sys/sockio.h",
               "sys/stat.h",
               "sys/statvfs.h",
               "sys/time.h",
               "sys/timeb.h",
               "sys/times.h",
               "sys/types.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/utsname.h",
               "sys/wait.h",
               "syslog.h",
               "tar.h",
               "termios.h",
               "time.h",
               "uchar.h",
               "unistd.h",
               "utime.h",
               "wchar.h",
               "wchar_t.h",
               "wctype.h"
    }

    // BSD Extensions
    headers! { cfg:
               "pty.h",
    }

    // Native API
    headers! { cfg:
               "kernel/OS.h",
               "kernel/fs_attr.h",
               "kernel/fs_index.h",
               "kernel/fs_info.h",
               "kernel/fs_query.h",
               "kernel/fs_volume.h",
               "kernel/image.h",
               "kernel/scheduler.h",
               "storage/StorageDefs.h",
               "support/Errors.h",
               "support/SupportDefs.h",
               "support/TypeConstants.h"
    }

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME: actually a union
            "sigval" => true,
            // FIXME: locale_t does not exist on Haiku
            "locale_t" => true,
            // FIXME: rusage has a different layout on Haiku
            "rusage" => true,
            // FIXME?: complains that rust aligns on 4 byte boundary, but
            //         Haiku does not align it at all.
            "in6_addr" => true,
            // The d_name attribute is an array of 1 on Haiku, with the
            // intention that the developer allocates a larger or smaller
            // piece of memory depending on the expected/actual size of the name.
            // Other platforms have sensible defaults. In Rust, the d_name field
            // is sized as the _POSIX_MAX_PATH, so that path names will fit in
            // newly allocated dirent objects. This breaks the automated tests.
            "dirent" => true,

            _ => false,
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // FIXME: locale_t does not exist on Haiku
            "locale_t" => true,
            // These cause errors, to be reviewed in the future
            "sighandler_t" => true,
            "pthread_t" => true,
            "pthread_condattr_t" => true,
            "pthread_mutexattr_t" => true,
            "pthread_rwlockattr_t" => true,
            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" => true,
            // FIXME: does not exist on haiku
            "open_wmemstream" => true,
            "mlockall" | "munlockall" => true,
            "tcgetsid" => true,
            "cfsetspeed" => true,
            // ignore for now, will be part of Haiku R1 beta 3
            "mlock" | "munlock" => true,
            // returns const char * on Haiku
            "strsignal" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME: these constants do not exist on Haiku
            "DT_UNKNOWN" | "DT_FIFO" | "DT_CHR" | "DT_DIR" | "DT_BLK" | "DT_REG" | "DT_LNK"
            | "DT_SOCK" => true,
            "USRQUOTA" | "GRPQUOTA" => true,
            "SIGIOT" => true,
            "ARPOP_REQUEST" | "ARPOP_REPLY" | "ATF_COM" | "ATF_PERM" | "ATF_PUBL"
            | "ATF_USETRAILERS" => true,
            // Haiku does not have MAP_FILE, but rustc requires it
            "MAP_FILE" => true,
            // The following does not exist on Haiku but is required by
            // several crates
            "FIOCLEX" => true,
            // just skip this one, it is not defined on Haiku beta 2 but
            // since it is meant as a mask and not a parameter it can exist
            // here
            "LOG_PRIMASK" => true,
            // not defined on Haiku, but [get|set]priority is, so they are
            // useful
            "PRIO_MIN" | "PRIO_MAX" => true,
            //
            _ => false,
        }
    });

    cfg.skip_field(move |struct_, field| {
        match (struct_, field) {
            // FIXME: the stat struct actually has timespec members, whereas
            //        the current representation has these unpacked.
            ("stat", "st_atime") => true,
            ("stat", "st_atime_nsec") => true,
            ("stat", "st_mtime") => true,
            ("stat", "st_mtime_nsec") => true,
            ("stat", "st_ctime") => true,
            ("stat", "st_ctime_nsec") => true,
            ("stat", "st_crtime") => true,
            ("stat", "st_crtime_nsec") => true,

            // these are actually unions, but we cannot represent it well
            ("siginfo_t", "sigval") => true,
            ("sem_t", "named_sem_id") => true,
            ("sigaction", "sa_sigaction") => true,
            ("sigevent", "sigev_value") => true,

            // skip these enum-type fields
            ("thread_info", "state") => true,
            ("image_info", "image_type") => true,
            _ => false,
        }
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME: for some reason the roundtrip check fails for cpu_info
        "cpu_info" => true,
        _ => false,
    });

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "area_info" | "port_info" | "port_message_info" | "team_info" | "sem_info"
            | "team_usage_info" | "thread_info" | "cpu_info" | "system_info"
            | "object_wait_info" | "image_info" | "attr_info" | "index_info" | "fs_info"
            | "FILE" | "DIR" | "Dl_info" => ty.to_string(),

            // is actually a union
            "sigval" => format!("union sigval"),
            t if is_union => format!("union {}", t),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {}", t),
            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Field is named `type` in C but that is a Rust keyword,
            // so these fields are translated to `type_` in the bindings.
            "type_" if struct_ == "object_wait_info" => "type".to_string(),
            "type_" if struct_ == "sem_t" => "type".to_string(),
            "type_" if struct_ == "attr_info" => "type".to_string(),
            "type_" if struct_ == "index_info" => "type".to_string(),
            "image_type" if struct_ == "image_info" => "type".to_string(),
            s => s.to_string(),
        }
    });
    cfg.generate("../src/lib.rs", "main.rs");
}
