use crate::graph::{self, GraphEvent};
use crate::graph_kinds::{
    self, LINK_DISPLAY_HAS_BACK_BUFFER, LINK_DISPLAY_HAS_FRONT_BUFFER, LINK_DISPLAY_SCANOUT,
    PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
};
use crate::shared_buffer;
use alloc::vec::Vec;
use thing_models::PropValue;

pub fn init() {
    crate::graph::events::subscribe(on_event);
}

fn on_event(event: &GraphEvent) {
    if let GraphEvent::ThingUpdated(id) = event {
        // Check if it's a display
        if let Some(kind) = crate::graph::get_thing_kind(*id) {
            if kind == crate::symbols::intern(graph_kinds::KIND_DISPLAY) {
                // Potential update to active buffer index.
                // We should check the property.
                if let Some(PropValue::I64(idx)) =
                    crate::graph::get_prop(*id, PROP_DISPLAY_ACTIVE_BUFFER_INDEX)
                {
                    process_display_update(*id, idx);
                }
            }
        }
    }
}

fn process_display_update(display_id: abi::ThingId, active_index: i64) {
    // 1. Identify source buffer
    let src_pred = if active_index == 1 {
        LINK_DISPLAY_HAS_BACK_BUFFER
    } else {
        LINK_DISPLAY_HAS_FRONT_BUFFER
    };

    let mut out = [None; 1];
    graph::neighbors(display_id, src_pred, &mut out);
    let src_buffer_id = match out[0] {
        Some(id) => id,
        None => return,
    };

    // 2. Identify destination buffer (the scanout VRAM wrapper)
    let mut out = [None; 1];
    graph::neighbors(display_id, LINK_DISPLAY_SCANOUT, &mut out);
    let dst_buffer_id = match out[0] {
        Some(id) => id,
        None => return,
    };

    // 3. Perform blit
    blit_buffers(src_buffer_id, dst_buffer_id);
}

fn blit_buffers(src_id: abi::ThingId, dst_id: abi::ThingId) {
    let hhdm_offset = match shared_buffer::hhdm_offset() {
        Some(offset) => offset,
        None => return,
    };

    let manager = shared_buffer::manager().lock();

    let src = match manager.get(&src_id) {
        Some(buf) => buf,
        None => return,
    };
    let dst = match manager.get(&dst_id) {
        Some(buf) => buf,
        None => return,
    };

    // Validate dimensions match
    if src.width != dst.width || src.height != dst.height {
        return;
    }

    let width = src.width as usize;
    let height = src.height as usize;
    let bpp = 4; // Assuming 32-bit for now
    let line_bytes = width * bpp;

    let src_stride = src.stride as usize;
    let dst_stride = dst.stride as usize;

    let src_frames = &src.frames;
    let dst_frames = &dst.frames;

    // We need to copy line by line.
    // Iterating pixels is too slow, we should copy chunks.
    // However, frames can be non-contiguous physically.
    // But virtually they are contiguous in the buffer concept?
    // Userland maps them contiguously. Kernel sees a list of PhysFrame.
    // We have to walk the frames.

    // Helper to get slice from list of frames
    // This is getting complicated to do efficiently without mapping.
    // But efficient is secondary to working.

    // Let's assume simplest case: linear copy frame by frame?
    // Using HHDM we can access physical memory directly.

    let mut remaining_bytes = height * src_stride; // Approximation if strides differ
    // Actually we only care about valid pixels. Stride might include padding.

    // Let's iterate lines.
    for y in 0..height {
        let src_offset = y * src_stride;
        let dst_offset = y * dst_stride;

        // Copy 'line_bytes' from (src + src_offset) to (dst + dst_offset)

        // We need to translate offset -> (frame_index, internal_offset)
        // PAGE_SIZE is 4096.

        let mut current_copy_len = 0;
        while current_copy_len < line_bytes {
            let chunk_size = line_bytes - current_copy_len; // How much left for this line

            let curr_src_offset = src_offset + current_copy_len;
            let curr_dst_offset = dst_offset + current_copy_len;

            let (src_frame_idx, src_frame_off) = (curr_src_offset / 4096, curr_src_offset % 4096);
            let (dst_frame_idx, dst_frame_off) = (curr_dst_offset / 4096, curr_dst_offset % 4096);

            // Min bytes we can copy without crossing a page boundary
            let src_rem = 4096 - src_frame_off;
            let dst_rem = 4096 - dst_frame_off;
            let copy_size = chunk_size.min(src_rem).min(dst_rem);

            if let (Some(src_frame), Some(dst_frame)) =
                (src_frames.get(src_frame_idx), dst_frames.get(dst_frame_idx))
            {
                let src_phys = src_frame.start_address + src_frame_off as u64;
                let dst_phys = dst_frame.start_address + dst_frame_off as u64;

                let src_virt = src_phys + hhdm_offset;
                let dst_virt = dst_phys + hhdm_offset;

                unsafe {
                    core::ptr::copy_nonoverlapping(
                        src_virt as *const u8,
                        dst_virt as *mut u8,
                        copy_size,
                    );
                }
            }

            current_copy_len += copy_size;
        }
    }
}
