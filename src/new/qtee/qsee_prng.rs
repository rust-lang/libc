//! Header: `qsee_prng.h`

extern "C" {
    pub fn qsee_prng_getdata(out: *mut u8, out_len: u32) -> u32;
}
