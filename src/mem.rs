use core::intrinsics::{volatile_set_memory};

use titanium::World;
use titanium::world;

extern {
    static mut _bss_clear_start: u8;
    static mut _bss_clear_end: u8;
}

fn bbs_memzero() {
    let start : usize = unsafe {&_bss_clear_start} as *const _ as usize;
    let end : usize = unsafe {&_bss_clear_end} as *const _ as usize;
    let size = end - start;
    unsafe {
        volatile_set_memory(&mut _bss_clear_start, 0, size);
    }
}

pub fn init(w : &mut world::Real) {

    w.local_irqs_disable();

    bbs_memzero();
}
