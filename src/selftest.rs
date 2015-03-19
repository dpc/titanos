use titanium;
use core::intrinsics::{transmute};
use core::iter::range;

extern {
    static _selftest_start: u64;
    static _selftest_end: u64;
}

#[link_section = ".selftest"]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
static p :  fn (mut uart : &mut titanium::drv::Uart) = testx;

fn testx(mut uart : &mut titanium::drv::Uart) {
    loop {
        uart.put('x' as u8);
    }
}

#[cfg(feature = "selftest")]
pub fn selftest(mut uart : &mut titanium::drv::Uart) {

    let start : usize = &_selftest_start as *const _ as usize;
    let end : usize = &_selftest_end as *const _ as usize;

    for test in range(start, end) {

        let f : fn (mut uart : &mut titanium::drv::Uart) = {
            unsafe {
                let addr : &usize = transmute(test);
                transmute(*addr)
            }
        };

        unsafe { (f)(uart) };
    }
}

#[cfg(not(feature = "selftest"))]
pub fn selftest(_ : &mut titanium::drv::Uart) {}
