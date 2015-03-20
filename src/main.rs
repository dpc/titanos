#![no_std]
#![no_main]
#![feature(no_std)]
#![feature(core)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(static_assert)]

#[macro_use]
extern crate core;

#[macro_use]
extern crate titanium;

// temporary here
extern crate arm_pl011;


use core::fmt::Write;

use titanium::drv::{Driver};
use titanium::drv::uart::UartWriter;

use arm_pl011::PL011;

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

    let mut writer = UartWriter::new(&uart);
    titanium::selftest::selftest(&mut writer);


    write!(&mut writer, "Hello Embedded World!").unwrap();

    loop { }
}

