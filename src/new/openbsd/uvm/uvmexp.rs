//! Header: `uvm/uvmexp.h`
//!
//! <https://github.com/openbsd/src/blob/master/sys/uvm/uvmexp.h>

use crate::prelude::*;

pub const VM_METER: c_int = 1;
pub const VM_LOADAVG: c_int = 2;
pub const VM_PSSTRINGS: c_int = 3;
pub const VM_UVMEXP: c_int = 4;
pub const VM_SWAPENCRYPT: c_int = 5;
pub const VM_NKMEMPAGES: c_int = 6;
pub const VM_ANONMIN: c_int = 7;
pub const VM_VTEXTMIN: c_int = 8;
pub const VM_VNODEMIN: c_int = 9;
pub const VM_MAXSLP: c_int = 10;
pub const VM_USPACE: c_int = 11;
pub const VM_MALLOC_CONF: c_int = 12;
/// Constants may change across releases.
/// See the [usage guidelines](crate#usage-guidelines) for details.
pub const VM_MAXID: c_int = 13;

s! {
    pub struct uvmexp {
        pub pagesize: c_int,
        pub pagemask: c_int,
        pub pageshift: c_int,
        pub npages: c_int,
        pub free: c_int,
        pub active: c_int,
        pub inactive: c_int,
        pub paging: c_int,
        pub wired: c_int,
        pub zeropages: c_int,
        pub reserve_pagedaemon: c_int,
        pub reserve_kernel: c_int,
        pub percpucaches: c_int,
        pub vnodepages: c_int,
        pub vtextpages: c_int,
        pub freemin: c_int,
        pub freetarg: c_int,
        pub inactart: c_int,
        pub wiredmax: c_int,
        pub anonmin: c_int,
        pub vtextmin: c_int,
        pub vnodemin: c_int,
        pub anonminpct: c_int,
        pub vtextminpct: c_int,
        pub vnodeminpct: c_int,
        pub nswapdev: c_int,
        pub swpages: c_int,
        pub swpginuse: c_int,
        pub swpgonly: c_int,
        pub nswget: c_int,
        pub nanon: c_int,
        pub unused05: c_int,
        pub unused06: c_int,
        pub faults: c_int,
        pub traps: c_int,
        pub intrs: c_int,
        pub swtch: c_int,
        pub softs: c_int,
        pub syscalls: c_int,
        pub pageins: c_int,
        pub pcphit: c_int,
        pub pcpmiss: c_int,
        pub pgswapin: c_int,
        pub pgswapout: c_int,
        pub forks: c_int,
        pub forks_ppwait: c_int,
        pub forks_sharevm: c_int,
        pub pga_zerohit: c_int,
        pub pga_zeromiss: c_int,
        pub unused09: c_int,
        pub fltnoram: c_int,
        pub fltnoanon: c_int,
        pub fltnoamap: c_int,
        pub fltpgwait: c_int,
        pub fltpgrele: c_int,
        pub fltrelck: c_int,
        pub fltnorelck: c_int,
        pub fltanget: c_int,
        pub fltanretry: c_int,
        pub fltamcopy: c_int,
        pub fltnamap: c_int,
        pub fltnomap: c_int,
        pub fltlget: c_int,
        pub fltget: c_int,
        pub flt_anon: c_int,
        pub flt_acow: c_int,
        pub flt_obj: c_int,
        pub flt_prcopy: c_int,
        pub flt_przero: c_int,
        pub fltup: c_int,
        pub fltnoup: c_int,
        pub pdwoke: c_int,
        pub pdrevs: c_int,
        pub pdswout: c_int,
        pub pdfreed: c_int,
        pub pdscans: c_int,
        pub pdanscan: c_int,
        pub pdobscan: c_int,
        pub pdreact: c_int,
        pub pdbusy: c_int,
        pub pdpageouts: c_int,
        pub pdpending: c_int,
        pub pddeact: c_int,
        pub swpskip: c_int,
        pub fpswtch: c_int,
        pub kmapent: c_int,
    }
}

s! {
    pub struct ps_strings {
        pub val: *mut c_void,
    }
}

c_enum! {
    pub enum uvm_exp_counters {
        pub faults,
        pub pageins,
        pub flt_noram,
        pub flt_noanon,
        pub flt_noamap,
        pub flt_pgwait,
        pub flt_pgrele,
        pub flt_relck,
        pub flt_norelck,
        pub flt_anget,
        pub flt_anretry,
        pub flt_amcopy,
        pub flt_namap,
        pub flt_nomap,
        pub flt_lget,
        pub flt_get,
        pub flt_anon,
        pub flt_acow,
        pub flt_obj,
        pub flt_prcopy,
        pub flt_przero,
        pub flt_up,
        pub flt_noup,
        pub exp_ncounters,
    }
}
