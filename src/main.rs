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
use core::option::Option::{self, Some, None};

use titanium::drv;
use titanium::drv::{Driver};
use titanium::drv::uart;
use titanium::hw;

use arch::pagetable;

use arm_pl011::PL011;

use core::ptr;

mod arch;
mod mem;
mod mm;
mod rust;

pub struct World<H : 'static>
where H : hw::HW
{
    pub hw : H,
    pub uart : *mut uart::UartWriter,
    pub page_pool : *mut mm::PageArena,
}

#[no_mangle]
static mut world : *mut World<hw::Real> = 0 as *mut _;

#[no_mangle]
pub extern "C" fn main() {

    arch::set_vbar();

    if arch::cpu_id() == 0 {
        mem::preinit();
        let mut dummy_uart = uart::DummyUartWriter;
        let page_arena : &'static mut mm::PageArena = mm::preinit();

        unsafe {
            world = transmute (&World {
            hw: hw::Real,
            uart: transmute(&mut dummy_uart as &mut uart::UartWriter),
            page_pool : page_arena,
        })};

        let mut uart = PL011::new(0x1c090000);
        uart.init( unsafe{&mut (*world).hw});

        let mut writer = uart::BlockingUartWriter::<hw::Real>::new(
            unsafe { transmute(&mut (*world).hw) },
            unsafe { transmute(&uart as &titanium::drv::Uart<hw::Real>) },
            );

        unsafe { (*world).uart = transmute(&mut writer as &mut uart::UartWriter)};

        write!(unsafe{&mut *(*world).uart}, "S").unwrap();
        write!(unsafe{&mut *(*world).uart}, "X").unwrap();

        titanium::selftest::selftest(unsafe{&mut *(*world).uart});
        write!(unsafe{&mut *(*world).uart}, "X").unwrap();

        write!(unsafe{&mut *(*world).uart}, "Y").unwrap();
        writeln!(unsafe{&mut *(*world).uart}, "Hello Embedded World!").unwrap();

        pagetable::init(unsafe{&mut (*world)});

    }

    loop { }
}
