#![no_std]

use abi::Predicate;

pub const KIND_WIDGET: &str = "Widget";

// Widget layout props (generic, not window-specific)
pub const PROP_X: &str = "x"; // absolute overlay x
pub const PROP_Y: &str = "y"; // absolute overlay y

pub const PROP_MIN_WIDTH: &str = "min_width";
pub const PROP_MIN_HEIGHT: &str = "min_height";
pub const PROP_MAX_WIDTH: &str = "max_width";
pub const PROP_MAX_HEIGHT: &str = "max_height";

pub const PROP_FLEX_DIRECTION: &str = "flex_direction"; // "row" | "column"
pub const PROP_FLEX_WRAP: &str = "flex_wrap"; // "nowrap" | "wrap"
pub const PROP_JUSTIFY_CONTENT: &str = "justify_content"; // "start" | "center" | "end" | "space-between" | "space-around"
pub const PROP_ALIGN_ITEMS: &str = "align_items"; // "start" | "center" | "end" | "stretch"

pub const PROP_FLEX_GROW: &str = "flex_grow"; // f32 encoded as I64 scaled? (see below) OR Text.
pub const PROP_FLEX_SHRINK: &str = "flex_shrink";
pub const PROP_GAP: &str = "gap";

pub const LINK_WIDGET_CHILD: Predicate = Predicate(0x0014);
