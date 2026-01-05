#![no_std]

extern crate alloc;

pub use thing_codec::Thing;
pub use abi::ids::{SymbolId, ThingId};
use thing_derive::Thing;
use serde::{Serialize, Deserialize};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.DisplayDevice")]
pub struct DisplayDevice {
    pub framebuffer: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
    pub refresh_hz: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Framebuffer")]
pub struct Framebuffer {
    pub bytespace: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
}

/// A Place is a container for Things.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Place")]
pub struct Place {
    pub name: SymbolId,
}

/// A Relationship is a directed link between two Things with a semantic predicate.
/// Note: Relationship is itself a Thing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Relationship")]
pub struct Relationship {
    pub from: ThingId,
    pub to: ThingId,
    pub predicate: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Surface")]
pub struct Surface {
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
    pub bytespace: ThingId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Bytespace")]
pub struct Bytespace {
    pub len: u64,
    pub flags: u32,
    pub _pad: u32,
    pub phys_base: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.MonotonicClock")]
pub struct MonotonicClock {
    pub now_ns: u64,
    pub resolution_ns: u64,
    pub source: SymbolId,
    pub last_update_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.SystemClock")]
pub struct SystemClock {
    pub unix_epoch_ns: i64,
    pub status: u32, // 0: Unset, 1: Set, 2: Slewing
    pub _pad: u32,
    pub last_set_mono_ns: u64,
    pub accuracy_ns: u64,
    pub source: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.TimeOffset")]
pub struct TimeOffset {
    pub offset_ns: i64,
    pub rate_ppb: i64,
    pub updated_mono_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.MouseStream")]
pub struct MouseStream {
    pub bytespace: ThingId,
    pub capacity: u32,
    pub sample_size: u32,
    pub write_index: u32,
    pub dropped: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.PointerState")]
pub struct Pointer {
    pub stream: ThingId,
    pub x: i32,
    pub y: i32,
    pub buttons: u32,
    pub updated_at_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Window")]
pub struct Window {
    pub surface: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z: i32,
    pub title: SymbolId,
}

/// EventStream: A unified ring buffer for high-frequency events.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.EventStream")]
pub struct EventStream {
    pub bytespace: ThingId,
    pub capacity_bytes: u32,
    pub max_record_bytes: u16,
    pub flags: u16,
    pub name: SymbolId,
}

// Aliases for transition

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.HardwareResource")]
pub struct HardwareInfo {
    pub name: SymbolId,
    pub resource_type: SymbolId, // "mmio", "ioport"
    pub start: u64,
    pub end: u64,
    pub irq: u32,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.RtcDevice")]
pub struct RtcDevice {
    pub source: SymbolId, // "cmos", "pl031", "stub"
    pub accuracy_ns: u64,
    pub base_seconds: u64,
    pub base_mono_ns: u64,
    pub flags: u32,
    pub _pad: u32,
}
