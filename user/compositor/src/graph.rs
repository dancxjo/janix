use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;
use abi::{KernelRequest, KernelResponse};
use thing_os::prelude::*;
use thing_os::{
    DisplayThing, Mode, ModeSwitchEvent, PrimaryDisplayBuffer, Surface, Window, active_mode,
    default_mode, graph_kinds, is_console_mode_active, load_thing, update_props,
};

use crate::layout::LayoutPolicy;

pub fn active_framebuffer<S: Sys>(sys: &mut S) -> Option<PrimaryDisplayBuffer> {
    thing_os::open_primary_display_buffer(sys).ok()
}

pub fn swap_display_buffers<S: Sys>(sys: &mut S, display_id: ThingId) -> Option<i64> {
    let display = load_thing::<DisplayThing>(sys, display_id)?;
    let current_index = display.active_buffer_index;
    let new_index = if current_index == 0 { 1 } else { 0 };

    if !update_props(
        sys,
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

pub fn handle_mode_switches<S: Sys>(sys: &mut S) {
    let systems: Vec<SystemThing> = list_things_by_kind(sys);
    let system = match systems.first() {
        Some(s) => s,
        None => return,
    };

    let targets = thing_os::link_targets(sys, system.id, graph_kinds::LINK_HAS_ACTIVE_MODE);
    if let Some(target_mode_id) = targets.first() {
        let current = current_mode(sys);
        if current.map(|m| m.id) != Some(*target_mode_id) {
             let msg = format!("compositor: observed active mode edge pointing to {}", target_mode_id.0);
             let leaked = Box::leak(msg.into_boxed_str());
             println(sys, leaked);
             
             // Check index
             if let Some(mode) = load_thing::<Mode>(sys, *target_mode_id) {
                 println(sys, "compositor: switching internal mode state");
                 set_active_mode(sys, mode.index);
                 
                 let msg = format!("MODE observed -> {}", mode.name);
                 let leaked = Box::leak(msg.into_boxed_str());
                 println(sys, leaked);
             }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutPolicy;
    use crate::test_support::{MockSys, list_responses, success};
    use abi::{KernelRequest, PropValue, ThingId, graph_kinds};
    use thing_os::thing_models::ModeSwitchEvent;

    fn mode(id: u64, index: u8, active: bool) -> Mode {
        Mode {
            id: ThingId(id),
            index,
            name: format!("mode-{index}"),
            place_id: Some(ThingId(index as u64)),
            active,
            layout_policy: None,
        }
    }


    #[test]
    fn layout_policy_from_mode_defaults_to_free() {
        let mut m = mode(1, 1, false);
        assert_eq!(layout_policy_for_mode(&m), LayoutPolicy::Free);
        m.layout_policy = Some(1);
        assert_eq!(layout_policy_for_mode(&m), LayoutPolicy::Tiled);
    }

    #[test]
    fn collect_windows_for_place_filters_non_matching() {
        let windows = vec![
            Window {
                id: ThingId(1),
                place_id: ThingId(7),
                x: 10,
                y: 10,
                width: 100,
                height: 100,
                z_index: 0,
                active: false,
                title: "a".into(),
                draggable: true,
                resizable: true,
                closable: true,
                minimizable: true,
            },
            Window {
                id: ThingId(2),
                place_id: ThingId(9),
                x: 0,
                y: 0,
                width: 10,
                height: 10,
                z_index: 1,
                active: false,
                title: "b".into(),
                draggable: true,
                resizable: true,
                closable: true,
                minimizable: true,
            },
        ];
        let responses = list_responses(windows, |w| w.id);
        let mut sys = MockSys::with_responses(responses);
        let collected = collect_windows_for_place(&mut sys, ThingId(7));
        assert_eq!(collected.len(), 1);
         assert_eq!(collected[0].id, ThingId(1));
    }

    #[test]
    fn collect_surfaces_for_windows_maps_to_ids() {
        let windows = vec![Window {
            id: ThingId(1),
            place_id: ThingId(1),
            x: 10,
            y: 10,
            width: 100,
            height: 100,
            z_index: 0,
            active: false,
            title: "a".into(),
            draggable: true,
            resizable: true,
            closable: true,
            minimizable: true,
        }];
        let surfaces = vec![
            Surface {
                id: ThingId(10),
                window_id: ThingId(1),
                kind: "text/plain".into(),
                text: "hello".into(),
                width: 100,
                height: 100,
                stride: 100,
                format: "Rgba8888".into(),
                shared_buffer_id: None,
                refresh_interval_ns: None,
                frames_presented: None,
                last_present_ns: None,
                power_state: None,
            },
            Surface {
                id: ThingId(11),
                window_id: ThingId(2),
                kind: "text/plain".into(),
                text: "ignore".into(),
                width: 100,
                height: 100,
                stride: 100,
                format: "Rgba8888".into(),
                shared_buffer_id: None,
                refresh_interval_ns: None,
                frames_presented: None,
                last_present_ns: None,
                power_state: None,
            },
        ];
        let responses = list_responses(surfaces, |s| s.id);
        let mut sys = MockSys::with_responses(responses);
        let map = collect_surfaces_for_windows(&mut sys, &windows);
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&ThingId(1)));
    }

    #[test]
    fn current_mode_falls_back_to_default() {
        let modes = vec![mode(1, 2, false), mode(2, 1, false)];
        let mut responses = list_responses(modes.clone(), |m| m.id);
        responses.extend(list_responses(modes, |m| m.id));
        let mut sys = MockSys::with_responses(responses);
        let mode = current_mode(&mut sys).expect("expected mode");
        assert_eq!(mode.index, 1, "default should pick lowest index");
    }
}
