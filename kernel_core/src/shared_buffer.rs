use crate::{graph, graph_kinds, memory::{self, PhysFrame}};
use abi::{MapFlags, PixelFormat, PropValue, SharedBufferInfo, ThingId};
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec as AllocVec;
use heapless::Vec;
use spin::Mutex;

#[path = "../../arch/src/shared_buffer.rs"]
mod arch_shared_buffer;
use arch_shared_buffer as shared_buffer;
pub use arch_shared_buffer::{FrameAllocator, FrameDescriptor, map_frames};

pub const MAX_FRAMES_PER_BUFFER: usize = 4096;
const PAGE_SIZE: u64 = 4096;

#[derive(Clone, Debug)]
pub struct SharedBuffer {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: PixelFormat,
    pub frames: Vec<PhysFrame, { MAX_FRAMES_PER_BUFFER }>,
}

impl SharedBuffer {
    pub fn size_bytes(&self) -> u64 {
        self.stride as u64 * self.height as u64
    }

    pub fn info(&self) -> SharedBufferInfo {
        SharedBufferInfo {
            width: self.width,
            height: self.height,
            stride: self.stride,
            pixel_format: self.pixel_format,
        }
    }
}

impl FrameDescriptor for PhysFrame {
    fn start_address(&self) -> u64 {
        self.start_address
    }
}

pub struct SharedBufferManager {
    buffers: BTreeMap<ThingId, SharedBuffer>,
}

impl SharedBufferManager {
    pub const fn new() -> Self {
        Self {
            buffers: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, id: ThingId, sb: SharedBuffer) {
        self.buffers.insert(id, sb);
    }

    pub fn get(&self, id: &ThingId) -> Option<&SharedBuffer> {
        self.buffers.get(id)
    }

    pub fn get_cloned(&self, id: &ThingId) -> Option<SharedBuffer> {
        self.buffers.get(id).cloned()
    }
}

static SHARED_BUFFERS: Mutex<SharedBufferManager> = Mutex::new(SharedBufferManager::new());

pub fn manager() -> &'static Mutex<SharedBufferManager> {
    &SHARED_BUFFERS
}

pub fn hhdm_offset() -> Option<u64> {
    let val = crate::memory::get_hhdm_offset();
    (val != 0).then_some(val)
}

pub fn create_shared_buffer_thing(
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
) -> Option<ThingId> {
    let pf_str: &'static str = match pixel_format {
        PixelFormat::Rgba8888 => "Rgba8888",
        PixelFormat::Bgra8888 => "Bgra8888",
    };

    let mut props = AllocVec::new();
    props.push((graph_kinds::PROP_WIDTH, PropValue::U64(width as u64)));
    props.push((graph_kinds::PROP_HEIGHT, PropValue::U64(height as u64)));
    props.push((graph_kinds::PROP_STRIDE, PropValue::U64(stride as u64)));
    props.push((
        graph_kinds::PROP_PIXEL_FORMAT,
        PropValue::Str(pf_str.into()),
    ));
    let boxed = Box::leak(props.into_boxed_slice());
    graph::create_thing(graph_kinds::KIND_SHARED_BUFFER, boxed)
}

pub fn register_shared_buffer(
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
    frames: Vec<PhysFrame, { MAX_FRAMES_PER_BUFFER }>,
) -> Result<ThingId, &'static str> {
    let buffer = SharedBuffer {
        width,
        height,
        stride,
        pixel_format,
        frames,
    };

    let buffer_id = create_shared_buffer_thing(width, height, stride, pixel_format)
        .ok_or("Failed to create SharedBuffer Thing")?;

    SHARED_BUFFERS.lock().insert(buffer_id, buffer);
    Ok(buffer_id)
}

pub fn create_display_for_buffer(
    buffer_id: ThingId,
    name: &'static str,
    info: &SharedBufferInfo,
) -> Option<ThingId> {
    let pf_str: &'static str = match info.pixel_format {
        PixelFormat::Rgba8888 => "Rgba8888",
        PixelFormat::Bgra8888 => "Bgra8888",
    };

    let props = &[
        (graph_kinds::PROP_NAME, PropValue::Str(name.into())),
        (graph_kinds::PROP_WIDTH, PropValue::U64(info.width as u64)),
        (graph_kinds::PROP_HEIGHT, PropValue::U64(info.height as u64)),
        (graph_kinds::PROP_STRIDE, PropValue::U64(info.stride as u64)),
        (
            graph_kinds::PROP_PIXEL_FORMAT,
            PropValue::Str(pf_str.into()),
        ),
    ];
    let display_id = graph::create_thing(graph_kinds::KIND_DISPLAY, props)?;
    let _ = graph::add_edge(display_id, graph_kinds::EDGE_DISPLAY_SCANOUT, buffer_id);
    Some(display_id)
}

struct KernelFrameAllocator;

impl FrameAllocator for KernelFrameAllocator {
    fn allocate_frame(&mut self) -> Option<u64> {
        memory::allocate_frame().map(|frame| frame.start_address)
    }
}

pub fn map_frames_into_current_as(
    vaddr: u64,
    frames: &[PhysFrame],
    flags: MapFlags,
) -> Result<(), &'static str> {
    let hhdm = hhdm_offset().ok_or("HHDM offset unknown for mapping")?;
    let mut allocator = KernelFrameAllocator;
    shared_buffer::map_frames(vaddr, frames, flags, &mut allocator, hhdm)
}

pub const fn align_up(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    let rem = value % align;
    if rem == 0 {
        value
    } else {
        value + (align - rem)
    }
}

pub const fn page_count_for_size(size: u64) -> u64 {
    align_up(size, PAGE_SIZE) / PAGE_SIZE
}
