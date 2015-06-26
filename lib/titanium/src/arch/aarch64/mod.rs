#[macro_use]
pub mod macros;
pub mod reg;
pub mod mmu;
pub mod semihosting;

use core::ops::Drop;

pub fn local_irqs_disable() {
    unsafe {
        asm!("msr daifset, #2"
             :
             :
             : "cc", "memory"
             : "volatile")
    }
}

pub fn local_irqs_enable() {
    unsafe {
        asm!("msr daifclr, #2"
             :
             :
             : "cc", "memory"
             : "volatile")
    }
}

fn cpu_flags_read() -> u64 {
    reg::daif::read()
}


fn cpu_flags_write(flag : u64) {
    reg::daif::write(flag);
}

pub struct CriticalSectionGuard {
    flags : u64,
}

impl Drop for CriticalSectionGuard {
    fn drop(&mut self) {
        cpu_flags_write(self.flags);
    }
}

pub fn critical_section_start() -> CriticalSectionGuard {
    let ret = CriticalSectionGuard {
        flags: cpu_flags_read()
    };

    local_irqs_disable();

    ret
}

/// dsb instruction
pub fn dsb_sy() {
    unsafe {
        asm!("dsb sy" :::: "volatile");
    }
}
/// dsb instruction
pub fn dsb() {
    unsafe {
        asm!("dsb ish" :::: "volatile");
    }
}

/// dmb instruction
pub fn dmb() {
    unsafe {
        asm!("dmb ish" :::: "volatile");
    }
}

/// wfi instruction
pub fn wfi() {
    unsafe {
        asm!("wfi" :::: "volatile");
    }
}


/// isb instruction
pub fn isb() {
    unsafe {
        asm!("isb" :::: "volatile");
    }
}
