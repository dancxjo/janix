//! CPU Framebuffer Target - software rendering implementation
//!
//! This is the default fallback target that uses the CPU rasterizer
//! to render into a mapped framebuffer.

use crate::damage::Damage;
use crate::drawlist::DrawList;
use crate::lowered::lower;
use crate::present::PresenterImpl;
use crate::raster::execute_lowered;
use crate::surface::PixelBuffer;
use crate::target::CompositorTarget;

/// CPU-based compositor target using software rasterization
pub struct CpuFramebufferTarget<'a> {
    surface: &'a mut PixelBuffer,
    presenter: &'a mut PresenterImpl,
}

impl<'a> CpuFramebufferTarget<'a> {
    pub fn new(surface: &'a mut PixelBuffer, presenter: &'a mut PresenterImpl) -> Self {
        Self { surface, presenter }
    }
}

impl CompositorTarget for CpuFramebufferTarget<'_> {
    fn submit(&mut self, drawlist: &DrawList) {
        let lowered = lower(drawlist);
        execute_lowered(self.surface, &lowered);
    }

    fn present(&mut self, damage: &Damage) {
        self.presenter.present(damage);
        self.presenter.pump();
    }
}
