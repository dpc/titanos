use core::intrinsics::{volatile_set_memory};
extern {
    static mut _bss_start: u8;
    static mut _bss_end: u8;
}

fn bbs_memzero() {
    let start : usize = unsafe {&_bss_start} as *const _ as usize;
    let end : usize = unsafe {&_bss_end} as *const _ as usize;
    let size = end - start;
    unsafe {
        volatile_set_memory(&mut _bss_start, 0, size);
    }
}

pub fn init() {
    bbs_memzero();
}
