use core::iter::range;
use titanium::io::{VolatileAccess, Default};
use titanium::consts::*;
use titanium::arch::consts::*;

use arch::{PTE_ATTRS_MMIO, PTE_ATTRS_RAM, PAGE_SIZE};
use mm::PageArena;

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
    pub fn map_all(&self) {
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

    pub fn start(&self) {
        let asid = 0;
        let addr = self.start as u64; // TODO: check alignment

        reg64_write!(ttbr0_el1,
                     asid << TTBR_ASID_SHIFT |
                     addr << TTBR_BADDR_SHIFT);

        // invalidate all to PoU
        reg32_write!(iciallu, 0);
        dsb!();
        // TODO: invalidate i- and c- cache by set-way
        // TODO: move to head?

        reg32_write!(tlbiall, 0);
        dsb!();
    }
}

