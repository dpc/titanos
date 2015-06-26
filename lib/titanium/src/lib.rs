#![feature(lang_items)]
#![feature(asm)]
#![feature(step_by)]
#![feature(custom_attribute)]
#![feature(core_intrinsics)]
#![feature(core_str_ext)]
#![no_std]

#![feature(trace_macros)]

#![crate_name="titanium"]


#[macro_use]
pub mod macros;

#[macro_use]
pub mod arch;

pub mod lang;
pub mod drv;
pub mod consts;
pub mod hw;

pub use hw::HW;

#[macro_use]
pub mod selftest;

mod titanium {
    pub use super::selftest;
}
