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

use core::intrinsics::transmute;

use core::fmt::Write;

use titanium::drv::{Driver};
use titanium::drv::uart;
use titanium::hw;


use arm_pl011::PL011;

mod arch;
mod mem;
mod mm;

struct World<H : 'static>
where H : hw::HW
{
    pub hw : H,
    pub uart : &'static mut uart::UartWriter,
    pub page_alloc :  &'static mut mm::PageArena,
}

#[no_mangle]
pub extern "C" fn main() {

    if arch::cpu_id() == 0 {
        mem::preinit();
        let mut dummy_uart = uart::DummyUartWriter;
        let page_arena : &'static mut mm::PageArena = mm::preinit();

        let mut world : World<hw::Real> = World {
            hw: hw::Real,
            uart: unsafe { transmute(&mut dummy_uart as &mut titanium::drv::uart::UartWriter) },
            page_alloc: page_arena,
        };


        let mut uart = PL011::new(0x1c090000);
        uart.init(&mut world.hw);

        let mut writer = uart::BlockingUartWriter::<hw::Real>::new(
            unsafe { transmute(&mut world.hw) },
            unsafe { transmute(&uart as &titanium::drv::Uart<hw::Real>) },
            );

        unsafe { world.uart = transmute(&mut writer as &mut titanium::drv::uart::UartWriter) };
        titanium::selftest::selftest(&mut writer);

        write!(world.uart, "Hello Embedded World!").unwrap();

    }

    loop { }
}
