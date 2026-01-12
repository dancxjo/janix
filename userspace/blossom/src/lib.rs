#![cfg_attr(not(any(test, feature = "std")), no_std)]

mod asset;
mod compositor;
mod present;
mod runtime;
mod scene;
mod surface;

pub use asset::{AssetHub, AssetJob, AssetJobQueue, AssetKind, AssetPack, AssetResultQueue, AssetUpdate};
pub use compositor::Compositor;
pub use present::{DisplaySurface, DriverPresenter, Presenter};
pub use runtime::BlossomRuntime;
pub use scene::SceneState;
pub use surface::{BackBuffer, PixelFormat, Surface};

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::sync::Arc;
    use alloc::vec;

    use super::asset::{apply_update, AssetHub, AssetPack, AssetUpdate};
    use super::asset::cursor::CursorTheme;
    use super::asset::wallpaper::WallpaperSurface;
    use super::{Compositor, PixelFormat, SceneState, Surface};

    #[test]
    fn asset_swap_changes_wallpaper_pixels() {
        let width = 64usize;
        let height = 48usize;
        let stride = width * 4;
        let mut buf = vec![0u8; stride * height];

        let pack = AssetPack::placeholder();
        let hub = AssetHub::new(Arc::clone(&pack));
        let mut surface = Surface {
            width,
            height,
            stride_bytes: stride,
            format: PixelFormat::Xrgb8888,
            buf: &mut buf,
        };

        let compositor = Compositor;
        let scene = SceneState {
            screen_width: width as u32,
            screen_height: height as u32,
            cursor_pos: (0, 0),
            assets: hub.current_pack(),
            now_ns: 0,
        };
        compositor.compose(&scene, &mut surface, None, None);
        let center = (height / 2) * stride + (width / 2) * 4;
        let before = u32::from_le_bytes([
            surface.buf[center],
            surface.buf[center + 1],
            surface.buf[center + 2],
            surface.buf[center + 3],
        ]);

        let wallpaper = WallpaperSurface {
            width: 2,
            height: 2,
            pixels: vec![0x00AA_33CC, 0x00AA_33CC, 0x00AA_33CC, 0x00AA_33CC],
        };
        let updated = apply_update(
            &pack,
            AssetUpdate::WallpaperReady(Arc::new(wallpaper)),
            1,
        );
        hub.submit_ready_pack(Arc::clone(&updated));

        let scene = SceneState {
            screen_width: width as u32,
            screen_height: height as u32,
            cursor_pos: (0, 0),
            assets: hub.current_pack(),
            now_ns: 0,
        };
        compositor.compose(&scene, &mut surface, None, None);
        let after = u32::from_le_bytes([
            surface.buf[center],
            surface.buf[center + 1],
            surface.buf[center + 2],
            surface.buf[center + 3],
        ]);

        assert_ne!(before, after);
    }

    #[test]
    fn cursor_animation_steps_frames() {
        let frame_a = bloom::cursor::CursorFrame::new(vec![0xFFFF0000; 1], 1, 1, 0, 0);
        let frame_b = bloom::cursor::CursorFrame::new(vec![0xFF00FF00; 1], 1, 1, 0, 0);
        let asset = bloom::cursor::CursorAsset {
            frames: vec![frame_a, frame_b],
            delays_ms: vec![10, 10],
            sequence: vec![],
            is_animated: true,
        };
        let theme = CursorTheme { asset };

        let first = theme.frame_at_ms(0).unwrap().pixels[0];
        let second = theme.frame_at_ms(15).unwrap().pixels[0];

        assert_ne!(first, second);
    }
}
