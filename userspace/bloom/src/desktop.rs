extern crate alloc;

use abi::errors::Errno;
use abi::syscall::vfs_flags::O_RDONLY;
use abi::vfs_watch::{flags as watch_flags, mask as watch_mask};
use alloc::string::String;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read, vfs_stat, vfs_watch_path};

use crate::asset::{AssetBank, Image};
use crate::geometry::Color;
use crate::session_fs;

pub const WATCH_BUFFER_BYTES: usize = 512;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WallpaperMode {
    Stretch,
    Center,
    Tile,
    Fit,
    #[default]
    Fill,
}

impl WallpaperMode {
    pub fn parse(text: &str) -> Self {
        match text.trim() {
            "stretch" => Self::Stretch,
            "center" => Self::Center,
            "tile" => Self::Tile,
            "fit" => Self::Fit,
            "fill" => Self::Fill,
            _ => Self::Fill,
        }
    }
}

pub struct BackgroundLayer {
    pub color: Color,
    pub image: Option<Image>,
    pub mode: WallpaperMode,
}

impl Default for BackgroundLayer {
    fn default() -> Self {
        Self {
            color: Color::from_u32(0xFF101018),
            image: None,
            mode: WallpaperMode::Fill,
        }
    }
}

pub struct DesktopState {
    watch_fd: Option<u32>,
    wallpaper_path: String,
    layer: BackgroundLayer,
}

impl DesktopState {
    pub fn new() -> Self {
        session_fs::ensure_session_roots();
        let mut state = Self {
            watch_fd: vfs_watch_path(
                session_fs::DESKTOP_ROOT,
                watch_mask::CREATE
                    | watch_mask::REMOVE
                    | watch_mask::MODIFY
                    | watch_mask::MOVE_FROM
                    | watch_mask::MOVE_TO,
                watch_flags::NONBLOCK | watch_flags::ONLYDIR,
            )
            .ok(),
            wallpaper_path: String::new(),
            layer: BackgroundLayer::default(),
        };
        state.reload();
        state
    }

    pub fn layer(&self) -> &BackgroundLayer {
        &self.layer
    }

    pub fn poll_watch_activity(&mut self) -> bool {
        if !drain_watch(self.watch_fd) {
            return false;
        }
        self.reload();
        true
    }

    fn reload(&mut self) {
        let wallpaper_path = session_fs::read_text(&session_fs::desktop_path("wallpaper"))
            .unwrap_or_default()
            .trim()
            .to_string();
        let mode = session_fs::read_text(&session_fs::desktop_path("mode"))
            .map(|text| WallpaperMode::parse(&text))
            .unwrap_or_default();
        let color = session_fs::read_text(&session_fs::desktop_path("background_color"))
            .and_then(|text| parse_color(&text))
            .unwrap_or(Color::from_u32(0xFF101018));

        let image = if wallpaper_path.is_empty() {
            None
        } else if wallpaper_path != self.wallpaper_path {
            load_image_from_path(&wallpaper_path)
        } else {
            self.layer
                .image
                .clone()
                .or_else(|| load_image_from_path(&wallpaper_path))
        };

        self.wallpaper_path = wallpaper_path;
        self.layer = BackgroundLayer { color, image, mode };
    }
}

impl Drop for DesktopState {
    fn drop(&mut self) {
        if let Some(fd) = self.watch_fd.take() {
            let _ = vfs_close(fd);
        }
    }
}

fn parse_color(text: &str) -> Option<Color> {
    let trimmed = text.trim();
    let hex = trimmed.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let value = u32::from_str_radix(hex, 16).ok()?;
    Some(Color::rgb(
        ((value >> 16) & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        (value & 0xFF) as u8,
    ))
}

fn load_image_from_path(path: &str) -> Option<Image> {
    let fd = vfs_open(path, O_RDONLY).ok()?;
    let (_, size, _) = vfs_stat(fd).ok()?;
    let image = AssetBank::load_wallpaper_immediate_from_fd_with_size(fd, size as usize, path);
    let _ = vfs_close(fd);
    image
}

fn drain_watch(fd: Option<u32>) -> bool {
    let Some(fd) = fd else {
        return false;
    };

    let mut saw_event = false;
    loop {
        let mut buf = [0u8; WATCH_BUFFER_BYTES];
        match vfs_read(fd, &mut buf) {
            Ok(0) => break,
            Ok(_) => saw_event = true,
            Err(Errno::EAGAIN) => break,
            Err(_) => break,
        }
    }
    saw_event
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hex_color() {
        assert_eq!(parse_color("#112233"), Some(Color::rgb(0x11, 0x22, 0x33)));
        assert_eq!(parse_color("112233"), None);
        assert_eq!(parse_color("#123"), None);
    }

    #[test]
    fn parses_wallpaper_mode() {
        assert_eq!(WallpaperMode::parse("stretch"), WallpaperMode::Stretch);
        assert_eq!(WallpaperMode::parse("center"), WallpaperMode::Center);
        assert_eq!(WallpaperMode::parse("tile"), WallpaperMode::Tile);
        assert_eq!(WallpaperMode::parse("fit"), WallpaperMode::Fit);
        assert_eq!(WallpaperMode::parse("fill"), WallpaperMode::Fill);
        assert_eq!(WallpaperMode::parse("unknown"), WallpaperMode::Fill);
    }
}
