#![no_std]

extern crate alloc;

use abi::{MapFlags, PixelFormat, Predicate, SharedBufferInfo, ThingId};
use alloc::string::{String, ToString};
use core::ptr;
use thing_macros::Thing;
use thing_models::graph_kinds;
use thing_os::prelude::*;
use thing_os::thing_models::DisplayPresentRequest;
use thing_os::{
    add_link, create_thing, link_targets, list_things_by_kind, load_thing, register_schema_for,
    shared_buffer_info, shared_buffer_map, update_props, DisplayThing, PropKey, PropType,
    PropValue, SysError,
};

const DEFAULT_REFRESH_INTERVAL_NS: u64 = 16_666_667;
const MIN_SLEEP_NS: u64 = 1_000_000;
const RETRY_INTERVAL_NS: u64 = 250_000_000;

pub struct FramebufferDriver {
    display_id: ThingId,
    fb_id: ThingId,
    present_request_id: Option<ThingId>,
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
    refresh_interval_ns: u64,
    frame_watch: Option<u64>,
    frames_presented: u64,
    last_present_ns: u64,
    front_buffer: SharedBufferView,
    back_buffer: SharedBufferView,
    scanout_buffer: SharedBufferView,
    active_buffer_index: i64,
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

        let logical_map_flags = MapFlags::READ.union(MapFlags::USER);
        let logical_map_flags = MapFlags::READ.union(MapFlags::USER);

        let front_buffer_id = match Self::display_buffer_target(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER,
        ) {
            Ok(id) => id,
            Err(_) => {
                println!("framebuffer_driver: creating front buffer");
                let id = thing_os::create_shared_buffer(width, height, pixel_format)?;
                let _ = add_link(
                    descriptor.display_id,
                    graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER,
                    id,
                );
                id
            }
        };

        let back_buffer_id = match Self::display_buffer_target(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER,
        ) {
            Ok(id) => id,
            Err(_) => {
                println!("framebuffer_driver: creating back buffer");
                let id = thing_os::create_shared_buffer(width, height, pixel_format)?;
                let _ = add_link(
                    descriptor.display_id,
                    graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER,
                    id,
                );
                id
            }
        };

        let front_buffer = Self::map_buffer_view(front_buffer_id, logical_map_flags)?;
        let back_buffer = Self::map_buffer_view(back_buffer_id, logical_map_flags)?;

        let scanout_map_flags = logical_map_flags.union(MapFlags::WRITE);
        let scanout_buffer =
            Self::map_buffer_view(descriptor.scanout_buffer_id, scanout_map_flags)?;

        // let active_buffer_index = Self::load_active_buffer_index(descriptor.display_id);
        // We load it fresh in tick

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
        let _ = register_schema_for::<DisplayPresentRequest>();
        println!("framebuffer_driver: creating framebuffer Thing");
        let fb_id = create_thing(&fb_thing).ok_or(SysError::Unexpected)?;
        println!("framebuffer_driver: created framebuffer Thing");

        let _ = add_link(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_FRONT_BUFFER,
            fb_id,
        );
        println!("framebuffer_driver: linked framebuffer to display");

        println!("framebuffer_driver: ensuring present request");
        let request = Self::ensure_present_request(fb_id)?;
        println!("framebuffer_driver: ensured present request");

        Ok(Self {
            display_id: descriptor.display_id,
            fb_id,
            present_request_id: Some(request.id),
            width,
            height,
            stride,
            pixel_format,
            refresh_interval_ns: fb_thing
                .refresh_interval_ns
                .unwrap_or(DEFAULT_REFRESH_INTERVAL_NS),
            frame_watch: Some(request.frame_index),
            frames_presented: 0,
            last_present_ns: 0,
            front_buffer,
            back_buffer,
            scanout_buffer,
            active_buffer_index: 0,
        })
    }

    fn ensure_present_request(fb_id: ThingId) -> Result<DisplayPresentRequest, SysError> {
        if let Some(existing) = Self::find_present_request(fb_id) {
            return Ok(existing);
        }

        let request = DisplayPresentRequest {
            id: ThingId(0),
            framebuffer_id: fb_id,
            frame_index: 0,
            requested_at_ns: 0,
            presented_at_ns: None,
            completed: true,
        };
        let id = create_thing(&request).ok_or(SysError::Unexpected)?;
        Ok(DisplayPresentRequest { id, ..request })
    }

    fn find_present_request(fb_id: ThingId) -> Option<DisplayPresentRequest> {
        list_things_by_kind::<DisplayPresentRequest>()
            .into_iter()
            .find(|req| req.framebuffer_id == fb_id)
    }

    fn tick(&mut self) -> u64 {
        let power_state = self.sync_framebuffer_state();
        if power_state != DisplayPowerState::On {
            return self.refresh_interval_ns.max(RETRY_INTERVAL_NS);
        }

        self.process_requests();
        self.refresh_interval_ns.max(MIN_SLEEP_NS)
    }

    fn sync_framebuffer_state(&mut self) -> DisplayPowerState {
        if let Some(fb) = load_thing::<DisplayFramebuffer>(self.fb_id) {
            if let Some(refresh) = fb.refresh_interval_ns {
                if refresh != 0 {
                    self.refresh_interval_ns = refresh;
                }
            }
            return fb.power_state;
        }
        DisplayPowerState::On
    }

    fn process_requests(&mut self) {
        if let Some(req_id) = self.present_request_id {
            if let Some(request) = load_thing::<DisplayPresentRequest>(req_id) {
                self.try_present(&request);
                return;
            }
            // If request is gone, drop the ID
            println!("framebuffer_driver: DisplayPresentRequest {} gone", req_id.0);
            self.present_request_id = None;
        }

        if self.present_request_id.is_none() {
            if let Some(request) = Self::find_present_request(self.fb_id) {
                println!("framebuffer_driver: found DisplayPresentRequest {}", request.id.0);
                self.frame_watch = Some(request.frame_index);
                self.present_request_id = Some(request.id);
            }
        }
    }

    fn try_present(&mut self, request: &DisplayPresentRequest) {
        if request.framebuffer_id != self.fb_id {
            return;
        }
        if request.completed {
            return;
        }
        if self.frame_watch == Some(request.frame_index) {
            return;
        }

        println!("framebuffer_driver: presenting frame {}", request.frame_index);
        self.blit_front_buffer();

        self.frame_watch = Some(request.frame_index);
        self.frames_presented = self.frames_presented.saturating_add(1);
        self.last_present_ns = Instant::now().t_ns;

        let _ = update_props(
            self.fb_id,
            &[
                (
                    graph_kinds::PROP_LAST_PRESENT_NS.to_string(),
                    PropValue::U64(self.last_present_ns),
                ),
                (
                    graph_kinds::PROP_FRAMES_PRESENTED.to_string(),
                    PropValue::U64(self.frames_presented),
                ),
            ],
        );

        let _ = update_props(
            request.id,
            &[
                (
                    graph_kinds::PROP_PRESENTED_AT_NS.to_string(),
                    PropValue::U64(self.last_present_ns),
                ),
                (
                    graph_kinds::PROP_COMPLETED.to_string(),
                    PropValue::Bool(true),
                ),
            ],
        );
    }

    fn display_buffer_target(display_id: ThingId, pred: Predicate) -> Result<ThingId, SysError> {
        let mut targets = link_targets(display_id, pred);
        targets.pop().ok_or(SysError::Unexpected)
    }

    fn clamp_active_buffer_index(value: i64) -> i64 {
        if value == 1 {
            1
        } else {
            0
        }
    }

    /*
    fn load_active_buffer_index(display_id: ThingId) -> i64 {
        let display = load_thing::<DisplayThing>(display_id);
        Self::clamp_active_buffer_index(display.map(|d| d.active_buffer_index).unwrap_or(0))
    }
    */

    fn map_buffer_view(buffer_id: ThingId, flags: MapFlags) -> Result<SharedBufferView, SysError> {
        let info = shared_buffer_info(buffer_id)?;
        let (ptr, size) = shared_buffer_map(buffer_id, flags)?;
        Ok(SharedBufferView {
            _id: buffer_id,
            info,
            ptr,
            size,
        })
    }

    fn blit_front_buffer(&mut self) {
        if let Some(display) = load_thing::<DisplayThing>(self.display_id) {
            let active_index = Self::clamp_active_buffer_index(display.active_buffer_index);
            self.active_buffer_index = active_index;
            let source = if active_index == 0 {
                &self.front_buffer
            } else {
                &self.back_buffer
            };
            let bytes = (source.info.stride as usize).saturating_mul(source.info.height as usize);
            let bytes =
                core::cmp::min(bytes, core::cmp::min(source.size, self.scanout_buffer.size));
            if bytes == 0 {
                return;
            }
            unsafe {
                ptr::copy_nonoverlapping(source.ptr, self.scanout_buffer.ptr, bytes);
            }
        }
    }
}

pub fn driver_main() -> ! {
    println!("framebuffer_driver: starting");

    let mut driver = loop {
        match FramebufferDriver::init() {
            Ok(driver) => break driver,
            Err(err) => {
                println!("framebuffer_driver: init failed ({:?}), retrying", err);
                sleep(Duration::from_nanos(RETRY_INTERVAL_NS));
            }
        }
    };

    println!(
        "framebuffer_driver: registered framebuffer {}x{} stride {} display {} fmt {:?}",
        driver.width, driver.height, driver.stride, driver.display_id.0, driver.pixel_format,
    );

    loop {
        let sleep_ns = driver.tick();
        sleep(Duration::from_nanos(sleep_ns));
    }
}

struct DisplayDescriptor {
    display_id: ThingId,
    scanout_buffer_id: ThingId,
    info: SharedBufferInfo,
}

struct SharedBufferView {
    _id: ThingId,
    info: SharedBufferInfo,
    ptr: *mut u8,
    size: usize,
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

#[derive(Thing, Clone, Debug)]
#[thing(description = "Userland-published framebuffer")]
struct DisplayFramebuffer {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    #[thing(via = "String")]
    pub pixel_format: PixelFormat,
    #[thing(via = "String")]
    pub power_state: DisplayPowerState,
    pub refresh_interval_ns: Option<u64>,
    pub frames_presented: u64,
    pub last_present_ns: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DisplayPowerState {
    On,
    Sleep,
    Off,
}

impl core::fmt::Display for DisplayPowerState {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::str::FromStr for DisplayPowerState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Off" => Ok(DisplayPowerState::Off),
            "Sleep" => Ok(DisplayPowerState::Sleep),
            _ => Ok(DisplayPowerState::On),
        }
    }
}

impl DisplayPowerState {
    const fn as_str(self) -> &'static str {
        match self {
            DisplayPowerState::On => "On",
            DisplayPowerState::Sleep => "Sleep",
            DisplayPowerState::Off => "Off",
        }
    }
}
