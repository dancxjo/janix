#![no_std]

extern crate alloc;

use abi::{Predicate, MapFlags, PixelFormat, SharedBufferInfo, ThingId};
use alloc::string::String;
use core::ptr;
use thing_os::prelude::*;
use thing_os::thing_models::DisplayPresentRequest;
use thing_os::{
    DisplayThing, SysError, add_link, link_targets, load_thing, shared_buffer_info,
    shared_buffer_map,
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
    fn init<S: Sys>(sys: &mut S) -> Result<Self, SysError> {
        println(sys, "framebuffer_driver: locating primary display");
        let descriptor = primary_display_descriptor(sys)?;
        println(sys, "framebuffer_driver: found display descriptor");

        let width = descriptor.info.width;
        let height = descriptor.info.height;
        let stride = descriptor.info.stride;
        let pixel_format = descriptor.info.pixel_format;

        log_dynamic(
            sys,
            format_args!(
                "framebuffer_driver: display {} {}x{} stride={} fmt={:?}",
                descriptor.display_id.0, width, height, stride, pixel_format,
            ),
        );

        let logical_map_flags = MapFlags::READ.union(MapFlags::USER);
        let front_buffer_id = Self::display_buffer_target(
            sys,
            descriptor.display_id,
            abi::graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER,
        )?;
        let back_buffer_id = Self::display_buffer_target(
            sys,
            descriptor.display_id,
            abi::graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER,
        )?;
        let front_buffer = Self::map_buffer_view(sys, front_buffer_id, logical_map_flags)?;
        let back_buffer = Self::map_buffer_view(sys, back_buffer_id, logical_map_flags)?;
        
        let scanout_map_flags = logical_map_flags.union(MapFlags::WRITE);
        let scanout_buffer =
            Self::map_buffer_view(sys, descriptor.scanout_buffer_id, scanout_map_flags)?;
        
        let active_buffer_index = Self::load_active_buffer_index(sys, descriptor.display_id);

        println(sys, "framebuffer_driver: describing framebuffer Thing");
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

        println(sys, "framebuffer_driver: registering schemas");
        let _ = register_schema_for::<DisplayFramebufferThing>(sys);
        let _ = register_schema_for::<DisplayPresentRequest>(sys);
        println(sys, "framebuffer_driver: creating framebuffer Thing");
        let fb_id = create_thing(sys, &fb_thing).ok_or(SysError::Unexpected)?;
        println(sys, "framebuffer_driver: created framebuffer Thing");

        let _ = add_link(
            sys,
            descriptor.display_id,
            abi::graph_kinds::LINK_DISPLAY_FRONT_BUFFER,
            fb_id,
        );
        println(sys, "framebuffer_driver: linked framebuffer to display");

        println(sys, "framebuffer_driver: ensuring present request");
        let request = Self::ensure_present_request(sys, fb_id)?;
        println(sys, "framebuffer_driver: ensured present request");

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
            active_buffer_index,
        })
    }

    fn ensure_present_request<S: Sys>(
        sys: &mut S,
        fb_id: ThingId,
    ) -> Result<DisplayPresentRequest, SysError> {
        if let Some(existing) = Self::find_present_request(sys, fb_id) {
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
        let id = create_thing(sys, &request).ok_or(SysError::Unexpected)?;
        Ok(DisplayPresentRequest { id, ..request })
    }

    fn find_present_request<S: Sys>(sys: &mut S, fb_id: ThingId) -> Option<DisplayPresentRequest> {
        list_things_by_kind::<S, DisplayPresentRequest>(sys)
            .into_iter()
            .find(|req| req.framebuffer_id == fb_id)
    }

    fn tick<S: Sys>(&mut self, sys: &mut S) -> u64 {
        let power_state = self.sync_framebuffer_state(sys);
        if power_state != DisplayPowerState::On {
            return self.refresh_interval_ns.max(RETRY_INTERVAL_NS);
        }

        self.process_requests(sys);
        self.refresh_interval_ns.max(MIN_SLEEP_NS)
    }

    fn sync_framebuffer_state<S: Sys>(&mut self, sys: &mut S) -> DisplayPowerState {
        if let Some(fb) = load_thing::<DisplayFramebufferThing>(sys, self.fb_id) {
            if let Some(refresh) = fb.refresh_interval_ns {
                if refresh != 0 {
                    self.refresh_interval_ns = refresh;
                }
            }
            return fb.power_state;
        }
        DisplayPowerState::On
    }

    fn process_requests<S: Sys>(&mut self, sys: &mut S) {
        if let Some(req_id) = self.present_request_id {
            if let Some(request) = load_thing::<DisplayPresentRequest>(sys, req_id) {
                self.try_present(sys, &request);
                return;
            }
            self.present_request_id = None;
        }

        if self.present_request_id.is_none() {
            if let Some(request) = Self::find_present_request(sys, self.fb_id) {
                self.frame_watch = Some(request.frame_index);
                self.present_request_id = Some(request.id);
            }
        }
    }

    fn try_present<S: Sys>(&mut self, sys: &mut S, request: &DisplayPresentRequest) {
        if request.framebuffer_id != self.fb_id {
            return;
        }
        if request.completed {
            return;
        }
        if self.frame_watch == Some(request.frame_index) {
            return;
        }

        self.blit_front_buffer(sys);

        self.frame_watch = Some(request.frame_index);
        self.frames_presented = self.frames_presented.saturating_add(1);
        self.last_present_ns = sys.time_monotonic_ns();

        let _ = update_props(
            sys,
            self.fb_id,
            &[
                (
                    abi::graph_kinds::PROP_LAST_PRESENT_NS,
                    PropValue::U64(self.last_present_ns),
                ),
                (
                    abi::graph_kinds::PROP_FRAMES_PRESENTED,
                    PropValue::U64(self.frames_presented),
                ),
            ],
        );

        let _ = update_props(
            sys,
            request.id,
            &[
                (
                    abi::graph_kinds::PROP_PRESENTED_AT_NS,
                    PropValue::U64(self.last_present_ns),
                ),
                (abi::graph_kinds::PROP_COMPLETED, PropValue::Bool(true)),
            ],
        );
    }

    fn display_buffer_target<S: Sys>(
        sys: &mut S,
        display_id: ThingId,
        pred: Predicate,
    ) -> Result<ThingId, SysError> {
        let mut targets = link_targets(sys, display_id, pred);
        targets.pop().ok_or(SysError::Unexpected)
    }

    fn clamp_active_buffer_index(value: i64) -> i64 {
        if value == 1 { 1 } else { 0 }
    }

    fn load_active_buffer_index<S: Sys>(sys: &mut S, display_id: ThingId) -> i64 {
        let display = load_thing::<DisplayThing>(sys, display_id);
        Self::clamp_active_buffer_index(display.map(|d| d.active_buffer_index).unwrap_or(0))
    }

    fn map_buffer_view<S: Sys>(
        sys: &mut S,
        buffer_id: ThingId,
        flags: MapFlags,
    ) -> Result<SharedBufferView, SysError> {
        let info = shared_buffer_info(sys, buffer_id)?;
        let (ptr, size) = shared_buffer_map(sys, buffer_id, flags)?;
        Ok(SharedBufferView {
            _id: buffer_id,
            info,
            ptr,
            size,
        })
    }

    fn blit_front_buffer<S: Sys>(&mut self, sys: &mut S) {
        if let Some(display) = load_thing::<DisplayThing>(sys, self.display_id) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{KernelRequest, KernelResponse, PropKey, PropValue, Thing, ThingId};
    use alloc::vec::Vec;
    use thing_os::{DisplayPresentRequest, doc_helpers::DocSys};

    fn thing_props<T: Thing>(thing: &T) -> &'static [Option<(PropKey, PropValue)>] {
        let mut props = Vec::new();
        thing.to_props(&mut props);
        DocSys::props_slice(props)
    }

    #[test]
    fn find_present_request_returns_matching_buffer() {
        let request = DisplayPresentRequest {
            id: ThingId(3),
            framebuffer_id: ThingId(5),
            frame_index: 7,
            requested_at_ns: 0,
            presented_at_ns: Some(1),
            completed: true,
        };
        let mut sys = DocSys::with_responses(vec![
            KernelResponse::ThingListEntry {
                id: Some(request.id),
            },
            KernelResponse::ThingData {
                id: request.id,
                kind: DisplayPresentRequest::KIND,
                props: thing_props(&request),
            },
            KernelResponse::ThingListEntry { id: None },
        ]);

        let found = FramebufferDriver::find_present_request(&mut sys, request.framebuffer_id);
        assert_eq!(found.map(|f| f.id), Some(request.id));
    }

    #[test]
    fn ensure_present_request_creates_when_missing() {
        let framebuffer = ThingId(11);
        let mut sys = DocSys::with_responses(vec![
            KernelResponse::ThingListEntry { id: None },
            KernelResponse::ThingCreated { id: ThingId(22) },
        ]);

        let request = FramebufferDriver::ensure_present_request(&mut sys, framebuffer)
            .expect("should create present request");
        assert_eq!(request.framebuffer_id, framebuffer);
        assert_eq!(request.id, ThingId(22));

        let requests = sys.requests.borrow();
        assert!(requests.iter().any(|request| matches!(
            request,
            KernelRequest::ThingCreate { kind, .. } if *kind == DisplayPresentRequest::KIND
        )));
    }
}

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "framebuffer_driver: starting");

    let mut driver = loop {
        match FramebufferDriver::init(sys) {
            Ok(driver) => break driver,
            Err(err) => {
                log_dynamic(
                    sys,
                    format_args!("framebuffer_driver: init failed ({:?}), retrying", err),
                );
                sys.sleep_for_ns(RETRY_INTERVAL_NS);
            }
        }
    };

    log_dynamic(
        sys,
        format_args!(
            "framebuffer_driver: registered framebuffer {}x{} stride {} display {} fmt {:?}",
            driver.width, driver.height, driver.stride, driver.display_id.0, driver.pixel_format,
        ),
    );

    loop {
        let sleep_ns = driver.tick(sys);
        sys.sleep_for_ns(sleep_ns);
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

fn primary_display_descriptor<S: Sys>(sys: &mut S) -> Result<DisplayDescriptor, SysError> {
    use thing_os::DisplayThing;

    let displays: Vec<DisplayThing> = list_things_by_kind(sys);
    let display = displays
        .iter()
        .find(|d| d.name == "display0")
        .or_else(|| displays.first())
        .cloned()
        .ok_or(SysError::Unexpected)?;

    let mut targets = link_targets(sys, display.id, abi::graph_kinds::LINK_DISPLAY_SCANOUT);
    let buffer_id = targets.pop().ok_or(SysError::Unexpected)?;
    let info = thing_os::shared_buffer_info(sys, buffer_id)?;

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
    const KIND: &'static str = abi::graph_kinds::KIND_DISPLAY_FRAMEBUFFER;
    const DESCRIPTION: &'static str = "Userland-published framebuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((
            abi::graph_kinds::PROP_NAME,
            PropValue::Str(self.name.clone()),
        ));
        out.push((abi::graph_kinds::PROP_WIDTH, PropValue::U64(self.width)));
        out.push((abi::graph_kinds::PROP_HEIGHT, PropValue::U64(self.height)));
        out.push((abi::graph_kinds::PROP_STRIDE, PropValue::U64(self.stride)));
        let fmt = match self.pixel_format {
            PixelFormat::Rgba8888 => "Rgba8888",
            PixelFormat::Bgra8888 => "Bgra8888",
        };
        out.push((
            abi::graph_kinds::PROP_PIXEL_FORMAT,
            PropValue::Str(fmt.into()),
        ));
        out.push((
            abi::graph_kinds::PROP_POWER_STATE,
            PropValue::Str(self.power_state.as_str().into()),
        ));
        if let Some(refresh) = self.refresh_interval_ns {
            out.push((
                abi::graph_kinds::PROP_REFRESH_INTERVAL_NS,
                PropValue::U64(refresh),
            ));
        }
        out.push((
            abi::graph_kinds::PROP_FRAMES_PRESENTED,
            PropValue::U64(self.frames_presented),
        ));
        out.push((
            abi::graph_kinds::PROP_LAST_PRESENT_NS,
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
            match prop.0 {
                abi::graph_kinds::PROP_NAME => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                abi::graph_kinds::PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                abi::graph_kinds::PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                abi::graph_kinds::PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                abi::graph_kinds::PROP_PIXEL_FORMAT => {
                    if let PropValue::Str(v) = &prop.1 {
                        pixel_format = if v == "Bgra8888" {
                            PixelFormat::Bgra8888
                        } else {
                            PixelFormat::Rgba8888
                        };
                    }
                }
                abi::graph_kinds::PROP_POWER_STATE => {
                    if let PropValue::Str(v) = &prop.1 {
                        power_state = DisplayPowerState::from_str(v);
                    }
                }
                abi::graph_kinds::PROP_REFRESH_INTERVAL_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        refresh_interval_ns = Some(v);
                    }
                }
                abi::graph_kinds::PROP_FRAMES_PRESENTED => {
                    if let PropValue::U64(v) = prop.1 {
                        frames_presented = v;
                    }
                }
                abi::graph_kinds::PROP_LAST_PRESENT_NS => {
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
            (abi::graph_kinds::PROP_NAME, PropType::Str),
            (abi::graph_kinds::PROP_WIDTH, PropType::U64),
            (abi::graph_kinds::PROP_HEIGHT, PropType::U64),
            (abi::graph_kinds::PROP_STRIDE, PropType::U64),
            (abi::graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
            (abi::graph_kinds::PROP_POWER_STATE, PropType::Str),
            (abi::graph_kinds::PROP_REFRESH_INTERVAL_NS, PropType::U64),
            (abi::graph_kinds::PROP_FRAMES_PRESENTED, PropType::U64),
            (abi::graph_kinds::PROP_LAST_PRESENT_NS, PropType::U64),
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
