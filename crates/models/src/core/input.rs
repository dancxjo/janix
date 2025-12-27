// use abi::ThingId; // Removed by agent
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyboardBody {
    /// e.g. "ps2"
    pub bus: abi::SymbolId,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawKeyEventStreamBody {
    pub head_seq: u64,
    pub capacity: u32,
    pub dropped: u64,
    pub events: alloc::vec::Vec<abi::wire::input::RawKeyEvent>,
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
    pub events: alloc::vec::Vec<abi::wire::input::KeyEvent>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TextEventStreamBody {
    pub head_seq: u64,
    pub capacity: u32,
    pub dropped: u64,
    pub events: alloc::vec::Vec<abi::wire::input::TextEvent>,
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeyEventBody {
    pub event: abi::wire::input::KeyEvent,
}
