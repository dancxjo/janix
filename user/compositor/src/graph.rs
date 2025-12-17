use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;
use abi::{KernelRequest, KernelResponse, PropKey, PropValue, ThingId};
use core::option::Option;
use thing_os::prelude::*;
use thing_os::{
    DisplayThing, Mode, ModeSwitchEvent, PrimaryDisplayBuffer, Surface, Window, active_mode,
    default_mode, graph_kinds, is_console_mode_active, load_thing, update_props, list_things_by_kind,
};

use crate::layout::LayoutPolicy;

pub fn active_framebuffer() -> Option<PrimaryDisplayBuffer> {
    thing_os::open_primary_display_buffer().ok()
}

pub fn swap_display_buffers(display_id: ThingId) -> Option<i64> {
    let display = load_thing::<DisplayThing>(display_id)?;
    let current_index = display.active_buffer_index;
    let new_index = if current_index == 0 { 1 } else { 0 };

    if !update_props(
        display_id,
        &[(
            graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
            PropValue::I64(new_index),
        )],
    ) {
        return None;
    }

    Some(new_index)
}

#[derive(Debug, Clone)]
struct SystemThing {
    id: ThingId,
}

impl thing_os::Thing for SystemThing {
    const KIND: &'static str = graph_kinds::KIND_SYSTEM;
    const DESCRIPTION: &'static str = "System Root";
    fn schema() -> &'static [(&'static str, abi::PropType)] { &[] }
    fn from_props(id: ThingId, _props: &[Option<(abi::PropKey, PropValue)>]) -> Self {
        SystemThing { id }
    }
    fn to_props(&self, _out: &mut Vec<(abi::PropKey, PropValue)>) {}
}

pub fn handle_mode_switches() {
    let systems: Vec<SystemThing> = list_things_by_kind();
    let system = match systems.first() {
        Some(s) => s,
        None => return,
    };

    let targets = thing_os::link_targets(system.id, graph_kinds::LINK_HAS_ACTIVE_MODE);
    if let Some(target_mode_id) = targets.first() {
        let current = current_mode();
        if current.map(|m| m.id) != Some(*target_mode_id) {
             let msg = alloc::format!("compositor: observed active mode edge pointing to {}", target_mode_id.0);
             let leaked = Box::leak(msg.into_boxed_str());
             println!("{}", leaked);
             
             // Check index
             if let Some(mode) = load_thing::<Mode>(*target_mode_id) {
                 println!("compositor: switching internal mode state");
                 set_active_mode(mode.index);
                 
                 let msg = format!("MODE observed -> {}", mode.name);
                 let leaked = Box::leak(msg.into_boxed_str());
                 println!("{}", leaked);
             }
        }
    }
}

pub fn current_mode() -> Option<Mode> {
    active_mode().or_else(|| default_mode())
}

pub fn collect_windows_for_place(place_id: ThingId) -> Vec<Window> {
    list_things_by_kind()
        .into_iter()
        .filter(|w: &Window| w.place_id == place_id)
        .collect()
}

pub fn collect_surfaces_for_windows(
    windows: &[Window],
) -> BTreeMap<ThingId, Surface> {
    let wanted: BTreeSet<ThingId> = windows.iter().map(|w| w.id).collect();
    let mut map = BTreeMap::new();
    for surface in list_things_by_kind::<Surface>() {
        if wanted.contains(&surface.window_id) {
            map.insert(surface.window_id, surface);
        }
    }
    map
}



pub fn console_mode_active() -> bool {
    is_console_mode_active()
}

pub fn layout_policy_for_mode(mode: &Mode) -> LayoutPolicy {
    mode.layout_policy
        .map(LayoutPolicy::from_i64)
        .unwrap_or_default()
}

fn set_active_mode(index: u8) {
    let modes: Vec<Mode> = list_things_by_kind();
    for mode in modes {
        let active = mode.index == index;
        let _ = update_props(
            mode.id,
            &[(graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(active))],
        );
    }
}
