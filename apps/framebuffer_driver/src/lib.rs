#![no_std]

extern crate alloc;

use abi::{PixelFormat, ThingId};
use alloc::string::String;
use userland::prelude::*;
use userland_std::SysError;

pub struct FramebufferDriver {
    fb_id: ThingId,
    fb_ptr: *mut u32,
    width: u32,
    height: u32,
    stride: u32,
    pixel_format: PixelFormat,
    frame_watch: Option<u64>,
}

impl FramebufferDriver {
    fn init<S: Sys>(sys: &mut S) -> Result<Self, SysError> {
        let display = userland_std::open_primary_display_buffer(sys)?;

        let fb_ptr = display.ptr as *mut u32;
        let width = display.info.width;
        let height = display.info.height;
        let stride = display.info.stride;
        let pixel_format = display.info.pixel_format;

        let fb_thing = DisplayFramebufferThing {
            id: ThingId(0),
            name: "fb0".into(),
            width: width as u64,
            height: height as u64,
            stride: stride as u64,
            pixel_format,
        };

        let _ = register_schema_for::<DisplayFramebufferThing>(sys);
        let fb_id = create_thing(sys, &fb_thing).ok_or(SysError::Unexpected)?;

        Ok(Self {
            fb_id,
            fb_ptr,
            width,
            height,
            stride,
            pixel_format,
            frame_watch: None,
        })
    }

    fn tick<S: Sys>(&mut self, _sys: &mut S) {
        let _ = self.fb_id;
    }
}

pub fn run<S: Sys>(sys: &mut S) -> ! {
    println(sys, "framebuffer_driver: starting");

    let mut driver = match FramebufferDriver::init(sys) {
        Ok(driver) => driver,
        Err(err) => {
            println(sys, "framebuffer_driver: failed to initialize primary display");
            let _ = err;
            sys.exit_thread();
        }
    };

    println(sys, "framebuffer_driver: mapped primary framebuffer");

    loop {
        driver.tick(sys);
        sys.sleep_for_ns(50_000_000);
    }
}

#[derive(Clone, Debug)]
struct DisplayFramebufferThing {
    pub id: ThingId,
    pub name: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub pixel_format: PixelFormat,
}

impl Thing for DisplayFramebufferThing {
    const KIND: &'static str = abi::graph_kinds::KIND_DISPLAY_FRAMEBUFFER;
    const DESCRIPTION: &'static str = "Userland-published framebuffer";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((abi::graph_kinds::PROP_NAME, PropValue::Str(self.name.clone())));
        out.push((abi::graph_kinds::PROP_WIDTH, PropValue::U64(self.width)));
        out.push((abi::graph_kinds::PROP_HEIGHT, PropValue::U64(self.height)));
        out.push((abi::graph_kinds::PROP_STRIDE, PropValue::U64(self.stride)));
        let fmt = match self.pixel_format {
            PixelFormat::Rgba8888 => "Rgba8888",
            PixelFormat::Bgra8888 => "Bgra8888",
        };
        out.push((abi::graph_kinds::PROP_PIXEL_FORMAT, PropValue::Str(fmt.into())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut pixel_format = PixelFormat::Rgba8888;

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
                _ => {}
            }
        }

        DisplayFramebufferThing {
            id,
            name,
            width,
            height,
            stride,
            pixel_format,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (abi::graph_kinds::PROP_NAME, PropType::Str),
            (abi::graph_kinds::PROP_WIDTH, PropType::U64),
            (abi::graph_kinds::PROP_HEIGHT, PropType::U64),
            (abi::graph_kinds::PROP_STRIDE, PropType::U64),
            (abi::graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
        ]
    }
}
