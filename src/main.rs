#![no_std]
#![no_main]
#![feature(asm)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(custom_attribute)]
#![feature(core_intrinsics)]
#![allow(unused)]

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

macro_rules! pr_info {
    ($fmt:expr) => (

        unsafe{
            $crate::world.uart.as_mut()
        }.unwrap().write_fmt(format_args!(concat!($fmt, "\n"))).unwrap()
        );
    ($fmt:expr, $($args:tt)*) => (

        unsafe{
            $crate::world.uart.as_mut()
        }.unwrap().write_fmt(format_args!(concat!($fmt, "\n"), $($args)*)).unwrap()
    );
}

macro_rules! pr_debug {
    ($($args:tt)*) => (pr_info!($($args)*));
}

mod arch;
mod mem;
mod mm;
mod rust;

pub struct World<H : 'static>
where H : hw::HW
{
    pub int : u64,
    pub hw : H,
    pub uart : Option<&'static mut uart::UartWriter>,
    pub page_pool : *mut mm::PageArena,
}

static mut world : World<hw::Real> = World {
    int : 0xdeadbeaf,
    hw: hw::Real,
    uart: None,
    page_pool: ptr::null_mut(),
};

#[no_mangle]
pub extern "C" fn main() {
    arch::set_vbar();

    if arch::cpu_id() == 0 {
        mem::preinit();
        let mut dummy_uart = uart::DummyUartWriter;
        unsafe {
            world.page_pool = mm::preinit();
        }

        unsafe {
            world.int = 0xdeadbeaf;
            world.hw = hw::Real;
            world.uart = transmute(&mut dummy_uart as &mut uart::UartWriter);
        }

        let mut uart = PL011::new(0x1c090000);
        uart.init( unsafe{&mut (world).hw});

        let mut writer = uart::BlockingUartWriter::<hw::Real>::new(
            unsafe { transmute(&mut (world).hw) },
            unsafe { transmute(&uart as &titanium::drv::Uart<hw::Real>) },
            );

        unsafe { (world).uart = transmute(&mut writer as &mut uart::UartWriter)};

        pr_debug!("Starting...");
        pagetable::init(unsafe{&mut world});
        titanium::selftest::selftest(*unsafe{world.uart.as_mut()}.unwrap() );

        pr_info!("Hello Embedded World!");
    }

    pr_debug!("Ready...");
    loop { }
}
