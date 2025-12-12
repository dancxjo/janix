use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use abi::{KernelRequest, KernelResponse};
use userland::prelude::*;
use userland_std::thing_models::MousePacketEvent;
use userland_std::{
    DisplayThing, Mode, ModeSwitchEvent, PrimaryDisplayBuffer, Surface, Window, active_mode,
    default_mode, graph_kinds, is_console_mode_active, load_thing, update_props,
};

use crate::layout::LayoutPolicy;

pub fn active_framebuffer<S: Sys>(sys: &mut S) -> Option<PrimaryDisplayBuffer> {
    userland_std::open_primary_display_buffer(sys).ok()
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

pub fn mouse_packets_since<S: Sys>(sys: &mut S, last_id: Option<ThingId>) -> Vec<MousePacketEvent> {
    let mut events = Vec::new();
    let mut cursor = last_id.unwrap_or(ThingId(u64::MAX));

    loop {
        match sys.syscall(KernelRequest::ThingList {
            kind: MousePacketEvent::KIND,
            start_after: cursor,
        }) {
            KernelResponse::ThingListEntry { id: Some(next_id) } => {
                if let Some(event) = load_thing::<MousePacketEvent>(sys, next_id) {
                    events.push(event);
                }
                cursor = next_id;
            }
            _ => break,
        }
    }
    
    // Sort logic is good for determinism even if kernel returns in order
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::LayoutPolicy;
    use crate::test_support::{MockSys, list_responses, success};
    use abi::{KernelRequest, PropValue, ThingId, graph_kinds};
    use userland_std::thing_models::ModeSwitchEvent;

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

    fn mouse_event(seq: u64) -> MousePacketEvent {
        MousePacketEvent {
            id: ThingId(seq),
            controller_id: ThingId(1),
            port_index: 0,
            sequence_index: seq,
            timestamp_ticks: seq * 10,
            buttons: 0,
            delta_x: 0,
            delta_y: 0,
            overflow_x: false,
            overflow_y: false,
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
                x: 0,
                y: 0,
                width: 10,
                height: 10,
                z_index: 0,
                active: false,
                title: "a".into(),
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
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            z_index: 0,
            active: false,
            title: "a".into(),
        }];
        let surfaces = vec![
            Surface {
                id: ThingId(10),
                window_id: ThingId(1),
                kind: "text/plain".into(),
                text: "hello".into(),
            },
            Surface {
                id: ThingId(11),
                window_id: ThingId(2),
                kind: "text/plain".into(),
                text: "ignore".into(),
            },
        ];
        let responses = list_responses(surfaces, |s| s.id);
        let mut sys = MockSys::with_responses(responses);
        let map = collect_surfaces_for_windows(&mut sys, &windows);
        assert_eq!(map.len(), 1);
        assert!(map.contains_key(&ThingId(1)));
    }

    #[test]
    fn mouse_packets_sorted_by_sequence_index() {
        let events = vec![mouse_event(5), mouse_event(2)];
        let responses = list_responses(events, |e| e.id);
        let mut sys = MockSys::with_responses(responses);
        let packets = mouse_packets_since(&mut sys, None);
        assert_eq!(packets.len(), 2);
        assert_eq!(packets[0].sequence_index, 2);
        assert_eq!(packets[1].sequence_index, 5);
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

    #[test]
    fn handle_mode_switches_applies_latest_request() {
        let events = vec![
            ModeSwitchEvent {
                id: ThingId(1),
                mode_index: 1,
                timestamp: 5,
            },
            ModeSwitchEvent {
                id: ThingId(2),
                mode_index: 2,
                timestamp: 10,
            },
        ];
        let mut responses = list_responses(events, |e| e.id);
        let modes = vec![mode(10, 1, false), mode(20, 2, false)];
        responses.extend(list_responses(modes, |m| m.id));
        responses.push(success());
        responses.push(success());

        let mut sys = MockSys::with_responses(responses);
        handle_mode_switches(&mut sys);
        let requests = sys.drain_requests();

        let updates: Vec<_> = requests
            .iter()
            .filter_map(|req| {
                if let KernelRequest::ThingUpdate { id, props } = req {
                    let active = props
                        .iter()
                        .find(|p| p.0 == graph_kinds::PROP_MODE_ACTIVE)
                        .map(|p| p.1.clone());
                    Some((*id, active))
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(
            updates,
            vec![
                (ThingId(10), Some(PropValue::Bool(false))),
                (ThingId(20), Some(PropValue::Bool(true)))
            ]
        );
    }
}
