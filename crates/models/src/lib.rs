#![no_std]

extern crate alloc;

use abi::ids::{SymbolId, ThingId};
use alloc::vec::Vec;
use thing_macros::Thing; // Ensure we have access to Vec for glue

// Include generated glue
include!("generated.rs");

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(
    kind = "kind.DisplayDevice",
    schema = "schema.DisplayDevice",
    version = 1
)]
pub struct DisplayDeviceBody {
    pub framebuffer: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
    pub refresh_hz: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.Framebuffer", schema = "schema.Framebuffer", version = 1)]
pub struct FramebufferBody {
    pub bytespace: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
}

/// A Place is a container for Things.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.Place", schema = "schema.Place", version = 1)]
pub struct PlaceBody {
    pub name: SymbolId,
}

/// A Relationship is a directed link between two Things with a semantic predicate.
/// Note: Relationship is itself a Thing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(
    kind = "kind.Relationship",
    schema = "schema.Relationship",
    version = 1
)]
pub struct RelationshipBody {
    pub from: ThingId,
    pub to: ThingId,
    pub predicate: SymbolId,
}

// Retrofitting existing bodies
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.Surface", schema = "schema.Surface", version = 1)]
pub struct SurfaceBody {
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
    pub bytespace: ThingId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.Bytespace", schema = "schema.Bytespace", version = 1)]
pub struct BytespaceBody {
    pub len: u64,
    pub flags: u32,
    pub _pad: u32,
    pub phys_base: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(
    kind = "kind.MonotonicClock",
    schema = "schema.MonotonicClock",
    version = 1
)]
pub struct MonotonicClockBody {
    pub now_ns: u64,
    pub resolution_ns: u64,
    pub source: SymbolId,
    pub last_update_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.SystemClock", schema = "schema.SystemClock", version = 1)]
pub struct SystemClockBody {
    pub unix_epoch_ns: i64,
    pub status: u32, // 0: Unset, 1: Set, 2: Slewing
    pub _pad: u32,
    pub last_set_mono_ns: u64,
    pub accuracy_ns: u64,
    pub source: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.TimeOffset", schema = "schema.TimeOffset", version = 1)]
pub struct TimeOffsetBody {
    pub offset_ns: i64,
    pub rate_ppb: i64,
    pub updated_mono_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.MouseStream", schema = "schema.MouseStream", version = 1)]
pub struct MouseStreamBody {
    pub bytespace: ThingId,
    pub capacity: u32,
    pub sample_size: u32,
    pub write_index: u32,
    pub dropped: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(
    kind = "kind.PointerState",
    schema = "schema.PointerState",
    version = 1
)]
pub struct PointerStateBody {
    pub stream: ThingId,
    pub x: i32,
    pub y: i32,
    pub buttons: u32,
    pub updated_at_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing)]
#[thing(kind = "kind.Window", schema = "schema.Window", version = 1)]
pub struct WindowBody {
    pub surface: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z: i32,
    pub title: SymbolId,
}
