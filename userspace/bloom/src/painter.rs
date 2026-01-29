use crate::asset::AssetBank;
use crate::drawlist::DrawList;
use crate::surface::Surface;
use stem::thing::ThingId;
use abi::schema::keys;

pub struct Painter<'a> {
    assets: &'a AssetBank,
}

impl<'a> Painter<'a> {
    pub fn new(assets: &'a AssetBank) -> Self {
        Self { assets }
    }

    pub fn paint_list_to_window(&mut self, wid: ThingId, list: &DrawList) -> Result<(), abi::errors::Errno> {
        let (Ok(bs_id_u64), Ok(w), Ok(h)) = (
            stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_BYTESPACE),
            stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_WIDTH),
            stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_HEIGHT),
        ) else {
            return Err(abi::errors::Errno::EINVAL);
        };

        let bs_id = ThingId::from_u64(bs_id_u64);
        let stride = stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_STRIDE).unwrap_or(w * 4);
        
        if let Ok(ptr) = stem::thing::sys::bytespace_map(bs_id) {
            let mut surf = unsafe { Surface::new(ptr as *mut u8, (stride * h) as usize, w as u32, h as u32, stride as u32) };
            surf.clear(); // Ensure surface is zeroed before painting
            
            // Paint the list using the unified rasterizer
            crate::raster::execute(&mut surf, list, false);

            let _ = stem::thing::sys::bytespace_unmap(bs_id, ptr);
        }

        Ok(())
    }

    pub fn paint_window_snapshot(&mut self, wid: ThingId) -> Result<(), abi::errors::Errno> {
        let (Ok(bs_id_u64), Ok(w), Ok(h)) = (
            stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_BYTESPACE),
            stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_WIDTH),
            stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_HEIGHT),
        ) else {
            return Err(abi::errors::Errno::EINVAL);
        };

        let bs_id = ThingId::from_u64(bs_id_u64);
        let stride = stem::thing::sys::prop_get(wid, keys::UI_SNAPSHOT_STRIDE).unwrap_or(w * 4);
        
        if let Ok(ptr) = stem::thing::sys::bytespace_map(bs_id) {
            let mut surf = unsafe { Surface::new(ptr as *mut u8, (stride * h) as usize, w as u32, h as u32, stride as u32) };
            surf.clear(); // Ensure surface is zeroed before painting
            let color_top = 0xFF303038u32;
            let color_bot = 0xFF101018u32;
            for y in 0..h as i32 {
                let r = ((color_top >> 16) & 0xFF) as i32 + (((color_bot >> 16) & 0xFF) as i32 - ((color_top >> 16) & 0xFF) as i32) * y / h as i32;
                let g = ((color_top >> 8) & 0xFF) as i32 + (((color_bot >> 8) & 0xFF) as i32 - ((color_top >> 8) & 0xFF) as i32) * y / h as i32;
                let b = (color_top & 0xFF) as i32 + ((color_bot & 0xFF) as i32 - (color_top & 0xFF) as i32) * y / h as i32;
                let color = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                for x in 0..w as i32 {
                    surf.put_px(x, y, color);
                }
            }
            
            let _ = stem::thing::sys::bytespace_unmap(bs_id, ptr);
        }

        Ok(())
    }

    /// Paint cursor snapshot to kernel graph bytespace.
    /// 
    /// This is used for kernel-level cursor snapshot management.
    /// For direct compositor blending, use `CursorRasterizer` instead.
    pub fn paint_cursor_snapshot(
        &mut self, 
        UI_CROWN: ThingId, 
        cursor_rasterizer: &mut crate::cursor_rasterizer::CursorRasterizer,
    ) -> Result<(), abi::errors::Errno> {
        let bs_id_u64 = stem::thing::sys::prop_get(UI_CROWN, keys::UI_CURSOR_SNAPSHOT_BYTESPACE).unwrap_or(0);
        if bs_id_u64 == 0 { return Err(abi::errors::Errno::ENOENT); }
        
        let bs_id = ThingId::from_u64(bs_id_u64);
        let w = stem::thing::sys::prop_get(UI_CROWN, keys::UI_CURSOR_SNAPSHOT_WIDTH).unwrap_or(32);
        let h = stem::thing::sys::prop_get(UI_CROWN, keys::UI_CURSOR_SNAPSHOT_HEIGHT).unwrap_or(32);
        let stride = stem::thing::sys::prop_get(UI_CROWN, keys::UI_CURSOR_SNAPSHOT_STRIDE).unwrap_or(w * 4);

        if let Ok(ptr) = stem::thing::sys::bytespace_map(bs_id) {
            let mut surf = unsafe { Surface::new(ptr as *mut u8, (stride * h) as usize, w as u32, h as u32, stride as u32) };
            
            // Fill with transparency
            surf.clear();

            // Get cursor snapshot from rasterizer if asset is available
            if let Some(asset) = self.assets.get_cursor() {
                if let Some(snapshot) = cursor_rasterizer.get_snapshot(&asset) {
                    // Blit the pre-composited cursor snapshot centered in the bytespace
                    let src_w = snapshot.image.width.min(w as u32);
                    let src_h = snapshot.image.height.min(h as u32);
                    for sy in 0..src_h as i32 {
                        for sx in 0..src_w as i32 {
                            let px = snapshot.image.pixels[(sy as usize) * (snapshot.image.width as usize) + (sx as usize)];
                            if (px >> 24) > 0 {
                                surf.put_px(sx, sy, px);
                            }
                        }
                    }
                }
            } else {
                // White arrow fallback
                for i in 0..16 {
                    for j in 0..i {
                        surf.put_px(j as i32, i as i32, 0xFFFFFFFF);
                    }
                }
            }

            let _ = stem::thing::sys::bytespace_unmap(bs_id, ptr);
        }
        
        Ok(())
    }
}
