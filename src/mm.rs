use core::option::Option;

use arch::PageTable;
use arch::PAGE_SIZE;

extern {
    static mut _pt_start: u8;
    static mut _pt_end: u8;
}

pub struct PageArena {
    start : usize,
    end : usize,
    current : usize,
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
        let ret = self.current + PAGE_SIZE;
        if ret >= self.end {
            Option::None
        } else {
            self.current = ret;
            Option::Some(ret)
        }
    }
}

pub fn init() {
    let start : usize = unsafe {&_pt_start} as *const _ as usize;
    let end : usize = unsafe {&_pt_end} as *const _ as usize;
    let mut arena = PageArena::new(start, end);
    let table = PageTable::new(&mut arena);

    //table.map_all();
}
