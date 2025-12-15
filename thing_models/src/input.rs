use abi::{PropKey, PropValue, ThingId};
use alloc::string::String;

#[derive(thing_macros::Thing, Clone, Copy, Debug)]
#[thing(description = "Raw scan code byte emitted by an input controller interrupt.")]
pub struct KeyScanEvent {
    pub id: ThingId,
    pub controller_id: ThingId,
    pub port_index: u8,
    pub scancode: u8,
    pub extended: bool,
    pub released: bool,
    pub sequence_index: u64,
    pub timestamp_ticks: u64,
}

impl KeyScanEvent {
    pub fn props_for(
        controller_id: ThingId,
        port_index: u8,
        scancode: u8,
        extended: bool,
        released: bool,
        sequence_index: u64,
        timestamp_ticks: u64,
    ) -> [(PropKey, PropValue); 7] {
        [
            ("controller_id", PropValue::U64(controller_id.0)),
            ("port_index", PropValue::U64(port_index as u64)),
            ("scancode", PropValue::U64(scancode as u64)),
            ("extended", PropValue::Bool(extended)),
            ("released", PropValue::Bool(released)),
            ("sequence_index", PropValue::U64(sequence_index)),
            ("timestamp_ticks", PropValue::U64(timestamp_ticks)),
        ]
    }
}

#[derive(thing_macros::Thing)]
#[thing(description = "User-facing character decoded from scan codes.")]
pub struct InputCharEvent {
    pub id: ThingId,
    pub ch: char,
    pub source_controller: ThingId,
    pub source_port_index: u8,
    pub sequence_index: u64,
}

impl InputCharEvent {
    pub fn props_for(
        ch: char,
        source_controller: ThingId,
        source_port_index: u8,
        sequence_index: u64,
    ) -> [(PropKey, PropValue); 4] {
        [
            ("ch", PropValue::Str(String::from(ch))),
            ("source_controller", PropValue::U64(source_controller.0)),
            (
                "source_port_index",
                PropValue::U64(source_port_index as u64),
            ),
            ("sequence_index", PropValue::U64(sequence_index)),
        ]
    }
}

#[derive(thing_macros::Thing)]
#[thing(description = "Raw PS/2 mouse packet decoded into deltas and buttons.")]
pub struct MousePacketEvent {
    pub id: ThingId,
    pub controller_id: ThingId,
    pub port_index: u64,
    pub sequence_index: u64,
    pub timestamp_ticks: u64,
    pub buttons: u64,
    pub delta_x: i64,
    pub delta_y: i64,
    pub overflow_x: bool,
    pub overflow_y: bool,
}

impl MousePacketEvent {
    #[allow(clippy::too_many_arguments)]
    pub fn props_for(
        controller_id: ThingId,
        port_index: u64,
        sequence_index: u64,
        timestamp_ticks: u64,
        buttons: u64,
        delta_x: i64,
        delta_y: i64,
        overflow_x: bool,
        overflow_y: bool,
    ) -> [(PropKey, PropValue); 9] {
        [
            ("controller_id", PropValue::U64(controller_id.0)),
            ("port_index", PropValue::U64(port_index)),
            ("sequence_index", PropValue::U64(sequence_index)),
            ("timestamp_ticks", PropValue::U64(timestamp_ticks)),
            ("buttons", PropValue::U64(buttons)),
            ("delta_x", PropValue::I64(delta_x)),
            ("delta_y", PropValue::I64(delta_y)),
            ("overflow_x", PropValue::Bool(overflow_x)),
            ("overflow_y", PropValue::Bool(overflow_y)),
        ]
    }
}
