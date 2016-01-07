/// Some lang items required by libcore and Rust
use world;

#[cfg(not(test))]
/// Entry point of panic from the libcore crate.
#[lang = "panic_fmt"]
pub extern fn rust_begin_unwind() -> ! {
    writeln!(unsafe{&mut *(*world).uart}, "PANIC").unwrap();
    loop {}
}

/// This function is invoked from rust's current __morestack function. Segmented
/// stacks are currently not enabled as segmented stacks, but rather one giant
/// stack segment. This means that whenever we run out of stack, we want to
/// truly consider it to be stack overflow rather than allocating a new stack.
#[cfg(not(test))] // in testing, use the original libstd's version
#[lang = "stack_exhausted"]
extern fn stack_exhausted() {
    writeln!(unsafe{&mut *(*world).uart}, "PANIC: Stack exhausted").unwrap();
    loop {}
}

#[lang="eh_personality"]
#[no_mangle] // referenced from rust_try.ll
#[allow(private_no_mangle_fns)]
extern fn rust_eh_personality()
{
    writeln!(unsafe{&mut *(*world).uart}, "PANIC: eh_personality?").unwrap();
    loop {}
}
