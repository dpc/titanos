use arch;
use core::intrinsics::{volatile_store, volatile_load, transmute};

pub trait UnsignedInt {
    fn zero() -> Self;
}

impl UnsignedInt for u8 {
    fn zero() -> u8 { 0 }
}

impl UnsignedInt for u16 {
    fn zero() -> u16 { 0 }
}

impl UnsignedInt for u32 {
    fn zero() -> u32 { 0 }
}

impl UnsignedInt for u64 {
    fn zero() -> u64 { 0 }
}

pub trait HW {
    fn local_irqs_disable(&mut self) { }
    fn local_irqs_enable(&mut self) { }
    fn write<T : UnsignedInt>(&mut self, _addr : usize, _data : T) { }
    fn read<T : UnsignedInt>(&mut self, _addr : usize) -> T { T::zero() }
}

pub struct Real;

impl HW for Real {
    fn local_irqs_disable(&mut self) {
        arch::local_irqs_disable()
    }
    fn local_irqs_enable(&mut self) {
        arch::local_irqs_enable()
    }
    fn write<T>(&mut self, addr : usize, data : T) {
        let addr : &mut T = unsafe { transmute(addr) };
        unsafe { volatile_store(addr, data); }
    }
    fn read<T>(&mut self, addr : usize) -> T {
        let addr : &mut T = unsafe { transmute(addr) };
        unsafe { volatile_load(addr) }
    }
}
