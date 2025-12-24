#![no_std]

extern crate alloc;

use abi::{MapFlags, PixelFormat, SharedBufferInfo, ThingId};
use alloc::string::ToString;
use thing_models::{DisplayFramebuffer, DisplayPowerState};
use thing_os::graph_kinds;
use thing_os::prelude::*;
use thing_os::{
    add_link, create_thing, link_targets, list_things_by_kind, register_schema_for,
    shared_buffer_info, shared_buffer_map, SysError,
};

const DEFAULT_REFRESH_INTERVAL_NS: u64 = 16_666_667;
const MIN_SLEEP_NS: u64 = 1_000_000_000; // Sleep for a second between checks
const RETRY_INTERVAL_NS: u64 = 250_000_000;

pub struct FramebufferDriver {
    display_id: ThingId,
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
}

impl FramebufferDriver {
    fn init() -> Result<Self, SysError> {
        println!("framebuffer_driver: locating primary display");
        let descriptor = primary_display_descriptor()?;
        println!("framebuffer_driver: found display descriptor");

        let width = descriptor.info.width;
        let height = descriptor.info.height;
        let stride = descriptor.info.stride;
        let pixel_format = descriptor.info.pixel_format;

        println!(
            "framebuffer_driver: display {} {}x{} stride={} fmt={:?}",
            descriptor.display_id.0, width, height, stride, pixel_format,
        );

        // We no longer create front/back buffers.
        // We simply expose the scanout buffer as the "framebuffer" for the compositor.

        println!("framebuffer_driver: describing framebuffer Thing");
        let fb_thing = DisplayFramebuffer {
            id: ThingId(0),
            name: "fb0".into(),
            width: width as u64,
            height: height as u64,
            stride: stride as u64,
            pixel_format,
            power_state: DisplayPowerState::On,
            refresh_interval_ns: Some(DEFAULT_REFRESH_INTERVAL_NS),
            frames_presented: 0,
            last_present_ns: 0,
        };

        println!("framebuffer_driver: registering schemas");
        let _ = register_schema_for::<DisplayFramebuffer>();

        println!("framebuffer_driver: creating framebuffer Thing");
        let fb_id = create_thing(&fb_thing).ok_or(SysError::Unexpected)?;
        println!("framebuffer_driver: created framebuffer Thing");

        // Link the scanout buffer as the "front buffer" for legacy compatibility with compositor's discovery logic
        // The compositor looks for LINK_DISPLAY_FRONT_BUFFER on the display.
        // Or it looks for active_framebuffer() via LINK_DISPLAY_FRONT_BUFFER on the display.
        // In the new model, we just link the scanout buffer as the front buffer.

        // Wait, the framebuffer Thing (fb_id) is metadata.
        // The compositor uses `active_framebuffer()` which calls `open_primary_display_buffer()`.
        // `open_primary_display_buffer` looks for `LINK_DISPLAY_HAS_FRONT_BUFFER` on the display.

        // Let's link the scanout buffer as LINK_DISPLAY_HAS_FRONT_BUFFER
        let _ = add_link(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER,
            descriptor.scanout_buffer_id,
        );

        // Also link the metadata Thing to the display
        let _ = add_link(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_FRONT_BUFFER, // This is what `compositor` might look for to find the metadata thing
            fb_id,
        );
        println!("framebuffer_driver: linked framebuffer to display");

        Ok(Self {
            display_id: descriptor.display_id,
            width,
            height,
            stride,
            pixel_format,
        })
    }
}

pub fn driver_main() -> ! {
    println!("framebuffer_driver: starting (single-buffer mode)");

    let driver = loop {
        match FramebufferDriver::init() {
            Ok(driver) => break driver,
            Err(err) => {
                println!("framebuffer_driver: init failed ({:?}), retrying", err);
                sleep(Duration::from_nanos(RETRY_INTERVAL_NS));
            }
        }
    };

    println!(
        "framebuffer_driver: configured display {} {}x{} stride={} fmt={:?}",
        driver.display_id.0, driver.width, driver.height, driver.stride, driver.pixel_format,
    );

    loop {
        // Driver is now just a placeholder/configurator.
        // In the future it might handle mode sets or hotplug.
        sleep(Duration::from_nanos(MIN_SLEEP_NS));
    }
}

struct DisplayDescriptor {
    display_id: ThingId,
    scanout_buffer_id: ThingId,
    info: SharedBufferInfo,
}

fn primary_display_descriptor() -> Result<DisplayDescriptor, SysError> {
    use thing_os::DisplayThing;

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
