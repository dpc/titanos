#![allow(unused)]
use core::mem;
use core;
use core::option::Option;
use core::option::Option::{Some, None};
use core::intrinsics::transmute;

use titanium::arch::reg::*;
use titanium::arch::mmu::*;
use titanium::arch::*;
use titanium::consts::*;
pub use titanium::drv;
pub use titanium::hw;

use mm::PageArena;
use World;

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

    fn as_table(&self) -> Option<&PageTableRaw> {
        let &Pte(p) = self;

        if pte::TYPE::from(p) == pte::TYPE_TABLE {
            Some(unsafe { transmute(pte::ADDR::from(p)) })
        } else {
            None
        }
    }
}

#[repr(C)]
struct PageTableRaw {
    pub entries : [Pte; ENTRIES],
}

struct PageTable {
    raw: &'static mut PageTableRaw,
    level : u8,
}

impl PageTable {
    pub fn map_recv(&mut self, start_va : u64, start_pa : u64, size : u64, attr : u64) {

        let mut va = start_va;
        let mut pa = start_va;
        let region_size = REGION_SIZE[self.level as usize];
        let idx_mask = IDX_MASK[self.level as usize];
        let sub_mask = REGION_SIZE[self.level as usize] - 1;

        loop {
            let i = idx_mask & va;
            let needs_subtable = (va & sub_mask != 0) || (pa & sub_mask != 0);

        }
    }
}

impl core::ops::Index<usize> for PageTableRaw {
    type Output = Pte;

    fn index<'a>(&'a self, idx : usize) -> &'a Pte {
        &self.entries[idx]
    }
}

impl core::ops::IndexMut<usize> for PageTableRaw {
    fn index_mut<'a>(&'a mut self, idx : usize) -> &'a mut Pte {
        &mut self.entries[idx]
    }
}

selftest!(page_table_size (_bla : &mut drv::uart::UartWriter) {
    mem::size_of::<PageTableRaw>() == PAGE_SIZE as usize
});

pub struct PageTableRoot {
    root : u64,
    level : u8,
}

impl PageTableRoot {
    pub fn new(world : &mut World<hw::Real>) -> PageTableRoot {
        let start = world.page_alloc.get();

        PageTableRoot {
            root: start.unwrap() as u64,
            level: START_LEVEL as u8,
        }
    }
}

impl PageTableRoot {

    pub fn root(&self) -> PageTable {
        PageTable {
            raw: unsafe { mem::transmute(self.root) },
            level: START_LEVEL as u8,
        }
    }

    pub fn map(&self, va : u64, pa : u64, size : u64) {
        self.root().map_recv(va, pa, size, 0);
    }

    pub fn start(&self) {
        let asid = 0;
        let addr = self.root; // TODO: check alignment

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

