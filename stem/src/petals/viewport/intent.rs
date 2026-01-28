//! Viewport intent types — output of gesture interpretation.

/// Intent produced by gesture interpretation.
///
/// These are high-level commands that can be applied to a viewport.
/// The controller converts raw input events into these intents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewportIntent {
    /// Pan by screen-space pixels.
    PanByScreen { dx: f32, dy: f32 },

    /// Zoom about a screen-space anchor point.
    ZoomAbout { screen_x: f32, screen_y: f32, factor: f32 },

    /// Begin a gesture (e.g., drag started).
    BeginGesture,

    /// End a gesture (e.g., drag ended).
    EndGesture,
}
