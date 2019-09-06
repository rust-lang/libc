#![feature(unwind_attributes)]
#[repr(C, align(16))] struct JmpBuf([u8; 2048]);

use std::mem::MaybeUninit;
static mut JMP_BUF: MaybeUninit<JmpBuf> = MaybeUninit::zeroed();

extern "C" {
    fn longjmp(env: *mut JmpBuf, status: i32) -> !;
    fn setjmp(env: *mut JmpBuf) -> i32;
}

unsafe extern "C" fn oh_noes() {
    longjmp(JMP_BUF.as_mut_ptr(), 1);
    panic!("ohh noes");
}

unsafe fn jumps() {
    if setjmp(JMP_BUF.as_mut_ptr()) != 0 {
        return;
    }
    oh_noes();
    panic!("oh noes");
}

fn main() {
    unsafe { jumps() };
    println!("Hello, world!");
}
