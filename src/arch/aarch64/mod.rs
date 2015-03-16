mod pagetable;

pub use titanium::arch::consts::*;
pub use self::pagetable::PageTable;

pub const NAME : &'static str = "aarch64";

pub const PTE_ATTRS_MMIO : u64 = 1 << PTE_XN_SHIFT;
pub const PTE_ATTRS_RAM : u64 = PTE_AP_RW << PTE_AP_SHIFT;
pub const PAGE_SIZE : usize = 64 * 1024;

pub fn cpu_id() -> u8 {
    (reg64_read!(mpidr_el1) & MPIDR_AFF0_MASK) as u8
}
