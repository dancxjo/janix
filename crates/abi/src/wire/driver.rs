extern crate alloc;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriverEvent {
    Irq { irq: u8 },
    Ps2Scancode { scancode: u8 },
    Ps2MouseByte { byte: u8 },
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriverPublish {
    Observation { thing_bytes: alloc::vec::Vec<u8> },
}
