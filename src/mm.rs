#![allow(unused)]
use core::option::Option;

use arch::PageTableRoot;
use arch::PAGE_SIZE;
use World;
use titanium::hw;

extern {
    static mut _pt_start: u8;
    static mut _pt_end: u8;
}

pub struct PageArena {
    _start : usize,
    end : usize,
    current : usize,
}

impl PageArena {

    pub fn new(start : usize, end : usize) -> PageArena {
        // TODO: check alignment and bug if wrong
        PageArena {
            _start: start,
            end: end,
            current: start,
        }
    }

    pub fn get(&mut self) -> Option<usize> {
        let ret = self.current + PAGE_SIZE;
        if ret >= self.end {
            Option::None
        } else {
            self.current = ret;
            Option::Some(ret)
        }
    }
}

static mut pool : PageArena = PageArena {
    _start: 0,
    end: 0,
    current: 0,
};

pub fn preinit() -> &'static mut PageArena {
    let start : usize = unsafe {&_pt_start} as *const _ as usize;
    let end : usize = unsafe {&_pt_end} as *const _ as usize;
    unsafe {
        pool._start = start;
        pool.current = start;
        pool.end = end;

    &mut pool
    }
}

pub fn init(world : &mut World<hw::Real>) {
    let _table = PageTableRoot::new(world);
}
