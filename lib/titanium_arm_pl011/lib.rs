#![no_std]
#![feature(linkage)]

#[macro_use]
extern crate titanium;

use titanium::drv;
use titanium::hw;

const PL011_DR : usize = 0x000;
const PL011_CR : usize = 0x0c0;

pub struct PL011 {
    base : usize,
}

impl PL011 {
    pub fn new(base : usize) -> PL011 {
        PL011 {
            base: base,
        }
    }
}

impl drv::Driver for PL011
{
    fn init<W>(&mut self, w : &mut W)
        where W : titanium::HW
    {
        w.write::<u16>(self.base + PL011_CR, 1 << 0 | 1 << 8);
    }
}

impl<W> drv::Uart<W> for PL011
where W : titanium::HW {
    fn put(&self, w : &mut W, ch : u8) {
        w.write::<u8>(self.base + PL011_DR, ch)
    }
}

struct Mock;

impl hw::HW for Mock {

}


selftest!(fn pl011_basic(_uart) {

    use titanium::drv::Driver;
    use titanium::drv::Uart;

    let mut hw = Mock;

    let base = 0x10000000;
    let mut pl011 = PL011::new(base);

    pl011.init(&mut hw);
    pl011.put(&mut hw, 1);

    true
});


