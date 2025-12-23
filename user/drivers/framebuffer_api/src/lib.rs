#![no_std]

extern crate alloc;
extern crate thing_models;

use abi::{PixelFormat, ThingId};
use alloc::string::{String, ToString};
use thing_macros::Thing;

#[derive(Thing, Clone, Debug)]
#[thing(description = "Userland-published framebuffer backed by a SharedBuffer")]
pub struct DisplayFramebuffer {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    #[thing(via = "String")]
    pub pixel_format: PixelFormat,
    #[thing(via = "String")]
    pub power_state: DisplayPowerState,
    pub refresh_interval_ns: Option<u64>,
    pub frames_presented: u64,
    pub last_present_ns: u64,
}

#[derive(Thing, Clone, Debug)]
#[thing(description = "A compositor request for a framebuffer driver to present a frame")]
pub struct DisplayPresentRequest {
    pub id: ThingId,
    pub framebuffer_id: ThingId,
    pub frame_index: u64,
    pub requested_at_ns: u64,
    pub presented_at_ns: Option<u64>,
    pub completed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisplayPowerState {
    On,
    Sleep,
    Off,
}

impl core::fmt::Display for DisplayPowerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl core::str::FromStr for DisplayPowerState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Off" => Ok(DisplayPowerState::Off),
            "Sleep" => Ok(DisplayPowerState::Sleep),
            "On" => Ok(DisplayPowerState::On),
            _ => Err(()),
        }
    }
}

impl DisplayPowerState {
    pub const fn as_str(self) -> &'static str {
        match self {
            DisplayPowerState::On => "On",
            DisplayPowerState::Sleep => "Sleep",
            DisplayPowerState::Off => "Off",
        }
    }
}
