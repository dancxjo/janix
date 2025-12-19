use alloc::string::ToString;
use crate::{
    PropKey, PropType, PropValue, Thing, ThingId, create_thing, find_thing, load_thing,
    register_schema_for, update_props,
};

#[cfg(target_os = "none")]
use alloc::vec::Vec;
#[cfg(not(target_os = "none"))]
use std::vec::Vec;

/// The reserved name value used by demo helpers to identify their shared state.
pub const DEMO_NAME_VAL: u64 = 0xCAFEBABE;

#[derive(Debug, Clone)]
/// Shared demonstration state that tracks hello/heartbeat ticks.
pub struct DemoState {
    pub id: ThingId,
    pub name: u64,
    pub hello_ticks: u64,
    pub heartbeat_ticks: u64,
}

impl Thing for DemoState {
    const KIND: &'static str = "DemoState";
    const DESCRIPTION: &'static str =
        "Shared state for demonstration applications tracking hello and heartbeat ticks";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name".to_string(), PropValue::U64(self.name)));
        out.push(("hello_ticks".to_string(), PropValue::U64(self.hello_ticks)));
        out.push(("heartbeat_ticks".to_string(), PropValue::U64(self.heartbeat_ticks)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = 0;
        let mut hello_ticks = 0;
        let mut heartbeat_ticks = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "name" => {
                    if let PropValue::U64(v) = prop.1 {
                        name = v;
                    }
                }
                "hello_ticks" => {
                    if let PropValue::U64(v) = prop.1 {
                        hello_ticks = v;
                    }
                }
                "heartbeat_ticks" => {
                    if let PropValue::U64(v) = prop.1 {
                        heartbeat_ticks = v;
                    }
                }
                _ => {}
            }
        }

        DemoState {
            id,
            name,
            hello_ticks,
            heartbeat_ticks,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("name", PropType::U64),
            ("hello_ticks", PropType::U64),
            ("heartbeat_ticks", PropType::U64),
        ]
    }
}

impl DemoState {
    /// Get the shared demo state, creating it if no existing Thing matches.
    pub fn get_or_create() -> Option<Self> {
        // Ensure schema is registered
        register_schema_for::<Self>();

        // Try to find existing
        if let Some(state) = find_thing::<Self>(|s| s.name == DEMO_NAME_VAL) {
            return Some(state);
        }

        // Create new
        let new_state = DemoState {
            id: ThingId(0), // Placeholder
            name: DEMO_NAME_VAL,
            hello_ticks: 0,
            heartbeat_ticks: 0,
        };

        let id = create_thing(&new_state)?;

        Some(DemoState {
            id,
            name: DEMO_NAME_VAL,
            hello_ticks: 0,
            heartbeat_ticks: 0,
        })
    }

    pub fn update_hello_ticks(&self, ticks: u64) -> bool {
        update_props(self.id, &[("hello_ticks".to_string(), PropValue::U64(ticks))])
    }

    /// Update the stored heartbeat tick count.
    pub fn update_heartbeat_ticks(&self, ticks: u64) -> bool {
        update_props(self.id, &[("heartbeat_ticks".to_string(), PropValue::U64(ticks))])
    }

    /// Read the current hello/heartbeat tick counters from kernel state.
    pub fn read(&self) -> Option<(u64, u64)> {
        let current = load_thing::<Self>(self.id)?;
        Some((current.hello_ticks, current.heartbeat_ticks))
    }
}
