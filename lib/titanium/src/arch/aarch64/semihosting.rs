use super::wfi;

/// Call semihosting exit
///
/// This should terminate any emulator supporting semihosting.
/// BUG: This does not seem to work in Qemu unfortunately, so
/// it just loops wasting power.
pub fn exit() -> ! {
    unsafe {
        asm!("mov x0, $0\n mov x1, $1\n hlt #0xf000"
             :
             : "r"(0x18), "r"(0x20026)
             : "r0", "r1", "memory"
             : "volatile");
    }

    loop {
        wfi();
    }
}
