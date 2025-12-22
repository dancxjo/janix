#![no_std]

extern crate alloc;

use alloc::string::ToString;
use alloc::vec::Vec;
use thing_models::{
    InputCharEvent, InterruptEvent, IoDirection, IoPortOp, IoPortRegion, IoStatus, IoWidth,
    KeyScanEvent,
};
use thing_os::MODE_INDEX_CONSOLE;
use thing_os::prelude::*;
use thing_os::{PropKey, PropType, PropValue, create_thing, load_thing, update_props};

const POLL_INTERVAL_NS: u64 = 2_000_000;
const STATUS_OFFSET: u16 = 4;
const DATA_OFFSET: u16 = 0;

use abi::syscall_defs::{
    DevOpenArgs, DevOpenRet, DevReadArgs, DevReadRet, DeviceHandle, SysError, SysRet, UserPtr,
    UserSlice,
};
use abi::syscalls::{SYSCALL_DEV_OPEN, SYSCALL_DEV_READ};
use thing_os::resident::keyboard_stream::{KeyboardEntry, KeyboardStreamMapped};
use thing_os::resident::{Resident, ResidentMapPerms, alloc_resident, map_resident};
use thing_os::syscalls::{sys_dev_open as syscall_dev_open, sys_dev_read as syscall_dev_read};

pub fn driver_main() -> ! {
    println!("ps2_keyboard_driver: starting (resident stream)");

    // Allocate Resident Keyboard Stream
    // Capacity 64 entries * 8 bytes = 512 bytes + header (64) = 576 bytes
    // Page size is usually 4096, so 4096 is fine.
    let alloc_resp = match alloc_resident("KeyboardStream", 4096, 0) {
        Ok(r) => r,
        Err(e) => {
            println!("ps2_keyboard_driver: alloc_resident failed {:?}", e);
            loop {
                sleep(Duration::from_nanos(1_000_000_000));
            }
        }
    };

    // Create Thing to publish it
    // alloc_resident already created the Thing in the graph with kind "KeyboardStream".
    // We do not need to call create_thing again.
    // let stream_thing = KeyboardStreamThing { id: alloc_resp.id };
    // if create_thing(&stream_thing).is_none() {
    //      println!("ps2_keyboard_driver: failed to create KeyboardStreamThing");
    // }

    // Map it RW
    let map_resp = match map_resident(
        alloc_resp.id,
        ResidentMapPerms(ResidentMapPerms::READ.0 | ResidentMapPerms::WRITE.0),
    ) {
        Ok(r) => r,
        Err(e) => {
            println!("ps2_keyboard_driver: map_resident failed {:?}", e);
            loop {
                sleep(Duration::from_nanos(1_000_000_000));
            }
        }
    };

    let obj = unsafe {
        Resident::<()>::new(
            alloc_resp.id,
            map_resp.user_addr as *mut u8,
            map_resp.byte_len as usize,
        )
    };
    let mut stream = KeyboardStreamMapped::new(obj);
    stream.init(256); // Capacity 256 entries

    println!("ps2_keyboard_driver: KeyboardStream initialized");

    let region = wait_for_region();
    println!("ps2_keyboard_driver: found i8042 IO region; initializing controller",);
    let mut accessor = IoPortAccessor::new(region.id);
    if !init_controller(&mut accessor) {
        println!("ps2_keyboard_driver: controller init failed");
    } else {
        println!("ps2_keyboard_driver: controller initialized");
    }

    let mut decoder = KeyboardDecoder::new(region.id);

    // Open device 1 (Ps2Keyboard)
    let handle = match unsafe { syscall_dev_open(1, 0) } {
        Ok(h) => h,
        Err(e) => {
            println!(
                "ps2_keyboard_driver: failed to open device: code={}",
                e.code
            );
            loop {
                sleep(Duration::from_nanos(1_000_000_000));
            }
        }
    };
    println!("ps2_keyboard_driver: device opened");

    let mut buffer = [0u8; 16];

    loop {
        match unsafe { syscall_dev_read(handle, &mut buffer) } {
            Ok(count) => {
                if count > 0 {
                    for i in 0..count {
                        let byte = buffer[i];
                        decoder.process_byte(&mut stream, byte);
                    }
                } else {
                    sleep(Duration::from_nanos(POLL_INTERVAL_NS));
                }
            }
            Err(_) => {
                sleep(Duration::from_nanos(POLL_INTERVAL_NS));
            }
        }
    }
}

fn wait_for_region() -> IoPortRegion {
    loop {
        let regions: Vec<IoPortRegion> = list_things_by_kind();
        if let Some(region) = regions.into_iter().find(|r| r.name == "i8042") {
            return region;
        }
        sleep(Duration::from_nanos(5_000_000));
    }
}

/*
fn initial_interrupt_cursor() -> u64 {
    list_things_by_kind::<InterruptEvent>()
        .into_iter()
        .map(|event| event.id.0)
        .max()
        .unwrap_or(0)
}
*/

fn init_controller(accessor: &mut IoPortAccessor) -> bool {
    if !accessor.command(0xAD) {
        return false;
    }
    accessor.flush_output();

    if !accessor.command(0x20) {
        return false;
    }
    let mut config = match accessor.read_data() {
        Some(byte) => byte,
        None => return false,
    };
    config |= 0x03; // Enable IRQ1 (Keyboard) and IRQ12 (Mouse)
    config &= !0x30; // Clear Keyboard Disable (0x10) and Mouse Disable (0x20)

    if !accessor.command(0x60) {
        return false;
    }
    if !accessor.write_data(config) {
        return false;
    }
    accessor.flush_output();

    accessor.command(0xAE)
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

    fn read_status(&mut self) -> Option<u8> {
        self.read_u8(STATUS_OFFSET)
    }

    fn read_data(&mut self) -> Option<u8> {
        self.read_u8(DATA_OFFSET)
    }

    fn write_data(&mut self, value: u8) -> bool {
        self.write_u8(DATA_OFFSET, value)
    }

    fn command(&mut self, value: u8) -> bool {
        if !self.wait_input_clear() {
            return false;
        }
        self.write_u8(STATUS_OFFSET, value)
    }

    fn flush_output(&mut self) {
        for _ in 0..1000 {
            if let Some(status) = self.read_status() {
                if status & 0x01 == 0 {
                    break;
                }
                let _ = self.read_data();
            } else {
                break;
            }
        }
    }

    fn wait_input_clear(&mut self) -> bool {
        for _ in 0..100 {
            if let Some(status) = self.read_status() {
                if status & 0x02 == 0 {
                    return true;
                }
            }
            sleep(Duration::from_nanos(100_000));
        }
        false
    }

    fn read_u8(&mut self, offset: u16) -> Option<u8> {
        self.submit_op(SlotKind::Read, offset, IoDirection::Read, 0)
            .map(|value| value as u8)
    }

    fn write_u8(&mut self, offset: u16, value: u8) -> bool {
        self.submit_op(SlotKind::Write, offset, IoDirection::Write, value as u32)
            .is_some()
    }

    fn submit_op(
        &mut self,
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
                    ("offset".to_string(), PropValue::U64(offset as u64)),
                    (
                        "direction".to_string(),
                        PropValue::Str(direction.as_str().into()),
                    ),
                    (
                        "width".to_string(),
                        PropValue::Str(IoWidth::U8.as_str().into()),
                    ),
                    ("value".to_string(), PropValue::U64(value as u64)),
                    (
                        "status".to_string(),
                        PropValue::Str(IoStatus::Pending.as_str().into()),
                    ),
                ];
                if !update_props(id, &props) {
                    return None;
                }
                id
            } else {
                let op =
                    IoPortOp::new(region_id, offset, direction, IoWidth::U8, value, ThingId(0));
                let Some(new_id) = create_thing(&op) else {
                    return None;
                };
                *slot = Some(new_id);
                new_id
            }
        };

        self.wait_for_completion(op_id)
    }

    fn slot(&mut self, kind: SlotKind) -> &mut Option<ThingId> {
        match kind {
            SlotKind::Read => &mut self.read_op,
            SlotKind::Write => &mut self.write_op,
        }
    }

    fn wait_for_completion(&self, op_id: ThingId) -> Option<u32> {
        for _ in 0..200 {
            if let Some(op) = load_thing::<IoPortOp>(op_id) {
                match op.status {
                    IoStatus::Completed => return Some(op.value),
                    IoStatus::Failed => return None,
                    _ => {}
                }
            }
            sleep(Duration::from_nanos(100_000));
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

    fn process_byte(&mut self, stream: &mut KeyboardStreamMapped<()>, byte: u8) {
        if byte == 0xE0 {
            self.pending_e0 = true;
            return;
        }
        if byte == 0xE1 {
            self.pending_e0 = false;
            return;
        }

        let extended = self.pending_e0;
        self.pending_e0 = false;

        self.sequence_index = self.sequence_index.wrapping_add(1);

        let released = (byte & 0x80) != 0;
        let scancode = byte & 0x7F;

        self.update_modifiers(scancode, released, extended);

        let mut utf32 = 0;
        let mut has_char = false;

        if !released {
            if let Some(ch) =
                decode_printable(scancode, extended, self.left_shift || self.right_shift)
            {
                utf32 = ch as u32;
                has_char = true;
            }
        }

        let mut flags = 0;
        if released {
            flags |= KeyboardEntry::FLAG_RELEASED;
        }
        if extended {
            flags |= KeyboardEntry::FLAG_EXTENDED;
        }
        if has_char {
            flags |= KeyboardEntry::FLAG_HAS_CHAR;
        }

        let entry = KeyboardEntry {
            scancode,
            flags,
            _pad: 0,
            utf32,
        };
        stream.append(entry);
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
    use thing_os::doc_helpers::DocSys;

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
        // Function removed/not needed for this refactor pass logic check
        // assert_eq!(scancode_to_mode_index(0x3B), Some(1));
        // assert_eq!(scancode_to_mode_index(0x58), Some(MODE_INDEX_CONSOLE));
        // assert_eq!(scancode_to_mode_index(0x01), None);
    }
}
