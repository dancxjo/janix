use crate::{
    PropKey, PropType, PropValue, Sys, Thing, ThingId, create_thing, find_thing, load_thing,
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
        out.push(("name", PropValue::U64(self.name)));
        out.push(("hello_ticks", PropValue::U64(self.hello_ticks)));
        out.push(("heartbeat_ticks", PropValue::U64(self.heartbeat_ticks)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = 0;
        let mut hello_ticks = 0;
        let mut heartbeat_ticks = 0;

        for prop in props.iter().flatten() {
            match prop.0 {
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
    pub fn get_or_create(sys: &impl Sys) -> Option<Self> {
        // Ensure schema is registered
        register_schema_for::<Self>(sys);

        // Try to find existing
        if let Some(state) = find_thing::<Self>(sys, |s| s.name == DEMO_NAME_VAL) {
            return Some(state);
        }

        // Create new
        let new_state = DemoState {
            id: ThingId(0), // Placeholder
            name: DEMO_NAME_VAL,
            hello_ticks: 0,
            heartbeat_ticks: 0,
        };

        let id = create_thing(sys, &new_state)?;

        Some(DemoState {
            id,
            name: DEMO_NAME_VAL,
            hello_ticks: 0,
            heartbeat_ticks: 0,
        })
    }

    pub fn update_hello_ticks(&self, sys: &impl Sys, ticks: u64) -> bool {
        update_props(sys, self.id, &[("hello_ticks", PropValue::U64(ticks))])
    }

    /// Update the stored heartbeat tick count.
    pub fn update_heartbeat_ticks(&self, sys: &impl Sys, ticks: u64) -> bool {
        update_props(sys, self.id, &[("heartbeat_ticks", PropValue::U64(ticks))])
    }

    /// Read the current hello/heartbeat tick counters from kernel state.
    ///
    /// # Examples
    ///
    /// ```
    /// use abi::{KernelResponse, PropValue, ThingId};
    /// use userland_std::demo_shared::{DemoState, DEMO_NAME_VAL};
    /// use userland_std::doc_helpers::DocSys;
    /// use userland_std::Thing;
    ///
    /// let props = DocSys::props_slice(vec![
    ///     ("name", PropValue::U64(DEMO_NAME_VAL)),
    ///     ("hello_ticks", PropValue::U64(4)),
    ///     ("heartbeat_ticks", PropValue::U64(5)),
    /// ]);
    /// let sys = DocSys::with_responses(vec![KernelResponse::ThingData {
    ///     id: ThingId(1),
    ///     kind: DemoState::KIND,
    ///     props,
    /// }]);
    /// let demo = DemoState {
    ///     id: ThingId(1),
    ///     name: DEMO_NAME_VAL,
    ///     hello_ticks: 0,
    ///     heartbeat_ticks: 0,
    /// };
    /// assert_eq!(demo.read(&sys), Some((4, 5)));
    /// ```
    pub fn read(&self, sys: &impl Sys) -> Option<(u64, u64)> {
        let current = load_thing::<Self>(sys, self.id)?;
        Some((current.hello_ticks, current.heartbeat_ticks))
    }
}
