//! Echo Mouse: Bristle Pointer Event Display
//!
//! Reads BristleEvents and prints only mouse events to the console.

#![no_std]
#![no_main]

use abi::hid::{
    BristleEventHeader, PointerMovePayload, PointerButtonPayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use stem::info;
use stem::syscall::{port_recv, PortHandle};

fn button_name(button: u8) -> &'static str {
    match button {
        0 => "Left",
        1 => "Right",
        2 => "Middle",
        _ => "?",
    }
}

fn parse_and_print_event(buf: &[u8]) {
    if buf.len() < 20 {
        return;
    }

    let header: BristleEventHeader = unsafe {
        core::ptr::read_unaligned(buf.as_ptr() as *const BristleEventHeader)
    };

    if header.magic != BRISTLE_EVENT_MAGIC {
        return;
    }
    if header.version != BRISTLE_EVENT_VERSION {
        return;
    }

    match header.event_type {
        3 => { // PointerMove
            if buf.len() >= 24 {
                let payload: PointerMovePayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerMovePayload)
                };
                let dx = payload.dx;
                let dy = payload.dy;
                info!("PointerMove dx={} dy={}", dx, dy);
            }
        }
        4 => { // PointerButtonDown
            if buf.len() >= 22 {
                let payload: PointerButtonPayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerButtonPayload)
                };
                let btn = payload.button;
                info!("PointerButtonDown {}", button_name(btn));
            }
        }
        5 => { // PointerButtonUp
            if buf.len() >= 22 {
                let payload: PointerButtonPayload = unsafe {
                    core::ptr::read_unaligned(buf.as_ptr().add(20) as *const PointerButtonPayload)
                };
                let btn = payload.button;
                info!("PointerButtonUp {}", button_name(btn));
            }
        }
        _ => {}
    }
}

#[stem::main]
fn main(evt_read_handle: usize) -> ! {
    let handle = evt_read_handle as PortHandle;

    info!("echo_mouse: online (handle={})", handle);
    info!("echo_mouse: ready for mouse events");

    let mut buf = [0u8; 256];

    loop {
        match port_recv(handle, &mut buf) {
            Ok(n) if n >= 20 => {
                parse_and_print_event(&buf[..n]);
            }
            _ => {
                stem::yield_now();
            }
        }
    }
}
