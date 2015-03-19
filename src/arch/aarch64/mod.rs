mod pagetable;

use titanium::arch::reg::mpidr_el1;
pub use self::pagetable::{PageTableController};

pub const PAGE_SIZE : usize = 64 * 1024;

pub fn cpu_id() -> u8 {
    (mpidr_el1::read() & mpidr_el1::AFF0::MASK) as u8
}
