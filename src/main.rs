#![feature(unwind_attributes)]


const LEN: usize = 4096;
#[repr(C, align(16))] struct JmpBuf([u8; LEN]);
static mut JMP_BUF: JmpBuf = JmpBuf([0; LEN]);

extern "C" {
    fn longjmp(env: *mut JmpBuf, status: i32) -> !;
    fn setjmp(env: *mut JmpBuf) -> i32;
}

unsafe extern "C" fn oh_noes() {
    longjmp(&mut JMP_BUF as *mut _, 1);
    panic!("ohh noes");
}

unsafe fn jumps() {
    if setjmp(&mut JMP_BUF as *mut _) != 0 {
        return;
    }
    oh_noes();
    panic!("oh noes");
}

fn main() {
    unsafe { jumps() };
    println!("Hello, world!");
}
