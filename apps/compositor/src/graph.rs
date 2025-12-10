use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use userland::prelude::*;
use userland_std::thing_models::MousePacketEvent;
use userland_std::{
    Mode, ModeSwitchEvent, PrimaryDisplayBuffer, Surface, Window, active_mode, default_mode,
    graph_kinds, is_console_mode_active,
};

use crate::layout::LayoutPolicy;

pub fn active_framebuffer<S: Sys>(sys: &mut S) -> Option<PrimaryDisplayBuffer> {
    userland_std::open_primary_display_buffer(sys).ok()
}

pub fn handle_mode_switches<S: Sys>(sys: &mut S) {
    let events: Vec<ModeSwitchEvent> = list_things_by_kind(sys);
    if let Some(latest) = events.into_iter().max_by_key(|e| e.timestamp) {
        set_active_mode(sys, latest.mode_index);
    }
}

pub fn current_mode<S: Sys>(sys: &mut S) -> Option<Mode> {
    active_mode(sys).or_else(|| default_mode(sys))
}

pub fn collect_windows_for_place<S: Sys>(sys: &mut S, place_id: ThingId) -> Vec<Window> {
    list_things_by_kind(sys)
        .into_iter()
        .filter(|w: &Window| w.place_id == place_id)
        .collect()
}

pub fn collect_surfaces_for_windows<S: Sys>(
    sys: &mut S,
    windows: &[Window],
) -> BTreeMap<ThingId, Surface> {
    let wanted: BTreeSet<ThingId> = windows.iter().map(|w| w.id).collect();
    let mut map = BTreeMap::new();
    for surface in list_things_by_kind::<S, Surface>(sys) {
        if wanted.contains(&surface.window_id) {
            map.insert(surface.window_id, surface);
        }
    }
    map
}

pub fn mouse_packets<S: Sys>(sys: &mut S) -> Vec<MousePacketEvent> {
    let mut events: Vec<MousePacketEvent> = list_things_by_kind(sys);
    events.sort_by_key(|e| e.sequence_index);
    events
}

pub fn console_mode_active<S: Sys>(sys: &mut S) -> bool {
    is_console_mode_active(sys)
}

pub fn layout_policy_for_mode(mode: &Mode) -> LayoutPolicy {
    mode.layout_policy
        .map(LayoutPolicy::from_i64)
        .unwrap_or_default()
}

fn set_active_mode<S: Sys>(sys: &mut S, index: u8) {
    let modes: Vec<Mode> = list_things_by_kind(sys);
    for mode in modes {
        let active = mode.index == index;
        let _ = update_props(
            sys,
            mode.id,
            &[(graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(active))],
        );
    }
}
