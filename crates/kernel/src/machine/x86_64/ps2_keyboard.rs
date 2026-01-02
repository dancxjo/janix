//! PS/2 Keyboard Driver for x86_64.
//!
//! Implements a simple ring buffer for capturing scancodes in the IRQ handler.

use crate::log::{self, Level};
use core::sync::atomic::{AtomicBool, Ordering};
use x86_64::instructions::port::Port;
use x86_64::instructions::interrupts;

/// Size of the scancode ring buffer.
const RING_SIZE: usize = 256;

static mut SC_BUFFER: [u8; RING_SIZE] = [0; RING_SIZE];
static mut HEAD: usize = 0; // Write index (IRQ)
static mut TAIL: usize = 0; // Read index (Syscall)
static OVERFLOWED: AtomicBool = AtomicBool::new(false);

/// Initialize the PS/2 keyboard.
pub fn init() {
    // For now, we assume the BIOS/Bootloader left the controller in a usable state.
    // We just flush the buffer.
    let _port = Port::<u8>::new(0x60);
    // Drain existing (approximate check, can't poll indefinitely without status reg)
}

/// IRQ Handler for Keyboard (IRQ 1).
///
/// This is called from the IDT trampoline.
pub unsafe fn irq_handler() {
    let mut port = Port::<u8>::new(0x60);
    let scancode = port.read();

    // Push to ring buffer
    // Calculate next head
    let next_head = (HEAD + 1) % RING_SIZE;

    if next_head == TAIL {
        // Full! Drop and set overflow flag.
        if !OVERFLOWED.swap(true, Ordering::Relaxed) {
             log::klog(Level::Warn, "PS2", "Keyboard Input Overflow!");
        }
    } else {
        SC_BUFFER[HEAD] = scancode;
        HEAD = next_head;
    }
}

/// Read scancodes from the ring buffer into `dst`.
pub fn read_scancodes(dst: &mut [u8]) -> usize {
    interrupts::without_interrupts(|| {
        let mut read_count = 0;

        unsafe {
            while TAIL != HEAD && read_count < dst.len() {
                dst[read_count] = SC_BUFFER[TAIL];
                TAIL = (TAIL + 1) % RING_SIZE;
                read_count += 1;
            }
        }

        read_count
    })
}

pub fn debug_dump() {
    interrupts::without_interrupts(|| {
        unsafe {
            let msg = alloc::format!("KBD Ring: Head={} Tail={} Ovf={}", HEAD, TAIL, OVERFLOWED.load(Ordering::Relaxed));
            log::klog(Level::Info, "PS2", &msg);
        }
    });
}
