use abi::ThingId;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use thing_os::prelude::*;
use thing_os::{
    DisplayThing, Window, list_things_by_kind, link_targets, load_thing, 
    update_props, intern, Predicate
};
use crate::schemas::{FramebufferInfo, Buffer, Surface};

pub fn find_framebuffer_setup() -> Option<(FramebufferInfo, Buffer, DisplayThing)> {
    let infos: Vec<FramebufferInfo> = list_things_by_kind();
    let fb_info = infos.first()?;

    // Find buffer
    let has_buffer = intern("pkg.framebuffer.has_buffer");
    let targets = link_targets(fb_info.id, Predicate(has_buffer.0 as u64));
    let buffer_id = targets.first()?;
    let buffer = load_thing::<Buffer>(*buffer_id)?;

    // Find display
    let displays: Vec<DisplayThing> = list_things_by_kind();
    let info_pred = intern("pkg.framebuffer.info");
    let display = displays.into_iter().find(|d| {
         let targets = link_targets(d.id, Predicate(info_pred.0 as u64));
         targets.contains(&fb_info.id)
    })?;

    Some((fb_info.clone(), buffer, display))
}

pub fn collect_all_windows() -> Vec<Window> {
    list_things_by_kind()
}

pub fn collect_surfaces_for_windows(windows: &[Window]) -> BTreeMap<ThingId, Surface> {
    let wanted: BTreeSet<ThingId> = windows.iter().map(|w| w.id).collect();
    let mut map = BTreeMap::new();
    for surface in list_things_by_kind::<Surface>() {
        // We match by window_id property. Surface struct has no window_id field in my schema!
        // Wait, schema.rs Surface:
        // width, height, format, buffer_index.
        // It DOES NOT have window_id.
        // The previous definition in thing_models::Surface (used by collect_surfaces_for_windows) had window_id.
        // I define pkg.compositor.Surface in schemas.rs.
        // If I use the new schemas for internal logic, I must ensure they match.
        // Or maybe I should use the old `thing_models::Surface` if userland creates it?
        // But Part 1.2 "Required schemas... Surface".
        // Schema definition only listed width, height, format, buffer_index.
        // How do we associate surface with window?
        // "RenderTree -> root_surface".
        // Windows might not be used in the "first light" version.
        // "4.2 Minimal Rendering... Do not attempt windows yet".
        
        // So for "First Light", I should disable window logic or stub it out.
        // But the existing compositor code relies heavily on windows.
        
        // I will return empty map for now or try to adapt.
        // If I want to fix windows later, I should add window_id to Surface schema.
        // But I should stick to the requested schema.
        // "Required schemas (minimal): pkg.compositor.Surface ... Fields: width, height, format, buffer_index".
        // It seems the new model assumes a RenderTree.
        
        // For now, I'll comment out window gathering or make it no-op.
        
        // But I need to preserve `collect_surfaces_for_windows` signature for `state.rs`.
        // I'll return empty map.
    }
    map
}
