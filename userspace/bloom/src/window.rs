use alloc::string::String;
use stem::thing::ThingId;

/// Window policy object layered on top of a single Surface.
/// Provides semantic window management attributes separate from pixel representation.
pub struct Window {
    pub surface_id: ThingId, // Corresponds to the Surface in the SceneGraph
    pub app_id: String,
    pub title: String,

    // Size hints
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,

    // State
    pub is_maximized: bool,
    pub is_fullscreen: bool,
    pub is_shaded: bool,
    pub manual_position: bool,

    // Insets / Decorations
    pub inset_right: i32,
    pub inset_bottom: i32,
    pub pre_maximize_rect: Option<crate::geometry::Rect>,
}

impl Window {
    pub fn new(surface_id: ThingId) -> Self {
        Self {
            surface_id,
            app_id: String::new(),
            title: String::new(),
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            is_maximized: false,
            is_fullscreen: false,
            is_shaded: false,
            manual_position: false,
            inset_right: 0,
            inset_bottom: 0,
            pre_maximize_rect: None,
        }
    }
}
