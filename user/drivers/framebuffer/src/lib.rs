#![no_std]

extern crate alloc;

// Panic handler is provided by thing_os::panic (in main.rs usually, but used here)

pub mod schemas;
pub use schemas::*;

use abi::{SharedBufferInfo, ThingId};
use alloc::string::ToString;
use alloc::vec::Vec;
use thing_os::graph_kinds;
use thing_os::prelude::*;
use thing_os::{
    add_link, create_thing, link_targets, list_things_by_kind, register_schema_for,
    shared_buffer_info, SysError,
    DisplayThing, Predicate,
};

const DEFAULT_REFRESH_INTERVAL_NS: u64 = 16_666_667;
const MIN_SLEEP_NS: u64 = 1_000_000_000;
const RETRY_INTERVAL_NS: u64 = 250_000_000;

pub struct FramebufferDriver {
    pub display_id: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: abi::syscall_defs::SymbolId,
    pub buffers: Vec<ThingId>,
}

impl FramebufferDriver {
    fn init() -> Result<Self, SysError> {
        println!("framebuffer_driver: registering schemas");
        let _ = register_schema_for::<FramebufferInfo>();
        let _ = register_schema_for::<Buffer>();
        let _ = register_schema_for::<PresentIntent>();

        println!("framebuffer_driver: locating primary display");
        let descriptor = primary_display_descriptor()?;
        println!("framebuffer_driver: found display descriptor");

        let width = descriptor.info.width;
        let height = descriptor.info.height;
        let stride = descriptor.info.stride;
        // descriptor.info.pixel_format is PixelFormat enum.
        // We need SymbolId. FramebufferInfo expects SymbolId.
        // We'll create a symbol for the format name.
        let format_str = format!("{:?}", descriptor.info.pixel_format);
        let pixel_format = thing_os::intern(&format_str);
        
        println!(
            "framebuffer_driver: display {} {}x{} stride={} fmt={:?}",
            descriptor.display_id.0, width, height, stride, format_str,
        );

        let fb_info = FramebufferInfo {
            id: ThingId(0),
            width: width,
            height: height,
            stride: stride,
            pixel_format: pixel_format,
            buffer_count: 1, // Currently single buffered in terms of "exposed" buffers perhaps? Or scanout.
            active_buffer: 0,
        };

        println!("framebuffer_driver: creating FramebufferInfo Thing");
        let fb_id = create_thing(&fb_info).ok_or(SysError::Unexpected)?;
        println!("framebuffer_driver: created FramebufferInfo Thing {}", fb_id.0);

        // Create Buffer Thing for the scanout buffer
        let buffer_thing = Buffer {
            id: ThingId(0),
            index: 0,
            size_bytes: (stride as u64) * (height as u64), // Approximate if unknown
            shared_buffer_id: descriptor.scanout_buffer_id,
        };
        
        println!("framebuffer_driver: creating Buffer Thing");
        let buffer_id = create_thing(&buffer_thing).ok_or(SysError::Unexpected)?;

        // Link Buffer to FramebufferInfo
        // We need a predicate. Let's use "pkg.framebuffer.has_buffer" or just generic link
        let has_buffer_pred = thing_os::intern("pkg.framebuffer.has_buffer");
        let _ = add_link(fb_id, Predicate(has_buffer_pred.0 as u64), buffer_id);

        // Link FramebufferInfo to Display so Compositor can find it.
        // Compositor looks for FramebufferInfo.
        // We can link it with "pkg.framebuffer.info"
        let info_pred = thing_os::intern("pkg.framebuffer.info");
        let _ = add_link(descriptor.display_id, Predicate(info_pred.0 as u64), fb_id);

        // Also publish to graph root or similar? 
        // The display is likely discoverable.

        Ok(Self {
            display_id: descriptor.display_id,
            width,
            height,
            stride,
            pixel_format,
            buffers: vec![buffer_id],
        })
    }
}

pub fn driver_main() -> ! {
    println!("framebuffer_driver: starting");

    let _driver = loop {
        match FramebufferDriver::init() {
            Ok(driver) => break driver,
            Err(err) => {
                println!("framebuffer_driver: init failed ({:?}), retrying", err);
                thing_os::time::sleep(thing_os::time::Duration::from_nanos(RETRY_INTERVAL_NS));
            }
        }
    };

    println!("framebuffer_driver: initialized successfully. Listening for events...");

    loop {
        // In the future, we might listen for PresentIntent completion or hotplug events.
        // For now, just sleep.
        thing_os::time::sleep(thing_os::time::Duration::from_nanos(MIN_SLEEP_NS));
    }
}

struct DisplayDescriptor {
    display_id: ThingId,
    scanout_buffer_id: ThingId,
    info: SharedBufferInfo,
}

fn primary_display_descriptor() -> Result<DisplayDescriptor, SysError> {
    let displays: Vec<DisplayThing> = list_things_by_kind();
    let display = displays
        .iter()
        .find(|d| d.name == "display0")
        .or_else(|| displays.first())
        .cloned()
        .ok_or(SysError::Unexpected)?;

    let mut targets = link_targets(display.id, graph_kinds::LINK_DISPLAY_SCANOUT);
    let buffer_id = targets.pop().ok_or(SysError::Unexpected)?;
    let info = thing_os::shared_buffer_info(buffer_id)?;

    Ok(DisplayDescriptor {
        display_id: display.id,
        scanout_buffer_id: buffer_id,
        info,
    })
}
