use alloc::string::ToString;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::format;
use abi::{KernelRequest, KernelResponse, ThingId};
use thing_models::{PropKey, PropValue, graph_kinds};
use thing_os::prelude::*;
use thing_os::{
    DisplayThing, PrimaryDisplayBuffer, Surface, Window,
    load_thing, update_props, list_things_by_kind,
};



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
            graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX.to_string(),
            PropValue::I64(new_index),
        )],
    ) {
        return None;
    }

    Some(new_index)
}



pub fn collect_all_windows() -> Vec<Window> {
    list_things_by_kind()
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


