//! PS/2 Mouse Driver for x86_64.
//!
//! Implements a simple ring buffer for capturing mouse packets in the IRQ handler,
//! decodes 3-byte PS/2 mouse packets, and publishes pointer state to the graph.

use crate::log::{self, Level};
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU8, Ordering};
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;

/// Size of the mouse byte ring buffer.
const RING_SIZE: usize = 256;

static mut MOUSE_BUFFER: [u8; RING_SIZE] = [0; RING_SIZE];
static mut HEAD: usize = 0; // Write index (IRQ)
static mut TAIL: usize = 0; // Read index (processing)
static OVERFLOWED: AtomicBool = AtomicBool::new(false);

// Pointer state - updated after each complete 3-byte packet
static POINTER_X: AtomicI32 = AtomicI32::new(0);
static POINTER_Y: AtomicI32 = AtomicI32::new(0);
static POINTER_BUTTONS: AtomicU8 = AtomicU8::new(0);

// Screen bounds for clamping (set during init based on framebuffer)
static SCREEN_WIDTH: AtomicI32 = AtomicI32::new(1280);
static SCREEN_HEIGHT: AtomicI32 = AtomicI32::new(720);

// Packet accumulation state
static mut PACKET_IDX: usize = 0;
static mut PACKET: [u8; 3] = [0; 3];

/// Initialize the PS/2 mouse.
pub fn init() {
    unsafe {
        init_mouse();
    }
    log::klog(Level::Info, "PS2", "input: discovered mouse");
}

/// Set screen bounds for pointer clamping.
pub fn set_bounds(width: u32, height: u32) {
    SCREEN_WIDTH.store(width as i32, Ordering::Relaxed);
    SCREEN_HEIGHT.store(height as i32, Ordering::Relaxed);
    // Center the pointer initially
    POINTER_X.store((width / 2) as i32, Ordering::Relaxed);
    POINTER_Y.store((height / 2) as i32, Ordering::Relaxed);
}

unsafe fn init_mouse() {
    let mut data_port = Port::<u8>::new(0x60);
    let mut cmd_port = Port::<u8>::new(0x64);

    // Wait for controller input buffer to be ready
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
    status |= 0x02;  // Enable IRQ12
    status &= !0x20; // Clear Disable Auxiliary Device

    wait_input();
    cmd_port.write(0x60); // Write CCB command
    wait_input();
    data_port.write(status);

    // 4. Use defaults
    wait_input();
    cmd_port.write(0xD4); // Next byte to aux
    wait_input();
    data_port.write(0xF6); // Set defaults
    wait_output();
    let _ack = data_port.read(); // Should be 0xFA

    // 5. Enable data reporting
    wait_input();
    cmd_port.write(0xD4); // Next byte to aux
    wait_input();
    data_port.write(0xF4); // Enable data reporting
    wait_output();
    let ack = data_port.read();

    log::klog(
        Level::Info,
        "PS2",
        &alloc::format!("Mouse init ACK=0x{:02x}", ack),
    );
}

/// IRQ Handler for Mouse (IRQ 12).
///
/// This is called from the IDT trampoline.
pub unsafe fn irq_handler() {
    let mut port = Port::<u8>::new(0x60);
    let byte = port.read();

    // Push to ring buffer
    let next_head = (HEAD + 1) % RING_SIZE;

    if next_head == TAIL {
        // Full! Drop and set overflow flag.
        if !OVERFLOWED.swap(true, Ordering::Relaxed) {
            log::klog(Level::Warn, "PS2", "Mouse Input Overflow!");
        }
    } else {
        MOUSE_BUFFER[HEAD] = byte;
        HEAD = next_head;
    }
}

/// Process pending mouse bytes into packets and update pointer state.
/// Should be called periodically (e.g., from timer tick or dedicated task).
pub fn process_packets() {
    interrupts::without_interrupts(|| unsafe {
        while TAIL != HEAD {
            let byte = MOUSE_BUFFER[TAIL];
            TAIL = (TAIL + 1) % RING_SIZE;

            // Synchronization: byte 0 must have bit 3 set
            if PACKET_IDX == 0 && (byte & 0x08) == 0 {
                // Out of sync, skip this byte
                continue;
            }

            PACKET[PACKET_IDX] = byte;
            PACKET_IDX += 1;

            if PACKET_IDX == 3 {
                // Complete packet!
                decode_packet();
                PACKET_IDX = 0;
            }
        }
    });
}

unsafe fn decode_packet() {
    let b0 = PACKET[0];
    let b1 = PACKET[1];
    let b2 = PACKET[2];

    // Decode buttons
    let buttons = b0 & 0x07; // L=bit0, R=bit1, M=bit2
    POINTER_BUTTONS.store(buttons, Ordering::Relaxed);

    // Decode X delta with sign extension
    let mut dx: i32 = b1 as i32;
    if (b0 & 0x10) != 0 {
        dx |= !0xFF; // Sign extend
    }

    // Decode Y delta with sign extension
    let mut dy: i32 = b2 as i32;
    if (b0 & 0x20) != 0 {
        dy |= !0xFF; // Sign extend
    }

    // Invert Y: PS/2 positive Y is up, screen coords positive Y is down
    dy = -dy;

    // Update absolute position with clamping
    let max_x = SCREEN_WIDTH.load(Ordering::Relaxed) - 1;
    let max_y = SCREEN_HEIGHT.load(Ordering::Relaxed) - 1;

    let new_x = (POINTER_X.load(Ordering::Relaxed) + dx).clamp(0, max_x);
    let new_y = (POINTER_Y.load(Ordering::Relaxed) + dy).clamp(0, max_y);

    POINTER_X.store(new_x, Ordering::Relaxed);
    POINTER_Y.store(new_y, Ordering::Relaxed);
}

/// Get current pointer state.
pub fn get_pointer_state() -> (i32, i32, u8) {
    (
        POINTER_X.load(Ordering::Relaxed),
        POINTER_Y.load(Ordering::Relaxed),
        POINTER_BUTTONS.load(Ordering::Relaxed),
    )
}

/// Publish pointer state to the graph.
/// Called periodically to update the pointer Thing.
pub fn publish_to_graph(pointer_thing_id: abi::ids::ThingId) {
    let (x, y, buttons) = get_pointer_state();

    // Create payload: x(i32), y(i32), buttons(u8), padding(3)
    let mut payload = [0u8; 12];
    payload[0..4].copy_from_slice(&x.to_le_bytes());
    payload[4..8].copy_from_slice(&y.to_le_bytes());
    payload[8] = buttons;
    // bytes 9-11 are padding/reserved

    graph::store::thing_set_inline_payload(pointer_thing_id, &payload);
}

/// Send EOI to PIC2 for IRQ 12
pub unsafe fn ack() {
    // IRQ 12 is on slave PIC, need to EOI both
    let mut cmd2 = Port::<u8>::new(0xA0);
    let mut cmd1 = Port::<u8>::new(0x20);
    cmd2.write(0x20); // EOI to slave
    cmd1.write(0x20); // EOI to master
}
