#![deny(warnings)]
#![allow(clippy::match_like_matches_macro)]

extern crate ctest2 as ctest;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::{env, io};

fn src_hotfix_dir() -> PathBuf {
    Path::new(&env::var_os("OUT_DIR").unwrap()).join("src-hotfix")
}

fn do_cc() {
    let target = env::var("TARGET").unwrap();
    if cfg!(unix) || target.contains("cygwin") {
        let exclude = ["redox", "wasi", "wali"];
        if !exclude.iter().any(|x| target.contains(x)) {
            let mut cmsg = cc::Build::new();

            cmsg.file("src/cmsg.c");

            if target.contains("solaris") || target.contains("illumos") {
                cmsg.define("_XOPEN_SOURCE", "700");
            }
            cmsg.compile("cmsg");
        }

        if (target.contains("linux") && !target.contains("wasm32"))
            || target.contains("android")
            || target.contains("emscripten")
            || target.contains("fuchsia")
            || target.contains("bsd")
            || target.contains("cygwin")
        {
            cc::Build::new().file("src/makedev.c").compile("makedev");
        }
    }
    if target.contains("android") || (target.contains("linux") && !target.contains("wasm32")) {
        cc::Build::new().file("src/errqueue.c").compile("errqueue");
    }
    if (target.contains("linux") && !target.contains("wasm32"))
        || target.contains("l4re")
        || target.contains("android")
        || target.contains("emscripten")
        || target.contains("solaris")
        || target.contains("illumos")
    {
        cc::Build::new().file("src/sigrt.c").compile("sigrt");
    }
}

fn do_ctest() {
    match &env::var("TARGET").unwrap() {
        t if t.contains("android") => test_android(t),
        t if t.contains("apple") => test_apple(t),
        t if t.contains("dragonfly") => test_dragonflybsd(t),
        t if t.contains("emscripten") => test_emscripten(t),
        t if t.contains("freebsd") => test_freebsd(t),
        t if t.contains("haiku") => test_haiku(t),
        t if t.contains("linux") => test_linux(t),
        t if t.contains("netbsd") => test_netbsd(t),
        t if t.contains("openbsd") => test_openbsd(t),
        t if t.contains("cygwin") => test_cygwin(t),
        t if t.contains("redox") => test_redox(t),
        t if t.contains("solaris") => test_solarish(t),
        t if t.contains("illumos") => test_solarish(t),
        t if t.contains("wasi") => test_wasi(t),
        t if t.contains("windows") => test_windows(t),
        t if t.contains("vxworks") => test_vxworks(t),
        t if t.contains("nto-qnx") => test_neutrino(t),
        t if t.contains("aix") => return test_aix(t),
        t => panic!("unknown target {t}"),
    }
}

fn ctest_cfg() -> ctest::TestGenerator {
    let mut cfg = ctest::TestGenerator::new();
    let libc_cfgs = ["libc_thread_local"];
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
    let mut semver_root = PathBuf::from("semver");

    // NOTE: Windows has the same `family` as `os`, no point in including it
    // twice.
    // NOTE: Android doesn't include the unix file (or the Linux file) because
    // there are some many definitions missing it's actually easier just to
    // maintain a file for Android.
    // NOTE: AIX doesn't include the unix file because there are definitions
    // missing on AIX. It is easier to maintain a file for AIX.
    if family != os && !matches!(os.as_str(), "android" | "aix") {
        process_semver_file(&mut output, &mut semver_root, &family);
    }
    // We don't do semver for unknown targets.
    if vendor != "unknown" {
        process_semver_file(&mut output, &mut semver_root, &vendor);
    }
    process_semver_file(&mut output, &mut semver_root, &os);
    let os_arch = format!("{os}-{arch}");
    process_semver_file(&mut output, &mut semver_root, &os_arch);
    if !target_env.is_empty() {
        let os_env = format!("{os}-{target_env}");
        process_semver_file(&mut output, &mut semver_root, &os_env);

        let os_env_arch = format!("{os}-{target_env}-{arch}");
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
        Err(err) => panic!("unexpected error opening file: {err}"),
    };
    let input = BufReader::new(input_file);

    writeln!(output, "// Source: {}.", path.display()).unwrap();
    output.write_all(b"use libc::{\n").unwrap();
    for line in input.lines() {
        let line = line.unwrap().into_bytes();
        match line.first() {
            // Ignore comments and empty lines.
            Some(b'#') | None => continue,
            _ => {
                output.write_all(b"    ").unwrap();
                output.write_all(&line).unwrap();
                output.write_all(b",\n").unwrap();
            }
        }
    }
    output.write_all(b"};\n\n").unwrap();
    path.pop();
}

fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");

    let hotfix_dir = src_hotfix_dir();
    if std::fs::exists(&hotfix_dir).unwrap() {
        std::fs::remove_dir_all(&hotfix_dir).unwrap();
    }

    // FIXME(ctest): ctest2 cannot parse `crate::` in paths, so replace them with `::`
    let re = regex::bytes::Regex::new(r"(?-u:\b)crate::").unwrap();
    copy_dir_hotfix(Path::new("../src"), &hotfix_dir, &re, b"::");

    do_cc();
    do_ctest();
    do_semver();
}

// FIXME(clippy): removing `replace` somehow fails the `Test tier1 (x86_64-pc-windows-msvc, windows-2022)` CI job
#[allow(clippy::only_used_in_recursion)]
fn copy_dir_hotfix(src: &Path, dst: &Path, regex: &regex::bytes::Regex, replace: &[u8]) {
    std::fs::create_dir(dst).unwrap();
    for entry in src.read_dir().unwrap() {
        let entry = entry.unwrap();
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir_hotfix(&src_path, &dst_path, regex, replace);
        } else {
            // Replace "crate::" with "::"
            let src_data = std::fs::read(&src_path).unwrap();
            let dst_data = regex.replace_all(&src_data, b"::");
            std::fs::write(&dst_path, &dst_data).unwrap();
        }
    }
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
        "copyfile.h",
        "crt_externs.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "fnmatch.h",
        "getopt.h",
        "glob.h",
        "grp.h",
        "iconv.h",
        "ifaddrs.h",
        "langinfo.h",
        "libgen.h",
        "libproc.h",
        "limits.h",
        "locale.h",
        "mach-o/dyld.h",
        "mach/mach_init.h",
        "mach/mach.h",
        "mach/mach_time.h",
        "mach/mach_types.h",
        "mach/mach_vm.h",
        "mach/thread_act.h",
        "mach/thread_policy.h",
        "malloc/malloc.h",
        "net/bpf.h",
        "net/dlil.h",
        "net/if.h",
        "net/if_arp.h",
        "net/if_dl.h",
        "net/if_mib.h",
        "net/if_utun.h",
        "net/if_var.h",
        "net/ndrv.h",
        "net/route.h",
        "netdb.h",
        "netinet/if_ether.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "netinet6/in6_var.h",
        "os/clock.h",
        "os/lock.h",
        "os/signpost.h",
        // FIXME(macos): Requires the macOS 14.4 SDK.
        //"os/os_sync_wait_on_address.h",
        "poll.h",
        "pthread.h",
        "pthread_spis.h",
        "pthread/introspection.h",
        "pthread/spawn.h",
        "pthread/stack_np.h",
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
        "sysdir.h",
        "sys/appleapiopts.h",
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
        "sys/random.h",
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
        "sys/vsock.h",
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
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(union): actually a union
            "sigval" => true,

            // FIXME(macos): The size is changed in recent macOSes.
            "malloc_zone_t" => true,
            // it is a moving target, changing through versions
            // also contains bitfields members
            "tcp_connection_info" => true,
            // FIXME(macos): The size is changed in recent macOSes.
            "malloc_introspection_t" => true,

            _ => false,
        }
    });

    cfg.skip_type(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(macos): Requires the macOS 14.4 SDK.
            "os_sync_wake_by_address_flags_t" | "os_sync_wait_on_address_flags_t" => true,

            // FIXME(macos): "'__uint128' undeclared" in C
            "__uint128" => true,

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
            // FIXME(macos): the value has been changed since Catalina (0xffff0000 -> 0x3fff0000).
            "SF_SETTABLE" => true,

            // FIXME(macos): XCode 13.1 doesn't have it.
            "TIOCREMOTE" => true,

            // FIXME(macos): Requires the macOS 14.4 SDK.
            "OS_SYNC_WAKE_BY_ADDRESS_NONE"
            | "OS_SYNC_WAKE_BY_ADDRESS_SHARED"
            | "OS_SYNC_WAIT_ON_ADDRESS_NONE"
            | "OS_SYNC_WAIT_ON_ADDRESS_SHARED" => true,

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

            // FIXME(1.0): std removed libresolv support: https://github.com/rust-lang/rust/pull/102766
            "res_init" => true,

            // FIXME(macos): remove once the target in CI is updated
            "pthread_jit_write_freeze_callbacks_np" => true,

            // FIXME(macos): ABI has been changed on recent macOSes.
            "os_unfair_lock_assert_owner" | "os_unfair_lock_assert_not_owner" => true,

            // FIXME(macos): Once the SDK get updated to Ventura's level
            "freadlink" | "mknodat" | "mkfifoat" => true,

            // FIXME(macos): Requires the macOS 14.4 SDK.
            "os_sync_wake_by_address_any"
            | "os_sync_wake_by_address_all"
            | "os_sync_wake_by_address_flags_t"
            | "os_sync_wait_on_address"
            | "os_sync_wait_on_address_flags_t"
            | "os_sync_wait_on_address_with_deadline"
            | "os_sync_wait_on_address_with_timeout" => true,

            _ => false,
        }
    });

    cfg.skip_field(move |struct_, field| {
        match (struct_, field) {
            // FIXME(macos): the array size has been changed since macOS 10.15 ([8] -> [7]).
            ("statfs", "f_reserved") => true,
            ("__darwin_arm_neon_state64", "__v") => true,
            // MAXPATHLEN is too big for auto-derive traits on arrays.
            ("vnode_info_path", "vip_path") => true,
            ("ifreq", "ifr_ifru") => true,
            ("in6_ifreq", "ifr_ifru") => true,
            ("ifkpi", "ifk_data") => true,
            ("ifconf", "ifc_ifcu") => true,
            // FIXME: this field has been incorporated into a resized `rmx_filler` array.
            ("rt_metrics", "rmx_state") => true,
            ("rt_metrics", "rmx_filler") => true,
            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        match (struct_, field) {
            // FIXME(union): actually a union
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

            t if is_union => format!("union {t}"),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {t}"),
            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", "espec.tv_nsec")
            }
            // FIXME(macos): sigaction actually contains a union with two variants:
            // a sa_sigaction with type: (*)(int, struct __siginfo *, void *)
            // a sa_handler with type sig_t
            "sa_sigaction" if struct_ == "sigaction" => "sa_handler".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME(macos): this type has the wrong ABI
        "max_align_t" if i686 => true,
        // Can't return an array from a C function.
        "uuid_t" | "vol_capabilities_set_t" => true,
        _ => false,
    });
    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_openbsd(target: &str) {
    assert!(target.contains("openbsd"));

    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");

    let x86_64 = target.contains("x86_64");

    headers! { cfg:
        "elf.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "fnmatch.h",
        "getopt.h",
        "libgen.h",
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
        [x86_64]:"machine/fpu.h",
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
        "search.h",
        "spawn.h",
        "signal.h",
        "string.h",
        "sys/file.h",
        "sys/futex.h",
        "sys/ioctl.h",
        "sys/ipc.h",
        "sys/mman.h",
        "sys/param.h",
        "sys/resource.h",
        "sys/shm.h",
        "sys/socket.h",
        "sys/time.h",
        "sys/uio.h",
        "sys/ktrace.h",
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
        "sys/reboot.h",
        "sys/syscall.h",
        "sys/shm.h",
        "sys/param.h",
    }

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(union): actually a union
            "sigval" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // Removed in OpenBSD 6.0
            "KERN_USERMOUNT" | "KERN_ARND" => true,
            // Removed in OpenBSD 7.2
            "KERN_NSELCOLL" => true,
            // Good chance it's going to be wrong depending on the host release
            "KERN_MAXID" | "NET_RT_MAXID" => true,
            "EV_SYSFLAGS" => true,

            // Removed in OpenBSD 7.7
            "ATF_COM" | "ATF_PERM" | "ATF_PUBL" | "ATF_USETRAILERS" => true,

            // Removed in OpenBSD 7.8
            "CTL_FS" | "SO_NETPROC" => true,

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

            // futex() has volatile arguments, but that doesn't exist in Rust.
            "futex" => true,

            // Available for openBSD 7.3
            "mimmutable" => true,

            // Removed in OpenBSD 7.5
            // https://marc.info/?l=openbsd-cvs&m=170239504300386
            "syscall" => true,

            _ => false,
        }
    });

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "DIR" | "Dl_info" | "Elf32_Phdr" | "Elf64_Phdr" => ty.to_string(),

            // OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {t}"),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {t}"),
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
            // ifr_ifru is defined is an union
            ("ifreq", "ifr_ifru") => true,
            _ => false,
        }
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_cygwin(target: &str) {
    assert!(target.contains("cygwin"));

    let mut cfg = ctest_cfg();
    cfg.define("_GNU_SOURCE", None);

    headers! { cfg:
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "errno.h",
        "fcntl.h",
        "fnmatch.h",
        "grp.h",
        "iconv.h",
        "langinfo.h",
        "limits.h",
        "locale.h",
        "net/if.h",
        "netdb.h",
        "netinet/tcp.h",
        "poll.h",
        "pthread.h",
        "pty.h",
        "pwd.h",
        "resolv.h",
        "sched.h",
        "semaphore.h",
        "signal.h",
        "spawn.h",
        "stddef.h",
        "stdlib.h",
        "string.h",
        "sys/cpuset.h",
        "sys/ioctl.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/param.h",
        "sys/quota.h",
        "sys/random.h",
        "sys/resource.h",
        "sys/select.h",
        "sys/socket.h",
        "sys/statvfs.h",
        "sys/times.h",
        "sys/types.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/vfs.h",
        "syslog.h",
        "termios.h",
        "unistd.h",
        "utime.h",
        "wait.h",
        "wchar.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "DIR" | "Dl_info" | "fd_set" => ty.to_string(),

            "Ioctl" => "int".to_string(),

            t if is_union => format!("union {t}"),

            t if t.ends_with("_t") => t.to_string(),

            // sigval is a struct in Rust, but a union in C:
            "sigval" => "union sigval".to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

            t => t.to_string(),
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME(cygwin): these constants do not exist on Cygwin
            "ARPOP_REQUEST" | "ARPOP_REPLY" | "ATF_COM" | "ATF_PERM" | "ATF_PUBL"
            | "ATF_USETRAILERS" => true,

            // not defined on Cygwin, but [get|set]priority is, so they are
            // useful
            "PRIO_MIN" | "PRIO_MAX" => true,

            // The following does not exist on Cygwin but is required by
            // several crates
            "FIOCLEX" | "SA_NOCLDWAIT" => true,

            _ => false,
        }
    });

    cfg.skip_signededness(move |c| match c {
        n if n.starts_with("pthread") => true,

        // For consistency with other platforms. Actually a function ptr.
        "sighandler_t" => true,

        _ => false,
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }

        false
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => {
                s.replace("e_nsec", ".tv_nsec")
            }

            // FIXME(cygwin): sigaction actually contains a union with two variants:
            // a sa_sigaction with type: (*)(int, struct __siginfo *, void *)
            // a sa_handler with type sig_t
            "sa_sigaction" if struct_ == "sigaction" => "sa_handler".to_string(),

            s => s.to_string(),
        }
    });

    cfg.skip_field(|struct_, field| {
        match (struct_, field) {
            // this is actually a union on linux, so we can't represent it well and
            // just insert some padding.
            ("ifreq", "ifr_ifru") => true,
            ("ifconf", "ifc_ifcu") => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
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

            // FIXME(cygwin): does not exist on Cygwin
            "mlockall" | "munlockall" => true,

            _ => false,
        }
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_windows(target: &str) {
    assert!(target.contains("windows"));
    let gnu = target.contains("gnu");
    let i686 = target.contains("i686");

    let mut cfg = ctest_cfg();
    if target.contains("msvc") {
        cfg.flag("/wd4324");
    }
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

            // FIXME(windows): these don't exist:
            "time64_t" => "__time64_t".to_string(),
            "ssize_t" => "SSIZE_T".to_string(),

            "sighandler_t" if !gnu => "_crt_signal_t".to_string(),
            "sighandler_t" if gnu => "__p_sig_fn_t".to_string(),

            t if is_union => format!("union {t}"),
            t if t.ends_with("_t") => t.to_string(),

            // Windows uppercase structs don't have `struct` in front:
            t if is_struct => {
                if ty.chars().next().unwrap().is_uppercase() {
                    t.to_string()
                } else if t == "stat" {
                    "struct __stat64".to_string()
                } else if t == "utimbuf" {
                    "struct __utimbuf64".to_string()
                } else {
                    // put `struct` in front of all structs:
                    format!("struct {t}")
                }
            }
            t => t.to_string(),
        }
    });

    cfg.fn_cname(move |name, cname| cname.unwrap_or(name).to_string());

    cfg.skip_type(move |name| match name {
        "SSIZE_T" if !gnu => true,
        "ssize_t" if !gnu => true,
        // FIXME(windows): The size and alignment of this type are incorrect
        "time_t" if gnu && i686 => true,
        _ => false,
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(windows): The size and alignment of this struct are incorrect
            "timespec" if gnu && i686 => true,
            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME(windows): API error:
            // SIG_ERR type is "void (*)(int)", not "int"
            "SIG_ERR" |
            // Similar for SIG_DFL/IGN/GET/SGE/ACK
            "SIG_DFL" | "SIG_IGN" | "SIG_GET" | "SIG_SGE" | "SIG_ACK" => true,
            // FIXME(windows): newer windows-gnu environment on CI?
            "_O_OBTAIN_DIR" if gnu => true,
            _ => false,
        }
    });

    cfg.skip_field(move |s, field| match s {
        "CONTEXT" if field == "Fp" => true,
        _ => false,
    });
    // FIXME(windows): All functions point to the wrong addresses?
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

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
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
        "fnmatch.h",
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

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
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

    // FIXME(solaris): This should be removed once new Nix crate is released.
    // See comment in src/unix/solarish/solaris.rs for these.
    if is_solaris {
        cfg.define("O_DIRECT", Some("0x2000000"));
        cfg.define("SIGINFO", Some("41"));
    }

    headers! {
        cfg:
        "aio.h",
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "door.h",
        "errno.h",
        "execinfo.h",
        "fcntl.h",
        "fnmatch.h",
        "getopt.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "langinfo.h",
        "limits.h",
        "link.h",
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
        "spawn.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "sys/auxv.h",
        "sys/file.h",
        "sys/filio.h",
        "sys/ioctl.h",
        "sys/lgrp_user.h",
        "sys/loadavg.h",
        "sys/mkdev.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/priv.h",
        "sys/pset.h",
        "sys/random.h",
        "sys/resource.h",
        "sys/sendfile.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/stropts.h",
        "sys/shm.h",
        "sys/systeminfo.h",
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
        "thread.h",
        "time.h",
        "priv.h",
        "ucontext.h",
        "unistd.h",
        "utime.h",
        "utmpx.h",
        "wchar.h",
    }

    if is_illumos {
        headers! { cfg:
            "sys/epoll.h",
            "sys/eventfd.h",
        }
    }

    if is_solaris {
        headers! { cfg:
            "sys/lgrp_user_impl.h",
        }
    }

    cfg.skip_type(move |ty| match ty {
        "sighandler_t" => true,
        _ => false,
    });

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "FILE" => "__FILE".to_string(),
        "DIR" | "Dl_info" => ty.to_string(),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {t}"),
        t if is_union => format!("union {t}"),
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
        "EPOLLEXCLUSIVE" if is_illumos => true,

        _ => false,
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
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

    cfg.skip_field_type(move |struct_, field| {
        // aio_buf is "volatile void*"
        struct_ == "aiocb" && field == "aio_buf"
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

            // anonymous union challenges
            "fpregset_t" if field == "fp_reg_set" => true,

            // The LX brand (integrated into some illumos distros) commandeered several of the
            // `uc_filler` fields to use for brand-specific state.
            "ucontext_t" if is_illumos && (field == "uc_filler" || field == "uc_brand_data") => {
                true
            }

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
            // FIXME(solarish): unskip these for next major release
            "setpriority" | "personality" => true,

            // signal is defined in terms of sighandler_t, so ignore
            "signal" => true,

            // Currently missing
            "cfmakeraw" | "cfsetspeed" => true,

            // const-ness issues
            "execv" | "execve" | "execvp" | "settimeofday" | "sethostname" => true,

            // FIXME(1.0): https://github.com/rust-lang/libc/issues/1272
            "fexecve" => true,

            // Solaris-different
            "getpwent_r" | "getgrent_r" | "updwtmpx" if is_illumos => true,
            "madvise" | "mprotect" if is_illumos => true,
            "door_call" | "door_return" | "door_create" if is_illumos => true,

            // The compat functions use these "native" functions linked to their
            // non-prefixed implementations in libc.
            "native_getpwent_r" | "native_getgrent_r" => true,

            // Not visible when build with _XOPEN_SOURCE=700
            "mmapobj" | "mmap64" | "meminfo" | "getpagesizes" | "getpagesizes2" => true,

            // These functions may return int or void depending on the exact
            // configuration of the compilation environment, but the return
            // value is not useful (always 0) so we can ignore it:
            "setservent" | "endservent" => true,

            // Following illumos#3729, getifaddrs was changed to a
            // redefine_extname symbol in order to preserve compatibility.
            // Until better symbol binding story is figured out, it must be
            // excluded from the tests.
            "getifaddrs" if is_illumos => true,

            // FIXME(ctest): Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using `&`/`&mut` here, see:
            // https://github.com/gnzlbg/ctest/issues/68
            "lio_listio" => true,

            // Exists on illumos too but, for now, is
            // [a recent addition](https://www.illumos.org/issues/17094).
            "secure_getenv" if is_illumos => true,

            _ => false,
        }
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_netbsd(target: &str) {
    assert!(target.contains("netbsd"));
    let mut cfg = ctest_cfg();

    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("_NETBSD_SOURCE", Some("1"));

    headers! {
        cfg:
        "elf.h",
        "errno.h",
        "fcntl.h",
        "fnmatch.h",
        "getopt.h",
        "libgen.h",
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
        "sys/exec_elf.h",
        "sys/xattr.h",
        "sys/extattr.h",
        "sys/file.h",
        "sys/ioctl.h",
        "sys/ioctl_compat.h",
        "sys/ipc.h",
        "sys/ktrace.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/ptrace.h",
        "sys/resource.h",
        "sys/shm.h",
        "sys/socket.h",
        "sys/statvfs.h",
        "sys/sysctl.h",
        "sys/time.h",
        "sys/times.h",
        "sys/timex.h",
        "sys/ucontext.h",
        "sys/ucred.h",
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
        "sys/reboot.h",
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

            t if is_union => format!("union {t}"),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

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
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(netbsd): sighandler_t is crazy across platforms
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
            "GRND_RANDOM" | "GRND_INSECURE" | "GRND_NONBLOCK" => true, // netbsd 10 minimum

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        #[expect(clippy::wildcard_in_or_patterns)]
        match name {
            // FIXME(netbsd): https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" => true,
            // FIXME: netbsd 10 minimum
            "getentropy" | "getrandom" => true,

            "getrlimit" | "getrlimit64" |    // non-int in 1st arg
            "setrlimit" | "setrlimit64" |    // non-int in 1st arg
            "prlimit" | "prlimit64" |        // non-int in 2nd arg

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

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
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
        "fnmatch.h",
        "getopt.h",
        "glob.h",
        "grp.h",
        "ifaddrs.h",
        "kenv.h",
        "kvm.h",
        "langinfo.h",
        "libgen.h",
        "limits.h",
        "link.h",
        "locale.h",
        "mqueue.h",
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
        "sys/cpuctl.h",
        "sys/eui64.h",
        "sys/ipc.h",
        "sys/kinfo.h",
        "sys/ktrace.h",
        "sys/malloc.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/procctl.h",
        "sys/ptrace.h",
        "sys/reboot.h",
        "sys/resource.h",
        "sys/rtprio.h",
        "sys/sched.h",
        "sys/shm.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/sysctl.h",
        "sys/time.h",
        "sys/times.h",
        "sys/timex.h",
        "sys/types.h",
        "sys/checkpoint.h",
        "sys/uio.h",
        "sys/un.h",
        "sys/utsname.h",
        "sys/wait.h",
        "syslog.h",
        "termios.h",
        "time.h",
        "ucontext.h",
        "unistd.h",
        "util.h",
        "utime.h",
        "utmpx.h",
        "vfs/ufs/quota.h",
        "vm/vm_map.h",
        "wchar.h",
        "iconv.h",
    }

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr" | "Elf64_Phdr" | "Elf32_Shdr"
            | "Elf64_Shdr" | "Elf32_Sym" | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr"
            | "Elf32_Chdr" | "Elf64_Chdr" => ty.to_string(),

            // FIXME(dragonflybsd): OSX calls this something else
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {t}"),

            t if t.ends_with("_t") => t.to_string(),

            // sigval is a struct in Rust, but a union in C:
            "sigval" => "union sigval".to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

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
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(dragonflybsd): These are tested as part of the linux_fcntl tests since
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
            "execv" | "execve" | "execvp" | "fexecve" => true,

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

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_wasi(target: &str) {
    assert!(target.contains("wasi"));
    let p2 = target.contains("wasip2");

    let mut cfg = ctest_cfg();
    cfg.define("_GNU_SOURCE", None);

    headers! { cfg:
        "ctype.h",
        "dirent.h",
        "errno.h",
        "fcntl.h",
        "fnmatch.h",
        "langinfo.h",
        "limits.h",
        "locale.h",
        "malloc.h",
        [p2]: "netdb.h",
        [p2]: "netinet/in.h",
        [p2]: "netinet/tcp.h",
        "poll.h",
        "sched.h",
        "stdbool.h",
        "stddef.h",
        "stdint.h",
        "stdio.h",
        "stdlib.h",
        "string.h",
        "sys/ioctl.h",
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
        "wasi/api.h",
        "wasi/libc-find-relpath.h",
        "wasi/libc-nocwd.h",
        "wasi/libc.h",
        "wchar.h",
    }

    // Currently `ctest2` doesn't support macros-in-static-expressions and will
    // panic on them. That affects `CLOCK_*` defines in wasi to set this here
    // to omit them.
    cfg.cfg("libc_ctest", None);

    // `ctest2` has a hard-coded list of default cfgs which doesn't include
    // wasip2, which is why it has to be set here manually.
    if p2 {
        cfg.cfg("target_env", Some("p2"));
    }

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "FILE" | "fd_set" | "DIR" => ty.to_string(),
        t if is_union => format!("union {t}"),
        t if t.starts_with("__wasi") && t.ends_with("_u") => format!("union {t}"),
        t if t.starts_with("__wasi") && is_struct => format!("struct {t}"),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {t}"),
        t => t.to_string(),
    });

    cfg.field_name(move |_struct, field| {
        match field {
            // deal with fields as rust keywords
            "type_" => "type".to_string(),
            s => s.to_string(),
        }
    });

    // These have a different and internal type in header files and are only
    // used here to generate a pointer to them in bindings so skip these tests.
    cfg.skip_static(|c| c.starts_with("_CLOCK_"));

    cfg.skip_const(|c| match c {
        // These constants aren't yet defined in wasi-libc.
        // Exposing them is being tracked by https://github.com/WebAssembly/wasi-libc/issues/531.
        "SO_BROADCAST" | "SO_LINGER" => true,

        _ => false,
    });

    cfg.skip_fn(|f| match f {
        // This function doesn't actually exist in libc's header files
        "__errno_location" => true,

        // The `timeout` argument to this function is `*const` in Rust but
        // mutable in C which causes a mismatch. Avoiding breakage by changing
        // this in wasi-libc and instead accepting that this is slightly
        // different.
        "select" => true,

        _ => false,
    });

    // d_name is declared as a flexible array in WASI libc, so it
    // doesn't support sizeof.
    cfg.skip_field(|s, field| s == "dirent" && field == "d_name");

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_android(target: &str) {
    assert!(target.contains("android"));
    let target_pointer_width = match target {
        t if t.contains("aarch64") || t.contains("x86_64") => 64,
        t if t.contains("i686") || t.contains("arm") => 32,
        t => panic!("unsupported target: {t}"),
    };
    let x86 = target.contains("i686") || target.contains("x86_64");
    let aarch64 = target.contains("aarch64");

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
               "fnmatch.h",
               "getopt.h",
               "grp.h",
               "ifaddrs.h",
               "libgen.h",
               "limits.h",
               "link.h",
               "linux/sysctl.h",
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
               "spawn.h",
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
               "sys/klog.h",
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
               "sys/user.h",
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
                "linux/filter.h",
                "linux/futex.h",
                "linux/fs.h",
                "linux/genetlink.h",
                "linux/if_alg.h",
                "linux/if_addr.h",
                "linux/if_ether.h",
                "linux/if_link.h",
                "linux/rtnetlink.h",
                "linux/if_tun.h",
                "linux/kexec.h",
                "linux/magic.h",
                "linux/membarrier.h",
                "linux/memfd.h",
                "linux/mempolicy.h",
                "linux/module.h",
                "linux/mount.h",
                "linux/net_tstamp.h",
                "linux/netfilter/nfnetlink.h",
                "linux/netfilter/nfnetlink_log.h",
                "linux/netfilter/nfnetlink_queue.h",
                "linux/netfilter/nf_tables.h",
                "linux/netfilter_arp.h",
                "linux/netfilter_bridge.h",
                "linux/netfilter_ipv4.h",
                "linux/netfilter_ipv6.h",
                "linux/netfilter_ipv6/ip6_tables.h",
                "linux/netlink.h",
                "linux/quota.h",
                "linux/reboot.h",
                "linux/seccomp.h",
                "linux/sched.h",
                "linux/sockios.h",
                "linux/uinput.h",
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

            t if is_union => format!("union {t}"),

            t if t.ends_with("_t") => t.to_string(),

            // sigval is a struct in Rust, but a union in C:
            "sigval" => "union sigval".to_string(),

            "Ioctl" => "int".to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |struct_, field| {
        match field {
            // Our stat *_nsec fields normally don't actually exist but are part
            // of a timeval struct
            s if s.ends_with("_nsec") && struct_.starts_with("stat") => s.to_string(),
            // FIXME(union): appears that `epoll_event.data` is an union
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
            // FIXME(android): `sighandler_t` type is incorrect, see:
            // https://github.com/rust-lang/libc/issues/1359
            "sighandler_t" => true,

            // These are tested in the `linux_elf.rs` file.
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            // These are intended to be opaque
            "posix_spawn_file_actions_t" => true,
            "posix_spawnattr_t" => true,

            // FIXME(android): "'__uint128' undeclared" in C
            "__uint128" => true,
            // Added in API level 24
            "if_nameindex" => true,

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
            // 'private' type
            "prop_info" => true,

            // These are tested in the `linux_elf.rs` file.
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            // FIXME(android): The type of `iv` has been changed.
            "af_alg_iv" => true,

            // FIXME(android): The size of struct has been changed:
            "inotify_event" => true,
            // FIXME(android): The field has been changed:
            "sockaddr_vm" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
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

            // The `ARPHRD_CAN` is tested in the `linux_if_arp.rs` tests:
            "ARPHRD_CAN" => true,

            // FIXME(deprecated): deprecated: not available in any header
            // See: https://github.com/rust-lang/libc/issues/1356
            "ENOATTR" => true,

            // FIXME(android): still necessary?
            "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true, // sighandler_t weirdness
            // FIXME(deprecated): deprecated - removed in glibc 2.26
            "SIGUNUSED" => true,

            // Needs a newer Android SDK for the definition
            "P_PIDFD" => true,

            // Requires Linux kernel 5.6
            "VMADDR_CID_LOCAL" => true,

            // FIXME(android): conflicts with standard C headers and is tested in
            // `linux_termios.rs` below:
            "BOTHER" => true,
            "IBSHIFT" => true,
            "TCGETS2" | "TCSETS2" | "TCSETSW2" | "TCSETSF2" => true,

            // is a private value for kernel usage normally
            "FUSE_SUPER_MAGIC" => true,
            // linux 5.12 min
            "MPOL_F_NUMA_BALANCING" => true,

            // GRND_INSECURE was added in platform-tools-30.0.0
            "GRND_INSECURE" => true,

            // kernel 5.10 minimum required
            "MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_RSEQ" | "MEMBARRIER_CMD_PRIVATE_EXPEDITED_RSEQ" => true,

            // kernel 5.18 minimum
            | "MADV_COLD"
            | "MADV_DONTNEED_LOCKED"
            | "MADV_PAGEOUT"
            | "MADV_POPULATE_READ"
            | "MADV_POPULATE_WRITE" => true,

            // kernel 5.6 minimum required
            "IPPROTO_MPTCP" | "IPPROTO_ETHERNET" => true,

            // kernel 6.2 minimum
            "TUN_F_USO4" | "TUN_F_USO6" | "IFF_NO_CARRIER" => true,

            // FIXME(android): NDK r22 minimum required
            | "FDB_NOTIFY_BIT"
            | "FDB_NOTIFY_INACTIVE_BIT"
            | "IFLA_ALT_IFNAME"
            | "IFLA_PERM_ADDRESS"
            | "IFLA_PROP_LIST"
            | "IFLA_PROTO_DOWN_REASON"
            | "NDA_FDB_EXT_ATTRS"
            | "NDA_NH_ID"
            | "NFEA_ACTIVITY_NOTIFY"
            | "NFEA_DONT_REFRESH"
            | "NFEA_UNSPEC" => true,

            // FIXME(android): NDK r23 minimum required
            | "IFLA_PARENT_DEV_BUS_NAME"
            | "IFLA_PARENT_DEV_NAME" => true,

            // FIXME(android): NDK r25 minimum required
            | "IFLA_GRO_MAX_SIZE"
            | "NDA_FLAGS_EXT"
            | "NTF_EXT_MANAGED" => true,

            // FIXME(android): NDK above r25 required
            | "IFLA_ALLMULTI"
            | "IFLA_DEVLINK_PORT"
            | "IFLA_GRO_IPV4_MAX_SIZE"
            | "IFLA_GSO_IPV4_MAX_SIZE"
            | "IFLA_TSO_MAX_SEGS"
            | "IFLA_TSO_MAX_SIZE"
            | "NDA_NDM_STATE_MASK"
            | "NDA_NDM_FLAGS_MASK"
            | "NDTPA_INTERVAL_PROBE_TIME_MS"
            | "NFQA_UNSPEC"
            | "NTF_EXT_LOCKED"
            | "ALG_SET_DRBG_ENTROPY" => true,

            // FIXME(android): Something has been changed on r26b:
            | "IPPROTO_MAX"
            | "NFNL_SUBSYS_COUNT"
            | "NF_NETDEV_NUMHOOKS"
            | "NFT_MSG_MAX"
            | "SW_MAX"
            | "SW_CNT" => true,

            // FIXME(android): aarch64 env cannot find it:
            | "PTRACE_GETREGS"
            | "PTRACE_SETREGS" if aarch64 => true,
            // FIXME(android): The value has been changed on r26b:
            | "SYS_syscalls" if aarch64 => true,

            // From `<include/linux/sched.h>`.
            | "PF_VCPU"
            | "PF_IDLE"
            | "PF_EXITING"
            | "PF_POSTCOREDUMP"
            | "PF_IO_WORKER"
            | "PF_WQ_WORKER"
            | "PF_FORKNOEXEC"
            | "PF_MCE_PROCESS"
            | "PF_SUPERPRIV"
            | "PF_DUMPCORE"
            | "PF_SIGNALED"
            | "PF_MEMALLOC"
            | "PF_NPROC_EXCEEDED"
            | "PF_USED_MATH"
            | "PF_USER_WORKER"
            | "PF_NOFREEZE"
            | "PF_KSWAPD"
            | "PF_MEMALLOC_NOFS"
            | "PF_MEMALLOC_NOIO"
            | "PF_LOCAL_THROTTLE"
            | "PF_KTHREAD"
            | "PF_RANDOMIZE"
            | "PF_NO_SETAFFINITY"
            | "PF_MCE_EARLY"
            | "PF_MEMALLOC_PIN"
            | "PF_BLOCK_TS"
            | "PF_SUSPEND_TASK" => true,

            // FIXME(android): Requires >= 6.12 kernel headers.
            "SOF_TIMESTAMPING_OPT_RX_FILTER" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME(android): https://github.com/rust-lang/libc/issues/1272
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
            "reallocarray" => true,
            "__system_property_wait" => true,

            // Added in API level 30, but tests use level 28.
            "memfd_create" | "mlock2" | "renameat2" | "statx" | "statx_timestamp" => true,

            // Added in glibc 2.25.
            "getentropy" => true,

            // Added in API level 28, but some tests use level 24.
            "getrandom" => true,

            // Added in API level 28, but some tests use level 24.
            "syncfs" => true,

            // Added in API level 28, but some tests use level 24.
            "pthread_attr_getinheritsched" | "pthread_attr_setinheritsched" => true,
            // Added in API level 28, but some tests use level 24.
            "fread_unlocked" | "fwrite_unlocked" | "fgets_unlocked" | "fflush_unlocked" => true,

            // Added in API level 28, but some tests use level 24.
            "aligned_alloc" => true,

            // Added in API level 26, but some tests use level 24.
            "getgrent" => true,

            // Added in API level 26, but some tests use level 24.
            "setgrent" => true,

            // Added in API level 26, but some tests use level 24.
            "endgrent" => true,

            // Added in API level 26, but some tests use level 24.
            "getdomainname" | "setdomainname" => true,

            // FIXME(android): bad function pointers:
            "isalnum" | "isalpha" | "iscntrl" | "isdigit" | "isgraph" | "islower" | "isprint"
            | "ispunct" | "isspace" | "isupper" | "isxdigit" | "isblank" | "tolower"
            | "toupper" => true,

            // Added in API level 24
            "if_nameindex" | "if_freenameindex" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value") ||
        // this one is an anonymous union
        (struct_ == "ff_effect" && field == "u") ||
        // FIXME(android): `sa_sigaction` has type `sighandler_t` but that type is
        // incorrect, see: https://github.com/rust-lang/libc/issues/1359
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // signalfd had SIGSYS fields added in Android 4.19, but CI does not have that version yet.
        (struct_ == "signalfd_siginfo" && field == "ssi_call_addr") ||
        // FIXME(android): Seems the type has been changed on NDK r26b
        (struct_ == "flock64" && (field == "l_start" || field == "l_len"))
    });

    cfg.skip_field(|struct_, field| {
        match (struct_, field) {
            // conflicting with `p_type` macro from <resolve.h>.
            ("Elf32_Phdr", "p_type") => true,
            ("Elf64_Phdr", "p_type") => true,

            // this is actually a union on linux, so we can't represent it well and
            // just insert some padding.
            ("siginfo_t", "_pad") => true,
            ("ifreq", "ifr_ifru") => true,
            ("ifconf", "ifc_ifcu") => true,

            _ => false,
        }
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");

    test_linux_like_apis(target);
}

fn test_freebsd(target: &str) {
    assert!(target.contains("freebsd"));
    let mut cfg = ctest_cfg();

    let freebsd_ver = which_freebsd();

    match freebsd_ver {
        Some(12) => cfg.cfg("freebsd12", None),
        Some(13) => cfg.cfg("freebsd13", None),
        Some(14) => cfg.cfg("freebsd14", None),
        Some(15) => cfg.cfg("freebsd15", None),
        _ => &mut cfg,
    };

    // For sched linux compat fn
    cfg.define("_WITH_CPU_SET_T", None);
    // Required for `getline`:
    cfg.define("_WITH_GETLINE", None);
    // Required for making freebsd11_stat available in the headers
    cfg.define("_WANT_FREEBSD11_STAT", None);

    let freebsd13 = matches!(freebsd_ver, Some(n) if n >= 13);
    let freebsd14 = matches!(freebsd_ver, Some(n) if n >= 14);
    let freebsd15 = matches!(freebsd_ver, Some(n) if n >= 15);

    headers! { cfg:
                "aio.h",
                "arpa/inet.h",
                "bsm/audit.h",
                "ctype.h",
                "dirent.h",
                "dlfcn.h",
                "elf.h",
                "errno.h",
                "execinfo.h",
                "fcntl.h",
                "fnmatch.h",
                "getopt.h",
                "glob.h",
                "grp.h",
                "iconv.h",
                "ifaddrs.h",
                "kenv.h",
                "langinfo.h",
                "libgen.h",
                "libutil.h",
                "limits.h",
                "link.h",
                "locale.h",
                "machine/elf.h",
                "machine/reg.h",
                "malloc_np.h",
                "memstat.h",
                "mqueue.h",
                "net/bpf.h",
                "net/if.h",
                "net/if_arp.h",
                "net/if_dl.h",
                "net/if_mib.h",
                "net/route.h",
                "netdb.h",
                "netinet/ip.h",
                "netinet/in.h",
                "netinet/sctp.h",
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
                "sys/auxv.h",
                "sys/cpuset.h",
                "sys/domainset.h",
                "sys/eui64.h",
                "sys/event.h",
                [freebsd13]:"sys/eventfd.h",
                "sys/extattr.h",
                "sys/file.h",
                "sys/ioctl.h",
                "sys/ipc.h",
                "sys/jail.h",
                "sys/mman.h",
                "sys/mount.h",
                "sys/msg.h",
                "sys/procctl.h",
                "sys/procdesc.h",
                "sys/ptrace.h",
                "sys/queue.h",
                "sys/random.h",
                "sys/reboot.h",
                "sys/resource.h",
                "sys/rtprio.h",
                "sys/sem.h",
                "sys/shm.h",
                "sys/socket.h",
                "sys/stat.h",
                "sys/statvfs.h",
                "sys/sysctl.h",
                "sys/thr.h",
                "sys/time.h",
                [freebsd14 || freebsd15]:"sys/timerfd.h",
                [freebsd13 || freebsd14 || freebsd15]:"dev/evdev/input.h",
                "sys/times.h",
                "sys/timex.h",
                "sys/types.h",
                "sys/proc.h",
                "kvm.h", // must be after "sys/types.h"
                "sys/ucontext.h",
                "sys/uio.h",
                "sys/ktrace.h",
                "sys/umtx.h",
                "sys/un.h",
                "sys/user.h",
                "sys/utsname.h",
                "sys/uuid.h",
                "sys/vmmeter.h",
                "sys/wait.h",
                "libprocstat.h",
                "devstat.h",
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
            "FILE"
            | "fd_set"
            | "Dl_info"
            | "DIR"
            | "Elf32_Phdr"
            | "Elf64_Phdr"
            | "Elf32_Auxinfo"
            | "Elf64_Auxinfo"
            | "devstat_select_mode"
            | "devstat_support_flags"
            | "devstat_type_flags"
            | "devstat_match_flags"
            | "devstat_priority" => ty.to_string(),

            // FIXME(freebsd): https://github.com/rust-lang/libc/issues/1273
            "sighandler_t" => "sig_t".to_string(),

            t if is_union => format!("union {t}"),

            t if t.ends_with("_t") => t.to_string(),

            // sigval is a struct in Rust, but a union in C:
            "sigval" => "union sigval".to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

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
            "type_" if struct_ == "sockstat" => "type".to_string(),
            "type_" if struct_ == "devstat_match_table" => "type".to_string(),
            "type_" if struct_ == "input_event" => "type".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // These constants were introduced in FreeBSD 13:
            "F_ADD_SEALS" | "F_GET_SEALS" | "F_SEAL_SEAL" | "F_SEAL_SHRINK" | "F_SEAL_GROW"
            | "F_SEAL_WRITE"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // These constants were introduced in FreeBSD 13:
            "EFD_CLOEXEC" | "EFD_NONBLOCK" | "EFD_SEMAPHORE" if Some(13) > freebsd_ver => true,

            // These constants were introduced in FreeBSD 12:
            "AT_RESOLVE_BENEATH" | "O_RESOLVE_BENEATH" if Some(12) > freebsd_ver => true,

            // These constants were introduced in FreeBSD 13:
            "O_DSYNC" | "O_PATH" | "O_EMPTY_PATH" | "AT_EMPTY_PATH" if Some(13) > freebsd_ver => {
                true
            }

            // These aliases were introduced in FreeBSD 13:
            // (note however that the constants themselves work on any version)
            "CLOCK_BOOTTIME" | "CLOCK_REALTIME_COARSE" | "CLOCK_MONOTONIC_COARSE"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // FIXME(deprecated): These are deprecated - remove in a couple of releases.
            // These constants were removed in FreeBSD 11 (svn r273250) but will
            // still be accepted and ignored at runtime.
            "MAP_RENAME" | "MAP_NORESERVE" => true,

            // FIXME(deprecated): These are deprecated - remove in a couple of releases.
            // These constants were removed in FreeBSD 11 (svn r262489),
            // and they've never had any legitimate use outside of the
            // base system anyway.
            "CTL_MAXID" | "KERN_MAXID" | "HW_MAXID" | "USER_MAXID" => true,

            // Deprecated and removed in FreeBSD 15.  It was never actually implemented.
            "TCP_MAXPEAKRATE" => true,

            // FIXME: This is deprecated - remove in a couple of releases.
            // This was removed in FreeBSD 14 (git 1b4701fe1e8) and never
            // should've been used anywhere anyway.
            "TDF_UNUSED23" => true,

            // Removed in FreeBSD 15
            "TDF_CANSWAP" | "TDF_SWAPINREQ" => true,

            // Unaccessible in FreeBSD 15
            "TDI_SWAPPED" | "P_SWAPPINGOUT" | "P_SWAPPINGIN" => true,

            // Removed in FreeBSD 14 (git a6b55ee6be1)
            "IFF_KNOWSEPOCH" => true,

            // Removed in FreeBSD 14 (git 7ff9ae90f0b)
            "IFF_NOGROUP" => true,

            // FIXME(deprecated): These are deprecated - remove in a couple of releases.
            // These symbols are not stable across OS-versions.  They were
            // changed for FreeBSD 14 in git revisions b62848b0c3f and
            // 2cf7870864e.
            "PRI_MAX_ITHD" | "PRI_MIN_REALTIME" | "PRI_MAX_REALTIME" | "PRI_MIN_KERN"
            | "PRI_MAX_KERN" | "PSWP" | "PVM" | "PINOD" | "PRIBIO" | "PVFS" | "PZERO" | "PSOCK"
            | "PWAIT" | "PLOCK" | "PPAUSE" | "PRI_MIN_TIMESHARE" | "PUSER" | "PI_AV" | "PI_NET"
            | "PI_DISK" | "PI_TTY" | "PI_DULL" | "PI_SOFT" => true,

            // This constant changed in FreeBSD 15 (git 3458bbd397783).  It was never intended to
            // be stable, and probably shouldn't be bound by libc at all.
            "RLIM_NLIMITS" => true,

            // This symbol changed in FreeBSD 14 (git 051e7d78b03), but the new
            // version should be safe to use on older releases.
            "IFCAP_CANTCHANGE" => true,

            // These were removed in FreeBSD 14 (git c6d31b8306e)
            "TDF_ASTPENDING" | "TDF_NEEDSUSPCHK" | "TDF_NEEDRESCHED" | "TDF_NEEDSIGCHK"
            | "TDF_ALRMPEND" | "TDF_PROFPEND" | "TDF_MACPEND" => true,

            // This constant was removed in FreeBSD 13 (svn r363622), and never
            // had any legitimate use outside of the base system anyway.
            "CTL_P1003_1B_MAXID" => true,

            // This was renamed in FreeBSD 12.2 and 13 (r352486).
            "CTL_UNSPEC" | "CTL_SYSCTL" => true,

            // This was renamed in FreeBSD 12.2 and 13 (r350749).
            "IPPROTO_SEP" | "IPPROTO_DCCP" => true,

            // This was changed to 96(0x60) in FreeBSD 13:
            // https://github.com/freebsd/freebsd/
            // commit/06b00ceaa914a3907e4e27bad924f44612bae1d7
            "MINCORE_SUPER" if Some(13) <= freebsd_ver => true,

            // Added in FreeBSD 13.0 (r356667)
            "GRND_INSECURE" if Some(13) > freebsd_ver => true,

            // Added in FreeBSD 13.0 (r349609)
            "PROC_PROTMAX_CTL"
            | "PROC_PROTMAX_STATUS"
            | "PROC_PROTMAX_FORCE_ENABLE"
            | "PROC_PROTMAX_FORCE_DISABLE"
            | "PROC_PROTMAX_NOFORCE"
            | "PROC_PROTMAX_ACTIVE"
            | "PROC_NO_NEW_PRIVS_CTL"
            | "PROC_NO_NEW_PRIVS_STATUS"
            | "PROC_NO_NEW_PRIVS_ENABLE"
            | "PROC_NO_NEW_PRIVS_DISABLE"
            | "PROC_WXMAP_CTL"
            | "PROC_WXMAP_STATUS"
            | "PROC_WX_MAPPINGS_PERMIT"
            | "PROC_WX_MAPPINGS_DISALLOW_EXEC"
            | "PROC_WXORX_ENFORCE"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // Added in FreeBSD 13.0 (r367776 and r367287)
            "SCM_CREDS2" | "LOCAL_CREDS_PERSISTENT" if Some(13) > freebsd_ver => true,

            // Added in FreeBSD 14
            "SPACECTL_DEALLOC" if Some(14) > freebsd_ver => true,

            // Added in FreeBSD 13.
            "KERN_PROC_SIGFASTBLK"
            | "USER_LOCALBASE"
            | "TDP_SIGFASTBLOCK"
            | "TDP_UIOHELD"
            | "TDP_SIGFASTPENDING"
            | "TDP2_COMPAT32RB"
            | "P2_PROTMAX_ENABLE"
            | "P2_PROTMAX_DISABLE"
            | "CTLFLAG_NEEDGIANT"
            | "CTL_SYSCTL_NEXTNOSKIP"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // Added in freebsd 14.
            "IFCAP_MEXTPG" if Some(14) > freebsd_ver => true,
            // Added in freebsd 13.
            "IFCAP_TXTLS4" | "IFCAP_TXTLS6" | "IFCAP_VXLAN_HWCSUM" | "IFCAP_VXLAN_HWTSO"
            | "IFCAP_TXTLS_RTLMT" | "IFCAP_TXTLS"
                if Some(13) > freebsd_ver =>
            {
                true
            }
            // Added in FreeBSD 13.
            "PS_FST_TYPE_EVENTFD" if Some(13) > freebsd_ver => true,

            // Added in FreeBSD 14.
            "MNT_RECURSE" | "MNT_DEFERRED" if Some(14) > freebsd_ver => true,

            // Added in FreeBSD 13.
            "MNT_EXTLS" | "MNT_EXTLSCERT" | "MNT_EXTLSCERTUSER" | "MNT_NOCOVER"
            | "MNT_EMPTYDIR"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // Added in FreeBSD 14.
            "PT_COREDUMP" | "PC_ALL" | "PC_COMPRESS" | "PT_GETREGSET" | "PT_SETREGSET"
            | "PT_SC_REMOTE"
                if Some(14) > freebsd_ver =>
            {
                true
            }

            // Added in FreeBSD 14.
            "F_KINFO" => true, // FIXME(freebsd): depends how frequent freebsd 14 is updated on CI, this addition went this week only.
            "SHM_RENAME_NOREPLACE"
            | "SHM_RENAME_EXCHANGE"
            | "SHM_LARGEPAGE_ALLOC_DEFAULT"
            | "SHM_LARGEPAGE_ALLOC_NOWAIT"
            | "SHM_LARGEPAGE_ALLOC_HARD"
            | "MFD_CLOEXEC"
            | "MFD_ALLOW_SEALING"
            | "MFD_HUGETLB"
            | "MFD_HUGE_MASK"
            | "MFD_HUGE_64KB"
            | "MFD_HUGE_512KB"
            | "MFD_HUGE_1MB"
            | "MFD_HUGE_2MB"
            | "MFD_HUGE_8MB"
            | "MFD_HUGE_16MB"
            | "MFD_HUGE_32MB"
            | "MFD_HUGE_256MB"
            | "MFD_HUGE_512MB"
            | "MFD_HUGE_1GB"
            | "MFD_HUGE_2GB"
            | "MFD_HUGE_16GB"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // Flags introduced in FreeBSD 14.
            "TCP_MAXUNACKTIME"
            | "TCP_IDLE_REDUCE"
            | "TCP_REMOTE_UDP_ENCAPS_PORT"
            | "TCP_DELACK"
            | "TCP_FIN_IS_RST"
            | "TCP_LOG_LIMIT"
            | "TCP_SHARED_CWND_ALLOWED"
            | "TCP_PROC_ACCOUNTING"
            | "TCP_USE_CMP_ACKS"
            | "TCP_PERF_INFO"
            | "TCP_LRD"
                if Some(14) > freebsd_ver =>
            {
                true
            }

            // Introduced in FreeBSD 14 then removed ?
            "TCP_LRD" if freebsd_ver >= Some(15) => true,

            // Added in FreeBSD 14
            "LIO_READV" | "LIO_WRITEV" | "LIO_VECTORED" if Some(14) > freebsd_ver => true,

            // Added in FreeBSD 13
            "FIOSSHMLPGCNF" if Some(13) > freebsd_ver => true,

            // Added in FreeBSD 14
            "IFCAP_NV" if Some(14) > freebsd_ver => true,

            // FIXME(freebsd): Removed in https://reviews.freebsd.org/D38574 and https://reviews.freebsd.org/D38822
            // We maybe should deprecate them once a stable release ships them.
            "IP_BINDMULTI" | "IP_RSS_LISTEN_BUCKET" => true,

            // FIXME(freebsd): Removed in https://reviews.freebsd.org/D39127.
            "KERN_VNODE" => true,

            // Added in FreeBSD 14
            "EV_KEEPUDATA" if Some(14) > freebsd_ver => true,

            // Added in FreeBSD 13.2
            "AT_USRSTACKBASE" | "AT_USRSTACKLIM" if Some(13) > freebsd_ver => true,

            // Added in FreeBSD 14
            "TFD_CLOEXEC" | "TFD_NONBLOCK" | "TFD_TIMER_ABSTIME" | "TFD_TIMER_CANCEL_ON_SET"
                if Some(14) > freebsd_ver =>
            {
                true
            }

            // Added in FreeBSD 14.1
            "KCMP_FILE" | "KCMP_FILEOBJ" | "KCMP_FILES" | "KCMP_SIGHAND" | "KCMP_VM"
                if Some(14) > freebsd_ver =>
            {
                true
            }

            // FIXME(freebsd): Removed in FreeBSD 15:
            "LOCAL_CONNWAIT" if freebsd_ver >= Some(15) => true,

            // FIXME(freebsd): The values has been changed in FreeBSD 15:
            "CLOCK_BOOTTIME" if Some(15) <= freebsd_ver => true,

            // Added in FreeBSD 14.0
            "TCP_FUNCTION_ALIAS" if Some(14) > freebsd_ver => true,

            // These constants may change or disappear in future OS releases, and they probably
            // have no legitimate use in applications anyway.
            "CAP_UNUSED0_44" | "CAP_UNUSED0_57" | "CAP_UNUSED1_22" | "CAP_UNUSED1_57"
            | "CAP_ALL0" | "CAP_ALL1" => true,

            // FIXME(freebsd): Removed in FreeBSD 15, deprecated in libc
            "TCP_PCAP_OUT" | "TCP_PCAP_IN" => true,

            // Added in FreeBSD 14.2
            "SO_SPLICE" if Some(14) > freebsd_ver => true,

            _ => false,
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // the struct "__kvm" is quite tricky to bind so since we only use a pointer to it
            // for now, it doesn't matter too much...
            "kvm_t" => true,
            // `eventfd(2)` and things come with it are added in FreeBSD 13
            "eventfd_t" if Some(13) > freebsd_ver => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // `procstat` is a private struct
            "procstat" => true,

            // `spacectl_range` was introduced in FreeBSD 14
            "spacectl_range" if Some(14) > freebsd_ver => true,

            // `ptrace_coredump` introduced in FreeBSD 14.
            "ptrace_coredump" if Some(14) > freebsd_ver => true,
            // `ptrace_sc_remote` introduced in FreeBSD 14.
            "ptrace_sc_remote" if Some(14) > freebsd_ver => true,

            // `sockcred2` is not available in FreeBSD 12.
            "sockcred2" if Some(13) > freebsd_ver => true,
            // `shm_largepage_conf` was introduced in FreeBSD 13.
            "shm_largepage_conf" if Some(13) > freebsd_ver => true,

            // Those are private types
            "memory_type" => true,
            "memory_type_list" => true,
            "pidfh" => true,
            "sctp_gen_error_cause"
            | "sctp_error_missing_param"
            | "sctp_remote_error"
            | "sctp_assoc_change"
            | "sctp_send_failed_event"
            | "sctp_stream_reset_event" => true,

            // FIXME(freebsd): Changed in FreeBSD 15
            "tcp_info" | "sockstat" if Some(15) >= freebsd_ver => true,

            // `splice` introduced in FreeBSD 14.2
            "splice" if Some(14) > freebsd_ver => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            // Also, `execvpe` is introduced in FreeBSD 14.1
            "execv" | "execve" | "execvp" | "execvpe" | "fexecve" => true,

            // The `uname` function in the `utsname.h` FreeBSD header is a C
            // inline function (has no symbol) that calls the `__xuname` symbol.
            // Therefore the function pointer comparison does not make sense for it.
            "uname" => true,

            // FIXME(ctest): Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using `&`/`&mut` here, see:
            // https://github.com/gnzlbg/ctest/issues/68
            "lio_listio" => true,

            // Those are introduced in FreeBSD 12.
            "clock_nanosleep" | "getrandom" | "elf_aux_info" | "setproctitle_fast"
            | "timingsafe_bcmp" | "timingsafe_memcmp"
                if Some(12) > freebsd_ver =>
            {
                true
            }

            // Those are introduced in FreeBSD 13.
            "memfd_create"
            | "shm_create_largepage"
            | "shm_rename"
            | "getentropy"
            | "eventfd"
            | "SOCKCRED2SIZE"
            | "getlocalbase"
            | "aio_readv"
            | "aio_writev"
            | "copy_file_range"
            | "eventfd_read"
            | "eventfd_write"
                if Some(13) > freebsd_ver =>
            {
                true
            }

            // Those are introduced in FreeBSD 14.
            "sched_getaffinity" | "sched_setaffinity" | "sched_getcpu" | "fspacectl"
                if Some(14) > freebsd_ver =>
            {
                true
            }

            // Those are introduced in FreeBSD 14.
            "timerfd_create" | "timerfd_gettime" | "timerfd_settime" if Some(14) > freebsd_ver => {
                true
            }

            // Those are introduced in FreeBSD 14.1.
            "kcmp" => true,

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
            // FIXME(freebsd): `sa_sigaction` has type `sighandler_t` but that type is
            // incorrect, see: https://github.com/rust-lang/libc/issues/1359
            ("sigaction", "sa_sigaction") => true,

            // conflicting with `p_type` macro from <resolve.h>.
            ("Elf32_Phdr", "p_type") => true,
            ("Elf64_Phdr", "p_type") => true,

            // not available until FreeBSD 12, and is an anonymous union there.
            ("xucred", "cr_pid__c_anonymous_union") => true,

            // m_owner field is a volatile __lwpid_t
            ("umutex", "m_owner") => true,
            // c_has_waiters field is a volatile int32_t
            ("ucond", "c_has_waiters") => true,
            // is PATH_MAX long but tests can't accept multi array as equivalent.
            ("kinfo_vmentry", "kve_path") => true,

            // a_un field is a union
            ("Elf32_Auxinfo", "a_un") => true,
            ("Elf64_Auxinfo", "a_un") => true,

            // union fields
            ("if_data", "__ifi_epoch") => true,
            ("if_data", "__ifi_lastchange") => true,
            ("ifreq", "ifr_ifru") => true,
            ("ifconf", "ifc_ifcu") => true,

            // anonymous struct
            ("devstat", "dev_links") => true,

            // FIXME(freebsd): structs too complicated to bind for now...
            ("kinfo_proc", "ki_paddr") => true,
            ("kinfo_proc", "ki_addr") => true,
            ("kinfo_proc", "ki_tracep") => true,
            ("kinfo_proc", "ki_textvp") => true,
            ("kinfo_proc", "ki_fd") => true,
            ("kinfo_proc", "ki_vmspace") => true,
            ("kinfo_proc", "ki_pcb") => true,
            ("kinfo_proc", "ki_tdaddr") => true,
            ("kinfo_proc", "ki_pd") => true,

            // Anonymous type.
            ("filestat", "next") => true,

            // We ignore this field because we needed to use a hack in order to make rust 1.19
            // happy...
            ("kinfo_proc", "ki_sparestrings") => true,

            // `__sem_base` is a private struct field
            ("semid_ds", "__sem_base") => true,

            // `snap_time` is a `long double`, but it's a nightmare to bind correctly in rust
            // for the moment, so it's a best effort thing...
            ("statinfo", "snap_time") => true,
            ("sctp_sndrcvinfo", "__reserve_pad") => true,
            ("sctp_extrcvinfo", "__reserve_pad") => true,
            // `tcp_snd_wscale` and `tcp_rcv_wscale` are bitfields
            ("tcp_info", "tcp_snd_wscale") => true,
            ("tcp_info", "tcp_rcv_wscale") => true,

            _ => false,
        }
    });
    if target.contains("arm") {
        cfg.skip_roundtrip(move |s| match s {
            // Can't return an array from a C function.
            "__gregset_t" => true,
            _ => false,
        });
    }

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_emscripten(target: &str) {
    assert!(target.contains("emscripten"));

    let mut cfg = ctest_cfg();
    cfg.define("_GNU_SOURCE", None); // FIXME(emscripten): ??

    headers! { cfg:
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "errno.h",
               "fcntl.h",
               "fnmatch.h",
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
               "sys/file.h",
               "sys/ioctl.h",
               "sys/ipc.h",
               "sys/mman.h",
               "sys/mount.h",
               "sys/msg.h",
               "sys/resource.h",
               "sys/sem.h",
               "sys/shm.h",
               "sys/socket.h",
               "sys/stat.h",
               "sys/statvfs.h",
               "sys/syscall.h",
               "sys/sysinfo.h",
               "sys/time.h",
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

            // LFS64 types have been removed in Emscripten 3.1.44
            // https://github.com/emscripten-core/emscripten/pull/19812
            "off64_t" => "off_t".to_string(),

            // typedefs don't need any keywords
            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

            // put `union` in front of all unions:
            t if is_union => format!("union {t}"),

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
            // Rust struct uses raw u64, rather than union
            "u64" if struct_ == "epoll_event" => "data.u64".to_string(),
            s => s.to_string(),
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // sighandler_t is crazy across platforms
            // FIXME(emscripten): is this necessary?
            "sighandler_t" => true,

            // No epoll support
            // https://github.com/emscripten-core/emscripten/issues/5033
            ty if ty.starts_with("epoll") => true,

            // LFS64 types have been removed in Emscripten 3.1.44
            // https://github.com/emscripten-core/emscripten/pull/19812
            t => t.ends_with("64") || t.ends_with("64_t"),
        }
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // This is actually a union, not a struct
            "sigval" => true,

            // FIXME(emscripten): Investigate why the test fails.
            // Skip for now to unblock CI.
            "pthread_condattr_t" => true,
            "pthread_mutexattr_t" => true,

            // No epoll support
            // https://github.com/emscripten-core/emscripten/issues/5033
            ty if ty.starts_with("epoll") => true,
            ty if ty.starts_with("signalfd") => true,

            // LFS64 types have been removed in Emscripten 3.1.44
            // https://github.com/emscripten-core/emscripten/pull/19812
            ty => ty.ends_with("64") || ty.ends_with("64_t"),
        }
    });

    cfg.skip_fn(move |name| {
        match name {
            // Emscripten does not support fork/exec/wait or any kind of multi-process support
            // https://github.com/emscripten-core/emscripten/blob/3.1.68/tools/system_libs.py#L1100
            "execv" | "execve" | "execvp" | "execvpe" | "fexecve" | "wait4" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME(deprecated): deprecated - SIGNUNUSED was removed in glibc 2.26
            // users should use SIGSYS instead
            "SIGUNUSED" => true,

            // FIXME(emscripten): emscripten uses different constants to constructs these
            n if n.contains("__SIZEOF_PTHREAD") => true,

            // No epoll support
            // https://github.com/emscripten-core/emscripten/issues/5033
            n if n.starts_with("EPOLL") => true,

            // No ptrace.h
            // https://github.com/emscripten-core/emscripten/pull/17704
            n if n.starts_with("PTRACE_") => true,

            // No quota.h
            // https://github.com/emscripten-core/emscripten/pull/17704
            n if n.starts_with("QIF_") => true,
            "USRQUOTA" | "GRPQUOTA" | "Q_GETFMT" | "Q_GETINFO" | "Q_SETINFO" | "Q_SYNC"
            | "Q_QUOTAON" | "Q_QUOTAOFF" | "Q_GETQUOTA" | "Q_SETQUOTA" => true,

            // `SYS_gettid` was removed in Emscripten v1.39.9
            // https://github.com/emscripten-core/emscripten/pull/10439
            "SYS_gettid" => true,

            // No personality.h
            // https://github.com/emscripten-core/emscripten/pull/17704
            "ADDR_NO_RANDOMIZE" | "MMAP_PAGE_ZERO" | "ADDR_COMPAT_LAYOUT" | "READ_IMPLIES_EXEC"
            | "ADDR_LIMIT_32BIT" | "SHORT_INODE" | "WHOLE_SECONDS" | "STICKY_TIMEOUTS"
            | "ADDR_LIMIT_3GB" => true,

            // `SIG_IGN` has been changed to -2 since 1 is a valid function address
            // https://github.com/emscripten-core/emscripten/pull/14883
            "SIG_IGN" => true,

            // Constants present in other linuxes but not emscripten
            "SI_DETHREAD" | "TRAP_PERF" => true,

            // LFS64 types have been removed in Emscripten 3.1.44
            // https://github.com/emscripten-core/emscripten/pull/19812
            n if n.starts_with("RLIM64") => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // This is a weird union, don't check the type.
        (struct_ == "ifaddrs" && field == "ifa_ifu") ||
        // sighandler_t type is super weird
        (struct_ == "sigaction" && field == "sa_sigaction") ||
        // sigval is actually a union, but we pretend it's a struct
        (struct_ == "sigevent" && field == "sigev_value")
    });

    cfg.skip_field(move |struct_, field| {
        // this is actually a union on linux, so we can't represent it well and
        // just insert some padding.
        (struct_ == "siginfo_t" && field == "_pad") ||
        // musl names this __dummy1 but it's still there
        (struct_ == "glob_t" && field == "gl_flags") ||
        // FIXME(emscripten): After musl 1.1.24, it have only one field `sched_priority`,
        // while other fields become reserved.
        (struct_ == "sched_param" && [
            "sched_ss_low_priority",
            "sched_ss_repl_period",
            "sched_ss_init_budget",
            "sched_ss_max_repl",
        ].contains(&field))
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_neutrino(target: &str) {
    assert!(target.contains("nto-qnx"));

    let mut cfg = ctest_cfg();
    if target.ends_with("_iosock") {
        let qnx_target_val = env::var("QNX_TARGET")
            .unwrap_or_else(|_| "QNX_TARGET_not_set_please_source_qnxsdp".into());

        cfg.include(qnx_target_val + "/usr/include/io-sock");
        headers! { cfg:
            "io-sock.h",
            "sys/types.h",
            "sys/socket.h",
            "sys/sysctl.h",
            "net/if.h",
            "net/if_arp.h"
        }
    }

    headers! { cfg:
        "ctype.h",
        "dirent.h",
        "dlfcn.h",
        "sys/elf.h",
        "fcntl.h",
        "fnmatch.h",
        "glob.h",
        "grp.h",
        "iconv.h",
        "ifaddrs.h",
        "limits.h",
        "sys/link.h",
        "locale.h",
        "sys/malloc.h",
        "rcheck/malloc.h",
        "malloc.h",
        "mqueue.h",
        "net/if.h",
        "net/if_arp.h",
        "net/route.h",
        "netdb.h",
        "netinet/in.h",
        "netinet/ip.h",
        "netinet/tcp.h",
        "netinet/udp.h",
        "netinet/ip_var.h",
        "sys/poll.h",
        "pthread.h",
        "pwd.h",
        "regex.h",
        "resolv.h",
        "sys/sched.h",
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
        "sys/sysctl.h",
        "sys/file.h",
        "sys/inotify.h",
        "sys/ioctl.h",
        "sys/ipc.h",
        "sys/mman.h",
        "sys/mount.h",
        "sys/msg.h",
        "sys/resource.h",
        "sys/sem.h",
        "sys/socket.h",
        "sys/stat.h",
        "sys/statvfs.h",
        "sys/swap.h",
        "sys/termio.h",
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
        "sys/time.h",
        "ucontext.h",
        "unistd.h",
        "utime.h",
        "utmp.h",
        "wchar.h",
        "aio.h",
        "nl_types.h",
        "langinfo.h",
        "unix.h",
        "nbutil.h",
        "aio.h",
        "net/bpf.h",
        "net/if_dl.h",
        "sys/syspage.h",

        // TODO: The following header file doesn't appear as part of the default headers
        //       found in a standard installation of Neutrino 7.1 SDP.  The structures/
        //       functions dependent on it are currently commented out.
        //"sys/asyncmsg.h",
    }

    // Create and include a header file containing
    // items which are not included in any official
    // header file.
    let internal_header = "internal.h";
    let out_dir = env::var("OUT_DIR").unwrap();
    cfg.header(internal_header);
    cfg.include(&out_dir);
    std::fs::write(
        out_dir.to_owned() + "/" + internal_header,
        "#ifndef __internal_h__
        #define __internal_h__
        void __my_thread_exit(const void **);
        #endif",
    )
    .unwrap();

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "FILE" | "fd_set" | "Dl_info" | "DIR" | "Elf32_Phdr" | "Elf64_Phdr" | "Elf32_Shdr"
            | "Elf64_Shdr" | "Elf32_Sym" | "Elf64_Sym" | "Elf32_Ehdr" | "Elf64_Ehdr"
            | "Elf32_Chdr" | "Elf64_Chdr" | "aarch64_qreg_t" | "syspage_entry_info"
            | "syspage_array_info" => ty.to_string(),

            "Ioctl" => "int".to_string(),

            t if is_union => format!("union {t}"),

            t if t.ends_with("_t") => t.to_string(),

            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),

            t => t.to_string(),
        }
    });

    cfg.field_name(move |_struct_, field| match field {
        "type_" => "type".to_string(),

        s => s.to_string(),
    });

    cfg.volatile_item(|i| {
        use ctest::VolatileItemKind::*;
        match i {
            // The following fields are volatie but since we cannot express that in
            // Rust types, we have to explicitly tell the checker about it here:
            StructField(ref n, ref f) if n == "aiocb" && f == "aio_buf" => true,
            StructField(ref n, ref f) if n == "qtime_entry" && f == "nsec_tod_adjust" => true,
            StructField(ref n, ref f) if n == "qtime_entry" && f == "nsec" => true,
            StructField(ref n, ref f) if n == "qtime_entry" && f == "nsec_stable" => true,
            StructField(ref n, ref f) if n == "intrspin" && f == "value" => true,
            _ => false,
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // FIXME(sighandler): `sighandler_t` type is incorrect, see:
            // https://github.com/rust-lang/libc/issues/1359
            "sighandler_t" => true,

            // Does not exist in Neutrino
            "locale_t" => true,

            // FIXME: "'__uint128' undeclared" in C
            "__uint128" => true,

            _ => false,
        }
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            "Elf64_Phdr" | "Elf32_Phdr" => true,

            // FIXME(union): This is actually a union, not a struct
            "sigval" => true,

            // union
            "_channel_connect_attr" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // These signal "functions" are actually integer values that are casted to a fn ptr
            // This causes the compiler to err because of "illegal cast of int to ptr".
            "SIG_DFL" => true,
            "SIG_IGN" => true,
            "SIG_ERR" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        // skip those that are manually verified
        match name {
            // FIXME: https://github.com/rust-lang/libc/issues/1272
            "execv" | "execve" | "execvp" | "execvpe" => true,

            // wrong signature
            "signal" => true,

            // wrong signature of callback ptr
            "__cxa_atexit" => true,

            // FIXME(ctest): Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using `&`/`&mut` here, see:
            // https://github.com/gnzlbg/ctest/issues/68
            "lio_listio" => true,

            // 2 fields are actually unions which we're simply representing
            // as structures.
            "ChannelConnectAttr" => true,

            // fields contains unions
            "SignalKillSigval" => true,
            "SignalKillSigval_r" => true,

            // Not defined in any headers.  Defined to work around a
            // stack unwinding bug.
            "__my_thread_exit" => true,

            // Wrong const-ness
            "dl_iterate_phdr" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        // sigval is actually a union, but we pretend it's a struct
        struct_ == "sigevent" && field == "sigev_value" ||
        // Anonymous structures
        struct_ == "_idle_hook" && field == "time"
    });

    cfg.skip_field(|struct_, field| {
        matches!(
            (struct_, field),
            ("__sched_param", "reserved")
            | ("sched_param", "reserved")
            | ("sigevent", "__padding1") // ensure alignment
            | ("sigevent", "__padding2") // union
            | ("sigevent", "__sigev_un2") // union
            | ("sigaction", "sa_sigaction") // sighandler_t type is super weird
            | ("syspage_entry", "__reserved") // does not exist
        )
    });

    cfg.skip_static(move |name| (name == "__dso_handle"));

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
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
    // FIXME(vxworks)
    cfg.skip_const(move |name| match name {
        // sighandler_t weirdness
        "SIG_DFL" | "SIG_ERR" | "SIG_IGN"
        // This is not defined in vxWorks
        | "RTLD_DEFAULT"   => true,
        _ => false,
    });
    // FIXME(vxworks)
    cfg.skip_type(move |ty| match ty {
        "stat64" | "sighandler_t" | "off64_t" => true,
        _ => false,
    });

    cfg.skip_field_type(move |struct_, field| match (struct_, field) {
        ("siginfo_t", "si_value") | ("stat", "st_size") | ("sigaction", "sa_u") => true,
        _ => false,
    });

    cfg.skip_roundtrip(|_| false);

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "DIR" | "FILE" | "Dl_info" | "RTP_DESC" => ty.to_string(),
        t if is_union => format!("union {t}"),
        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {t}"),
        t => t.to_string(),
    });

    // FIXME(vxworks)
    cfg.skip_fn(move |name| match name {
        // sigval
        "sigqueue" | "_sigqueue"
        // sighandler_t
        | "signal"
        // not used in static linking by default
        | "dlerror" => true,
        _ => false,
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn config_gnu_bits(target: &str, cfg: &mut ctest::TestGenerator) {
    let pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();
    if target.contains("gnu")
        && target.contains("linux")
        && !target.ends_with("x32")
        && !target.contains("riscv32")
        && pointer_width == "32"
    {
        match env::var("RUST_LIBC_UNSTABLE_GNU_TIME_BITS") {
            Ok(val) if val == "64" => {
                cfg.define("_FILE_OFFSET_BITS", Some("64"));
                cfg.define("_TIME_BITS", Some("64"));
                cfg.cfg("gnu_file_offset_bits64", None);
                cfg.cfg("linux_time_bits64", None);
                cfg.cfg("gnu_time_bits64", None);
            }
            Ok(val) if val != "32" => {
                panic!("RUST_LIBC_UNSTABLE_GNU_TIME_BITS may only be set to '32' or '64'")
            }
            _ => {
                match env::var("RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS") {
                    Ok(val) if val == "64" => {
                        cfg.define("_FILE_OFFSET_BITS", Some("64"));
                        cfg.cfg("gnu_file_offset_bits64", None);
                    }
                    Ok(val) if val != "32" => {
                        panic!("RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS may only be set to '32' or '64'")
                    }
                    _ => {}
                }
            }
        }
    }
}

fn test_linux(target: &str) {
    assert!(target.contains("linux"));

    // target_env
    let gnu = target.contains("gnu");
    let musl = target.contains("musl") || target.contains("ohos");
    let uclibc = target.contains("uclibc");

    match (gnu, musl, uclibc) {
        (true, false, false) => (),
        (false, true, false) => (),
        (false, false, true) => (),
        (_, _, _) => panic!("linux target lib is gnu: {gnu}, musl: {musl}, uclibc: {uclibc}"),
    }

    let arm = target.contains("arm");
    let aarch64 = target.contains("aarch64");
    let i686 = target.contains("i686");
    let ppc = target.contains("powerpc");
    let ppc64 = target.contains("powerpc64");
    let s390x = target.contains("s390x");
    let sparc64 = target.contains("sparc64");
    let x32 = target.contains("x32");
    let x86_32 = target.contains("i686");
    let x86_64 = target.contains("x86_64");
    let gnueabihf = target.contains("gnueabihf");
    let x86_64_gnux32 = target.contains("gnux32") && x86_64;
    let riscv64 = target.contains("riscv64");
    let loongarch64 = target.contains("loongarch64");
    let wasm32 = target.contains("wasm32");
    let uclibc = target.contains("uclibc");

    let musl_v1_2_3 = env::var("RUST_LIBC_UNSTABLE_MUSL_V1_2_3").is_ok();
    let old_musl = musl && !musl_v1_2_3;

    let mut cfg = ctest_cfg();
    if musl_v1_2_3 {
        cfg.cfg("musl_v1_2_3", None);
    }
    cfg.define("_GNU_SOURCE", None);
    // This macro re-defines fscanf,scanf,sscanf to link to the symbols that are
    // deprecated since glibc >= 2.29. This allows Rust binaries to link against
    // glibc versions older than 2.29.
    cfg.define("__GLIBC_USE_DEPRECATED_SCANF", None);

    config_gnu_bits(target, &mut cfg);

    headers! { cfg:
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "elf.h",
               "fcntl.h",
               "fnmatch.h",
               "getopt.h",
               "glob.h",
               [gnu]: "gnu/libc-version.h",
               "grp.h",
               "iconv.h",
               "ifaddrs.h",
               "langinfo.h",
               "libgen.h",
               "limits.h",
               "link.h",
               "linux/sysctl.h",
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
               "sys/klog.h",
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
               // Also unavailable on gnueabihf with glibc 2.30.
               // https://sourceware.org/git/?p=glibc.git;a=commitdiff;h=6b33f373c7b9199e00ba5fbafd94ac9bfb4337b1
               [(x86_64 || x86_32 || arm) && !gnueabihf]: "sys/io.h",
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
        [loongarch64 || riscv64]: "asm/hwcap.h",
        "asm/mman.h",
    }

    if !wasm32 {
        headers! { cfg:
            [gnu]: "linux/aio_abi.h",
            "linux/can.h",
            "linux/can/raw.h",
            "linux/can/j1939.h",
            "linux/cn_proc.h",
            "linux/connector.h",
            "linux/dccp.h",
            "linux/errqueue.h",
            "linux/falloc.h",
            "linux/filter.h",
            "linux/fs.h",
            "linux/futex.h",
            "linux/genetlink.h",
            "linux/if.h",
            "linux/if_addr.h",
            "linux/if_alg.h",
            "linux/if_ether.h",
            "linux/if_packet.h",
            "linux/if_tun.h",
            "linux/if_xdp.h",
            "linux/input.h",
            "linux/ipv6.h",
            "linux/kexec.h",
            "linux/keyctl.h",
            "linux/magic.h",
            "linux/memfd.h",
            "linux/membarrier.h",
            "linux/mempolicy.h",
            "linux/mman.h",
            "linux/module.h",
            "linux/mount.h",
            "linux/net_tstamp.h",
            "linux/netfilter/nfnetlink.h",
            "linux/netfilter/nfnetlink_log.h",
            "linux/netfilter/nfnetlink_queue.h",
            "linux/netfilter/nf_tables.h",
            "linux/netfilter_arp.h",
            "linux/netfilter_bridge.h",
            "linux/netfilter_ipv4.h",
            "linux/netfilter_ipv6.h",
            "linux/netfilter_ipv6/ip6_tables.h",
            "linux/netlink.h",
            "linux/nsfs.h",
            "linux/openat2.h",
            // FIXME(linux): some items require Linux >= 5.6:
            "linux/ptp_clock.h",
            "linux/ptrace.h",
            "linux/quota.h",
            "linux/random.h",
            "linux/reboot.h",
            "linux/rtnetlink.h",
            "linux/sched.h",
            "linux/sctp.h",
            "linux/seccomp.h",
            "linux/sock_diag.h",
            "linux/sockios.h",
            "linux/tls.h",
            "linux/uinput.h",
            "linux/vm_sockets.h",
            "linux/wait.h",
            "linux/wireless.h",
            "sys/fanotify.h",
            // <sys/auxv.h> is not present on uclibc
            [!uclibc]: "sys/auxv.h",
            [gnu || musl]: "linux/close_range.h",
        }
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

            "Ioctl" if gnu => "unsigned long".to_string(),
            "Ioctl" => "int".to_string(),

            // LFS64 types have been removed in musl 1.2.4+
            "off64_t" if musl => "off_t".to_string(),

            // typedefs don't need any keywords
            t if t.ends_with("_t") => t.to_string(),
            // put `struct` in front of all structs:.
            t if is_struct => format!("struct {t}"),
            // put `union` in front of all unions:
            t if is_union => format!("union {t}"),

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
            // FIXME(linux): epoll_event.data is actually a union in C, but in Rust
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
        // FIXME(musl): very recent additions to musl, not yet released.
        // also apparently some glibc versions
        if ty == "Elf32_Relr" || ty == "Elf64_Relr" {
            return true;
        }
        if sparc64 && (ty == "Elf32_Rela" || ty == "Elf64_Rela") {
            return true;
        }
        match ty {
            // FIXME(sighandler): `sighandler_t` type is incorrect, see:
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

            // For internal use only, to define architecture specific ioctl constants with a libc
            // specific type.
            "Ioctl" => true,

            // FIXME: "'__uint128' undeclared" in C
            "__uint128" => true,

            t => {
                if musl {
                    // LFS64 types have been removed in musl 1.2.4+
                    t.ends_with("64") || t.ends_with("64_t")
                } else {
                    false
                }
            }
        }
    });

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }

        // FIXME(linux): CI has old headers
        if ty == "ptp_sys_offset_extended" {
            return true;
        }

        // LFS64 types have been removed in musl 1.2.4+
        if musl && (ty.ends_with("64") || ty.ends_with("64_t")) {
            return true;
        }

        // FIXME(linux): sparc64 CI has old headers
        if sparc64 && (ty == "uinput_ff_erase" || ty == "uinput_abs_setup") {
            return true;
        }

        // FIXME(#1558): passing by value corrupts the value for reasons not understood.
        if (gnu && sparc64) && (ty == "ip_mreqn" || ty == "hwtstamp_config") {
            return true;
        }

        // FIXME(rust-lang/rust#43894): pass by value for structs that are not an even 32/64 bits
        // on big-endian systems corrupts the value for unknown reasons.
        if (sparc64 || ppc || ppc64 || s390x)
            && (ty == "sockaddr_pkt"
                || ty == "tpacket_auxdata"
                || ty == "tpacket_hdr_variant1"
                || ty == "tpacket_req3"
                || ty == "tpacket_stats_v3"
                || ty == "tpacket_req_u")
        {
            return true;
        }

        // FIXME(musl): musl doesn't compile with `struct fanout_args` for unknown reasons.
        if musl && ty == "fanout_args" {
            return true;
        }
        if sparc64 && ty == "fanotify_event_info_error" {
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

            // FIXME(union): This is actually a union, not a struct
            "sigval" => true,

            // This type is tested in the `linux_termios.rs` file since there
            // are header conflicts when including them with all the other
            // structs.
            "termios2" => true,

            // FIXME(linux): remove once we set minimum supported glibc version.
            // ucontext_t added a new field as of glibc 2.28; our struct definition is
            // conservative and omits the field, but that means the size doesn't match for newer
            // glibcs (see https://github.com/rust-lang/libc/issues/1410)
            "ucontext_t" if gnu => true,

            // FIXME(linux): Somehow we cannot include headers correctly in glibc 2.30.
            // So let's ignore for now and re-visit later.
            // Probably related: https://gcc.gnu.org/bugzilla/show_bug.cgi?id=91085
            "statx" => true,
            "statx_timestamp" => true,

            // On Linux, the type of `ut_exit` field of struct `utmpx`
            // can be an anonymous struct, so an extra struct,
            // which is absent in musl, has to be defined.
            "__exit_status" if musl => true,

            // clone_args might differ b/w libc versions
            "clone_args" => true,

            // Might differ between kernel versions
            "open_how" => true,

            // Linux >= 6.13 (pidfd_info.exit_code: Linux >= 6.15)
            // Might differ between kernel versions
            "pidfd_info" => true,

            "sctp_initmsg" | "sctp_sndrcvinfo" | "sctp_sndinfo" | "sctp_rcvinfo"
            | "sctp_nxtinfo" | "sctp_prinfo" | "sctp_authinfo" => true,

            // FIXME(linux): requires >= 6.1 kernel headers
            "canxl_frame" => true,

            // FIXME(linux): The size of `iv` has been changed since Linux v6.0
            // https://github.com/torvalds/linux/commit/94dfc73e7cf4a31da66b8843f0b9283ddd6b8381
            "af_alg_iv" => true,

            // FIXME(linux): Requires >= 5.1 kernel headers.
            // Everything that uses install-musl.sh has 4.19 kernel headers.
            "tls12_crypto_info_aes_gcm_256"
                if (aarch64 || arm || i686 || s390x || x86_64) && musl =>
            {
                true
            }

            // FIXME(linux): Requires >= 5.11 kernel headers.
            // Everything that uses install-musl.sh has 4.19 kernel headers.
            "tls12_crypto_info_chacha20_poly1305"
                if (aarch64 || arm || i686 || s390x || x86_64) && musl =>
            {
                true
            }

            // FIXME(linux): Requires >= 5.3 kernel headers.
            // Everything that uses install-musl.sh has 4.19 kernel headers.
            "xdp_options" if musl => true,

            // FIXME(linux): Requires >= 5.4 kernel headers.
            // Everything that uses install-musl.sh has 4.19 kernel headers.
            "xdp_ring_offset" | "xdp_mmap_offsets" if musl => true,

            // FIXME(linux): Requires >= 6.8 kernel headers.
            // A field was added in 6.8.
            // https://github.com/torvalds/linux/commit/341ac980eab90ac1f6c22ee9f9da83ed9604d899
            // The previous version of the struct was removed in 6.11 due to a bug.
            // https://github.com/torvalds/linux/commit/32654bbd6313b4cfc82297e6634fa9725c3c900f
            "xdp_umem_reg" => true,

            // FIXME(linux): Requires >= 5.9 kernel headers.
            // Everything that uses install-musl.sh has 4.19 kernel headers.
            "xdp_statistics" if musl => true,

            // FIXME(linux): Requires >= 6.8 kernel headers.
            "xsk_tx_metadata"
            | "__c_anonymous_xsk_tx_metadata_union"
            | "xsk_tx_metadata_request"
            | "xsk_tx_metadata_completion" => true,

            // A new field was added in kernel 5.4, this is the old version for backwards compatibility.
            // https://github.com/torvalds/linux/commit/77cd0d7b3f257fd0e3096b4fdcff1a7d38e99e10
            "xdp_ring_offset_v1" | "xdp_mmap_offsets_v1" => true,

            // Multiple new fields were added in kernel 5.9, this is the old version for backwards compatibility.
            // https://github.com/torvalds/linux/commit/77cd0d7b3f257fd0e3096b4fdcff1a7d38e99e10
            "xdp_statistics_v1" => true,

            // A new field was added in kernel 5.4, this is the old version for backwards compatibility.
            // https://github.com/torvalds/linux/commit/c05cd3645814724bdeb32a2b4d953b12bdea5f8c
            "xdp_umem_reg_v1" => true,

            // Is defined in `<linux/sched/types.h>` but if this file is included at the same time
            // as `<sched.h>`, the `struct sched_param` is defined twice, causing the compilation to
            // fail. The problem doesn't seem to be present in more recent versions of the linux
            // kernel so we can drop this and test the type once this new version is used in CI.
            "sched_attr" => true,

            // FIXME(linux): Requires >= 6.9 kernel headers.
            "epoll_params" => true,

            // FIXME(linux): Requires >= 6.12 kernel headers.
            "dmabuf_cmsg" | "dmabuf_token" => true,

            // FIXME(linux): Requires >= 6.12 kernel headers.
            "mnt_ns_info" => true,

            // FIXME(linux): Requires >= 6.4 kernel headers.
            "ptrace_sud_config" => true,

            // Struct has changed for new musl versions
            "tcp_info" if old_musl => true,

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
                || name.starts_with("KEXEC_")
                || name.starts_with("MS_")
                || name.starts_with("MSG_")
                || name.starts_with("OPEN_TREE_")
                || name.starts_with("P_")
                || name.starts_with("PF_")
                || name.starts_with("PIDFD_")
                || name.starts_with("RLIMIT_")
                || name.starts_with("RTEXT_FILTER_")
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
        if musl {
            // FIXME(linux): Requires >= 5.0 kernel headers
            if name == "SECCOMP_GET_NOTIF_SIZES"
               || name == "SECCOMP_FILTER_FLAG_NEW_LISTENER"
               || name == "SECCOMP_FILTER_FLAG_TSYNC_ESRCH"
               || name == "SECCOMP_USER_NOTIF_FLAG_CONTINUE"  // requires >= 5.5
               || name == "SECCOMP_ADDFD_FLAG_SETFD"  // requires >= 5.9
               || name == "SECCOMP_ADDFD_FLAG_SEND"   // requires >= 5.9
               || name == "SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV"  // requires >= 5.19
            {
                return true;
            }
            // FIXME(linux): Requires >= 4.20 kernel headers
            if name == "PTP_SYS_OFFSET_EXTENDED" {
                return true;
            }
            // FIXME(linux): Requires >= 5.4 kernel headers
            if name == "PTP_CLOCK_GETCAPS2"
                || name == "PTP_ENABLE_PPS2"
                || name == "PTP_EXTTS_REQUEST2"
                || name == "PTP_PEROUT_REQUEST2"
                || name == "PTP_PIN_GETFUNC2"
                || name == "PTP_PIN_SETFUNC2"
                || name == "PTP_SYS_OFFSET2"
                || name == "PTP_SYS_OFFSET_PRECISE2"
                || name == "PTP_SYS_OFFSET_EXTENDED2"
            {
                return true;
            }
            // FIXME(linux): Requires >= 5.4.1 kernel headers
            if name.starts_with("J1939")
                || name.starts_with("RTEXT_FILTER_")
                || name.starts_with("SO_J1939")
                || name.starts_with("SCM_J1939")
            {
                return true;
            }
            // FIXME(linux): Requires >= 5.10 kernel headers
            if name.starts_with("MEMBARRIER_CMD_REGISTER")
                || name.starts_with("MEMBARRIER_CMD_PRIVATE")
            {
                return true;
            }
            // LFS64 types have been removed in musl 1.2.4+
            if name.starts_with("RLIM64") {
                return true;
            }
            // CI fails because musl targets use Linux v4 kernel
            if name.starts_with("NI_IDN") {
                return true;
            }
            // FIXME: Requires >= 6.3 kernel headers
            if loongarch64 && (name == "MFD_NOEXEC_SEAL" || name == "MFD_EXEC") {
                return true;
            }
            // FIXME: Requires >= 6.3 (6.6) kernel headers
            if name == "PR_GET_MDWE" || name == "PR_MDWE_NO_INHERIT" || name == "PR_MDWE_REFUSE_EXEC_GAIN" || name == "PR_SET_MDWE" {
                return true;
            }
            // Requires musl >= 1.2
            if old_musl && (name == "SO_PREFER_BUSY_POLL"
                || name == "SO_BUSY_POLL_BUDGET")
            {
                return true;
            }
            // FIXME(musl): Not in musl yet
            if name == "SO_NETNS_COOKIE"
                || name == "SO_BUF_LOCK"
                || name == "SO_RESERVE_MEM"
                || name == "SO_TXREHASH"
                || name == "SO_RCVMARK"
                || name == "SO_PASSPIDFD"
                || name == "SO_PEERPIDFD"
                || name == "SO_DEVMEM_LINEAR"
                || name == "SO_DEVMEM_DMABUF"
                || name == "SO_DEVMEM_DONTNEED"
            {
                        return true;
            }
            // FIXME(musl): Not in musl yet
            if name == "SCM_DEVMEM_LINEAR"
                || name == "SCM_DEVMEM_DMABUF"
            {
                        return true;
            }
            // Values changed in newer musl versions on these arches
            if old_musl && (riscv64 || x86_64) && name == "O_LARGEFILE" {
                return true;
            }
            // Values changed in newer musl versions
            if old_musl && name == "RLIM_NLIMITS" {
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
            | "F_SEAL_WRITE"
            | "F_SEAL_FUTURE_WRITE"
            | "F_SEAL_EXEC" => true,
            // The `ARPHRD_CAN` is tested in the `linux_if_arp.rs` tests
            // because including `linux/if_arp.h` causes some conflicts:
            "ARPHRD_CAN" => true,

            // FIXME(deprecated): deprecated: not available in any header
            // See: https://github.com/rust-lang/libc/issues/1356
            "ENOATTR" => true,

            // FIXME(deprecated): SIGUNUSED was removed in glibc 2.26
            // Users should use SIGSYS instead.
            "SIGUNUSED" => true,

            // FIXME(linux): conflicts with glibc headers and is tested in
            // `linux_termios.rs` below:
            | "BOTHER"
            | "IBSHIFT"
            | "TCGETS2"
            | "TCSETS2"
            | "TCSETSW2"
            | "TCSETSF2" => true,

            // FIXME(musl): on musl the pthread types are defined a little differently
            // - these constants are used by the glibc implementation.
            n if musl && n.contains("__SIZEOF_PTHREAD") => true,

            // FIXME(linux): It was extended to 4096 since glibc 2.31 (Linux 5.4).
            // We should do so after a while.
            "SOMAXCONN" if gnu => true,

            // deprecated: not available from Linux kernel 5.6:
            "VMADDR_CID_RESERVED" => true,

            // IPPROTO_MAX was increased in 5.6 for IPPROTO_MPTCP:
            | "IPPROTO_MAX"
            | "IPPROTO_ETHERNET"
            | "IPPROTO_MPTCP" => true,

            // FIXME(linux): Not yet implemented on sparc64
            "SYS_clone3" if sparc64 => true,

            // FIXME(linux): Not defined on ARM, gnueabihf, musl, PowerPC, riscv64, s390x, and sparc64.
            "SYS_memfd_secret" if arm | gnueabihf | musl | ppc | riscv64 | s390x | sparc64 => true,

            // FIXME(linux): Added in Linux 5.16
            // https://github.com/torvalds/linux/commit/039c0ec9bb77446d7ada7f55f90af9299b28ca49
            "SYS_futex_waitv" => true,

            // FIXME(linux): Added in Linux 5.17
            // https://github.com/torvalds/linux/commit/c6018b4b254971863bd0ad36bb5e7d0fa0f0ddb0
            "SYS_set_mempolicy_home_node" => true,

            // FIXME(linux): Added in Linux 5.18
            // https://github.com/torvalds/linux/commit/8b5413647262dda8d8d0e07e14ea1de9ac7cf0b2
            "NFQA_PRIORITY" => true,

            // FIXME(linux): requires more recent kernel headers on CI
            | "UINPUT_VERSION"
            | "SW_MAX"
            | "SW_CNT"
                if ppc64 || riscv64 => true,

            // FIXME(linux): requires more recent kernel headers on CI
            "SECCOMP_FILTER_FLAG_WAIT_KILLABLE_RECV" if sparc64 => true,

            // FIXME(linux): Not currently available in headers on ARM and musl.
            "NETLINK_GET_STRICT_CHK" if arm => true,

            // Skip as this signal codes and trap reasons need newer headers
            "SI_DETHREAD" | "TRAP_PERF" => true,

            // kernel constants not available in uclibc 1.0.34
            | "EXTPROC"
            | "IPPROTO_BEETPH"
            | "IPPROTO_MPLS"
            | "IPV6_HDRINCL"
            | "IPV6_MULTICAST_ALL"
            | "IPV6_PMTUDISC_INTERFACE"
            | "IPV6_PMTUDISC_OMIT"
            | "IPV6_ROUTER_ALERT_ISOLATE"
            | "PACKET_MR_UNICAST"
            | "RUSAGE_THREAD"
            | "SHM_EXEC"
            | "UDP_GRO"
            | "UDP_SEGMENT"
                if uclibc => true,

            // headers conflicts with linux/pidfd.h
            "PIDFD_NONBLOCK" => true,
            // Linux >= 6.9
            "PIDFD_THREAD"
            | "PIDFD_SIGNAL_THREAD"
            | "PIDFD_SIGNAL_THREAD_GROUP"
            | "PIDFD_SIGNAL_PROCESS_GROUP" => true,
            // Linux >= 6.11
            "PIDFD_GET_CGROUP_NAMESPACE"
            | "PIDFD_GET_IPC_NAMESPACE"
            | "PIDFD_GET_MNT_NAMESPACE"
            | "PIDFD_GET_NET_NAMESPACE"
            | "PIDFD_GET_PID_NAMESPACE"
            | "PIDFD_GET_PID_FOR_CHILDREN_NAMESPACE"
            | "PIDFD_GET_TIME_NAMESPACE"
            | "PIDFD_GET_TIME_FOR_CHILDREN_NAMESPACE"
            | "PIDFD_GET_USER_NAMESPACE"
            | "PIDFD_GET_UTS_NAMESPACE" => true,
            // Linux >= 6.13
            "PIDFD_GET_INFO"
            | "PIDFD_INFO_PID"
            | "PIDFD_INFO_CREDS"
            | "PIDFD_INFO_CGROUPID"
            | "PIDFD_INFO_SIZE_VER0" => true,
            // Linux >= 6.15
            "PIDFD_INFO_EXIT" | "PIDFD_SELF" | "PIDFD_SELF_PROCESS" => true,

            // is a private value for kernel usage normally
            "FUSE_SUPER_MAGIC" => true,

            // linux 5.17 min
            "PR_SET_VMA" | "PR_SET_VMA_ANON_NAME" => true,

            // present in recent kernels only
            "PR_SCHED_CORE" | "PR_SCHED_CORE_CREATE" | "PR_SCHED_CORE_GET" | "PR_SCHED_CORE_MAX" | "PR_SCHED_CORE_SCOPE_PROCESS_GROUP" | "PR_SCHED_CORE_SCOPE_THREAD" | "PR_SCHED_CORE_SCOPE_THREAD_GROUP" | "PR_SCHED_CORE_SHARE_FROM" | "PR_SCHED_CORE_SHARE_TO" => true,

            // present in recent kernels only >= 5.13
            "PR_PAC_SET_ENABLED_KEYS" | "PR_PAC_GET_ENABLED_KEYS" => true,
            // present in recent kernels only >= 5.19
            "PR_SME_SET_VL" | "PR_SME_GET_VL" | "PR_SME_VL_LEN_MAX" | "PR_SME_SET_VL_INHERIT" | "PR_SME_SET_VL_ONE_EXEC" => true,

            // Added in Linux 5.14
            "FUTEX_LOCK_PI2" => true,

            // Added in  linux 6.1
            "STATX_DIOALIGN"
            | "CAN_RAW_XL_FRAMES"
            | "CANXL_HDR_SIZE"
            | "CANXL_MAX_DLC"
            | "CANXL_MAX_DLC_MASK"
            | "CANXL_MAX_DLEN"
            | "CANXL_MAX_MTU"
            | "CANXL_MIN_DLC"
            | "CANXL_MIN_DLEN"
            | "CANXL_MIN_MTU"
            | "CANXL_MTU"
            | "CANXL_PRIO_BITS"
            | "CANXL_PRIO_MASK"
            | "CANXL_SEC"
            | "CANXL_XLF"
             => true,

            // FIXME(linux): Parts of netfilter/nfnetlink*.h require more recent kernel headers:
            | "RTNLGRP_MCTP_IFADDR" // linux v5.17+
            | "RTNLGRP_TUNNEL" // linux v5.18+
            | "RTNLGRP_STATS" // linux v5.18+
                => true,

            // FIXME(linux): The below is no longer const in glibc 2.34:
            // https://github.com/bminor/glibc/commit/5d98a7dae955bafa6740c26eaba9c86060ae0344
            | "PTHREAD_STACK_MIN"
            | "SIGSTKSZ"
            | "MINSIGSTKSZ"
                if gnu => true,

            // FIXME(linux): Linux >= 5.16:
            // https://github.com/torvalds/linux/commit/42df6e1d221dddc0f2acf2be37e68d553ad65f96
            "NF_NETDEV_EGRESS" if sparc64 => true,
            // value changed
            "NF_NETDEV_NUMHOOKS" if sparc64 => true,

            // FIXME(linux): requires Linux >= v5.8
            "IF_LINK_MODE_TESTING" if sparc64 => true,

            // DIFF(main): fixed in 1.0 with e9abac9ac2
            "CLONE_CLEAR_SIGHAND" | "CLONE_INTO_CGROUP" => true,

            // kernel 6.1 minimum
            "MADV_COLLAPSE" => true,

            // kernel 6.2 minimum
            "TUN_F_USO4" | "TUN_F_USO6" | "IFF_NO_CARRIER" => true,

            // kernel 6.9 minimum
            "RWF_NOAPPEND" => true,

            // kernel 6.11 minimum
            "RWF_ATOMIC" => true,

            // kernel 6.14 minimum
            "RWF_DONTCACHE" => true,

            // FIXME(linux): Requires more recent kernel headers
            | "IFLA_PARENT_DEV_NAME"     // linux v5.13+
            | "IFLA_PARENT_DEV_BUS_NAME" // linux v5.13+
            | "IFLA_GRO_MAX_SIZE"        // linux v5.16+
            | "IFLA_TSO_MAX_SIZE"        // linux v5.18+
            | "IFLA_TSO_MAX_SEGS"        // linux v5.18+
            | "IFLA_ALLMULTI"            // linux v6.0+
            | "MADV_DONTNEED_LOCKED"     // linux v5.18+
                => true,
            "SCTP_FUTURE_ASSOC" | "SCTP_CURRENT_ASSOC" | "SCTP_ALL_ASSOC" | "SCTP_PEER_ADDR_THLDS_V2" => true, // linux 5.5+

            // kernel 6.5 minimum
            "MOVE_MOUNT_BENEATH" => true,
            // FIXME(linux): Requires linux 6.1
            "ALG_SET_KEY_BY_KEY_SERIAL" | "ALG_SET_DRBG_ENTROPY" => true,

            // FIXME(linux): Requires more recent kernel headers
            | "FAN_FS_ERROR"                      // linux v5.16+
            | "FAN_RENAME"                        // linux v5.17+
            | "FAN_REPORT_TARGET_FID"             // linux v5.17+
            | "FAN_REPORT_DFID_NAME_TARGET"       // linux v5.17+
            | "FAN_MARK_EVICTABLE"                // linux v5.19+
            | "FAN_MARK_IGNORE"                   // linux v6.0+
            | "FAN_MARK_IGNORE_SURV"              // linux v6.0+
            | "FAN_EVENT_INFO_TYPE_ERROR"         // linux v5.16+
            | "FAN_EVENT_INFO_TYPE_OLD_DFID_NAME" // linux v5.17+
            | "FAN_EVENT_INFO_TYPE_NEW_DFID_NAME" // linux v5.17+
            | "FAN_RESPONSE_INFO_NONE"            // linux v5.16+
            | "FAN_RESPONSE_INFO_AUDIT_RULE"      // linux v5.16+
            | "FAN_INFO"                          // linux v5.16+
                => true,

            // musl doesn't use <linux/fanotify.h> in <sys/fanotify.h>
            "FAN_REPORT_PIDFD"
            | "FAN_REPORT_DIR_FID"
            | "FAN_REPORT_NAME"
            | "FAN_REPORT_DFID_NAME"
            | "FAN_EVENT_INFO_TYPE_DFID_NAME"
            | "FAN_EVENT_INFO_TYPE_DFID"
            | "FAN_EVENT_INFO_TYPE_PIDFD"
            | "FAN_NOPIDFD"
            | "FAN_EPIDFD"
            if musl => true,

            // FIXME(linux): Requires linux 6.5
            "NFT_MSG_MAX" => true,

            // FIXME(linux): Requires >= 6.6 kernel headers.
            "XDP_USE_SG"
            | "XDP_PKT_CONTD"
                =>
            {
                true
            }

            // FIXME(linux): Requires >= 6.6 kernel headers.
            "PR_MDWE_NO_INHERIT" => true,

            // FIXME(linux): Requires >= 6.8 kernel headers.
            "XDP_UMEM_TX_SW_CSUM"
            | "XDP_TXMD_FLAGS_TIMESTAMP"
            | "XDP_TXMD_FLAGS_CHECKSUM"
            | "XDP_TX_METADATA"
                =>
            {
                true
            }

            // FIXME(linux): Requires >= 6.11 kernel headers.
            "XDP_UMEM_TX_METADATA_LEN"
                =>
            {
                true
            }

            // FIXME(linux): Requires >= 6.11 kernel headers.
            "NS_GET_MNTNS_ID" | "NS_GET_PID_FROM_PIDNS" | "NS_GET_TGID_FROM_PIDNS" | "NS_GET_PID_IN_PIDNS" | "NS_GET_TGID_IN_PIDNS" => true,
            // FIXME(linux): Requires >= 6.12 kernel headers.
            "MNT_NS_INFO_SIZE_VER0" | "NS_MNT_GET_INFO" | "NS_MNT_GET_NEXT" | "NS_MNT_GET_PREV" => true,

            // FIXME(linux): Requires >= 6.6 kernel headers.
            "SYS_fchmodat2" => true,

            // FIXME(linux): Requires >= 6.10 kernel headers.
            "SYS_mseal" => true,

            // FIXME(linux): seems to not be available all the time (from <include/linux/sched.h>:
            "PF_VCPU"
            | "PF_IDLE"
            | "PF_EXITING"
            | "PF_POSTCOREDUMP"
            | "PF_IO_WORKER"
            | "PF_WQ_WORKER"
            | "PF_FORKNOEXEC"
            | "PF_MCE_PROCESS"
            | "PF_SUPERPRIV"
            | "PF_DUMPCORE"
            | "PF_SIGNALED"
            | "PF_MEMALLOC"
            | "PF_NPROC_EXCEEDED"
            | "PF_USED_MATH"
            | "PF_USER_WORKER"
            | "PF_NOFREEZE"
            | "PF_KSWAPD"
            | "PF_MEMALLOC_NOFS"
            | "PF_MEMALLOC_NOIO"
            | "PF_LOCAL_THROTTLE"
            | "PF_KTHREAD"
            | "PF_RANDOMIZE"
            | "PF_NO_SETAFFINITY"
            | "PF_MCE_EARLY"
            | "PF_MEMALLOC_PIN"
            | "PF_BLOCK_TS"
            | "PF_SUSPEND_TASK" => true,

            // FIXME(linux): Requires >= 6.9 kernel headers.
            "EPIOCSPARAMS"
            | "EPIOCGPARAMS" => true,

            // FIXME(linux): Requires >= 6.11 kernel headers.
            "MAP_DROPPABLE" => true,

            // FIXME(linux): Requires >= 6.2 kernel headers.
            "SOF_TIMESTAMPING_OPT_ID_TCP" => true,

            // FIXME(linux): Requires >= 6.12 kernel headers.
            "SOF_TIMESTAMPING_OPT_RX_FILTER" => true,

            // FIXME(linux): Requires >= 6.12 kernel headers.
            "SO_DEVMEM_LINEAR"
            | "SO_DEVMEM_DMABUF"
            | "SO_DEVMEM_DONTNEED"
            | "SCM_DEVMEM_LINEAR"
            | "SCM_DEVMEM_DMABUF" => true,
            // FIXME(linux): Requires >= 6.4 kernel headers.
            "PTRACE_SET_SYSCALL_USER_DISPATCH_CONFIG" | "PTRACE_GET_SYSCALL_USER_DISPATCH_CONFIG" => true,

            // FIXME(linux): Requires >= 6.6 kernel headers.
            "PROC_EVENT_NONZERO_EXIT" => true,

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

            // FIXME(linux): Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using `&`/`&mut` here, see:
            // https://github.com/gnzlbg/ctest/issues/68
            "lio_listio" if musl => true,

            // Needs glibc 2.34 or later.
            "posix_spawn_file_actions_addclosefrom_np" if gnu && sparc64 => true,
            // Needs glibc 2.35 or later.
            "posix_spawn_file_actions_addtcsetpgrp_np" if gnu && sparc64 => true,

            // FIXME(linux): Deprecated since glibc 2.30. Remove fn once upstream does.
            "sysctl" if gnu => true,

            // FIXME(linux): It now takes c_void instead of timezone since glibc 2.31.
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

            // FIXME(musl): This needs musl 1.2.2 or later.
            "gettid" if old_musl => true,

            // Needs glibc 2.33 or later.
            "mallinfo2" => true,

            "reallocarray" if old_musl => true,

            // Not defined in uclibc as of 1.0.34
            "gettid" if uclibc => true,

            // Needs musl 1.2.3 or later.
            "pthread_getname_np" if old_musl => true,

            // pthread_sigqueue uses sigval, which was initially declared
            // as a struct but should be defined as a union. However due
            // to the issues described here: https://github.com/rust-lang/libc/issues/2816
            // it can't be changed from struct.
            "pthread_sigqueue" => true,

            // There are two versions of basename(3) on Linux with glibc, see
            //
            // https://man7.org/linux/man-pages/man3/basename.3.html
            //
            // If libgen.h is included, then the POSIX version will be available;
            // If _GNU_SOURCE is defined and string.h is included, then the GNU one
            // will be used.
            //
            // libc exposes both of them, providing a prefix to differentiate between
            // them.
            //
            // Because the name with prefix is not a valid symbol in C, we have to
            // skip the tests.
            "posix_basename" if gnu => true,
            "gnu_basename" if gnu => true,

            // FIXME(linux): function pointers changed since Ubuntu 23.10
            "strtol" | "strtoll" | "strtoul" | "strtoull" | "fscanf" | "scanf" | "sscanf" => true,

            // Added in musl 1.2.5
            "preadv2" | "pwritev2" if musl => true,

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
        (struct_ == "sockaddr_can" && field == "can_addr") ||
        // `anonymous_1` is an anonymous union
        (struct_ == "ptp_perout_request" && field == "anonymous_1") ||
        // `anonymous_2` is an anonymous union
        (struct_ == "ptp_perout_request" && field == "anonymous_2") ||
        // FIXME(linux): `adjust_phase` requires >= 5.7 kernel headers
        // FIXME(linux): `max_phase_adj` requires >= 5.19 kernel headers
        // the rsv field shrunk when those fields got added, so is omitted too
        (struct_ == "ptp_clock_caps" && (loongarch64 || sparc64) && (["adjust_phase", "max_phase_adj", "rsv"].contains(&field)))
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
        // FIXME(musl): After musl 1.1.24, it have only one field `sched_priority`,
        // while other fields become reserved.
        (struct_ == "sched_param" && [
            "sched_ss_low_priority",
            "sched_ss_repl_period",
            "sched_ss_init_budget",
            "sched_ss_max_repl",
        ].contains(&field) && musl) ||
        // After musl 1.1.24, the type becomes `int` instead of `unsigned short`.
        (struct_ == "ipc_perm" && field == "__seq" && old_musl && aarch64) ||
        // glibc uses unnamed fields here and Rust doesn't support that yet
        (struct_ == "timex" && field.starts_with("__unused")) ||
        // FIXME(linux): It now takes mode_t since glibc 2.31 on some targets.
        (struct_ == "ipc_perm" && field == "mode"
            && ((x86_64 || i686 || arm || riscv64) && gnu || x86_64_gnux32)
        ) ||
        // the `u` field is in fact an anonymous union
        (gnu && struct_ == "ptrace_syscall_info" && (field == "u" || field == "pad")) ||
        // the vregs field is a `__uint128_t` C's type.
        (struct_ == "user_fpsimd_struct" && field == "vregs") ||
        // Linux >= 5.11 tweaked the `svm_zero` field of the `sockaddr_vm` struct.
        // https://github.com/torvalds/linux/commit/dc8eeef73b63ed8988224ba6b5ed19a615163a7f
        (struct_ == "sockaddr_vm" && field == "svm_zero") ||
        // the `ifr_ifru` field is an anonymous union
        (struct_ == "ifreq" && field == "ifr_ifru") ||
        // the `ifc_ifcu` field is an anonymous union
        (struct_ == "ifconf" && field == "ifc_ifcu") ||
        // glibc uses a single array `uregs` instead of individual fields.
        (struct_ == "user_regs" && arm) ||
        // the `ifr_ifrn` field is an anonymous union
        (struct_ == "iwreq" && field == "ifr_ifrn") ||
        // the `key` field is a zero-sized array
        (struct_ == "iw_encode_ext" && field == "key") ||
        // the `tcpi_snd_rcv_wscale` map two bitfield fields stored in a u8
        (struct_ == "tcp_info" && field == "tcpi_snd_rcv_wscale") ||
        // the `tcpi_delivery_fastopen_bitfields` map two bitfield fields stored in a u8
        (musl && struct_ == "tcp_info" && field == "tcpi_delivery_fastopen_bitfields") ||
        // either fsid_t or int[2] type
        (struct_ == "fanotify_event_info_fid" && field == "fsid") ||
        // `handle` is a VLA
        (struct_ == "fanotify_event_info_fid" && field == "handle") ||
        // `anonymous_1` is an anonymous union
        (struct_ == "ptp_perout_request" && field == "anonymous_1") ||
        // `anonymous_2` is an anonymous union
        (struct_ == "ptp_perout_request" && field == "anonymous_2") ||
        // FIXME(linux): `adjust_phase` requires >= 5.7 kernel headers
        // FIXME(linux): `max_phase_adj` requires >= 5.19 kernel headers
        // the rsv field shrunk when those fields got added, so is omitted too
        (struct_ == "ptp_clock_caps" && (loongarch64 || sparc64) && (["adjust_phase", "max_phase_adj", "rsv"].contains(&field))) ||
        // invalid application of 'sizeof' to incomplete type 'long unsigned int[]'
        (musl && struct_ == "mcontext_t" && field == "__extcontext" && loongarch64) ||
        // FIXME(#4121): a new field was added from `f_spare`
        (struct_ == "statvfs" && field == "__f_spare") ||
        (struct_ == "statvfs64" && field == "__f_spare") ||
        // the `xsk_tx_metadata_union` field is an anonymous union
        (struct_ == "xsk_tx_metadata" && field == "xsk_tx_metadata_union") ||
        // After musl 1.2.0, the type becomes `int` instead of `long`.
        (old_musl && struct_ == "utmpx" && field == "ut_session")
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME(1.0):
        "mcontext_t" if s390x => true,
        // FIXME(union): This is actually a union.
        "fpreg_t" if s390x => true,

        // The test doesn't work on some env:
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
            if sparc64 && gnu =>
        {
            true
        }

        // The following types contain Flexible Array Member fields which have unspecified calling
        // convention. The roundtripping tests deliberately pass the structs by value to check "by
        // value" layout consistency, but this would be UB for the these types.
        "inotify_event" => true,
        "fanotify_event_info_fid" => true,
        "cmsghdr" => true,

        // FIXME(linux): the call ABI of max_align_t is incorrect on these platforms:
        "max_align_t" if i686 || ppc64 => true,

        _ => false,
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");

    test_linux_like_apis(target);
}

// This function tests APIs that are incompatible to test when other APIs
// are included (e.g. because including both sets of headers clashes)
fn test_linux_like_apis(target: &str) {
    let gnu = target.contains("gnu");
    let musl = target.contains("musl") || target.contains("ohos");
    let linux = target.contains("linux");
    let wali = target.contains("linux") && target.contains("wasm32");
    let emscripten = target.contains("emscripten");
    let android = target.contains("android");
    assert!(linux || android || emscripten);

    if linux || android || emscripten {
        // test strerror_r from the `string.h` header
        let mut cfg = ctest_cfg();
        config_gnu_bits(target, &mut cfg);
        cfg.skip_type(|_| true).skip_static(|_| true);

        headers! { cfg: "string.h" }
        cfg.skip_fn(|f| match f {
            "strerror_r" => false,
            _ => true,
        })
        .skip_const(|_| true)
        .skip_struct(|_| true);
        cfg.generate(src_hotfix_dir().join("lib.rs"), "linux_strerror_r.rs");
    }

    if linux || android || emscripten {
        // test fcntl - see:
        // http://man7.org/linux/man-pages/man2/fcntl.2.html
        let mut cfg = ctest_cfg();
        config_gnu_bits(target, &mut cfg);

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
                t if is_struct => format!("struct {t}"),
                t if is_union => format!("union {t}"),
                t => t.to_string(),
            });

        cfg.generate(src_hotfix_dir().join("lib.rs"), "linux_fcntl.rs");
    }

    if (linux && !wali) || android {
        // test termios
        let mut cfg = ctest_cfg();
        config_gnu_bits(target, &mut cfg);
        cfg.header("asm/termbits.h");
        cfg.header("linux/termios.h");
        cfg.skip_type(|_| true)
            .skip_static(|_| true)
            .skip_fn(|_| true)
            .skip_const(|c| match c {
                "BOTHER" | "IBSHIFT" => false,
                "TCGETS2" | "TCSETS2" | "TCSETSW2" | "TCSETSF2" => false,
                _ => true,
            })
            .skip_struct(|s| s != "termios2")
            .type_name(move |ty, is_struct, is_union| match ty {
                "Ioctl" if gnu => "unsigned long".to_string(),
                "Ioctl" => "int".to_string(),
                t if is_struct => format!("struct {t}"),
                t if is_union => format!("union {t}"),
                t => t.to_string(),
            });
        cfg.generate(src_hotfix_dir().join("lib.rs"), "linux_termios.rs");
    }

    if linux || android {
        // test IPV6_ constants:
        let mut cfg = ctest_cfg();
        config_gnu_bits(target, &mut cfg);
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
                t if is_struct => format!("struct {t}"),
                t if is_union => format!("union {t}"),
                t => t.to_string(),
            });
        cfg.generate(src_hotfix_dir().join("lib.rs"), "linux_ipv6.rs");
    }

    if (linux && !wali) || android {
        // Test Elf64_Phdr and Elf32_Phdr
        // These types have a field called `p_type`, but including
        // "resolve.h" defines a `p_type` macro that expands to `__p_type`
        // making the tests for these fails when both are included.
        let mut cfg = ctest_cfg();
        config_gnu_bits(target, &mut cfg);
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
        cfg.generate(src_hotfix_dir().join("lib.rs"), "linux_elf.rs");
    }

    if (linux && !wali) || android {
        // Test `ARPHRD_CAN`.
        let mut cfg = ctest_cfg();
        config_gnu_bits(target, &mut cfg);
        cfg.header("linux/if_arp.h");
        cfg.skip_fn(|_| true)
            .skip_static(|_| true)
            .skip_const(move |name| match name {
                "ARPHRD_CAN" => false,
                _ => true,
            })
            .skip_struct(|_| true)
            .skip_type(|_| true);
        cfg.generate(src_hotfix_dir().join("lib.rs"), "linux_if_arp.rs");
    }
}

fn which_freebsd() -> Option<i32> {
    if let Ok(version) = env::var("RUST_LIBC_UNSTABLE_FREEBSD_VERSION") {
        let vers = version.parse().unwrap();
        println!("cargo:warning=setting FreeBSD version to {vers}");
        return Some(vers);
    }

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
        s if s.starts_with("14") => Some(14),
        s if s.starts_with("15") => Some(15),
        _ => None,
    }
}

fn test_haiku(target: &str) {
    assert!(target.contains("haiku"));

    let mut cfg = ctest_cfg();
    cfg.flag("-Wno-deprecated-declarations");
    cfg.define("__USE_GNU", Some("1"));
    cfg.define("_GNU_SOURCE", None);
    cfg.language(ctest::Lang::CXX);

    // POSIX API
    headers! { cfg:
               "alloca.h",
               "arpa/inet.h",
               "arpa/nameser.h",
               "arpa/nameser_compat.h",
               "assert.h",
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
               "utmpx.h",
               "wchar.h",
               "wchar_t.h",
               "wctype.h"
    }

    // BSD Extensions
    headers! { cfg:
               "ifaddrs.h",
               "libutil.h",
               "link.h",
               "pty.h",
               "stdlib.h",
               "stringlist.h",
               "sys/link_elf.h",
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
               "storage/FindDirectory.h",
               "storage/StorageDefs.h",
               "support/Errors.h",
               "support/SupportDefs.h",
               "support/TypeConstants.h"
    }

    cfg.skip_struct(move |ty| {
        if ty.starts_with("__c_anonymous_") {
            return true;
        }
        match ty {
            // FIXME(union): actually a union
            "sigval" => true,
            // FIXME(haiku): locale_t does not exist on Haiku
            "locale_t" => true,
            // FIXME(haiku): rusage has a different layout on Haiku
            "rusage" => true,
            // FIXME(haiku): complains that rust aligns on 4 byte boundary, but
            //         Haiku does not align it at all.
            "in6_addr" => true,
            // The d_name attribute is an array of 1 on Haiku, with the
            // intention that the developer allocates a larger or smaller
            // piece of memory depending on the expected/actual size of the name.
            // Other platforms have sensible defaults. In Rust, the d_name field
            // is sized as the _POSIX_MAX_PATH, so that path names will fit in
            // newly allocated dirent objects. This breaks the automated tests.
            "dirent" => true,
            // The following structs contain function pointers, which cannot be initialized
            // with mem::zeroed(), so skip the automated test
            "image_info" | "thread_info" => true,

            "Elf64_Phdr" => true,

            // is an union
            "cpuid_info" => true,

            _ => false,
        }
    });

    cfg.skip_type(move |ty| {
        match ty {
            // FIXME(haiku): locale_t does not exist on Haiku
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
            // FIXME(haiku): https://github.com/rust-lang/libc/issues/1272
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
            // uses an enum as a parameter argument, which is incorrectly
            // translated into a struct argument
            "find_path" => true,

            "get_cpuid" => true,

            // uses varargs parameter
            "ioctl" => true,

            _ => false,
        }
    });

    cfg.skip_const(move |name| {
        match name {
            // FIXME(haiku): these constants do not exist on Haiku
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
            // FIXME(time): the stat struct actually has timespec members, whereas
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
            ("fpu_state", "_fpreg") => true,
            ("cpu_topology_node_info", "data") => true,
            // these fields have a simplified data definition in libc
            ("fpu_state", "_xmm") => true,
            ("savefpu", "_fp_ymm") => true,

            // skip these enum-type fields
            ("thread_info", "state") => true,
            ("image_info", "image_type") => true,
            _ => false,
        }
    });

    cfg.skip_roundtrip(move |s| match s {
        // FIXME(1.0): for some reason the roundtrip check fails for cpu_info
        "cpu_info" => true,
        _ => false,
    });

    cfg.type_name(move |ty, is_struct, is_union| {
        match ty {
            // Just pass all these through, no need for a "struct" prefix
            "area_info"
            | "port_info"
            | "port_message_info"
            | "team_info"
            | "sem_info"
            | "team_usage_info"
            | "thread_info"
            | "cpu_info"
            | "system_info"
            | "object_wait_info"
            | "image_info"
            | "attr_info"
            | "index_info"
            | "fs_info"
            | "FILE"
            | "DIR"
            | "Dl_info"
            | "topology_level_type"
            | "cpu_topology_node_info"
            | "cpu_topology_root_info"
            | "cpu_topology_package_info"
            | "cpu_topology_core_info" => ty.to_string(),

            // enums don't need a prefix
            "directory_which" | "path_base_directory" | "cpu_platform" | "cpu_vendor" => {
                ty.to_string()
            }

            // is actually a union
            "sigval" => "union sigval".to_string(),
            t if is_union => format!("union {t}"),
            t if t.ends_with("_t") => t.to_string(),
            t if is_struct => format!("struct {t}"),
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
            "type_" if struct_ == "cpu_topology_node_info" => "type".to_string(),
            "image_type" if struct_ == "image_info" => "type".to_string(),
            s => s.to_string(),
        }
    });
    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}

fn test_aix(target: &str) {
    assert!(target.contains("aix"));

    // ctest generates arguments supported only by clang, so make sure to
    // run with CC=clang. While debugging, "CFLAGS=-ferror-limit=<large num>"
    // is useful to get more error output.
    let mut cfg = ctest_cfg();
    cfg.define("_THREAD_SAFE", None);

    // Avoid the error for definitions such as '{0, 0, 0, 1}' for
    // 'IN6ADDR_LOOPBACK_INIT' in netinent/in.h.
    cfg.flag("-Wno-missing-braces");

    headers! { cfg:
               "aio.h",
               "ctype.h",
               "dirent.h",
               "dlfcn.h",
               "errno.h",
               "fcntl.h",
               "fnmatch.h",
               "glob.h",
               "grp.h",
               "iconv.h",
               "langinfo.h",
               "libgen.h",
               "limits.h",
               "locale.h",
               "malloc.h",
               "mntent.h",
               "mqueue.h",
               "netinet/in.h", // this needs be before net/if.h
               "poll.h", // this needs be before net/if.h
               "sys/pollset.h", // this needs to be before net/if.h
               "net/if.h",
               "net/bpf.h", // this needs to be after net/if.h
               "net/if_dl.h",
               "netdb.h",
               "netinet/tcp.h",
               "pthread.h",
               "pwd.h",
               "rpcsvc/mount.h",
               "rpcsvc/rstat.h",
               "regex.h",
               "resolv.h",
               "sched.h",
               "search.h",
               "semaphore.h",
               "signal.h",
               "spawn.h",
               "stddef.h",
               "stdint.h",
               "stdio.h",
               "stdlib.h",
               "string.h",
               "strings.h",
               "sys/aacct.h",
               "sys/acct.h",
               "sys/dr.h",
               "sys/file.h",
               "sys/io.h",
               "sys/ioctl.h",
               "sys/ipc.h",
               "sys/ldr.h",
               "sys/mman.h",
               "sys/msg.h",
               "sys/reg.h",
               "sys/resource.h",
               "sys/sem.h",
               "sys/shm.h",
               "sys/socket.h",
               "sys/stat.h",
               "sys/statfs.h",
               "sys/statvfs.h",
               "sys/stropts.h",
               "sys/termio.h",
               "sys/time.h",
               "sys/times.h",
               "sys/types.h",
               "sys/uio.h",
               "sys/un.h",
               "sys/user.h",
               "sys/utsname.h",
               "sys/vattr.h",
               "sys/vminfo.h",
               "sys/wait.h",
               "sys/xti.h",
               "syslog.h",
               "termios.h",
               "thread.h",
               "time.h",
               "ucontext.h",
               "unistd.h",
               "utime.h",
               "utmp.h",
               "utmpx.h",
               "wchar.h",
    }

    cfg.skip_type(move |ty| match ty {
        // AIX does not define type 'sighandler_t'.
        "sighandler_t" => true,

        // The alignment of 'double' does not agree between C and Rust for AIX.
        // We are working on a resolution.
        "c_double" => true,

        _ => false,
    });

    cfg.type_name(move |ty, is_struct, is_union| match ty {
        "DIR" => ty.to_string(),
        "FILE" => ty.to_string(),
        "ACTION" => ty.to_string(),

        // 'sigval' is a struct in Rust, but a union in C.
        "sigval" => format!("union sigval"),

        t if t.ends_with("_t") => t.to_string(),
        t if is_struct => format!("struct {}", t),
        t if is_union => format!("union {}", t),
        t => t.to_string(),
    });

    cfg.skip_const(move |name| match name {
        // Skip 'sighandler_t' assignments.
        "SIG_DFL" | "SIG_ERR" | "SIG_IGN" => true,

        _ => false,
    });

    cfg.skip_struct(move |ty| {
        match ty {
            // FIXME(union): actually a union.
            "sigval" => true,

            // '__poll_ctl_ext_u' and '__pollfd_ext_u' are for unnamed unions.
            "__poll_ctl_ext_u" => true,
            "__pollfd_ext_u" => true,

            // 'struct fpreg_t' is not defined in AIX headers. It is created to
            // allow type 'double' to be used in signal contexts.
            "fpreg_t" => true,

            _ => false,
        }
    });

    cfg.skip_field_type(move |struct_, field| {
        match (struct_, field) {
            // AIX does not define 'sighandler_t'.
            ("sigaction", "sa_sigaction") => true,

            // The type of 'fpr' is 'fpreg_t' which is created to allow type
            // 'double' to be used in signal contexts.
            ("__context64", "fpr") => true,
            ("__tm_context_t", "fpr") => true,

            _ => false,
        }
    });

    cfg.skip_field(move |s, field| {
        match s {
            // The field 'u' is actually a unnamed union in the AIX header.
            "poll_ctl_ext" if field == "u" => true,

            // The field 'data' is actually a unnamed union in the AIX header.
            "pollfd_ext" if field == "data" => true,

            _ => false,
        }
    });

    cfg.skip_fn(move |name| {
        match name {
            // 'sighandler_t' is not defined on AIX.
            "signal" => true,

            // The function is only available under macro _USE_IRS in 'netdb.h'.
            "hstrerror" => true,

            // _ALL_SOURCE signatures for these functions differ from POSIX's
            // on AIX.
            "poll" => true,
            "readlinkat" => true,
            "readlink" => true,
            "pselect" => true,

            // The AIX signature differs from POSIX's, issue opened.
            "gai_strerror" => true,

            // AIX implements POSIX-compliant versions of these functions
            // using 'static' wrappers in the headers, which in turn call
            // the corresponding system libc functions prefixed with '_posix_'
            // (e.g., '_posix_aio_read' for 'aio_read').
            // On the Rust side, these functions resolve directly to the
            // POSIX-compliant versions in the system libc. As a result,
            // function pointer comparisons between the C and Rust sides
            // would fail.
            "getpwuid_r" | "getpwnam_r" | "getgrgid_r" | "getgrnam_r" | "aio_cancel"
            | "aio_error" | "aio_fsync" | "aio_read" | "aio_return" | "aio_suspend"
            | "aio_write" | "select" => true,

            // 'getdtablesize' is a constant in the AIX header but it is
            // a real function in libc which the Rust side is resolved to.
            // The function pointer comparison test would fail.
            "getdtablesize" => true,

            // FIXME(ctest): Our API is unsound. The Rust API allows aliasing
            // pointers, but the C API requires pointers not to alias.
            // We should probably be at least using '&'/'&mut' here, see:
            // https://github.com/gnzlbg/ctest/issues/68.
            "lio_listio" => true,

            // The function is only available under macro _KERNEL in 'proto_uipc.h'.
            "getpeereid" => true,

            _ => false,
        }
    });

    cfg.volatile_item(|i| {
        use ctest::VolatileItemKind::*;
        match i {
            // 'aio_buf' is of type 'volatile void**' but since we cannot
            // express that in Rust types, we have to explicitly tell the
            // checker about it here.
            StructField(ref n, ref f) if n == "aiocb" && f == "aio_buf" => true,

            _ => false,
        }
    });

    cfg.generate(src_hotfix_dir().join("lib.rs"), "main.rs");
}
