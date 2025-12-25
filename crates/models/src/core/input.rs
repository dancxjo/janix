use abi::ThingId;
use serde::{Serialize, Deserialize};

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
