pub mod pagetable;

use titanium::arch::reg::{mpidr_el1, vbar_el1};
use core::intrinsics::{transmute};

pub const PAGE_SIZE : usize = 64 * 1024;

#[allow(private_no_mangle_fns)]
#[no_mangle]
pub unsafe extern fn isr_handler_wrapper() {
    asm!("" :::: "volatile");
    pr_info!("ISR");
    loop {}
}

pub fn set_vbar() {
    vbar_el1::write(unsafe {transmute(isr_handler_wrapper)});

}
pub fn cpu_id() -> u8 {
    (mpidr_el1::read() & mpidr_el1::AFF0::MASK) as u8
}
