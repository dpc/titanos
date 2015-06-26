#![allow(unused)]
use core::mem;
use core;
use core::cmp;
use core::ops::{Fn,FnMut};
use core::option::Option;
use core::option::Option::{Some, None};
use core::intrinsics::transmute;

use titanium::arch::reg::*;
use titanium::arch::mmu::*;
use titanium::arch::*;
use titanium::consts::*;
pub use titanium::drv;
pub use titanium::hw;

use core::ops;
use mm::PageArena;
use World;

const ENTRIES : usize = 8192;
const _PER_LEVEL : u64 = 13;
const PAGE_SIZE : u64  = 64 * 1024;

const START_LEVEL : u8 = 2;
const END_LEVEL : u8 = 3;

/// Region size at a given level of translation
const REGION_SIZE : [u64; 4] = [0, ENTRIES as u64 * SZ_512MB as u64, SZ_512MB as u64, SZ_64KB as u64];
const IDX_MASK : [u64; 4] = [0, 0, L2_IDX::MASK, L3_IDX::MASK];
const IDX_SHIFT : [u64; 4] = [0, 0, L2_IDX::SHIFT, L3_IDX::SHIFT];

def_bitfields!(u64,
               L2_IDX(41, 29),
               L3_IDX(28, 16),
               LOW(16, 0),
               );

const TNSZ : u64 = 22;
const IA_WIDTH : u64 = 42; // IA[41:16]

const PTE_ATTRS_MMIO : u64 = 1 << pte::XN::SHIFT;
const PTE_ATTRS_RAM : u64 = pte::AP_RW << pte::AP::SHIFT;

/// Raw PTE, being just u64
#[repr(C)]
#[derive(Copy, Clone)]
struct PteRaw(u64);

/// PTE is a reference to Raw PTE
/// and the level of it
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pte<'a> {
    raw : &'a PteRaw,
    level : u8,
    va : u64,
}

/// PteMut is a mut reference to Raw PTE
/// and the level of it
#[repr(C)]
pub struct PteMut<'a> {
    raw : &'a mut PteRaw,
    level : u8,
    va : u64,
}

impl<'a> ops::Deref for PteMut<'a> {
    type Target = Pte<'a>;

    fn deref(&self) -> &Pte<'a> {
       unsafe { mem::transmute(&self) }
    }
}

impl<'a> Pte<'a> {
    fn as_raw<'b>(&'b self) -> &'b u64 {
        let &PteRaw(ref raw) = self.raw;
        raw
    }

    fn can_be_table(&self) -> bool {
        self.level != END_LEVEL
    }

    fn is_valid(&self) -> bool {
        pte::TYPE::from(*self.as_raw()) == pte::TYPE_INVALID
    }

    fn is_table(&self) -> bool {
        (pte::TYPE::from(*self.as_raw()) == pte::TYPE_TABLE) && (self.level != END_LEVEL)
    }

    fn as_table<'b>(&'b self) -> PageTable<'b> {
        debug_assert!(self.is_table());
        PageTable{
            raw: unsafe { transmute(pte::ADDR::MASK & self.as_raw()) },
            level: self.level + 1,
            va: self.va,
        }
    }
}

impl<'a> PteMut<'a> {
    fn as_raw<'b>(&'b mut self) -> &'b mut u64 {
        let &mut PteRaw(ref mut raw) = self.raw;
        raw
    }

    fn clear(&mut self) {
        *self.as_raw() = 0;
    }

    fn write(&mut self, mapping : Mapping) {
        debug_assert!(mapping.va == self.va);
        debug_assert!(mapping.size == REGION_SIZE[self.level as usize]);
        debug_assert!(mapping.attr & !(pte::BLOCK_UATTRS::MASK | pte::BLOCK_LATTRS::MASK | pte::TABLE_ATTRS::MASK) == 0);
        debug_assert!(mapping.pa & !pte::ADDR::MASK == 0);
        debug_assert!(mapping.va & !pte::ADDR::MASK == 0);
        debug_assert!(mapping.pa & (mapping.size - 1) == 0);
        debug_assert!(mapping.va & (mapping.size - 1) == 0);

        *self.as_raw() = mapping.pa | mapping.attr |
            if self.level == END_LEVEL {
                pte::TYPE_TABLE << pte::TYPE::SHIFT
            } else {
                pte::TYPE_BLOCK << pte::TYPE::SHIFT
            };
    }

    /// Create new table in place of invalid PTE
    fn create_table<'b, 'w, H>(&'b mut self, world : &'w mut World<H>, attrs : u64) -> PageTableMut<'b>
        where H : hw::HW
    {
        debug_assert!(!self.is_table());
        debug_assert!(!self.is_valid());
        debug_assert!(self.can_be_table());

        let start = unsafe{&mut *(((*world).page_pool))}.get().unwrap();
        *self.as_raw() = start as u64 | (pte::TABLE_ATTRS::MASK & attrs) | pte::TYPE_TABLE << pte::TYPE::SHIFT;
        for idx in 0..ENTRIES {
            self.as_table_mut().pte(idx).clear();
        }

        self.as_table_mut()
    }

    /// Rewrite valid PTE as TABLE of finer granularity
    fn expand_to_table<'b, 'w, H>(&'b mut self, world : &'w mut World<H>) -> PageTableMut<'b>
        where H : hw::HW
    {
        debug_assert!(self.can_be_table());

        let old_raw = *self.as_raw();

        let start = unsafe{&mut *((*world).page_pool)}.get().unwrap();
        let attrs = (pte::BLOCK_UATTRS::MASK | pte::BLOCK_LATTRS::MASK | pte::TABLE_ATTRS::MASK) & old_raw;
        *self.as_raw() = start as u64 | attrs | (pte::TYPE_TABLE << pte::TYPE::SHIFT);

        let mapping = Mapping {
            va: self.va,
            size: REGION_SIZE[self.level as usize],
            pa:  pte::ADDR::MASK & old_raw,
            attr: attrs,
        };

        self.as_table_mut().map(world, mapping);

        self.as_table_mut()
    }

    fn as_table_mut<'b>(&'b mut self) -> PageTableMut<'b> {
        debug_assert!(self.is_table());
        PageTableMut{
            raw: unsafe { transmute(pte::ADDR::MASK & *self.as_raw()) },
            level: self.level + 1,
            va: self.va,
        }
    }
}

/// Page table as an array of PTEs
#[repr(C)]
struct PageTableRaw {
    entries : [PteRaw; ENTRIES],
}

/// PageTable reference with level information
pub struct PageTable<'a> {
    raw: &'a PageTableRaw,
    level : u8,
    va : u64,
}

/// Mutable PageTable reference with level information
pub struct PageTableMut<'a> {
    raw: &'a mut PageTableRaw,
    level : u8,
    va : u64,
}

#[derive(Copy, Clone)]
pub struct Mapping {
    va : u64,
    pa : u64,
    size : u64,
    attr : u64,
}

impl<'a> PageTable<'a> {
    pub fn pte<'b>(&'b self, i : usize) -> Pte<'b> {
        debug_assert!(i < ENTRIES);
        Pte {
            raw: &self.raw.entries[i],
            level: self.level,
            va: self.va + i as u64 * REGION_SIZE[self.level as usize],
        }
    }
}

impl<'a> PageTableMut<'a> {
    pub fn install(&mut self) {
        assert!(self.level == START_LEVEL);
        let asid = 0;
        let addr = self.pte(0).as_raw() as *const _ as u64; // TODO: check alignment

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

        sctlr_el1::write(
            sctlr_el1::M::MASK
            );

        isb();
    }

    pub fn pte<'b>(&'b mut self, i : usize) -> PteMut<'b> {
        debug_assert!(i < ENTRIES);
        PteMut {
            raw: &mut self.raw.entries[i],
            level: self.level,
            va: self.va + i as u64 * REGION_SIZE[self.level as usize],
        }
    }

    pub fn with_pte<'b, F>(&'b mut self, i : usize, mut f : F)
        where F : FnMut(PteMut<'b>) {
        debug_assert!(i < ENTRIES);
        f(PteMut {
            raw: &mut self.raw.entries[i],
            level: self.level,
            va: self.va + i as u64 * REGION_SIZE[self.level as usize],
        });
    }

    pub fn map<'w, H>(&mut self, world : &'w mut World<H>, mapping : Mapping)
        where H : hw::HW
    {
        let level = self.level;
        let region_size = REGION_SIZE[level as usize];
        let region_mask = region_size - 1;
        let idx_mask = IDX_MASK[level as usize];
        let idx_shift = IDX_SHIFT[level as usize];

        let mut va = mapping.va;
        let mut pa = mapping.pa;
        let mut left = mapping.size;

        loop {
            if left == 0 {
                break;
            }

            let va_start_aligned = va & !region_mask;
            let va_end_aligned = va_start_aligned + region_size;
            let va_end = cmp::min(va + left, va_end_aligned);
            let size = va_end - va;

            let idx = ((va & idx_mask) >> idx_shift) as usize;

            self.with_pte(idx, |mut pte| {
                let mapping = Mapping{
                    va: va,
                    pa: pa,
                    size: size,
                    attr: mapping.attr
                };
                if pte.is_table() {
                    let mut table = pte.as_table_mut();
                    table.map(world, mapping);
                } else if (region_size == size) && ((pa & region_mask) == 0)  {
                    pte.write(mapping);
                } else if pte.is_valid() {
                    let mut table = pte.expand_to_table(world);
                    table.map(world, mapping);
                } else {
                    let mut table = pte.create_table(world, mapping.attr);
                    table.map(world, mapping);
                }
            });

            debug_assert!(left >= size);
            left -= size;
            va += size;
            pa += size;

        }
        debug_assert!(left == 0)
    }
}

selftest!(fn page_table_size(_uart) {
    mem::size_of::<PageTableRaw>() == PAGE_SIZE as usize
});

static mut root_table_raw : PageTableRaw = PageTableRaw {
    entries : [PteRaw(0); ENTRIES],
};

// TODO: protect from aliasing?
fn root_mut() -> PageTableMut<'static> {
    PageTableMut {
        raw: unsafe { &mut root_table_raw },
        level: START_LEVEL,
        va: 0,
    }
}

pub fn root() -> PageTable<'static> {
    PageTable {
        raw: unsafe { &mut root_table_raw },
        level: START_LEVEL,
        va: 0,
    }
}

const MAIR : u64 = 0xff00;
const MAIR_IDX_SO : u64 = 0; // strongly-ordered
const MAIR_IDX_MEM : u64 = 1; // normal memory

pub fn init<'w, H>(world : &'w mut World<H>)
        where H : hw::HW
{
    let mut root_table = root_mut();
    {
        let attr : u64 =
            pte::AF::MASK |
            pte::NS::MASK |
            pte::ATTRINDX::to(MAIR_IDX_SO) |
            pte::SH::to(pte::SH_INNER) |
            pte::AP::to(pte::AP_KERNEL)
            ;

        let mapping = Mapping{
            va: 0x0,
            pa: 0x0,
            size: 0x80000000,
            attr: attr,
        };

        root_table.map(world, mapping);
    }

    {
        let attr : u64 =
            pte::AF::MASK |
            pte::NS::MASK |
            pte::ATTRINDX::to(MAIR_IDX_MEM) |
            pte::SH::to(pte::SH_INNER) |
            pte::AP::to(pte::AP_KERNEL)
            ;
        let mapping = Mapping{
            va: 0x80000000,
            pa: 0x80000000,
            size: 0x80000000,
            attr: attr,
        };
        root_table.map(world, mapping);
    }

    mair_el1::write(MAIR);

    tcr_el1::write(
        tcr_el1::TG1::to(reg::TG1_64K) |
        tcr_el1::TG0::to(reg::TG0_64K) |
        tcr_el1::T0SZ::to(22) |
        tcr_el1::T1SZ::to(22)
        );
    root_table.install();
}
