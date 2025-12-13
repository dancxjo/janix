#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use thing_models::{
    InputCharEvent, InterruptEvent, InterruptRequest, IoDirection, IoPortOp, IoPortRegion,
    IoStatus, IoWidth, KeyScanEvent, ModeSwitchEvent,
};
use userland::prelude::*;
use userland_std::MODE_INDEX_CONSOLE;

const POLL_INTERVAL_NS: u64 = 2_000_000;
const STATUS_OFFSET: u16 = 4;
const DATA_OFFSET: u16 = 0;

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "ps2_keyboard_driver: starting");
    let _ = register_schema_for::<KeyScanEvent>(sys);
    let _ = register_schema_for::<InputCharEvent>(sys);
    let _ = register_schema_for::<ModeSwitchEvent>(sys);

    let region = wait_for_region(sys);
    println(
        sys,
        "ps2_keyboard_driver: found i8042 IO region; initializing controller",
    );
    let mut accessor = IoPortAccessor::new(region.id);
    if !init_controller(sys, &mut accessor) {
        println(sys, "ps2_keyboard_driver: controller init failed");
    } else {
        println(sys, "ps2_keyboard_driver: controller initialized");
        // Create an InterruptRequest Thing so the kernel will unmask IRQ1 via PIC.
        let _ = register_schema_for::<InterruptRequest>(sys);
        let irq_req = InterruptRequest {
            id: ThingId(0),
            irq_line: 1,
            enabled: true,
            owner_process: None,
        };
        if let Some(_id) = create_thing(sys, &irq_req) {
            println(sys, "ps2_keyboard_driver: created InterruptRequest");
        } else {
            println(
                sys,
                "ps2_keyboard_driver: failed to create InterruptRequest",
            );
        }
    }

    let mut decoder = KeyboardDecoder::new(region.id);
    let mut last_irq_id = initial_interrupt_cursor(sys);

    loop {
        println(sys, "ps2_keyboard_driver: listing InterruptEvent things");
        let events: Vec<InterruptEvent> = list_things_by_kind(sys);

        /*
         * Temporarily avoid calling slice::sort (which uses unsafe helpers)
         * while we gather diagnostics — perform a simple linear scan
         * instead. This reduces exposure to potential UB in the standard
         * library sort implementation and helps determine whether the
         * crash is triggered by the sort.
         */
        let mut handled = false;
        let mut max_seen = last_irq_id;
        for event in &events {
            if event.irq_line != 1 {
                continue;
            }
            if event.id.0 <= last_irq_id {
                continue;
            }
            if event.id.0 > max_seen {
                max_seen = event.id.0;
            }
            handled = true;
            drain_pending_bytes(sys, &mut accessor, &mut decoder);
        }
        last_irq_id = max_seen;

        if !handled {
            sys.sleep_for_ns(POLL_INTERVAL_NS);
        }
    }
}

fn wait_for_region<S: Sys>(sys: &mut S) -> IoPortRegion {
    loop {
        let regions: Vec<IoPortRegion> = list_things_by_kind(sys);
        if let Some(region) = regions.into_iter().find(|r| r.name == "i8042") {
            return region;
        }
        sys.sleep_for_ns(5_000_000);
    }
}

fn initial_interrupt_cursor<S: Sys>(sys: &mut S) -> u64 {
    list_things_by_kind::<S, InterruptEvent>(sys)
        .into_iter()
        .map(|event| event.id.0)
        .max()
        .unwrap_or(0)
}

fn init_controller<S: Sys>(sys: &mut S, accessor: &mut IoPortAccessor) -> bool {
    if !accessor.command(sys, 0xAD) {
        return false;
    }
    accessor.flush_output(sys);

    if !accessor.command(sys, 0x20) {
        return false;
    }
    let mut config = match accessor.read_data(sys) {
        Some(byte) => byte,
        None => return false,
    };
    config |= 0x01;
    config &= !0x10;

    if !accessor.command(sys, 0x60) {
        return false;
    }
    if !accessor.write_data(sys, config) {
        return false;
    }
    accessor.flush_output(sys);

    accessor.command(sys, 0xAE)
}

fn drain_pending_bytes<S: Sys>(
    sys: &mut S,
    accessor: &mut IoPortAccessor,
    decoder: &mut KeyboardDecoder,
) {
    loop {
        match accessor.read_status(sys) {
            Some(status) if status & 0x01 != 0 => {
                if status & 0x20 != 0 {
                    break;
                }
                if let Some(byte) = accessor.read_data(sys) {
                    decoder.process_byte(sys, byte);
                } else {
                    break;
                }
            }
            _ => break,
        }
    }
}

struct IoPortAccessor {
    region_id: ThingId,
    read_op: Option<ThingId>,
    write_op: Option<ThingId>,
}

impl IoPortAccessor {
    fn new(region_id: ThingId) -> Self {
        Self {
            region_id,
            read_op: None,
            write_op: None,
        }
    }

    fn read_status<S: Sys>(&mut self, sys: &mut S) -> Option<u8> {
        self.read_u8(sys, STATUS_OFFSET)
    }

    fn read_data<S: Sys>(&mut self, sys: &mut S) -> Option<u8> {
        self.read_u8(sys, DATA_OFFSET)
    }

    fn write_data<S: Sys>(&mut self, sys: &mut S, value: u8) -> bool {
        self.write_u8(sys, DATA_OFFSET, value)
    }

    fn command<S: Sys>(&mut self, sys: &mut S, value: u8) -> bool {
        if !self.wait_input_clear(sys) {
            return false;
        }
        self.write_u8(sys, STATUS_OFFSET, value)
    }

    fn flush_output<S: Sys>(&mut self, sys: &mut S) {
        while let Some(status) = self.read_status(sys) {
            if status & 0x01 == 0 {
                break;
            }
            let _ = self.read_data(sys);
        }
    }

    fn wait_input_clear<S: Sys>(&mut self, sys: &mut S) -> bool {
        for _ in 0..100 {
            if let Some(status) = self.read_status(sys) {
                if status & 0x02 == 0 {
                    return true;
                }
            }
            sys.sleep_for_ns(100_000);
        }
        false
    }

    fn read_u8<S: Sys>(&mut self, sys: &mut S, offset: u16) -> Option<u8> {
        self.submit_op(sys, SlotKind::Read, offset, IoDirection::Read, 0)
            .map(|value| value as u8)
    }

    fn write_u8<S: Sys>(&mut self, sys: &mut S, offset: u16, value: u8) -> bool {
        self.submit_op(
            sys,
            SlotKind::Write,
            offset,
            IoDirection::Write,
            value as u32,
        )
        .is_some()
    }

    fn submit_op<S: Sys>(
        &mut self,
        sys: &mut S,
        slot_kind: SlotKind,
        offset: u16,
        direction: IoDirection,
        value: u32,
    ) -> Option<u32> {
        let region_id = self.region_id;
        let op_id = {
            let slot = self.slot(slot_kind);
            if let Some(id) = *slot {
                let props = [
                    ("offset", PropValue::U64(offset as u64)),
                    ("direction", PropValue::Str(direction.as_str().into())),
                    ("width", PropValue::Str(IoWidth::U8.as_str().into())),
                    ("value", PropValue::U64(value as u64)),
                    ("status", PropValue::Str(IoStatus::Pending.as_str().into())),
                ];
                if !update_props(sys, id, &props) {
                    return None;
                }
                id
            } else {
                let op =
                    IoPortOp::new(region_id, offset, direction, IoWidth::U8, value, ThingId(0));
                let Some(new_id) = create_thing(sys, &op) else {
                    return None;
                };
                *slot = Some(new_id);
                new_id
            }
        };

        self.wait_for_completion(sys, op_id)
    }

    fn slot(&mut self, kind: SlotKind) -> &mut Option<ThingId> {
        match kind {
            SlotKind::Read => &mut self.read_op,
            SlotKind::Write => &mut self.write_op,
        }
    }

    fn wait_for_completion<S: Sys>(&self, sys: &mut S, op_id: ThingId) -> Option<u32> {
        for _ in 0..200 {
            if let Some(op) = load_thing::<IoPortOp>(sys, op_id) {
                match op.status {
                    IoStatus::Completed => return Some(op.value),
                    IoStatus::Failed => return None,
                    _ => {}
                }
            }
            sys.sleep_for_ns(100_000);
        }
        None
    }
}

enum SlotKind {
    Read,
    Write,
}

struct KeyboardDecoder {
    controller_id: ThingId,
    sequence_index: u64,
    pending_e0: bool,
    left_shift: bool,
    right_shift: bool,
    left_ctrl: bool,
    right_ctrl: bool,
    left_alt: bool,
    right_alt: bool,
}

impl KeyboardDecoder {
    fn new(controller_id: ThingId) -> Self {
        Self {
            controller_id,
            sequence_index: 0,
            pending_e0: false,
            left_shift: false,
            right_shift: false,
            left_ctrl: false,
            right_ctrl: false,
            left_alt: false,
            right_alt: false,
        }
    }

    fn process_byte<S: Sys>(&mut self, sys: &mut S, byte: u8) {
        let seq = self.next_sequence();
        let timestamp = sys.time_monotonic_ns();
        record_scan_event(sys, self.controller_id, byte, seq, timestamp);

        if let Some(mode_index) = scancode_to_mode_index(byte) {
            emit_mode_switch(sys, mode_index, timestamp);
        }

        if let Some(ch) = self.feed(byte) {
            emit_char(sys, self.controller_id, ch, seq);
        }
    }

    fn next_sequence(&mut self) -> u64 {
        self.sequence_index = self.sequence_index.saturating_add(1);
        self.sequence_index
    }

    fn feed(&mut self, byte: u8) -> Option<char> {
        if byte == 0xE0 {
            self.pending_e0 = true;
            return None;
        }
        if byte == 0xE1 {
            self.pending_e0 = false;
            return None;
        }

        let extended = self.pending_e0;
        self.pending_e0 = false;

        let released = (byte & 0x80) != 0;
        let scancode = byte & 0x7F;

        self.update_modifiers(scancode, released, extended);
        if released {
            return None;
        }

        decode_printable(scancode, extended, self.left_shift || self.right_shift)
    }

    fn update_modifiers(&mut self, scancode: u8, released: bool, extended: bool) {
        let pressed = !released;
        match (scancode, extended) {
            (0x2A, _) => self.left_shift = pressed,
            (0x36, _) => self.right_shift = pressed,
            (0x1D, false) => self.left_ctrl = pressed,
            (0x1D, true) => self.right_ctrl = pressed,
            (0x38, false) => self.left_alt = pressed,
            (0x38, true) => self.right_alt = pressed,
            _ => {}
        }
    }
}

fn scancode_to_mode_index(byte: u8) -> Option<u8> {
    let released = (byte & 0x80) != 0;
    if released {
        return None;
    }
    let scancode = byte & 0x7F;
    match scancode {
        0x3B => Some(1),
        0x3C => Some(2),
        0x3D => Some(3),
        0x3E => Some(4),
        0x3F => Some(5),
        0x40 => Some(6),
        0x41 => Some(7),
        0x42 => Some(8),
        0x43 => Some(9),
        0x44 => Some(10),
        0x57 => Some(11),
        0x58 => Some(MODE_INDEX_CONSOLE),
        _ => None,
    }
}

fn emit_mode_switch<S: Sys>(sys: &mut S, mode_index: u8, timestamp: u64) {
    let event = ModeSwitchEvent {
        id: ThingId(0),
        mode_index,
        timestamp,
    };
    let _ = create_thing(sys, &event);
}

fn record_scan_event<S: Sys>(
    sys: &mut S,
    controller_id: ThingId,
    scancode: u8,
    sequence_index: u64,
    timestamp_ns: u64,
) {
    let event = KeyScanEvent {
        id: ThingId(0),
        controller_id,
        port_index: 0,
        scancode,
        extended: false,
        released: (scancode & 0x80) != 0,
        sequence_index,
        timestamp_ticks: timestamp_ns,
    };
    let _ = create_thing(sys, &event);
}

fn emit_char<S: Sys>(sys: &mut S, controller_id: ThingId, ch: char, sequence_index: u64) {
    let event = InputCharEvent {
        id: ThingId(0),
        ch,
        source_controller: controller_id,
        source_port_index: 0,
        sequence_index,
    };
    let _ = create_thing(sys, &event);
}

fn decode_printable(scancode: u8, _extended: bool, shift: bool) -> Option<char> {
    let ch = match scancode {
        0x02 => {
            if shift {
                '!'
            } else {
                '1'
            }
        }
        0x03 => {
            if shift {
                '@'
            } else {
                '2'
            }
        }
        0x04 => {
            if shift {
                '#'
            } else {
                '3'
            }
        }
        0x05 => {
            if shift {
                '$'
            } else {
                '4'
            }
        }
        0x06 => {
            if shift {
                '%'
            } else {
                '5'
            }
        }
        0x07 => {
            if shift {
                '^'
            } else {
                '6'
            }
        }
        0x08 => {
            if shift {
                '&'
            } else {
                '7'
            }
        }
        0x09 => {
            if shift {
                '*'
            } else {
                '8'
            }
        }
        0x0A => {
            if shift {
                '('
            } else {
                '9'
            }
        }
        0x0B => {
            if shift {
                ')'
            } else {
                '0'
            }
        }
        0x0C => {
            if shift {
                '_'
            } else {
                '-'
            }
        }
        0x0D => {
            if shift {
                '+'
            } else {
                '='
            }
        }
        0x10 => letter('q', shift),
        0x11 => letter('w', shift),
        0x12 => letter('e', shift),
        0x13 => letter('r', shift),
        0x14 => letter('t', shift),
        0x15 => letter('y', shift),
        0x16 => letter('u', shift),
        0x17 => letter('i', shift),
        0x18 => letter('o', shift),
        0x19 => letter('p', shift),
        0x1A => {
            if shift {
                '{'
            } else {
                '['
            }
        }
        0x1B => {
            if shift {
                '}'
            } else {
                ']'
            }
        }
        0x1C => '\n',
        0x1E => letter('a', shift),
        0x1F => letter('s', shift),
        0x20 => letter('d', shift),
        0x21 => letter('f', shift),
        0x22 => letter('g', shift),
        0x23 => letter('h', shift),
        0x24 => letter('j', shift),
        0x25 => letter('k', shift),
        0x26 => letter('l', shift),
        0x27 => {
            if shift {
                ':'
            } else {
                ';'
            }
        }
        0x28 => {
            if shift {
                '"'
            } else {
                '\''
            }
        }
        0x29 => {
            if shift {
                '~'
            } else {
                '`'
            }
        }
        0x2B => {
            if shift {
                '|'
            } else {
                '\\'
            }
        }
        0x2C => letter('z', shift),
        0x2D => letter('x', shift),
        0x2E => letter('c', shift),
        0x2F => letter('v', shift),
        0x30 => letter('b', shift),
        0x31 => letter('n', shift),
        0x32 => letter('m', shift),
        0x33 => {
            if shift {
                '<'
            } else {
                ','
            }
        }
        0x34 => {
            if shift {
                '>'
            } else {
                '.'
            }
        }
        0x35 => {
            if shift {
                '?'
            } else {
                '/'
            }
        }
        0x39 => ' ',
        0x0E => '\u{0008}',
        _ => return None,
    };
    Some(ch)
}

fn letter(base: char, shift: bool) -> char {
    if shift {
        base.to_ascii_uppercase()
    } else {
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, PropKey, PropValue, Thing, ThingId};
    use alloc::vec::Vec;
    use thing_models::{InputCharEvent, KeyScanEvent};
    use userland_std::doc_helpers::DocSys;

    fn thing_sequence(props: &'static [(PropKey, PropValue)]) -> Option<u64> {
        props.iter().find_map(|(key, value)| {
            if *key == "sequence_index" {
                if let PropValue::U64(v) = value {
                    return Some(*v);
                }
            }
            None
        })
    }

    #[test]
    fn scancode_to_mode_index_handles_function_keys() {
        assert_eq!(scancode_to_mode_index(0x3B), Some(1));
        assert_eq!(scancode_to_mode_index(0x58), Some(MODE_INDEX_CONSOLE));
        assert_eq!(scancode_to_mode_index(0x01), None);
    }

    #[test]
    fn decode_printable_respects_shift() {
        assert_eq!(decode_printable(0x02, false, false), Some('1'));
        assert_eq!(decode_printable(0x02, false, true), Some('!'));
    }

    #[test]
    fn keyboard_decoder_emits_scan_and_char_events() {
        let mut sys = DocSys::with_responses(vec![
            KernelResponse::ThingCreated { id: ThingId(10) },
            KernelResponse::ThingCreated { id: ThingId(11) },
            KernelResponse::ThingCreated { id: ThingId(12) },
            KernelResponse::ThingCreated { id: ThingId(13) },
        ]);
        let mut decoder = KeyboardDecoder::new(ThingId(3));

        decoder.process_byte(&mut sys, 0x02);
        decoder.process_byte(&mut sys, 0x02);

        let requests = sys.requests.borrow();
        let scan_sequences: Vec<_> = requests
            .iter()
            .filter_map(|request| match request {
                KernelRequest::ThingCreate { kind, props } if *kind == KeyScanEvent::KIND => {
                    thing_sequence(props)
                }
                _ => None,
            })
            .collect();
        assert_eq!(scan_sequences, vec![1, 2]);

        let char_sequences: Vec<_> = requests
            .iter()
            .filter_map(|request| match request {
                KernelRequest::ThingCreate { kind, props } if *kind == InputCharEvent::KIND => {
                    thing_sequence(props)
                }
                _ => None,
            })
            .collect();
        assert_eq!(char_sequences, vec![1, 2]);
    }
}
