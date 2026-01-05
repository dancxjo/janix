#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use abi::events::{EventStreamHeader, HEADER_SIZE};

mod hid;
mod transport;

use transport::{InterruptInSource, SyntheticMouseSource};

const RING_CAPACITY: u32 = 64 * 1024;
const BYTESPACE_SIZE: u64 = (HEADER_SIZE as u32 + RING_CAPACITY) as u64;
const MAPPED_ADDR: u64 = 0x6000_0000;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("INPUTD: Starting...");

    // 1. Create or Find bytespace
    let bs = memory::bytespace_create(BYTESPACE_SIZE);
    if bs.0 == 0 {
        log_info("INPUTD: ERROR: Failed to create bytespace");
        sys_exit(1);
    }
    
    // 2. Register Name
    graph::thing_register_name(bs, "bytespace.input.pointer0");
    log_info("INPUTD: Created bytespace.input.pointer0");

    // 3. Map it
    let mapped = memory::space_map(bs, MAPPED_ADDR, 0, BYTESPACE_SIZE);
    if mapped != MAPPED_ADDR {
        log_info("INPUTD: ERROR: Failed to map bytespace");
        sys_exit(1);
    }

    // 4. Initialize Header
    let ptr = MAPPED_ADDR as *mut u8;
    let header = EventStreamHeader::new(RING_CAPACITY);
    unsafe {
        ptr.cast::<EventStreamHeader>().write(header);
    }

    // 5. Initialize Writer
    let mut writer = unsafe { 
        thing_std::event::EventStreamWriter::new(ptr, RING_CAPACITY) 
    };

    log_info("INPUTD: Ready. EventStream initialized.");

    // 6. Transport Loop
    let mut source = SyntheticMouseSource::new();
    let mut buf = [0u8; 64];
    let mut ts = 0u64;

    loop {
        if let Some(len) = source.poll(&mut buf, &mut ts) {
            let endpoint_data = &buf[..len];
            // Parse HID
            if let Some(mouse) = hid::parse_boot_mouse(endpoint_data) {
                // Convert to PointerDelta Payload
                let mut payload = abi::events::PointerDeltaPayload::default();
                payload.dx = mouse.dx;
                payload.dy = mouse.dy;
                payload.buttons = mouse.buttons;
                payload.wheel = mouse.wheel;
                
                // Pack into bytes
                let payload_bytes = unsafe {
                    core::slice::from_raw_parts(
                        &payload as *const _ as *const u8,
                        core::mem::size_of::<abi::events::PointerDeltaPayload>()
                    )
                };

                // Push to stream
                writer.push(
                    abi::events::EV_POINTER_DELTA, 
                    0, 
                    0, 
                    payload_bytes
                );
            }
        }
        
        sched_yield();
    }
}
