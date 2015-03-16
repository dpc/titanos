use core::intrinsics::{volatile_store, volatile_load, transmute};

pub trait VolatileAccess {
    fn write_u8(addr : usize, data : u8) {
        let addr : &mut u8 = unsafe { transmute(addr) };
        unsafe { volatile_store(addr, data); }
    }

    fn write_u16(addr : usize, data : u16) {
        let addr : &mut u16 = unsafe { transmute(addr) };
        unsafe { volatile_store(addr, data); }
    }

    fn read_u8(addr : usize) -> u8 {
        let addr : &mut u8 = unsafe { transmute(addr) };
        unsafe { volatile_load(addr) }
    }

    // This is here only because compiler complains about using Self somewhere
    fn dummy(&self) { }
}

pub struct Default;

impl VolatileAccess for Default { }


