#![no_std]
#![no_main]
#![feature(no_std)]
#![feature(core)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(custom_attribute)]
#![feature(core_intrinsics)]
#![allow(unused)]

#[macro_use]
extern crate core;

#[macro_use]
extern crate titanium;

// temporary here
extern crate arm_pl011;

use core::intrinsics::transmute;

use core::fmt::Write;

use titanium::drv;
use titanium::drv::{Driver};
use titanium::drv::uart;
use titanium::hw;


use arch::pagetable;

use arm_pl011::PL011;

mod arch;
mod mem;
mod mm;

pub struct World<H : 'static>
where H : hw::HW
{
    pub hw : H,
    pub uart : &'static mut uart::UartWriter,
    pub page_pool : &'static mut mm::PageArena,
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
            page_pool : page_arena,
        };

        let mut uart = PL011::new(0x1c090000);
        uart.init(&mut world.hw);

        let mut writer = uart::BlockingUartWriter::<hw::Real>::new(
            unsafe { transmute(&mut world.hw) },
            unsafe { transmute(&uart as &titanium::drv::Uart<hw::Real>) },
            );

        unsafe { world.uart = transmute(&mut writer as &mut titanium::drv::uart::UartWriter) };
        writeln!(world.uart, "Self or not").unwrap();
        titanium::selftest::selftest(&mut writer);

//        writeln!(world.uart, "THE {:x} start: {:x} end: {:x} current: {:x}", world.page_pool as *const _ as usize, world.page_pool.start, world.page_pool.end, world.page_pool.current).unwrap();
        writeln!(world.uart, "Hello Embedded World!").unwrap();

        pagetable::init(&mut world);

    }

    loop { }
}
