    {#- Requires the presence of an `ident` variable +#}

    /// Tests whether the alias/struct/union `x` when passed to C and back to Rust remains unchanged.
    ///
    /// It checks if the size is the same as well as if the padding bytes are all in the
    /// correct place.
    pub fn roundtrip_{{ ident }}() {
        type U = {{ ident }};
        extern "C" {
            fn __test_roundtrip_{{ ident }}(
                size: i32, x: MaybeUninit<U>, e: *mut c_int, pad: *const bool
            ) -> U;
        }
        let pad = roundtrip_padding_{{ ident }}();
        assert_eq!(pad.len(), size_of::<U>());
        unsafe {
            let mut error: c_int = 0;
            // Fill both x and y with non-zero, deterministic test patterns
            // Here the pattern is every byte that is a multiple of 256 is set to 42,
            // and the rest are filled incrementally for c, decrementally for d.
            // We use volatile writes to prevent compiler optimization.
            let mut y = MaybeUninit::<U>::zeroed();
            let mut x = MaybeUninit::<U>::zeroed();
            let x_ptr = x.as_mut_ptr().cast::<u8>();
            let y_ptr = y.as_mut_ptr().cast::<u8>();
            let sz = size_of::<U>();
            for i in 0..sz {
                let c: u8 = (i % 256) as u8;
                let c = if c == 0 { 42 } else { c };
                let d: u8 = 255_u8 - (i % 256) as u8;
                let d = if d == 0 { 42 } else { d };
                x_ptr.add(i).write_volatile(c);
                y_ptr.add(i).write_volatile(d);
            }
            // Now we test that the data sent from Rust to C is the same, and from C to Rust is
            // also the same.
            let r: U = __test_roundtrip_{{ ident }}(sz as i32, x, &mut error, pad.as_ptr());
            if error != 0 {
                FAILED.store(true, Ordering::Relaxed);
                return;
            }
            for (i, elem) in pad.iter().enumerate() {
                if *elem { continue; }
                let rust = (*y_ptr.add(i)) as usize;
                let c = (&raw const r).cast::<u8>().add(i).read_volatile() as usize;
                if rust != c {
                    eprintln!(
                        "rust [{i}] = {rust} != {c} (C): C \"{{ ident }}\" -> Rust",
                    );
                    FAILED.store(true, Ordering::Relaxed);
                }
            }
        }
    }