use abi::ThingId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardBody {
    /// e.g. "ps2"
    pub bus: abi::SymbolId,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyEventBody {
    /// The keyboard device ThingId that originated the event
    pub device: ThingId,

    /// Raw PS/2 set 1 scancode byte (v0)
    pub scancode: u8,

    /// True if this is a key release (break) event (v0 set-1: high bit set)
    pub is_release: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyEventCompact {
    pub scancode: u8,
    pub is_release: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyEventStreamBody {
    pub head_seq: u64,
    pub capacity: u32,
    pub dropped: u64,
    pub events: alloc::vec::Vec<KeyEventCompact>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MouseBody {
    /// e.g. "ps2"
    pub bus: abi::SymbolId,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PointerEventCompact {
    pub dx: i16,
    pub dy: i16,
    pub scroll: i8,
    pub buttons: u8, // bitmask: 1=Left, 2=Right, 4=Middle
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PointerEventStreamBody {
    pub head_seq: u64,
    pub capacity: u32,
    pub dropped: u64,
    pub events: alloc::vec::Vec<PointerEventCompact>,
}
