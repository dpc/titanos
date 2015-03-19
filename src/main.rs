#![no_std]
#![no_main]
#![feature(no_std)]
#![feature(core)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(static_assert)]

#[cfg(not(test))]
extern crate core;

//#[cfg(test)]
//#[macro_use]
//extern crate std;

#[macro_use]
extern crate titanium;

// temporary here
extern crate arm_pl011;
use arm_pl011::PL011;
use titanium::drv::{Driver, Uart};

mod arch;
mod mem;
mod mm;
mod selftest;

use core::intrinsics::{volatile_store, volatile_load};

static mut x : u32 = 0;

#[no_mangle]
pub extern "C" fn main()
{
    if arch::cpu_id() == 0 {
        mem::init();
        mm::init();
    }


    let mut uart = PL011::new(0x1c090000);
    uart.init();

    selftest::selftest(&mut uart);

    loop {
        let mut tx = unsafe { volatile_load(&mut x) };

        uart.put('a' as u8 + (tx % ('z' as u8 - 'a' as u8 + 1) as u32) as u8);

        tx += 1;
        unsafe { volatile_store(&mut x,  tx ); }
    }
}

