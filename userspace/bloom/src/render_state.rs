use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use crate::asset::Image;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RasterKey {
    Svg { node_id: u64, w: u32, h: u32, content_hash: u64 },
    Text { node_id: u64, w: u32, h: u32, content_hash: u64 },
}

pub struct RenderState {
    pub raster_cache: BTreeMap<RasterKey, Arc<Image>>,
}

impl RenderState {
    pub fn new() -> Self {
        Self {
            raster_cache: BTreeMap::new(),
        }
    }
}
