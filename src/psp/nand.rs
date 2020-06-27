use super::c_void;
extern "C" {
    pub fn sceNandSetWriteProtect(protect_flag: i32) -> i32;
    pub fn sceNandLock(write_flag: i32) -> i32;
    pub fn sceNandUnlock();
    pub fn sceNandReadStatus() -> i32;
    pub fn sceNandReset(flag: i32) -> i32;
    pub fn sceNandReadId(buf: *mut c_void, size: usize) -> i32;
    pub fn sceNandReadPages(
        ppn: u32,
        buf: *mut c_void,
        buf2: *mut c_void,
        count: u32,
    ) -> i32;
    pub fn sceNandGetPageSize() -> i32;
    pub fn sceNandGetPagesPerBlock() -> i32;
    pub fn sceNandGetTotalBlocks() -> i32;
    pub fn sceNandReadBlockWithRetry(
        ppn: u32,
        buf: *mut c_void,
        buf2: *mut c_void,
    ) -> i32;
    pub fn sceNandIsBadBlock(ppn: u32) -> i32;
}
