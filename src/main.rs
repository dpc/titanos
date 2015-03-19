#![no_std]
#![no_main]
#![feature(no_std)]
#![feature(core)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(static_assert)]

#[cfg(not(test))]
extern crate core;

#[macro_use]
extern crate titanium;

// temporary here
extern crate arm_pl011;
use arm_pl011::PL011;
use titanium::drv::{Driver, Uart};

use core::str::StrExt;
mod arch;
mod mem;
mod mm;

#[no_mangle]
pub extern "C" fn main()
{
    if arch::cpu_id() == 0 {
        mem::init();
        mm::init();
    }

    let mut uart = PL011::new(0x1c090000);
    uart.init();

    titanium::selftest::selftest(&mut uart);

    for ch in "Hello Embedded World!".bytes() {
        uart.put(ch);
    }

    loop { }
}

