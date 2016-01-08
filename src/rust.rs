/// Some lang items required by libcore and Rust
use world;
use core;

#[cfg(not(test))]
/// Entry point of panic from the libcore crate.
#[lang = "panic_fmt"]
extern fn panic_fmt(args: &core::fmt::Arguments,
                    file: &str,
                    line: u32) -> ! {

    pr_info!("PANIC: {}:{}", file, line);
    loop {}
}
/// This function is invoked from rust's current __morestack function. Segmented
/// stacks are currently not enabled as segmented stacks, but rather one giant
/// stack segment. This means that whenever we run out of stack, we want to
/// truly consider it to be stack overflow rather than allocating a new stack.
#[cfg(not(test))] // in testing, use the original libstd's version
#[lang = "stack_exhausted"]
extern fn stack_exhausted() {
    pr_info!("PANIC: Stack exhausted");
    loop {}
}

#[lang="eh_personality"]
#[no_mangle] // referenced from rust_try.ll
#[allow(private_no_mangle_fns)]
extern fn rust_eh_personality()
{
    pr_info!("PANIC: eh_personality?");
    loop {}
}
