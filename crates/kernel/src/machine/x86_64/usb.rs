//! xHCI kernel-facing helpers (IRQ flag bytespace).
//!
//! Exposes a tiny bytespace that is bumped from the xHCI interrupt handler so
//! a userspace driver can detect interrupts without kernel-side parsing.

use core::sync::atomic::{AtomicU32, Ordering};

/// Single-page IRQ counter exposed as a bytespace.
#[repr(C, align(4096))]
struct XhciIrqPage {
    counter: AtomicU32,
    _pad: [u8; 4096 - core::mem::size_of::<AtomicU32>()],
}

static mut XHCI_IRQ_PAGE: XhciIrqPage = XhciIrqPage {
    counter: AtomicU32::new(0),
    _pad: [0; 4096 - core::mem::size_of::<AtomicU32>()],
};

/// Physical address of the IRQ counter bytespace.
pub fn irq_counter_phys() -> u64 {
    let virt = unsafe { &XHCI_IRQ_PAGE as *const _ as u64 };
    crate::machine::machine().virt_to_phys(virt)
}

/// Length of the IRQ bytespace (one page).
pub const fn irq_bytespace_len() -> u64 {
    core::mem::size_of::<XhciIrqPage>() as u64
}

/// IRQ handler: bump the counter so userland can observe interrupts.
pub unsafe fn xhci_irq_handler() {
    XHCI_IRQ_PAGE.counter.fetch_add(1, Ordering::Relaxed);
}
