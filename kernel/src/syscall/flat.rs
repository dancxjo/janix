use super::dispatch;

/// Flat entry point for assembly.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_dispatch_flat(
    n: usize,
    a0: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
) -> isize {
    let ret = dispatch(n, [a0, a1, a2, a3, a4, a5]);
    ret
}
