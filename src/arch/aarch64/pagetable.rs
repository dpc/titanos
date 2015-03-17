use core::iter::range;
use titanium::io::{VolatileAccess, Default};
use titanium::arch::reg::*;
use titanium::arch::mmu::*;
use titanium::consts::*;

use super::PAGE_SIZE;
use mm::PageArena;

const PTE_ATTRS_MMIO : u64 = 1 << PTE_XN::SHIFT;
const PTE_ATTRS_RAM : u64 = PTE_AP_RW << PTE_AP::SHIFT;

pub struct PageTable<A = Default>
where A : VolatileAccess {
    start : usize,
    _access : A,
}

impl PageTable<Default> {
    pub fn new(arena : &mut PageArena) -> PageTable<Default> {
        let start = arena.get();

        PageTable {
            start: start.unwrap(),
            _access: Default,
        }
    }
}

impl<A> PageTable<A>
where A : VolatileAccess {

    // TODO: This is so lame ...
    pub fn _map_all(&self) {
        for i in range(0, PAGE_SIZE / 8) {
            let pte_addr = self.start + i * 8;

            let addr = (i << SZ_512MB_SHIFT) as u64;
            let attr = if addr < 0x80000000 {
                PTE_ATTRS_MMIO
            } else {
                PTE_ATTRS_RAM
            };
            A::write_u64(pte_addr, PTE_TYPE_BLOCK | attr | addr);
        }
    }

    pub fn _start(&self) {
        let asid = 0;
        let addr = self.start as u64; // TODO: check alignment

        ttbr0_el1::write(
            asid << ttbr0_el1::ASID::SHIFT |
            addr << ttbr0_el1::BADDR::SHIFT
            );

        // invalidate all to PoU
        unsafe { asm!("ic iallu"); }
        dsb!();
        // TODO: invalidate i- and c- cache by set-way
        // TODO: move to head?

        unsafe { asm!("tlbi alle1"); }
        dsb!();
    }
}

