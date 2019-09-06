#[repr(C)] struct JmpBuf([i64; 64]);

use std::mem::MaybeUninit;
static mut JMP_BUF: MaybeUninit<JmpBuf> = MaybeUninit::uninit();

extern "C" {
    fn longjmp(env: *mut JmpBuf, status: i64);
    fn setjmp(env: *mut JmpBuf) -> i64;
}

unsafe fn jumps() {
    if setjmp(JMP_BUF.as_mut_ptr()) != 0 {
        return;
    }
    std::panic::catch_unwind(|| longjmp(JMP_BUF.as_mut_ptr(), 1)).unwrap();
    panic!("oh noes");
}

fn main() {
    unsafe { jumps() };
    println!("Hello, world!");
}
