//! Compare libc's ICMP6_FILTER functions against the actual C macros, for
//! various inputs.

#[cfg(unix)]
mod t {

    use std::mem;

    use libc::{
        self,
        icmp6_filter,
    };

    extern "C" {
        pub fn icmp6_filter_willpass(typ: u8, filt: *const icmp6_filter) -> bool;
        pub fn icmp6_filter_willblock(typ: u8, filt: *const icmp6_filter) -> bool;
        pub fn icmp6_filter_setpassall(filt: *mut icmp6_filter);
        pub fn icmp6_filter_setblockall(filt: *mut icmp6_filter);
        pub fn icmp6_filter_setpass(typ: u8, filt: *mut icmp6_filter);
        pub fn icmp6_filter_setblock(typ: u8, filt: *mut icmp6_filter);
    }

    // Two ICMPv6 filters are equal if they agree on all ICMPv6 types
    // There are only 255 types so we can be complete.
    fn assert_filters_eq(filt1: &icmp6_filter, filt2: &icmp6_filter) {
        for typ in 0..255 {
            unsafe {
                assert_eq!(
                    icmp6_filter_willpass(typ, filt1),
                    icmp6_filter_willpass(typ, filt2)
                );
                assert_eq!(
                    icmp6_filter_willblock(typ, filt1),
                    icmp6_filter_willblock(typ, filt2)
                );
            }
        }
    }

    #[test]
    fn test_icmp6_filter_setpassall() {
        unsafe {
            let mut filt1 = mem::zeroed::<icmp6_filter>();
            let mut filt2 = mem::zeroed::<icmp6_filter>();
            libc::ICMP6_FILTER_SETPASSALL(&mut filt1);
            icmp6_filter_setpassall(&mut filt2);
            assert_filters_eq(&filt1, &filt2);
        }
    }

    #[test]
    fn test_icmp6_filter_setblockall() {
        unsafe {
            let mut filt1 = mem::zeroed::<icmp6_filter>();
            let mut filt2 = mem::zeroed::<icmp6_filter>();
            libc::ICMP6_FILTER_SETBLOCKALL(&mut filt1);
            icmp6_filter_setblockall(&mut filt2);
            assert_filters_eq(&filt1, &filt2);
        }
    }

    #[test]
    fn test_icmp6_filter_setblock() {
        for typ in 0..255 {
            unsafe {
                let mut filt1 = mem::zeroed::<icmp6_filter>();
                let mut filt2 = mem::zeroed::<icmp6_filter>();
                icmp6_filter_setpassall(&mut filt1);
                icmp6_filter_setpassall(&mut filt2);
                libc::ICMP6_FILTER_SETBLOCK(typ, &mut filt1);
                icmp6_filter_setblock(typ, &mut filt2);
                assert_filters_eq(&filt1, &filt2);
            }
        }
    }

    #[test]
    fn test_icmp6_filter_setpass() {
        for typ in 0..255 {
            unsafe {
                let mut filt1 = mem::zeroed::<icmp6_filter>();
                let mut filt2 = mem::zeroed::<icmp6_filter>();
                icmp6_filter_setblockall(&mut filt1);
                icmp6_filter_setblockall(&mut filt2);
                libc::ICMP6_FILTER_SETPASS(typ, &mut filt1);
                icmp6_filter_setpass(typ, &mut filt2);
                assert_filters_eq(&filt1, &filt2);
            }
        }
    }

    #[test]
    fn test_icmp6_filter_willpass_willblock() {
        unsafe {
            let mut filt1 = mem::zeroed::<icmp6_filter>();
            let mut filt2 = mem::zeroed::<icmp6_filter>();
            icmp6_filter_setblockall(&mut filt1);
            icmp6_filter_setblockall(&mut filt2);

            let mut seed = 0xdeadbeefu32;
            for _ in 0..255 {
                seed = seed.wrapping_mul(0x915f77f5);
                let typ = (seed >> 23) as u8;
                if (seed >> 31) & 1 == 0 {
                    icmp6_filter_setblock(typ, &mut filt1);
                    libc::ICMP6_FILTER_SETBLOCK(typ, &mut filt2);
                } else {
                    icmp6_filter_setpass(typ, &mut filt1);
                    libc::ICMP6_FILTER_SETPASS(typ, &mut filt2);
                }
                assert_filters_eq(&filt1, &filt2);
            }
        }
    }
}
