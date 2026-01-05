//! PS/2 Mouse Driver for x86_64.
//!
//! Uses the unified EventStream format with EV_POINTER_DELTA records.
//! Bloom maps the bytespace and polls events using EventStreamReader.

use crate::log::{self, Level};
use abi::events::{
    EventStreamHeader, EventRecord, PointerDeltaPayload,
    EV_POINTER_DELTA, HEADER_SIZE, RECORD_HEADER_SIZE, record_size,
};
use core::sync::atomic::Ordering;
use x86_64::instructions::port::Port;

/// Ring buffer capacity in bytes (for event records)
pub const RING_CAPACITY_BYTES: u32 = 8192;

/// Total bytespace size (header + ring)
pub const RING_BYTESPACE_SIZE: usize = HEADER_SIZE + RING_CAPACITY_BYTES as usize;

/// For backwards compat with boot.rs
pub const RING_CAPACITY: u32 = RING_CAPACITY_BYTES;

/// Size of a pointer delta record (header + payload, 8-byte aligned)
const POINTER_RECORD_SIZE: usize = record_size(core::mem::size_of::<PointerDeltaPayload>());

/// Static allocation for the EventStream buffer
#[repr(C, align(4096))]
struct EventStreamBuffer {
    header: EventStreamHeader,
    ring: [u8; RING_CAPACITY_BYTES as usize],
}

static mut STREAM_BUFFER: EventStreamBuffer = EventStreamBuffer {
    header: EventStreamHeader {
        magic: abi::events::EVENT_STREAM_MAGIC,
        version: abi::events::EVENT_STREAM_VERSION,
        header_bytes: HEADER_SIZE as u16,
        capacity_bytes: RING_CAPACITY_BYTES,
        write_seq: core::sync::atomic::AtomicU64::new(0),
        write_off: core::sync::atomic::AtomicU32::new(0),
        dropped: core::sync::atomic::AtomicU64::new(0),
        reserved: [0; 2],
    },
    ring: [0u8; RING_CAPACITY_BYTES as usize],
};

/// Packet accumulation state for 3-byte PS/2 protocol
static mut PACKET_IDX: usize = 0;
static mut PACKET: [u8; 3] = [0; 3];

/// Get the physical address of the ring buffer for bytespace creation.
pub fn get_ring_phys_addr() -> u64 {
    let virt = unsafe { &raw const STREAM_BUFFER as u64 };
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
    log::klog(Level::Info, "PS2", "input: discovered mouse (EventStream)");
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

    wait_input();
    cmd_port.write(0xA8);

    wait_input();
    cmd_port.write(0x20);
    wait_output();
    let mut status = data_port.read();

    status |= 0x02;
    status &= !0x20;

    wait_input();
    cmd_port.write(0x60);
    wait_input();
    data_port.write(status);

    wait_input();
    cmd_port.write(0xD4);
    wait_input();
    data_port.write(0xF6);
    wait_output();
    let _ack = data_port.read();

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
pub unsafe fn irq_handler() {
    let mut port = Port::<u8>::new(0x60);
    let byte = port.read();

    if PACKET_IDX == 0 && (byte & 0x08) == 0 {
        return;
    }

    PACKET[PACKET_IDX] = byte;
    PACKET_IDX += 1;

    if PACKET_IDX == 3 {
        let (dx, dy, buttons) = decode_packet();
        write_pointer_event(dx, dy, buttons, 0);
        PACKET_IDX = 0;
    }
}

/// Decode a 3-byte PS/2 packet
unsafe fn decode_packet() -> (i16, i16, u16) {
    let b0 = PACKET[0];
    let b1 = PACKET[1];
    let b2 = PACKET[2];

    let buttons = (b0 & 0x07) as u16;

    let mut dx: i16 = b1 as i16;
    if (b0 & 0x10) != 0 {
        dx |= !0xFF;
    }

    let mut dy: i16 = b2 as i16;
    if (b0 & 0x20) != 0 {
        dy |= !0xFF;
    }

    dy = -dy;

    (dx, dy, buttons)
}

/// Write an EV_POINTER_DELTA record to the EventStream
unsafe fn write_pointer_event(dx: i16, dy: i16, buttons: u16, wheel: i16) {
    let header = &raw mut STREAM_BUFFER.header;
    let ring = &raw mut STREAM_BUFFER.ring;

    let write_off = (*header).write_off.load(Ordering::Relaxed);
    let write_seq = (*header).write_seq.load(Ordering::Relaxed);

    // Check if we have space for this record
    if (write_off as usize + POINTER_RECORD_SIZE) > RING_CAPACITY_BYTES as usize {
        // Wrap around - for simplicity, just reset to beginning
        (*header).write_off.store(0, Ordering::Relaxed);
    }

    let offset = (*header).write_off.load(Ordering::Relaxed) as usize;
    let record_ptr = (*ring).as_mut_ptr().add(offset);

    // Write EventRecord header
    let record_len = POINTER_RECORD_SIZE as u16;
    core::ptr::write_volatile(record_ptr as *mut u16, record_len.to_le());
    core::ptr::write_volatile(record_ptr.add(2) as *mut u16, EV_POINTER_DELTA.to_le());
    core::ptr::write_volatile(record_ptr.add(4) as *mut u16, 0u16.to_le()); // flags
    core::ptr::write_volatile(record_ptr.add(6) as *mut u16, 0u16.to_le()); // reserved

    let next_seq = write_seq + 1;
    core::ptr::write_volatile(record_ptr.add(8) as *mut u64, next_seq.to_le());

    let t_ns = crate::sched::TIMER_TICKS.load(Ordering::Relaxed) * 1_000_000;
    core::ptr::write_volatile(record_ptr.add(16) as *mut u64, t_ns.to_le());

    // Write PointerDeltaPayload
    let payload_ptr = record_ptr.add(RECORD_HEADER_SIZE);
    core::ptr::write_volatile(payload_ptr as *mut i16, dx.to_le());
    core::ptr::write_volatile(payload_ptr.add(2) as *mut i16, dy.to_le());
    core::ptr::write_volatile(payload_ptr.add(4) as *mut u16, buttons.to_le());
    core::ptr::write_volatile(payload_ptr.add(6) as *mut i16, wheel.to_le());
    core::ptr::write_volatile(payload_ptr.add(8) as *mut i16, 0i16.to_le()); // reserved

    // Update write offset and sequence (release ordering for consumers)
    let new_offset = offset + POINTER_RECORD_SIZE;
    (*header).write_off.store(new_offset as u32, Ordering::Release);
    (*header).write_seq.store(next_seq, Ordering::Release);
}

pub fn set_bounds(_width: u32, _height: u32) {}
pub fn set_pointer_thing_id(_id: abi::ids::ThingId) {}
pub fn process_packets() {}
pub fn publish_if_dirty() -> bool { false }

/// Get diagnostic info (write_seq as u32, dropped as u32 for compat)
pub fn get_diagnostics() -> (u32, u32, bool) {
    unsafe {
        (
            STREAM_BUFFER.header.write_seq.load(Ordering::Relaxed) as u32,
            STREAM_BUFFER.header.dropped.load(Ordering::Relaxed) as u32,
            false,
        )
    }
}
