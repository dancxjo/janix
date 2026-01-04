use core::sync::atomic::AtomicU64;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ExceptionContext {
    pub x: [u64; 30],
    pub lr: u64,
    pub sp: u64,
    pub elr_el1: u64,
    pub spsr_el1: u64,
    pub esr_el1: u64,
    pub far_el1: u64,
}

pub static IRQ_COUNT: AtomicU64 = AtomicU64::new(0);

#[no_mangle]
pub extern "C" fn aarch64_handle_exception(ctx: &mut ExceptionContext, vector: u64) -> u64 {
    unsafe {
        crate::serial::write(b"AArch64 EXCEPTION: ");
        crate::serial::write_hex(vector);
        crate::serial::write(
            b"
ELR=",
        );
        crate::serial::write_hex(ctx.elr_el1);
        crate::serial::write(b" ESR=");
        crate::serial::write_hex(ctx.esr_el1);
        crate::serial::write(
            b"
",
        );
    }
    0
}
