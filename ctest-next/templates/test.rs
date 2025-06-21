use std::mem;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

{% if rustc_version < RustcVersion::new(1, 30, 0) %}
    static FAILED: AtomicBool = std::sync::atomic::ATOMIC_BOOL_INIT;
    static NTESTS: AtomicUsize = std::sync::atomic::ATOMIC_USIZE_INIT;
{% else %}
    static FAILED: AtomicBool = AtomicBool::new(false);
    static NTESTS: AtomicUsize = AtomicUsize::new(0);
{% endif %}

fn main() {
    eprintln!("RUNNING ALL TESTS");
    run_all();
    if FAILED.load(Ordering::SeqCst) {
        panic!("some tests failed");
    } else {
        eprintln!("PASSED {} tests", NTESTS.load(Ordering::SeqCst));
    }
}

trait Pretty {
    fn pretty(&self) -> String;
}

impl<'a> Pretty for &'a str {
    fn pretty(&self) -> String { format!("{:?}", self) }
}
impl<T> Pretty for *const T {
    fn pretty(&self) -> String { format!("{:?}", self) }
}
impl<T> Pretty for *mut T {
    fn pretty(&self) -> String { format!("{:?}", self) }
}
macro_rules! p {
    ($($i:ident)*) => ($(
        impl Pretty for $i {
            fn pretty(&self) -> String {
                format!("{} ({:#x})", self, self)
            }
        }
    )*)
}
p! { i8 i16 i32 i64 u8 u16 u32 u64 usize isize }

fn same<T: Eq + Pretty>(rust: T, c: T, attr: &str) {
    if rust != c {
        eprintln!("bad {}: rust: {} != c {}", attr, rust.pretty(),
                    c.pretty());
        FAILED.store(true, Ordering::SeqCst);
    } else {
        NTESTS.fetch_add(1, Ordering::SeqCst);
    }
}

macro_rules! offset_of {
    ($ty:ident, $field:ident) => ({
        let value = std::mem::MaybeUninit::<$ty>::uninit();
        let base_pointer = value.as_ptr();
        let offset_pointer = std::ptr::addr_of!((*base_pointer).$field);
        (offset_pointer as u64) - (base_pointer as u64)
    })
}

{% for constant in ffi_items.constants() %}
    {% include "constants/test_constant.rs" %}
{% endfor %}

#[inline(never)]
fn run_all() {
    {% for constant in ffi_items.constants() %}
        const_{{ constant.ident() }}();
    {% endfor %}
}