#[cfg(feature = "rt")]
use crate::syscall::exit;

#[cfg(feature = "rt")]
extern "Rust" {
    fn main(arg: usize) -> i32;
}

#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
#[no_mangle]
unsafe extern "C" fn entry_impl(arg: usize) -> ! {
    let code = main(arg);
    exit(code);
}

#[cfg(all(target_arch = "x86_64", feature = "rt"))]
core::arch::global_asm!(r#"
    .section .text.entry
    .global _start
    _start:
        // Kernel jumps here. RSP is 16-byte aligned (e.g. 0x400000).
        // RDI holds the argument.
        // CALL instruction pushes 8 bytes, so RSP becomes aligned-8.
        // This satisfies the System V ABI for the callee.
        call entry_impl
        ud2
"#);
