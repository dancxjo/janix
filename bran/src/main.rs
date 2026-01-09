#![no_std]
#![no_main]

//! The Boot Runtime Abstraction Node (BRAN) is the seed coat around the kernel.
//! It wraps the abstract kernel with the low-level mechanisms to speak to the
//! architecture. Importantly, most hardware belongs in userspace, not here.
//! This is just the layer between the kernel and the boot environment. All
//! speaking with limine should happen here as well. Nothing beyond this layer
//! should know about limine or booting, except through the implementation of
//! the BootRuntime trait.

mod arch;

use arch::{hcf, SerialPort};
use core::assert;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();

#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(BASE_REVISION.is_supported());
    indicate_progress();
    let runtime = SerialPort;
    kernel::start(runtime);
}

struct BranDisplay {
    buffer: &'static mut [u32],
    width: usize,
    height: usize,
    pitch: usize, // in bytes
}

impl BranDisplay {
    fn new(framebuffer: &limine::framebuffer::Framebuffer) -> Self {
        let width = framebuffer.width() as usize;
        let height = framebuffer.height() as usize;
        let pitch = framebuffer.pitch() as usize;
        let buffer = unsafe {
            core::slice::from_raw_parts_mut(
                framebuffer.addr().cast::<u32>(),
                (pitch * height) / 4,
            )
        };
        
        Self {
            buffer,
            width,
            height,
            pitch,
        }
    }
}

impl OriginDimensions for BranDisplay {
    fn size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }
}

impl DrawTarget for BranDisplay {
    type Color = Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(point, color) in pixels.into_iter() {
            if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 && point.y < self.height as i32 {
                 let x = point.x as usize;
                 let y = point.y as usize;
                 // Assuming 32 bpp (4 bytes per pixel)
                 let offset = (y * self.pitch + x * 4) / 4;
                 
                 // Pack version of Rgb888 to u32 (0x00RRGGBB)
                 let color_u32 = ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | (color.b() as u32);
                 
                 self.buffer[offset] = color_u32;
            }
        }
        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let intersection = area.intersection(&self.bounding_box());
        
        if intersection.size == Size::zero() {
            return Ok(());
        }

        let color_u32 = ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | (color.b() as u32);

        let min_x = intersection.top_left.x as usize;
        let min_y = intersection.top_left.y as usize;
        let max_x = (intersection.top_left.x as usize) + (intersection.size.width as usize);
        let max_y = (intersection.top_left.y as usize) + (intersection.size.height as usize);

        for y in min_y..max_y {
            let row_start = (y * self.pitch + min_x * 4) / 4;
            let row_end = (y * self.pitch + max_x * 4) / 4;
            
            // Optimization for filling: use slice fill if contiguous? 
            // Framebuffer rows might have padding (pitch > width * 4).
            // So we fill row by row.
            self.buffer[row_start..row_end].fill(color_u32);
        }
        
        Ok(())
    }
}

fn indicate_progress() {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            let mut display = BranDisplay::new(&framebuffer);
            // Default Mallard Teal: #00474F
            let _ = display.clear(Rgb888::new(0x00, 0x47, 0x4F));
        }
    }
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hcf()
}
