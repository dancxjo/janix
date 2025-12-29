use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBody {
    pub title: heapless::String<64>,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub content: heapless::String<1024>,
    pub seq: u64,
}
