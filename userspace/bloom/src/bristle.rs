use abi::hid::{
    BristleEventHeader, PointerMovePayload, PointerButtonPayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION, KeyEventPayload,
};
use stem::syscall::{port_recv, PortHandle};

use crate::cursor::CursorState;

pub fn poll_bristle(handle: PortHandle, cursor: &mut CursorState, keys: &mut alloc::collections::BTreeSet<abi::hid::Key>, w: i32, h: i32) {
    let mut buf = [0u8; 256];

    loop {
        let n = match port_recv(handle, &mut buf) {
            Ok(n) => n,
            Err(_) => return,
        };

        if n == 0 {
            return;
        }

        if n < BristleEventHeader::SIZE {
            continue;
        }

        let header: BristleEventHeader = unsafe {
            core::ptr::read_unaligned(buf.as_ptr() as *const BristleEventHeader)
        };
        let magic = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.magic)) };
        let version = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.version)) };
        let event_type = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.event_type)) };
        let payload_len = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.payload_len)) } as usize;

        if magic != BRISTLE_EVENT_MAGIC || version != BRISTLE_EVENT_VERSION {
            continue;
        }

        let total = BristleEventHeader::SIZE + payload_len;
        if n < total {
            continue;
        }

        match event_type {
            1 => { // KeyDown
                if payload_len >= KeyEventPayload::SIZE {
                        let payload: KeyEventPayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const KeyEventPayload,
                        )
                    };
                    keys.insert(payload.key());
                }
            }
            2 => { // KeyUp
                if payload_len >= KeyEventPayload::SIZE {
                        let payload: KeyEventPayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const KeyEventPayload,
                        )
                    };
                    keys.remove(&payload.key());
                }
            }
            3 => {
                if payload_len >= PointerMovePayload::SIZE {
                    let payload: PointerMovePayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerMovePayload,
                        )
                    };
                    let dx = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dx)) };
                    let dy = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dy)) };
                    cursor.apply_move(dx, dy, w, h);
                }
            }
            4 => {
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerButtonPayload,
                        )
                    };
                    let btn = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.button)) };
                    cursor.button_down(btn);
                }
            }
            5 => {
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerButtonPayload,
                        )
                    };
                    let btn = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.button)) };
                    cursor.button_up(btn);
                }
            }
            _ => {}
        }
    }
}
