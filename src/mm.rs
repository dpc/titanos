use core::option::Option;

use arch::PAGE_SIZE;
use World;
use titanium::hw;

extern {
    static _pt_start: u8;
    static _pt_end: u8;
}

pub struct PageArena {
    pub start : usize,
    pub end : usize,
    pub current : usize,
}

impl PageArena {

    pub fn new(start : usize, end : usize) -> PageArena {
        // TODO: check alignment and bug if wrong
        PageArena {
            start: start,
            end: end,
            current: start,
        }
    }

    pub fn get(&mut self) -> Option<usize> {
        if self.current < self.end {
            let ret = self.current;
            self.current = ret + PAGE_SIZE;
            Option::Some(ret)
        } else {
            Option::None
        }
    }
}

static mut pool : PageArena = PageArena {
    start: 0,
    end: 0,
    current: 0,
};

pub fn preinit() -> &'static mut PageArena {
    let start : usize = &_pt_start as *const u8 as usize;
    let end : usize = &_pt_end as *const u8 as usize;
    unsafe {
        pool.start = start;
        pool.end = end;
        pool.current = start;

        &mut pool
    }
}

pub fn init(world : &mut World<hw::Real>) {
    //let _table = PageTableRoot::new(world);
}
