#![no_std]

extern crate alloc;
extern crate thing_models;

use abi::{PixelFormat, ThingId};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
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

use thing_models::{PropKey, PropType, PropValue, Thing};

#[derive(Clone, Debug)]
pub struct DisplayPresentRequest {
    pub id: ThingId,
    pub framebuffer_id: ThingId,
    pub frame_index: u64,
    pub requested_at_ns: u64,
    pub presented_at_ns: Option<u64>,
    pub completed: bool,
    pub damage_count: u32,
    pub damage_rects: Vec<u8>,
}

impl Thing for DisplayPresentRequest {
    const KIND: &'static str = "DisplayPresentRequest";
    const DESCRIPTION: &'static str = "A compositor request for a framebuffer driver to present a frame";

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("framebuffer_id", PropType::U64),
            ("frame_index", PropType::U64),
            ("requested_at_ns", PropType::U64),
            ("presented_at_ns", PropType::U64),
            ("completed", PropType::Bool),
            ("damage_count", PropType::U64),
            ("damage_rects", PropType::Blob),
        ]
    }

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("framebuffer_id".to_string(), PropValue::U64(self.framebuffer_id.0)));
        out.push(("frame_index".to_string(), PropValue::U64(self.frame_index)));
        out.push(("requested_at_ns".to_string(), PropValue::U64(self.requested_at_ns)));
        if let Some(val) = self.presented_at_ns {
            out.push(("presented_at_ns".to_string(), PropValue::U64(val)));
        }
        out.push(("completed".to_string(), PropValue::Bool(self.completed)));
        out.push(("damage_count".to_string(), PropValue::U64(self.damage_count as u64)));
        out.push(("damage_rects".to_string(), PropValue::Blob(self.damage_rects.clone())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut framebuffer_id = ThingId(0);
        let mut frame_index = 0;
        let mut requested_at_ns = 0;
        let mut presented_at_ns = None;
        let mut completed = false;
        let mut damage_count = 0;
        let mut damage_rects = Vec::new();

        for prop_opt in props {
            if let Some((k, v)) = prop_opt {
                match (k.as_str(), v) {
                    ("framebuffer_id", PropValue::U64(val)) => framebuffer_id = ThingId(*val),
                    ("frame_index", PropValue::U64(val)) => frame_index = *val,
                    ("requested_at_ns", PropValue::U64(val)) => requested_at_ns = *val,
                    ("presented_at_ns", PropValue::U64(val)) => presented_at_ns = Some(*val),
                    ("completed", PropValue::Bool(val)) => completed = *val,
                    ("damage_count", PropValue::U64(val)) => damage_count = *val as u32,
                    ("damage_rects", PropValue::Blob(val)) => damage_rects = val.clone(),
                    _ => {}
                }
            }
        }

        Self {
            id,
            framebuffer_id,
            frame_index,
            requested_at_ns,
            presented_at_ns,
            completed,
            damage_count,
            damage_rects,
        }
    }
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
