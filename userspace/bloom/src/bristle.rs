use abi::hid::{
    BristleEventHeader, Key, KeyEventPayload, PointerButtonPayload, PointerMovePayload,
    BRISTLE_EVENT_MAGIC, BRISTLE_EVENT_VERSION,
};
use alloc::vec::Vec;
use stem::syscall::{channel_try_recv, ChannelHandle};

use crate::cursor::CursorState;

/// Structured pointer event for window manager processing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerEvent {
    /// Relative movement (accelerated dx, dy).
    Move { dx: i16, dy: i16 },
    /// Button pressed at current cursor position.
    ButtonDown { button: u8 },
    /// Button released at current cursor position.
    ButtonUp { button: u8 },
}

#[derive(Clone, Copy)]
pub struct MouseAccelConfig {
    pub enabled: bool,
    pub accel_strength: f32,
    pub speed_scale: f32,
    pub max_gain: Option<f32>,
    /// Base sensitivity multiplier applied to all movement
    pub base_sensitivity: f32,
}

impl Default for MouseAccelConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            accel_strength: 1.2, // Increased from 0.8 for more responsive acceleration
            speed_scale: 200.0,  // Lowered from 500.0 for earlier acceleration onset
            max_gain: Some(6.0), // Increased from 4.0 for faster max speed
            base_sensitivity: 2.0, // 2x base multiplier for all movement
        }
    }
}

#[derive(Default)]
pub struct MouseAccelState {
    last_timestamp_ns: Option<u64>,
}

#[derive(Default, Clone, Copy)]
pub struct PollStats {
    pub had_key_event: bool,
    pub had_pointer_event: bool,
    pub key_event_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub pressed: bool,
    pub mods: u8,
    pub repeat: bool,
    pub timestamp_ns: u64,
}

fn apply_mouse_accel(delta: (i16, i16), dt_s: f32, cfg: &MouseAccelConfig) -> (i16, i16) {
    // Always apply base sensitivity
    let dx = delta.0 as f32 * cfg.base_sensitivity;
    let dy = delta.1 as f32 * cfg.base_sensitivity;

    if !cfg.enabled || dt_s <= 0.0 || cfg.speed_scale <= 0.0 {
        let ax = libm::roundf(dx) as i32;
        let ay = libm::roundf(dy) as i32;
        return (
            ax.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
            ay.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
        );
    }

    let speed = libm::sqrtf(dx * dx + dy * dy) / dt_s;
    let mut gain = 1.0 + cfg.accel_strength * libm::logf(1.0 + speed / cfg.speed_scale);
    if let Some(max_gain) = cfg.max_gain {
        if gain > max_gain {
            gain = max_gain;
        }
    }

    let ax = libm::roundf(dx * gain) as i32;
    let ay = libm::roundf(dy * gain) as i32;
    (
        ax.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
        ay.clamp(i16::MIN as i32, i16::MAX as i32) as i16,
    )
}

pub fn poll_bristle(
    handle: ChannelHandle,
    cursor: &mut CursorState,
    keys: &mut alloc::collections::BTreeSet<abi::hid::Key>,
    key_events: &mut Vec<KeyEvent>,
    accel_cfg: &MouseAccelConfig,
    accel_state: &mut MouseAccelState,
    w: i32,
    h: i32,
) -> PollStats {
    let mut buf = [0u8; 256];
    let mut stats = PollStats::default();

    loop {
        let n = match channel_try_recv(handle, &mut buf) {
            Ok(n) => n,
            Err(_) => return stats,
        };

        if n == 0 {
            return stats;
        }

        let mut offset = 0usize;
        while offset + BristleEventHeader::SIZE <= n {
            let header_ptr = unsafe { buf.as_ptr().add(offset) };
            let header: BristleEventHeader =
                unsafe { core::ptr::read_unaligned(header_ptr as *const BristleEventHeader) };
            let magic = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.magic)) };
            let version = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.version)) };
            let event_type =
                unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.event_type)) };
            let payload_len =
                unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.payload_len)) }
                    as usize;
            let timestamp_ns =
                unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.timestamp_ns)) };

            if magic != BRISTLE_EVENT_MAGIC || version != BRISTLE_EVENT_VERSION {
                break;
            }

            let total = BristleEventHeader::SIZE + payload_len;
            if offset + total > n {
                break;
            }

            let payload_ptr = unsafe { header_ptr.add(BristleEventHeader::SIZE) };
            match event_type {
                1 => {
                    if payload_len >= KeyEventPayload::SIZE {
                        let payload: KeyEventPayload = unsafe {
                            core::ptr::read_unaligned(payload_ptr as *const KeyEventPayload)
                        };
                        keys.insert(payload.key());
                        stats.had_key_event = true;
                        stats.key_event_count += 1;
                        key_events.push(KeyEvent {
                            key: payload.key(),
                            pressed: true,
                            mods: payload.mods,
                            repeat: payload.is_repeat(),
                            timestamp_ns,
                        });
                    }
                }
                2 => {
                    if payload_len >= KeyEventPayload::SIZE {
                        let payload: KeyEventPayload = unsafe {
                            core::ptr::read_unaligned(payload_ptr as *const KeyEventPayload)
                        };
                        keys.remove(&payload.key());
                        stats.had_key_event = true;
                        stats.key_event_count += 1;
                        key_events.push(KeyEvent {
                            key: payload.key(),
                            pressed: false,
                            mods: payload.mods,
                            repeat: false,
                            timestamp_ns,
                        });
                    }
                }
                3 => {
                    if payload_len >= PointerMovePayload::SIZE {
                        let payload: PointerMovePayload = unsafe {
                            core::ptr::read_unaligned(payload_ptr as *const PointerMovePayload)
                        };
                        let dx =
                            unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dx)) };
                        let dy =
                            unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dy)) };
                        let dt_s = match accel_state.last_timestamp_ns {
                            Some(prev) if timestamp_ns > prev => {
                                (timestamp_ns - prev) as f32 * 1.0e-9
                            }
                            _ => 0.0,
                        };
                        accel_state.last_timestamp_ns = Some(timestamp_ns);

                        let dt_s = if dt_s > 0.25 { 0.0 } else { dt_s };
                        let (ax, ay) = apply_mouse_accel((dx, dy), dt_s, accel_cfg);
                        cursor.apply_move(ax, ay, w, h);
                        stats.had_pointer_event = true;
                    }
                }
                4 => {
                    if payload_len >= PointerButtonPayload::SIZE {
                        let payload: PointerButtonPayload = unsafe {
                            core::ptr::read_unaligned(payload_ptr as *const PointerButtonPayload)
                        };
                        let btn = unsafe {
                            core::ptr::read_unaligned(core::ptr::addr_of!(payload.button))
                        };
                        cursor.button_down(btn);
                        stats.had_pointer_event = true;
                    }
                }
                5 => {
                    if payload_len >= PointerButtonPayload::SIZE {
                        let payload: PointerButtonPayload = unsafe {
                            core::ptr::read_unaligned(payload_ptr as *const PointerButtonPayload)
                        };
                        let btn = unsafe {
                            core::ptr::read_unaligned(core::ptr::addr_of!(payload.button))
                        };
                        cursor.button_up(btn);
                        stats.had_pointer_event = true;
                    }
                }
                _ => {}
            }

            offset += total;
        }
    }
}

/// Poll for pointer events and return them as structured events.
///
/// Unlike `poll_bristle`, this does not mutate cursor state.
/// The caller is responsible for applying movement and button state
/// after processing events through the window manager.
pub fn poll_pointer_events(
    handle: ChannelHandle,
    accel_cfg: &MouseAccelConfig,
    accel_state: &mut MouseAccelState,
) -> Vec<PointerEvent> {
    let mut events = Vec::new();
    let mut buf = [0u8; 256];

    loop {
        let n = match channel_try_recv(handle, &mut buf) {
            Ok(n) => n,
            Err(_) => return events,
        };

        if n == 0 {
            return events;
        }

        if n < BristleEventHeader::SIZE {
            continue;
        }

        let header: BristleEventHeader =
            unsafe { core::ptr::read_unaligned(buf.as_ptr() as *const BristleEventHeader) };
        let magic = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.magic)) };
        let version = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.version)) };
        let event_type =
            unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.event_type)) };
        let payload_len =
            unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.payload_len)) } as usize;
        let timestamp_ns =
            unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(header.timestamp_ns)) };

        if magic != BRISTLE_EVENT_MAGIC || version != BRISTLE_EVENT_VERSION {
            continue;
        }

        let total = BristleEventHeader::SIZE + payload_len;
        if n < total {
            continue;
        }

        match event_type {
            3 => {
                // PointerMove
                if payload_len >= PointerMovePayload::SIZE {
                    let payload: PointerMovePayload = unsafe {
                        core::ptr::read_unaligned(
                            buf.as_ptr().add(BristleEventHeader::SIZE) as *const PointerMovePayload
                        )
                    };
                    let dx = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dx)) };
                    let dy = unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.dy)) };
                    let dt_s = match accel_state.last_timestamp_ns {
                        Some(prev) if timestamp_ns > prev => (timestamp_ns - prev) as f32 * 1.0e-9,
                        _ => 0.0,
                    };
                    accel_state.last_timestamp_ns = Some(timestamp_ns);

                    let dt_s = if dt_s > 0.25 { 0.0 } else { dt_s };
                    let (ax, ay) = apply_mouse_accel((dx, dy), dt_s, accel_cfg);
                    events.push(PointerEvent::Move { dx: ax, dy: ay });
                }
            }
            4 => {
                // PointerButtonDown
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(buf.as_ptr().add(BristleEventHeader::SIZE)
                            as *const PointerButtonPayload)
                    };
                    let btn =
                        unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.button)) };
                    events.push(PointerEvent::ButtonDown { button: btn });
                }
            }
            5 => {
                // PointerButtonUp
                if payload_len >= PointerButtonPayload::SIZE {
                    let payload: PointerButtonPayload = unsafe {
                        core::ptr::read_unaligned(buf.as_ptr().add(BristleEventHeader::SIZE)
                            as *const PointerButtonPayload)
                    };
                    let btn =
                        unsafe { core::ptr::read_unaligned(core::ptr::addr_of!(payload.button)) };
                    events.push(PointerEvent::ButtonUp { button: btn });
                }
            }
            _ => {}
        }
    }
}
