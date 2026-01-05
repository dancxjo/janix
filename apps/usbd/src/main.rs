#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use core::sync::atomic::Ordering;
use thing_std::*;
use abi::mouse_ring::{MouseRingHeader, MouseSample, MOUSE_RING_MAGIC};

const XHCI_MMIO_VADDR: u64 = 0x8A00_0000;
const XHCI_MMIO_LEN: u64 = 0x10000;
const IRQ_FLAG_VADDR: u64 = 0x8B00_0000;
const MOUSE_RING_VADDR: u64 = 0x8300_0000;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("USBD: starting (xHCI bootstrap)");

    // Map MMIO bytespace if present
    let mmio_ptr = if let Some(bs_id) = thing_find("bytespace.usb.xhci0.mmio") {
        memory::space_map(bs_id, XHCI_MMIO_VADDR, 0, XHCI_MMIO_LEN);
        Some(XHCI_MMIO_VADDR as *const u8)
    } else {
        log_info("USBD: no xHCI MMIO bytespace found");
        None
    };

    // Map mouse input bytespace so we can publish deltas
    let (mouse_hdr, mouse_samples_base) = if let Some(mouse_bs_id) = thing_find("bytespace.mouse_input") {
        memory::space_map(mouse_bs_id, MOUSE_RING_VADDR, 0, 8192);
        let hdr = MOUSE_RING_VADDR as *mut MouseRingHeader;
        let samples = (MOUSE_RING_VADDR + core::mem::size_of::<MouseRingHeader>() as u64) as *mut u8;
        (Some(hdr), Some(samples))
    } else {
        log_info("USBD: no mouse bytespace found");
        (None, None)
    };

    if let Some(ptr) = mmio_ptr {
        unsafe {
            // Capability registers
            let cap_length = core::ptr::read_volatile(ptr) as u16;
            let hci_version = u16::from_le_bytes([
                core::ptr::read_volatile(ptr.add(2)),
                core::ptr::read_volatile(ptr.add(3)),
            ]);
            log_info(&format!(
                "USBD: xHCI CAPLEN={} HCIVERSION={:#x}",
                cap_length, hci_version
            ));
        }
    }

    // Map IRQ counter bytespace (optional)
    let irq_ptr = if let Some(bs_id) = thing_find("bytespace.irq.usb.xhci0") {
        memory::space_map(bs_id, IRQ_FLAG_VADDR, 0, 4096);
        Some(IRQ_FLAG_VADDR as *const core::sync::atomic::AtomicU32)
    } else {
        None
    };

    let mut last_irq = 0u32;
    loop {
        if let Some(counter) = irq_ptr {
            let val = unsafe { (*counter).load(core::sync::atomic::Ordering::Relaxed) };
            if val != last_irq {
                last_irq = val;
                log_info(&format!("USBD: xHCI IRQ count {}", val));

                // Publish a tiny synthetic delta to prove the pipeline works
                if let (Some(hdr), Some(samples_base)) = (mouse_hdr, mouse_samples_base) {
                    push_mouse_sample(hdr, samples_base, 1, 0, 0);
                }
            }
        }
        sched_yield();
    }
}

fn push_mouse_sample(
    hdr: *mut MouseRingHeader,
    samples_base: *mut u8,
    dx: i16,
    dy: i16,
    buttons: u16,
) {
    // Safety: caller ensures mapping is valid
    unsafe {
        if hdr.is_null() {
            return;
        }
        // Validate magic
        if (*hdr).magic != MOUSE_RING_MAGIC {
            return;
        }
        let capacity = (*hdr).capacity;
        if capacity == 0 {
            return;
        }

        let write_idx = (*hdr).write.load(Ordering::Relaxed);
        let slot = (write_idx % capacity) as usize;
        let sample_ptr =
            samples_base.add(slot * core::mem::size_of::<MouseSample>()) as *mut MouseSample;

        core::ptr::write_volatile(
            sample_ptr,
            MouseSample {
                t_ns: time::monotonic_now(),
                dx,
                dy,
                wheel: 0,
                buttons,
            },
        );

        (*hdr)
            .write
            .store(write_idx.wrapping_add(1), Ordering::Release);
    }
}
