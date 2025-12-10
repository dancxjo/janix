use abi::{PropKey, PropValue, ThingId};
use alloc::string::String;

#[derive(thing_macros::Thing)]
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
