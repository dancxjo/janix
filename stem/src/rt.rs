#[cfg(feature = "rt")]
use crate::syscall::exit;

#[cfg(feature = "rt")]
extern "Rust" {
    fn main() -> i32;
}

#[cfg(feature = "rt")]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // For x86_64 and likely others, we might just call a rust function if we don't need significant setup.
    // However, sticking to the standard "entry point calls main then exit" pattern.
    // Making this a naked function to avoid preamble issues, but calling inner implementation immediately.
    
    // NOTE: Naked functions in Rust require specific asm syntax for each arch.
    // To keep v0 simple and multi-arch without repeating naked asm 4 times here,
    // we can assume the kernel/linker jumps to a symbol `_start` which is a valid function.
    // If we don't mark it naked, the compiler might touch the stack.
    // But since we are at the very process entry, `sp` should be valid (set by kernel).
    // Let's try a standard function first, marked `extern "C"`.
    // If this causes issues (e.g. using dirty stack slots), we'll upgrade to naked asm.
    
    entry_impl()
}

#[cfg(feature = "rt")]
unsafe fn entry_impl() -> ! {
    let code = main();
    exit(code);
}
