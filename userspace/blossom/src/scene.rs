extern crate alloc;

use alloc::sync::Arc;

use crate::asset::AssetPack;

pub struct SceneState {
    pub screen_width: u32,
    pub screen_height: u32,
    pub cursor_pos: (i32, i32),
    pub assets: Arc<AssetPack>,
    pub now_ns: u64,
}
