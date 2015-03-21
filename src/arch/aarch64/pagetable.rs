#![allow(unused)]
use core::mem;
use core;
use core::option::Option;
use core::option::Option::{Some, None};
use core::intrinsics::transmute;

use titanium::io::{VolatileAccess, Default};
use titanium::arch::reg::*;
use titanium::arch::mmu::*;
use titanium::arch::*;
use titanium::consts::*;
pub use titanium::drv;
pub use titanium::world;

use mm::PageArena;

const ENTRIES : usize = 8192;
const _PER_LEVEL : u64 = 13;
const PAGE_SIZE : u64  = 64 * 1024;

const START_LEVEL : u64 = 2;

/// Region size at a given level of translation
const REGION_SIZE : [u64; 5] = [0, ENTRIES as u64 * SZ_512MB as u64, SZ_512MB as u64, SZ_64KB as u64, 1];
const IDX_MASK : [u64; 5] = [0, 0, L2_IDX::MASK, L3_IDX::MASK, 0];

def_bitfields!(u64,
               L2_IDX(41, 29),
               L3_IDX(28, 16),
               LOW(16, 0),
               );

const TNSZ : u64 = 22;
const IA_WIDTH : u64 = 42; // IA[41:16]

const PTE_ATTRS_MMIO : u64 = 1 << pte::XN::SHIFT;
const PTE_ATTRS_RAM : u64 = pte::AP_RW << pte::AP::SHIFT;

#[repr(C)]
struct Pte(u64);

impl Pte {

    fn from_u64(val : u64) -> Pte {
        Pte(val)
    }

    fn as_table(&self) -> Option<&PageTable> {
        let &Pte(p) = self;

        if pte::TYPE::from(p) == pte::TYPE_TABLE {
            Some(unsafe { transmute(self) })
        } else {
            None
        }
    }
}

#[repr(C)]
struct PageTable {
    pub entries : [Pte; ENTRIES],
}


impl core::ops::Index<usize> for PageTable {
    type Output = Pte;

    fn index<'a>(&'a self, idx : &usize) -> &'a Pte {
        &self.entries[*idx]
    }
}

impl core::ops::IndexMut<usize> for PageTable {
    fn index_mut<'a>(&'a mut self, idx : &usize) -> &'a mut Pte {
        &mut self.entries[*idx]
    }
}

selftest!(page_table_size (_bla : &mut drv::uart::UartWriter<world::Real>) {
    mem::size_of::<PageTable>() == PAGE_SIZE as usize
});

selftest!(page_table (_bla : &mut drv::uart::UartWriter<world::Real>) {
    true
});

pub struct PageTableController<A = Default>
where A : VolatileAccess {
    start : usize,
    _access : A,
}

impl PageTableController<Default> {
    pub fn new(arena : &mut PageArena) -> PageTableController<Default> {
        let start = arena.get();

        PageTableController {
            start: start.unwrap(),
            _access: Default,
        }
    }
}

impl<A> PageTableController<A>
where A : VolatileAccess {

    pub fn root(&self) -> &mut PageTable {
        unsafe {
            mem::transmute(self.start)
        }
    }

    pub fn map_recv(&self, start_va : u64, start_pa : u64, size : u64, attr : u64, level : usize) {

        let mut va = start_va;
        let mut pa = start_va;
        let region_size = REGION_SIZE[level as usize];
        let idx_mask = IDX_MASK[level];
        let sub_mask = REGION_SIZE[level] - 1;

        loop {
            let i = idx_mask & va;
            let needs_block = (va & sub_mask != 0) || (pa & sub_mask != 0);

        }
    }

    pub fn map(&self, va : u64, pa : u64, size : u64) {

        let level = START_LEVEL as usize;

        self.map_recv(va, pa, size, 0, level);
    }


    // TODO: This is so lame ...
    pub fn map_all(&self) {
        let table = self.root();

        for i in (0..PAGE_SIZE as usize / 8) {

            let addr = (i << SZ_512MB_SHIFT) as u64;
            let attr = if addr < 0x80000000 {
                PTE_ATTRS_MMIO
            } else {
                PTE_ATTRS_RAM
            };

            table[i] = Pte::from_u64(pte::TYPE_BLOCK | attr | addr);
        }
    }

    pub fn start(&self) {
        let asid = 0;
        let addr = self.start as u64; // TODO: check alignment

        ttbr0_el1::write(
            asid << ttbr0_el1::ASID::SHIFT |
            addr << ttbr0_el1::BADDR::SHIFT
            );


        ttbr1_el1::write(
            asid << ttbr0_el1::ASID::SHIFT |
            addr << ttbr0_el1::BADDR::SHIFT
            );

        // invalidate all to PoU
        unsafe { asm!("ic ialluis" :::: "volatile"); }
        dsb_sy();
        isb();

        // TODO: invalidate i- and c- cache by set-way
        // TODO: move to head?

        // TODO: fails ATM
        // unsafe { asm!("tlbi alle1is" :::: "volatile"); }
        dsb_sy();
        isb();
    }
}

