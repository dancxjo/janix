//! PS/2 Mouse Driver for x86_64.
//!
//! Uses a lock-free ring buffer of MouseSample structs for high-throughput
//! mouse input. Bloom maps the bytespace and polls samples directly.

use abi::mouse_ring::{MouseRingHeader, MouseSample, ring_bytespace_size};
use crate::log::{self, Level};
use core::sync::atomic::{AtomicPtr, Ordering};
use x86_64::instructions::port::Port;

/// Ring buffer capacity (number of samples)
pub const RING_CAPACITY: u32 = 256;

/// Total bytespace size for the mouse ring
pub const RING_BYTESPACE_SIZE: usize = ring_bytespace_size(RING_CAPACITY);

/// Static allocation for the ring buffer (header + samples)
#[repr(C, align(4096))]
struct RingBuffer {
    header: MouseRingHeader,
    samples: [MouseSample; RING_CAPACITY as usize],
}

static mut RING_BUFFER: RingBuffer = RingBuffer {
    header: MouseRingHeader {
        magic: abi::mouse_ring::MOUSE_RING_MAGIC,
        version: abi::mouse_ring::MOUSE_RING_VERSION,
        capacity: RING_CAPACITY,
        sample_size: core::mem::size_of::<MouseSample>() as u32,
        write: core::sync::atomic::AtomicU32::new(0),
        dropped: core::sync::atomic::AtomicU32::new(0),
        _reserved: [0; 2],
    },
    samples: [MouseSample { t_ns: 0, dx: 0, dy: 0, wheel: 0, buttons: 0 }; RING_CAPACITY as usize],
};

/// Packet accumulation state for 3-byte PS/2 protocol
static mut PACKET_IDX: usize = 0;
static mut PACKET: [u8; 3] = [0; 3];

/// Get the physical address of the ring buffer for bytespace creation.
pub fn get_ring_phys_addr() -> u64 {
    let virt = unsafe { &RING_BUFFER as *const _ as u64 };
    // Kernel statics are in the kernel text/data region (0xffffffff80...),
    // which has a different mapping than HHDM. Use machine's virt_to_phys.
    crate::machine::machine().virt_to_phys(virt)
}

/// Get the size of the ring buffer bytespace.
pub fn get_ring_size() -> usize {
    RING_BYTESPACE_SIZE
}

/// Initialize the PS/2 mouse.
pub fn init() {
    unsafe {
        init_mouse();
    }
    log::klog(Level::Info, "PS2", "input: discovered mouse");
}

unsafe fn init_mouse() {
    let mut data_port = Port::<u8>::new(0x60);
    let mut cmd_port = Port::<u8>::new(0x64);

    fn wait_input() {
        let mut status = Port::<u8>::new(0x64);
        for _ in 0..100_000 {
            if unsafe { status.read() } & 0x02 == 0 {
                return;
            }
        }
    }

    fn wait_output() {
        let mut status = Port::<u8>::new(0x64);
        for _ in 0..100_000 {
            if unsafe { status.read() } & 0x01 != 0 {
                return;
            }
        }
    }

    // 1. Enable auxiliary device (mouse)
    wait_input();
    cmd_port.write(0xA8);

    // 2. Get Compaq Status Byte (CCB)
    wait_input();
    cmd_port.write(0x20);
    wait_output();
    let mut status = data_port.read();

    // 3. Enable IRQ12 (bit 1) and clear Disable Aux (bit 5)
    status |= 0x02;
    status &= !0x20;

    wait_input();
    cmd_port.write(0x60);
    wait_input();
    data_port.write(status);

    // 4. Use defaults
    wait_input();
    cmd_port.write(0xD4);
    wait_input();
    data_port.write(0xF6);
    wait_output();
    let _ack = data_port.read();

    // 5. Enable data reporting
    wait_input();
    cmd_port.write(0xD4);
    wait_input();
    data_port.write(0xF4);
    wait_output();
    let ack = data_port.read();

    log::klog(
        Level::Info,
        "PS2",
        &alloc::format!("Mouse init ACK=0x{:02x}", ack),
    );
}

/// IRQ Handler for Mouse (IRQ 12).
/// Reads byte, accumulates into 3-byte packets, writes MouseSample to ring.
pub unsafe fn irq_handler() {
    let mut port = Port::<u8>::new(0x60);
    let byte = port.read();

    // Synchronization: byte 0 must have bit 3 set
    if PACKET_IDX == 0 && (byte & 0x08) == 0 {
        // Out of sync, skip
        return;
    }

    PACKET[PACKET_IDX] = byte;
    PACKET_IDX += 1;

    if PACKET_IDX == 3 {
        // Complete packet - decode and write to ring
        let sample = decode_packet();
        write_sample_to_ring(sample);
        PACKET_IDX = 0;
    }
}

/// Decode a 3-byte PS/2 packet into MouseSample
unsafe fn decode_packet() -> MouseSample {
    let b0 = PACKET[0];
    let b1 = PACKET[1];
    let b2 = PACKET[2];

    // Decode buttons
    let buttons = (b0 & 0x07) as u16;

    // Decode X delta with sign extension
    let mut dx: i16 = b1 as i16;
    if (b0 & 0x10) != 0 {
        dx |= !0xFF; // Sign extend
    }

    // Decode Y delta with sign extension  
    let mut dy: i16 = b2 as i16;
    if (b0 & 0x20) != 0 {
        dy |= !0xFF; // Sign extend
    }

    // Invert Y: PS/2 positive Y is up, screen coords positive Y is down
    dy = -dy;

    // Get timestamp (use timer ticks for now, could use HPET)
    let t_ns = crate::sched::TIMER_TICKS.load(Ordering::Relaxed) * 1_000_000; // ~1ms per tick

    MouseSample {
        t_ns,
        dx,
        dy,
        wheel: 0,
        buttons,
    }
}

/// Write a sample to the ring buffer (called from IRQ context)
unsafe fn write_sample_to_ring(sample: MouseSample) {
    let header = &mut RING_BUFFER.header;
    let samples = &mut RING_BUFFER.samples;
    
    let write_idx = header.write.load(Ordering::Relaxed);
    let slot = (write_idx % RING_CAPACITY) as usize;
    
    // Write sample to slot
    samples[slot] = sample;
    
    // Increment write index (release semantics for consumers)
    header.write.store(write_idx.wrapping_add(1), Ordering::Release);
}

/// Set screen bounds (for compatibility, not used in ring model)
pub fn set_bounds(_width: u32, _height: u32) {
    // Bounds handling moved to consumer (Bloom)
}

/// Deprecated: graph ID setting (not used in ring model)
pub fn set_pointer_thing_id(_id: abi::ids::ThingId) {
    // Graph updates handled elsewhere
}

/// Deprecated: packet processing (now done in IRQ handler)
pub fn process_packets() {
    // No-op - processing now happens in IRQ handler
}

/// Deprecated: graph publishing (not used in ring model)
pub fn publish_if_dirty() -> bool {
    false
}

/// Get diagnostic info
pub fn get_diagnostics() -> (u32, u32, bool) {
    unsafe {
        (
            RING_BUFFER.header.write.load(Ordering::Relaxed),
            RING_BUFFER.header.dropped.load(Ordering::Relaxed),
            false,
        )
    }
}
