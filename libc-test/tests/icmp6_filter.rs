//! Compare libc's ICMP6_FILTER family of functions against the actual C macros,
//! for every ICMPv6 type.

#[cfg(any(
    all(target_os = "linux", not(target_arch = "wasm32")),
    target_os = "android",
    target_os = "emscripten",
    target_vendor = "apple",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris",
    target_os = "illumos",
))]
mod t {
    use std::{
        mem,
        slice,
    };

    use libc::{
        self,
        icmp6_filter,
    };

    extern "C" {
        fn icmp6_filter_willpass(typ: u8, filt: *const icmp6_filter) -> bool;
        fn icmp6_filter_willblock(typ: u8, filt: *const icmp6_filter) -> bool;
        fn icmp6_filter_setpassall(filt: *mut icmp6_filter);
        fn icmp6_filter_setblockall(filt: *mut icmp6_filter);
        fn icmp6_filter_setpass(typ: u8, filt: *mut icmp6_filter);
        fn icmp6_filter_setblock(typ: u8, filt: *mut icmp6_filter);
    }

    fn as_bytes(filt: &icmp6_filter) -> &[u8] {
        let ptr = (filt as *const icmp6_filter).cast::<u8>();
        unsafe { slice::from_raw_parts(ptr, mem::size_of::<icmp6_filter>()) }
    }

    /// A filter that passes every type, built by the C macro.
    fn passall() -> icmp6_filter {
        unsafe {
            let mut filt = mem::zeroed();
            icmp6_filter_setpassall(&mut filt);
            filt
        }
    }

    /// A filter that blocks every type, built by the C macro.
    fn blockall() -> icmp6_filter {
        unsafe {
            let mut filt = mem::zeroed();
            icmp6_filter_setblockall(&mut filt);
            filt
        }
    }

    #[test]
    fn test_icmp6_filter_setpassall() {
        let mut filt = blockall();
        libc::ICMP6_FILTER_SETPASSALL(&mut filt);
        assert_eq!(as_bytes(&filt), as_bytes(&passall()));
    }

    #[test]
    fn test_icmp6_filter_setblockall() {
        let mut filt = passall();
        libc::ICMP6_FILTER_SETBLOCKALL(&mut filt);
        assert_eq!(as_bytes(&filt), as_bytes(&blockall()));
    }

    #[test]
    fn test_icmp6_filter_setpass() {
        for init in [passall(), blockall()] {
            for typ in 0..=u8::MAX {
                let mut filt1 = init;
                let mut filt2 = init;
                libc::ICMP6_FILTER_SETPASS(typ, &mut filt1);
                unsafe { icmp6_filter_setpass(typ, &mut filt2) };
                assert_eq!(as_bytes(&filt1), as_bytes(&filt2), "type {typ}");
            }
        }
    }

    #[test]
    fn test_icmp6_filter_setblock() {
        for init in [passall(), blockall()] {
            for typ in 0..=u8::MAX {
                let mut filt1 = init;
                let mut filt2 = init;
                libc::ICMP6_FILTER_SETBLOCK(typ, &mut filt1);
                unsafe { icmp6_filter_setblock(typ, &mut filt2) };
                assert_eq!(as_bytes(&filt1), as_bytes(&filt2), "type {typ}");
            }
        }
    }

    #[test]
    fn test_icmp6_filter_willpass_willblock() {
        let mut filters = vec![passall(), blockall()];
        for typ in 0..=u8::MAX {
            let mut pass_one = blockall();
            let mut block_one = passall();
            unsafe {
                icmp6_filter_setpass(typ, &mut pass_one);
                icmp6_filter_setblock(typ, &mut block_one);
            }
            filters.push(pass_one);
            filters.push(block_one);
        }

        for filt in &filters {
            for typ in 0..=u8::MAX {
                unsafe {
                    assert_eq!(
                        libc::ICMP6_FILTER_WILLPASS(typ, filt),
                        icmp6_filter_willpass(typ, filt),
                        "type {typ}"
                    );
                    assert_eq!(
                        libc::ICMP6_FILTER_WILLBLOCK(typ, filt),
                        icmp6_filter_willblock(typ, filt),
                        "type {typ}"
                    );
                }
            }
        }
    }
}
