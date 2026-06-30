//! Header: `net/bpf.h`
//!
//! <https://github.com/apple-oss-distributions/xnu/blob/main/bsd/net/bpf.h>

use crate::prelude::*;

s! {
    pub struct bpf_insn {
        pub code: c_ushort,
        pub jt: c_uchar,
        pub jf: c_uchar,
        pub k: u32,
    }

    pub struct bpf_program {
        pub bf_len: c_uint,
        pub bf_insns: *mut bpf_insn,
    }
}
