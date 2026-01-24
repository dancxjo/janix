extern crate alloc;

use alloc::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::frame::AssetGeneration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Arc<[u32]>,
    /// Generation when this asset became ready
    pub gen: AssetGeneration,
}

impl Image {
    /// Compute decoded bytes for this image
    pub fn decoded_bytes(&self) -> usize {
        (self.width as usize) * (self.height as usize) * 4
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CursorFrame {
    pub image: Image,
    pub delay_ms: u32,
    pub hotspot_x: u32,
    pub hotspot_y: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CursorAsset {
    Static(CursorFrame),
    Animated { frames: Arc<[CursorFrame]> },
}

impl CursorAsset {
    pub fn generation(&self) -> AssetGeneration {
        match self {
            CursorAsset::Static(f) => f.image.gen,
            CursorAsset::Animated { frames } => frames
                .first()
                .map(|f| f.image.gen)
                .unwrap_or(AssetGeneration::ZERO),
        }
    }

    /// Compute decoded bytes for cursor asset
    pub fn decoded_bytes(&self) -> usize {
        match self {
            CursorAsset::Static(f) => f.image.decoded_bytes(),
            CursorAsset::Animated { frames } => {
                frames.iter().map(|f| f.image.decoded_bytes()).sum()
            }
        }
    }
}
