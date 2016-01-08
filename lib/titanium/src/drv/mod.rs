pub mod uart;

pub use self::uart::Uart;
pub use hw::HW;

pub trait Driver {
    fn init<H>(&mut self, hw: &mut H)
        where H : HW
        ;
}
