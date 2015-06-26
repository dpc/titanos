use core::fmt;
use core::str::StrExt;
use core::result::Result;

use hw::HW;

pub trait Uart<W : HW> {
    fn put(&self, &mut W, ch : u8);
}

pub trait UartWriter : fmt::Write { }

pub struct DummyUartWriter;

impl UartWriter for DummyUartWriter { }

impl fmt::Write for DummyUartWriter {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Result::Ok(())
    }
}

pub struct BlockingUartWriter<H : 'static+HW> {
    uart : &'static Uart<H>,
    hw : &'static mut H,
}

impl<H : HW> UartWriter for BlockingUartWriter<H> { }

impl<H> BlockingUartWriter<H>
where H : HW {
    pub fn new(hw : &'static mut H, uart : &'static Uart<H>) -> BlockingUartWriter<H> {
        BlockingUartWriter { uart: uart, hw: hw }
    }
}

impl<H> fmt::Write for BlockingUartWriter<H>
where H : HW {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            self.uart.put(self.hw, ch);
        }
        Result::Ok(())
    }
}
