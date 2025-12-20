//! UI and mode-related Thing models.

extern crate alloc;

use abi::{PropKey, PropType, PropValue, Thing, ThingId};
use crate::graph_kinds;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

pub const MODE_INDEX_CONSOLE: u8 = 12;

#[derive(Clone, Debug)]
pub struct Mode {
    pub id: ThingId,
    pub index: u8,
    pub name: String,
    pub place_id: Option<ThingId>,
    pub active: bool,
    pub layout_policy: Option<i64>,
}

impl Thing for Mode {
    const KIND: &'static str = graph_kinds::KIND_MODE;
    const DESCRIPTION: &'static str = "Logical mode (F1-F12) selecting a workspace";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_MODE_INDEX.to_string(),
            PropValue::U64(self.index as u64),
        ));
        out.push((graph_kinds::PROP_NAME.to_string(), PropValue::Str(self.name.clone())));
        if let Some(place) = self.place_id {
            out.push((graph_kinds::PROP_MODE_PLACE.to_string(), PropValue::U64(place.0)));
        }
        out.push((graph_kinds::PROP_MODE_ACTIVE.to_string(), PropValue::Bool(self.active)));
        if let Some(policy) = self.layout_policy {
            out.push((graph_kinds::PROP_MODE_LAYOUT_POLICY.to_string(), PropValue::I64(policy)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut index = 0;
        let mut name = String::new();
        let mut place_id = None;
        let mut active = false;
        let mut layout_policy = None;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_MODE_INDEX => {
                    if let PropValue::U64(v) = prop.1 {
                        index = v as u8;
                    }
                }
                graph_kinds::PROP_NAME => {
                    if let PropValue::Str(ref v) = prop.1 {
                        name = v.clone();
                    }
                }
                graph_kinds::PROP_MODE_PLACE => {
                    if let PropValue::U64(v) = prop.1 {
                        place_id = Some(ThingId(v));
                    }
                }
                graph_kinds::PROP_MODE_ACTIVE => {
                    if let PropValue::Bool(v) = prop.1 {
                        active = v;
                    }
                }
                graph_kinds::PROP_MODE_LAYOUT_POLICY => {
                    if let PropValue::I64(v) = prop.1 {
                        layout_policy = Some(v);
                    }
                }
                _ => {}
            }
        }

        Mode {
            id,
            index,
            name,
            place_id,
            active,
            layout_policy,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_MODE_INDEX, PropType::U64),
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_MODE_PLACE, PropType::U64),
            (graph_kinds::PROP_MODE_ACTIVE, PropType::Bool),
            (graph_kinds::PROP_MODE_LAYOUT_POLICY, PropType::I64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Place {
    pub id: ThingId,
    pub name: String,
    pub layout_mode: Option<String>,
}

impl Thing for Place {
    const KIND: &'static str = graph_kinds::KIND_PLACE;
    const DESCRIPTION: &'static str = "Workspace root for a mode";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_NAME.to_string(), PropValue::Str(self.name.clone())));
        if let Some(mode) = &self.layout_mode {
            out.push((graph_kinds::PROP_LAYOUT_MODE.to_string(), PropValue::Str(mode.clone())));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut layout_mode = None;
        for prop in props.iter().flatten() {
            if prop.0 == graph_kinds::PROP_NAME {
                if let PropValue::Str(ref v) = prop.1 {
                    name = v.clone();
                }
            } else if prop.0 == graph_kinds::PROP_LAYOUT_MODE {
                if let PropValue::Str(ref v) = prop.1 {
                    layout_mode = Some(v.clone());
                }
            }
        }
        Place {
            id,
            name,
            layout_mode,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_NAME, PropType::Str),
            (graph_kinds::PROP_LAYOUT_MODE, PropType::Str),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Window {
    pub id: ThingId,
    pub place_id: ThingId,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub z_index: i32,
    pub active: bool,
    pub title: String,
    pub draggable: bool,
    pub resizable: bool,
    pub closable: bool,
    pub minimizable: bool,
}

impl Thing for Window {
    const KIND: &'static str = graph_kinds::KIND_WINDOW;
    const DESCRIPTION: &'static str = "A window owned by an app";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_PLACE_ID.to_string(), PropValue::U64(self.place_id.0)));
        out.push((graph_kinds::PROP_WINDOW_X.to_string(), PropValue::I64(self.x as i64)));
        out.push((graph_kinds::PROP_WINDOW_Y.to_string(), PropValue::I64(self.y as i64)));
        out.push((graph_kinds::PROP_WINDOW_WIDTH.to_string(),
            PropValue::I64(self.width as i64),
        ));
        out.push((graph_kinds::PROP_WINDOW_HEIGHT.to_string(),
            PropValue::I64(self.height as i64),
        ));
        out.push((graph_kinds::PROP_Z_INDEX.to_string(),
            PropValue::I64(self.z_index as i64),
        ));
        out.push((graph_kinds::PROP_WINDOW_ACTIVE.to_string(),
            PropValue::Bool(self.active),
        ));
        out.push((graph_kinds::PROP_TITLE.to_string(), PropValue::Str(self.title.clone())));
        out.push((graph_kinds::PROP_DRAGGABLE.to_string(), PropValue::Bool(self.draggable)));
        out.push((graph_kinds::PROP_RESIZABLE.to_string(), PropValue::Bool(self.resizable)));
        out.push((graph_kinds::PROP_CLOSABLE.to_string(), PropValue::Bool(self.closable)));
        out.push((graph_kinds::PROP_MINIMIZABLE.to_string(), PropValue::Bool(self.minimizable)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut place_id = ThingId(0);
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        let mut z_index = 0;
        let mut active = false;
        let mut title = String::new();
        let mut draggable = true;
        let mut resizable = true;
        let mut closable = true;
        let mut minimizable = true;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_PLACE_ID => {
                    if let PropValue::U64(v) = prop.1 {
                        place_id = ThingId(v);
                    }
                }
                graph_kinds::PROP_WINDOW_X => {
                    if let PropValue::I64(v) = prop.1 {
                        x = v as i32;
                    }
                }
                graph_kinds::PROP_WINDOW_Y => {
                    if let PropValue::I64(v) = prop.1 {
                        y = v as i32;
                    }
                }
                graph_kinds::PROP_WINDOW_WIDTH => {
                    if let PropValue::I64(v) = prop.1 {
                        width = v as i32;
                    }
                }
                graph_kinds::PROP_WINDOW_HEIGHT => {
                    if let PropValue::I64(v) = prop.1 {
                        height = v as i32;
                    }
                }
                graph_kinds::PROP_Z_INDEX => {
                    if let PropValue::I64(v) = prop.1 {
                        z_index = v as i32;
                    }
                }
                graph_kinds::PROP_WINDOW_ACTIVE => {
                    if let PropValue::Bool(v) = prop.1 {
                        active = v;
                    }
                }
                graph_kinds::PROP_TITLE => {
                    if let PropValue::Str(ref v) = prop.1 {
                        title = v.clone();
                    }
                }
                graph_kinds::PROP_DRAGGABLE => {
                    if let PropValue::Bool(v) = prop.1 {
                        draggable = v;
                    }
                }
                graph_kinds::PROP_RESIZABLE => {
                    if let PropValue::Bool(v) = prop.1 {
                        resizable = v;
                    }
                }
                graph_kinds::PROP_CLOSABLE => {
                    if let PropValue::Bool(v) = prop.1 {
                        closable = v;
                    }
                }
                graph_kinds::PROP_MINIMIZABLE => {
                    if let PropValue::Bool(v) = prop.1 {
                        minimizable = v;
                    }
                }
                _ => {}
            }
        }

        Window {
            id,
            place_id,
            x,
            y,
            width,
            height,
            z_index,
            active,
            title,
            draggable,
            resizable,
            closable,
            minimizable,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_PLACE_ID, PropType::U64),
            (graph_kinds::PROP_WINDOW_X, PropType::I64),
            (graph_kinds::PROP_WINDOW_Y, PropType::I64),
            (graph_kinds::PROP_WINDOW_WIDTH, PropType::I64),
            (graph_kinds::PROP_WINDOW_HEIGHT, PropType::I64),
            (graph_kinds::PROP_Z_INDEX, PropType::I64),
            (graph_kinds::PROP_WINDOW_ACTIVE, PropType::Bool),
            (graph_kinds::PROP_TITLE, PropType::Str),
            (graph_kinds::PROP_DRAGGABLE, PropType::Bool),
            (graph_kinds::PROP_RESIZABLE, PropType::Bool),
            (graph_kinds::PROP_CLOSABLE, PropType::Bool),
            (graph_kinds::PROP_MINIMIZABLE, PropType::Bool),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Surface {
    pub id: ThingId,
    pub window_id: ThingId,
    pub kind: String,
    pub text: String,
    pub width: u64,
    pub height: u64,
    pub stride: u64,
    pub format: String,
    pub shared_buffer_id: Option<ThingId>,
    pub refresh_interval_ns: Option<u64>,
    pub frames_presented: Option<u64>,
    pub last_present_ns: Option<u64>,
    pub power_state: Option<String>,
}

impl Thing for Surface {
    const KIND: &'static str = graph_kinds::KIND_SURFACE;
    const DESCRIPTION: &'static str = "Renderable surface attached to a window";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_WINDOW_ID.to_string(),
            PropValue::U64(self.window_id.0),
        ));
        out.push((graph_kinds::PROP_SURFACE_KIND.to_string(),
            PropValue::Str(self.kind.clone()),
        ));
        out.push((graph_kinds::PROP_SURFACE_TEXT.to_string(),
            PropValue::Str(self.text.clone()),
        ));
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height)));
        out.push((graph_kinds::PROP_STRIDE.to_string(), PropValue::U64(self.stride)));
        out.push((graph_kinds::PROP_PIXEL_FORMAT.to_string(), PropValue::Str(self.format.clone())));
        if let Some(sb) = self.shared_buffer_id {
            out.push((graph_kinds::PROP_SHARED_BUFFER_ID.to_string(), PropValue::U64(sb.0)));
        }
        if let Some(interval) = self.refresh_interval_ns {
            out.push((graph_kinds::PROP_REFRESH_INTERVAL_NS.to_string(), PropValue::U64(interval)));
        }
        if let Some(frames) = self.frames_presented {
            out.push((graph_kinds::PROP_FRAMES_PRESENTED.to_string(), PropValue::U64(frames)));
        }
        if let Some(last) = self.last_present_ns {
            out.push((graph_kinds::PROP_LAST_PRESENT_NS.to_string(), PropValue::U64(last)));
        }
        if let Some(power) = &self.power_state {
            out.push((graph_kinds::PROP_POWER_STATE.to_string(), PropValue::Str(power.clone())));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut window_id = ThingId(0);
        let mut kind = String::from("text");
        let mut text = String::new();
        let mut width = 0;
        let mut height = 0;
        let mut stride = 0;
        let mut format = String::new();
        let mut shared_buffer_id = None;
        let mut refresh_interval_ns = None;
        let mut frames_presented = None;
        let mut last_present_ns = None;
        let mut power_state = None;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_WINDOW_ID => {
                    if let PropValue::U64(v) = prop.1 {
                        window_id = ThingId(v);
                    }
                }
                graph_kinds::PROP_SURFACE_KIND => {
                    if let PropValue::Str(ref v) = prop.1 {
                        kind = v.clone();
                    }
                }


                graph_kinds::PROP_SURFACE_TEXT => {
                    if let PropValue::Str(ref v) = prop.1 {
                        text = v.clone();
                    }
                }
                graph_kinds::PROP_WIDTH => {
                    if let PropValue::U64(v) = prop.1 {
                        width = v;
                    }
                }
                graph_kinds::PROP_HEIGHT => {
                    if let PropValue::U64(v) = prop.1 {
                        height = v;
                    }
                }
                graph_kinds::PROP_STRIDE => {
                    if let PropValue::U64(v) = prop.1 {
                        stride = v;
                    }
                }
                graph_kinds::PROP_PIXEL_FORMAT => {
                    if let PropValue::Str(ref v) = prop.1 {
                        format = v.clone();
                    }
                }
                graph_kinds::PROP_SHARED_BUFFER_ID => {
                    if let PropValue::U64(v) = prop.1 {
                        shared_buffer_id = Some(ThingId(v));
                    }
                }
                graph_kinds::PROP_REFRESH_INTERVAL_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        refresh_interval_ns = Some(v);
                    }
                }
                graph_kinds::PROP_FRAMES_PRESENTED => {
                    if let PropValue::U64(v) = prop.1 {
                        frames_presented = Some(v);
                    }
                }
                graph_kinds::PROP_LAST_PRESENT_NS => {
                    if let PropValue::U64(v) = prop.1 {
                        last_present_ns = Some(v);
                    }
                }
                graph_kinds::PROP_POWER_STATE => {
                    if let PropValue::Str(ref v) = prop.1 {
                        power_state = Some(v.clone());
                    }
                }
                _ => {}
            }
        }

        Surface {
            id,
            window_id,
            kind,
            text,
            width,
            height,
            stride,
            format,
            shared_buffer_id,
            refresh_interval_ns,
            frames_presented,
            last_present_ns,
            power_state,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WINDOW_ID, PropType::U64),
            (graph_kinds::PROP_SURFACE_KIND, PropType::Str),
            (graph_kinds::PROP_SURFACE_TEXT, PropType::Str),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_STRIDE, PropType::U64),
            (graph_kinds::PROP_PIXEL_FORMAT, PropType::Str),
            (graph_kinds::PROP_SHARED_BUFFER_ID, PropType::U64),
            (graph_kinds::PROP_REFRESH_INTERVAL_NS, PropType::U64),
            (graph_kinds::PROP_FRAMES_PRESENTED, PropType::U64),
            (graph_kinds::PROP_LAST_PRESENT_NS, PropType::U64),
            (graph_kinds::PROP_POWER_STATE, PropType::Str),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct ModeSwitchEvent {
    pub id: ThingId,
    pub mode_index: u8,
    pub timestamp: u64,
}

impl Thing for ModeSwitchEvent {
    const KIND: &'static str = graph_kinds::KIND_MODE_SWITCH_EVENT;
    const DESCRIPTION: &'static str = "A request to switch to a mode";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_MODE_INDEX.to_string(),
            PropValue::U64(self.mode_index as u64),
        ));
        out.push((graph_kinds::PROP_TIMESTAMP.to_string(), PropValue::U64(self.timestamp)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut mode_index = 0;
        let mut timestamp = 0;
        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_MODE_INDEX => {
                    if let PropValue::U64(v) = prop.1 {
                        mode_index = v as u8;
                    }
                }
                graph_kinds::PROP_TIMESTAMP => {
                    if let PropValue::U64(v) = prop.1 {
                        timestamp = v;
                    }
                }
                _ => {}
            }
        }
        ModeSwitchEvent {
            id,
            mode_index,
            timestamp,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_MODE_INDEX, PropType::U64),
            (graph_kinds::PROP_TIMESTAMP, PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct View {
    pub id: ThingId,
    pub parent_id: Option<ThingId>,
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub visible: bool,
    pub text: Option<String>,
    pub font: Option<String>,
    pub font_size: u64,
    pub fg_color: u32,
    pub bg_color: u32,
}

impl Thing for View {
    const KIND: &'static str = graph_kinds::KIND_VIEW;
    const DESCRIPTION: &'static str = "A generic view component (text, shape, etc)";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        if let Some(pid) = self.parent_id {
            out.push((graph_kinds::PROP_WINDOW_ID.to_string(), PropValue::U64(pid.0)));
        } 
        out.push((graph_kinds::PROP_WINDOW_X.to_string(), PropValue::I64(self.x))); 
        out.push((graph_kinds::PROP_WINDOW_Y.to_string(), PropValue::I64(self.y)));
        out.push((graph_kinds::PROP_WIDTH.to_string(), PropValue::U64(self.width as u64)));
        out.push((graph_kinds::PROP_HEIGHT.to_string(), PropValue::U64(self.height as u64)));
        out.push((graph_kinds::PROP_VISIBLE.to_string(), PropValue::Bool(self.visible)));
        if let Some(text) = &self.text {
            out.push((graph_kinds::PROP_TEXT.to_string(), PropValue::Str(text.clone())));
        }
        if let Some(font) = &self.font {
            out.push((graph_kinds::PROP_FONT_NAME.to_string(), PropValue::Str(font.clone())));
        }
        out.push((graph_kinds::PROP_FONT_SIZE.to_string(), PropValue::U64(self.font_size)));
        out.push((graph_kinds::PROP_FG_COLOR.to_string(), PropValue::U64(self.fg_color as u64)));
        out.push((graph_kinds::PROP_BG_COLOR.to_string(), PropValue::U64(self.bg_color as u64)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut parent_id = None;
        let mut x = 0;
        let mut y = 0;
        let mut width = 0;
        let mut height = 0;
        let mut visible = true;
        let mut text = None;
        let mut font = None;
        let mut font_size = 12;
        let mut fg_color = 0xFFFFFFFF;
        let mut bg_color = 0x00000000;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_WINDOW_ID => if let PropValue::U64(v) = prop.1 { parent_id = Some(ThingId(v)); },
                graph_kinds::PROP_WINDOW_X => if let PropValue::I64(v) = prop.1 { x = v; },
                graph_kinds::PROP_WINDOW_Y => if let PropValue::I64(v) = prop.1 { y = v; },
                graph_kinds::PROP_WIDTH => if let PropValue::U64(v) = prop.1 { width = v as i64; },
                graph_kinds::PROP_HEIGHT => if let PropValue::U64(v) = prop.1 { height = v as i64; },
                graph_kinds::PROP_VISIBLE => if let PropValue::Bool(v) = prop.1 { visible = v; },
                graph_kinds::PROP_TEXT => if let PropValue::Str(v) = &prop.1 { text = Some(v.clone()); },
                graph_kinds::PROP_FONT_NAME => if let PropValue::Str(v) = &prop.1 { font = Some(v.clone()); },
                graph_kinds::PROP_FONT_SIZE => if let PropValue::U64(v) = prop.1 { font_size = v; },
                graph_kinds::PROP_FG_COLOR => if let PropValue::U64(v) = prop.1 { fg_color = v as u32; },
                graph_kinds::PROP_BG_COLOR => if let PropValue::U64(v) = prop.1 { bg_color = v as u32; },
                _ => {}
            }
        }

        View {
            id, parent_id, x, y, width, height, visible, text, font, font_size, fg_color, bg_color,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WINDOW_ID, PropType::U64),
            (graph_kinds::PROP_WINDOW_X, PropType::I64),
            (graph_kinds::PROP_WINDOW_Y, PropType::I64),
            (graph_kinds::PROP_WIDTH, PropType::U64),
            (graph_kinds::PROP_HEIGHT, PropType::U64),
            (graph_kinds::PROP_VISIBLE, PropType::Bool),
            (graph_kinds::PROP_TEXT, PropType::Str),
            (graph_kinds::PROP_FONT_NAME, PropType::Str),
            (graph_kinds::PROP_FONT_SIZE, PropType::U64),
            (graph_kinds::PROP_FG_COLOR, PropType::U64),
            (graph_kinds::PROP_BG_COLOR, PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub id: ThingId,
    pub x: i64,
    pub y: i64,
    pub shape: String,
    pub visible: bool,
}

impl Thing for Cursor {
    const KIND: &'static str = graph_kinds::KIND_CURSOR;
    const DESCRIPTION: &'static str = "System pointer/cursor state";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_WINDOW_X.to_string(), PropValue::I64(self.x)));
        out.push((graph_kinds::PROP_WINDOW_Y.to_string(), PropValue::I64(self.y)));
        out.push((graph_kinds::PROP_CURSOR_SHAPE.to_string(), PropValue::Str(self.shape.clone())));
        out.push((graph_kinds::PROP_VISIBLE.to_string(), PropValue::Bool(self.visible)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut x = 0;
        let mut y = 0;
        let mut shape = String::from("arrow");
        let mut visible = true;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_WINDOW_X => if let PropValue::I64(v) = prop.1 { x = v; },
                graph_kinds::PROP_WINDOW_Y => if let PropValue::I64(v) = prop.1 { y = v; },
                graph_kinds::PROP_CURSOR_SHAPE => if let PropValue::Str(v) = &prop.1 { shape = v.clone(); },
                graph_kinds::PROP_VISIBLE => if let PropValue::Bool(v) = prop.1 { visible = v; },
                _ => {}
            }
        }

        Cursor {
            id, x, y, shape, visible,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WINDOW_X, PropType::I64),
            (graph_kinds::PROP_WINDOW_Y, PropType::I64),
            (graph_kinds::PROP_CURSOR_SHAPE, PropType::Str),
            (graph_kinds::PROP_VISIBLE, PropType::Bool),
        ]
    }
}
