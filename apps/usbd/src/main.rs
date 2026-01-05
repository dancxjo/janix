//! USB Daemon - xHCI Controller Driver
//!
//! This daemon initializes the xHCI controller with DMA buffers,
//! submits a NO-OP command, and verifies completion.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use core::sync::atomic::Ordering;
use thing_std::*;
use usb_xhci::{CommandRing, ErstEntry, EventRing, Trb, XhciController};

// Virtual address layout for DMA buffers
const XHCI_MMIO_VADDR: u64 = 0x8A00_0000;
const XHCI_MMIO_LEN: u64 = 0x10000;
const IRQ_FLAG_VADDR: u64 = 0x8B00_0000;

// DMA buffer virtual addresses (4K each)
const DMA_DCBAA_VADDR: u64 = 0x8C00_0000;
const DMA_CMD_RING_VADDR: u64 = 0x8C01_0000;
const DMA_ERST_VADDR: u64 = 0x8C02_0000;
const DMA_EVENT_RING_VADDR: u64 = 0x8C03_0000;

const PAGE_SIZE: u64 = 4096;

#[no_mangle]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("USBD: starting (xHCI DMA bring-up)");

    // Map MMIO bytespace if present
    let mmio_ptr = if let Some(bs_id) = thing_find("bytespace.usb.xhci0.mmio") {
        memory::space_map(bs_id, XHCI_MMIO_VADDR, 0, XHCI_MMIO_LEN);
        Some(XHCI_MMIO_VADDR as *mut u8)
    } else {
        log_info("USBD: no xHCI MMIO bytespace found");
        None
    };

    let mmio_ptr = match mmio_ptr {
        Some(p) => p,
        None => {
            log_info("USBD: cannot proceed without xHCI MMIO");
            loop { sched_yield(); }
        }
    };

    // Allocate DMA buffers
    log_info("USBD: allocating DMA bytespaces");

    let (dcbaa_id, dcbaa_phys) = alloc_and_map_dma(PAGE_SIZE, DMA_DCBAA_VADDR);
    let (cmd_ring_id, cmd_ring_phys) = alloc_and_map_dma(PAGE_SIZE, DMA_CMD_RING_VADDR);
    let (erst_id, erst_phys) = alloc_and_map_dma(PAGE_SIZE, DMA_ERST_VADDR);
    let (event_ring_id, event_ring_phys) = alloc_and_map_dma(PAGE_SIZE, DMA_EVENT_RING_VADDR);

    log_info(&format!("USBD: DCBAA phys={:#x}", dcbaa_phys));
    log_info(&format!("USBD: CMD ring phys={:#x}", cmd_ring_phys));
    log_info(&format!("USBD: ERST phys={:#x}", erst_phys));
    log_info(&format!("USBD: Event ring phys={:#x}", event_ring_phys));

    // Verify DMA buffers are allocated (phys != 0)
    if dcbaa_phys == 0 || cmd_ring_phys == 0 || erst_phys == 0 || event_ring_phys == 0 {
        log_info("USBD: FATAL: DMA allocation failed (phys=0)");
        loop { sched_yield(); }
    }

    // Suppress unused variable warnings
    let _ = (dcbaa_id, cmd_ring_id, erst_id, event_ring_id);

    // Initialize xHCI controller
    let xhci = unsafe { XhciController::new(mmio_ptr) };
    let cap = xhci.cap();
    log_info(&format!("USBD: xHCI CAPLEN={} HCIVERSION={:#x}", cap.caplength, cap.hciversion));
    log_info(&format!("USBD: max_slots={} max_ports={}", cap.max_slots(), cap.max_ports()));

    // 1. Wait for controller ready
    if !xhci.wait_ready() {
        log_info("USBD: controller not ready (CNR=1)");
        loop { sched_yield(); }
    }
    log_info("USBD: controller ready");

    // 2. Stop controller
    if !xhci.stop() {
        log_info("USBD: failed to stop controller");
    }
    log_info("USBD: controller stopped");

    // 3. Reset controller
    if !xhci.reset() {
        log_info("USBD: reset failed");
        loop { sched_yield(); }
    }
    log_info("USBD: controller reset complete");

    // 4. Set max slots enabled (use max available)
    xhci.set_max_slots(cap.max_slots());
    log_info(&format!("USBD: max slots set to {}", cap.max_slots()));

    // 5. Initialize DCBAA (Device Context Base Address Array)
    unsafe {
        let dcbaa_ptr = DMA_DCBAA_VADDR as *mut u64;
        for i in 0..256 {
            core::ptr::write_volatile(dcbaa_ptr.add(i), 0);
        }
    }
    xhci.set_dcbaap(dcbaa_phys);
    log_info("USBD: DCBAAP programmed");

    // 6. Initialize command ring
    let cmd_ring_capacity = (PAGE_SIZE as usize) / 16; // 256 TRBs
    let mut cmd_ring = unsafe {
        CommandRing::new(
            DMA_CMD_RING_VADDR as *mut Trb,
            cmd_ring_phys,
            cmd_ring_capacity,
        )
    };
    xhci.set_crcr(cmd_ring_phys, true); // RCS = 1
    log_info("USBD: CRCR programmed");

    // 7. Initialize Event Ring Segment Table
    let event_ring_capacity = (PAGE_SIZE as usize) / 16; // 256 TRBs
    unsafe {
        let erst_ptr = DMA_ERST_VADDR as *mut ErstEntry;
        core::ptr::write_volatile(erst_ptr, ErstEntry {
            ring_segment_base: event_ring_phys,
            ring_segment_size: event_ring_capacity as u16,
            reserved: 0,
            reserved2: 0,
        });
    }

    // 8. Initialize Event Ring (zero it)
    unsafe {
        let event_ring_ptr = DMA_EVENT_RING_VADDR as *mut u8;
        for i in 0..PAGE_SIZE as usize {
            core::ptr::write_volatile(event_ring_ptr.add(i), 0);
        }
    }

    let mut event_ring = unsafe {
        EventRing::new(
            DMA_EVENT_RING_VADDR as *const Trb,
            event_ring_phys,
            event_ring_capacity,
        )
    };

    // 9. Configure Interrupter 0
    xhci.setup_interrupter(erst_phys, 1, event_ring_phys);
    log_info("USBD: Interrupter 0 configured");

    // 10. Start controller
    xhci.start();
    log_info("USBD: controller started (RUN=1)");

    // 11. Submit NO-OP command
    let noop = Trb::no_op_cmd(true);
    let noop_phys = unsafe { cmd_ring.enqueue(noop) };
    log_info(&format!("USBD: NO-OP TRB enqueued at phys={:#x?}", noop_phys));

    // 12. Ring doorbell 0 (host controller)
    xhci.ring_doorbell(0, 0);
    log_info("USBD: doorbell 0 rung");

    // 13. Poll for completion event
    log_info("USBD: polling event ring for completion...");
    let mut poll_count = 0u32;
    let max_polls = 1_000_000;
    
    loop {
        if let Some(event) = unsafe { event_ring.poll() } {
            let trb_type = event.trb_type();
            let cc = event.completion_code();
            log_info(&format!(
                "USBD: Event TRB type={} cc={}", 
                trb_type, cc
            ));

            if trb_type == usb_xhci::trb_type::COMMAND_COMPLETION {
                if cc == usb_xhci::completion_code::SUCCESS {
                    log_info("USBD: xHCI: no-op completed!");
                } else {
                    log_info(&format!("USBD: NO-OP failed with cc={}", cc));
                }
                
                // Update ERDP to acknowledge
                xhci.update_erdp(event_ring.dequeue_phys());
                xhci.clear_event_interrupt();
                break;
            }
        }

        poll_count += 1;
        if poll_count >= max_polls {
            log_info("USBD: timeout waiting for completion event");
            break;
        }

        if poll_count % 100_000 == 0 {
            log_info(&format!("USBD: poll count {}", poll_count));
        }

        // Yield occasionally
        if poll_count % 1000 == 0 {
            sched_yield();
        }
    }

    // Map IRQ counter bytespace (optional)
    let irq_ptr = if let Some(bs_id) = thing_find("bytespace.irq.usb.xhci0") {
        memory::space_map(bs_id, IRQ_FLAG_VADDR, 0, PAGE_SIZE);
        Some(IRQ_FLAG_VADDR as *const core::sync::atomic::AtomicU32)
    } else {
        None
    };

    if let Some(counter) = irq_ptr {
        let val = unsafe { (*counter).load(Ordering::Relaxed) };
        log_info(&format!("USBD: xHCI IRQ count {}", val));
    }

    log_info("USBD: initialization complete, idling");

    // Keep running and report IRQ changes
    let mut last_irq = 0u32;
    loop {
        if let Some(counter) = irq_ptr {
            let val = unsafe { (*counter).load(Ordering::Relaxed) };
            if val != last_irq {
                last_irq = val;
                log_info(&format!("USBD: xHCI IRQ count {}", val));
            }
        }
        sched_yield();
    }
}

/// Allocate a DMA bytespace and map it, returning (ThingId, phys_base)
fn alloc_and_map_dma(size: u64, vaddr: u64) -> (ThingId, u64) {
    let (id, phys_base) = memory::dma_bytespace_create(size);
    if id.0 == 0 {
        log_info(&format!("USBD: DMA alloc failed for vaddr={:#x}", vaddr));
        return (ThingId(0), 0);
    }

    // Map the bytespace
    // Note: space_map expects full ThingId, but DMA syscall returns only low bits
    // We use the ID directly since it's stored as low bits only
    memory::space_map(id, vaddr, 0, size);

    (id, phys_base)
}
