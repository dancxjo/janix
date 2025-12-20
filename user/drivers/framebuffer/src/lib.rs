#![no_std]

extern crate alloc;

use abi::{Predicate, MapFlags, PixelFormat, SharedBufferInfo, ThingId};
use thing_models::graph_kinds;
use alloc::string::{String, ToString};
use core::ptr;
use thing_os::prelude::*;
use thing_os::thing_models::DisplayPresentRequest;
use thing_os::{
    DisplayThing, SysError, add_link, link_targets, load_thing, shared_buffer_info,
    shared_buffer_map, create_thing, register_schema_for, update_props, list_things_by_kind,
    PropKey, PropValue, PropType,
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
        let front_buffer_id = Self::display_buffer_target(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER,
        )?;
        let back_buffer_id = Self::display_buffer_target(
            descriptor.display_id,
            graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER,
        )?;
        let front_buffer = Self::map_buffer_view(front_buffer_id, logical_map_flags)?;
        let back_buffer = Self::map_buffer_view(back_buffer_id, logical_map_flags)?;
        
        let scanout_map_flags = logical_map_flags.union(MapFlags::WRITE);
        let scanout_buffer =
            Self::map_buffer_view(descriptor.scanout_buffer_id, scanout_map_flags)?;
        
        // let active_buffer_index = Self::load_active_buffer_index(descriptor.display_id);
        // We load it fresh in tick

        println!("framebuffer_driver: describing framebuffer Thing");
        let fb_thing = DisplayFramebufferThing {
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
        let _ = register_schema_for::<DisplayFramebufferThing>();
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

    fn ensure_present_request(
        fb_id: ThingId,
    ) -> Result<DisplayPresentRequest, SysError> {
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
        if let Some(fb) = load_thing::<DisplayFramebufferThing>(self.fb_id) {
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
            self.present_request_id = None;
        }

        if self.present_request_id.is_none() {
            if let Some(request) = Self::find_present_request(self.fb_id) {
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
                (graph_kinds::PROP_COMPLETED.to_string(), PropValue::Bool(true)),
            ],
        );
    }

    fn display_buffer_target(
        display_id: ThingId,
        pred: Predicate,
    ) -> Result<ThingId, SysError> {
        let mut targets = link_targets(display_id, pred);
        targets.pop().ok_or(SysError::Unexpected)
    }

    fn clamp_active_buffer_index(value: i64) -> i64 {
        if value == 1 { 1 } else { 0 }
    }

    /*
    fn load_active_buffer_index(display_id: ThingId) -> i64 {
        let display = load_thing::<DisplayThing>(display_id);
        Self::clamp_active_buffer_index(display.map(|d| d.active_buffer_index).unwrap_or(0))
    }
    */

    fn map_buffer_view(
        buffer_id: ThingId,
        flags: MapFlags,
    ) -> Result<SharedBufferView, SysError> {
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

#[derive(Clone, Debug)]
struct DisplayFramebufferThing {
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: PixelFormat,
    pub power_state: DisplayPowerState,
    pub refresh_interval_ns: Option<u64>,
    pub frames_presented: u64,
    pub last_present_ns: u64,
}

impl Thing for DisplayFramebufferThing {
    const KIND: &'static str = graph_kinds::KIND_DISPLAY_FRAMEBUFFER;
    const DESCRIPTION: &'static str = "Userland-published framebuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((
            graph_kinds::PROP_NAME.to_string(),
            PropValue::Str(self.name.clone()),
        ));
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        let fmt = match self.pixel_format {
            PixelFormat::Rgba8888 => "Rgba8888",
            PixelFormat::Bgra8888 => "Bgra8888",
        };
        out.push((
            graph_kinds::PROP_PIXEL_FORMAT.to_string(),
            PropValue::Str(fmt.into()),
        ));
        out.push((
            graph_kinds::PROP_POWER_STATE.to_string(),
            PropValue::Str(self.power_state.as_str().into()),
        ));
        if let Some(refresh) = self.refresh_interval_ns {
            out.push((
                graph_kinds::PROP_REFRESH_INTERVAL_NS.to_string(),
                PropValue::U64(refresh),
            ));
        }
        out.push((
            graph_kinds::PROP_FRAMES_PRESENTED.to_string(),
            PropValue::U64(self.frames_presented),
        ));
        out.push((
            graph_kinds::PROP_LAST_PRESENT_NS.to_string(),
            PropValue::U64(self.last_present_ns),
        ));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = PixelFormat::Rgba8888;
        let mut power_state = DisplayPowerState::On;
        let mut refresh_interval_ns = None;
        let mut frames_presented = 0;
        let mut last_present_ns = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_NAME => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                graph_kinds::PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                graph_kinds::PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                graph_kinds::PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                graph_kinds::PROP_PIXEL_FORMAT => {
                    if let PropValue::Str(v) = &prop.1 {
                        pixel_format = if v == "Bgra8888" {
                            PixelFormat::Bgra8888
                        } else {
                            PixelFormat::Rgba8888
                        };
                    }
                }
                graph_kinds::PROP_POWER_STATE => {
                    if let PropValue::Str(v) = &prop.1 {
                        power_state = DisplayPowerState::from_str(v);
                    }
                }
                graph_kinds::PROP_REFRESH_INTERVAL_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        refresh_interval_ns = Some(v);
                    }
                }
                graph_kinds::PROP_FRAMES_PRESENTED => {
                    if let PropValue::U64(v) = prop.1 {
                        frames_presented = v;
                    }
                }
                graph_kinds::PROP_LAST_PRESENT_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        last_present_ns = v;
                    }
                }
                _ => {}
            }
        }

        DisplayFramebufferThing {
            name,
            width,
            height,
            stride,
            pixel_format,
            power_state,
            refresh_interval_ns,
            frames_presented,
            last_present_ns,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
            (graph_kinds::PROP_POWER_STATE, PropType::Str),
            (graph_kinds::PROP_REFRESH_INTERVAL_NS, PropType::U64),
            (graph_kinds::PROP_FRAMES_PRESENTED, PropType::U64),
            (graph_kinds::PROP_LAST_PRESENT_NS, PropType::U64),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DisplayPowerState {
    On,
    Sleep,
    Off,
}

impl DisplayPowerState {
    const fn as_str(self) -> &'static str {
        match self {
            DisplayPowerState::On => "On",
            DisplayPowerState::Sleep => "Sleep",
            DisplayPowerState::Off => "Off",
        }
    }

    fn from_str(value: &str) -> Self {
        match value {
            "Off" => DisplayPowerState::Off,
            "Sleep" => DisplayPowerState::Sleep,
            _ => DisplayPowerState::On,
        }
    }
}
