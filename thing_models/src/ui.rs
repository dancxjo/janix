//! UI and mode-related Thing models.

extern crate alloc;

use abi::{PropKey, PropType, PropValue, Thing, ThingId, graph_kinds};
use alloc::string::String;
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
        out.push((
            graph_kinds::PROP_MODE_INDEX,
            PropValue::U64(self.index as u64),
        ));
        out.push((graph_kinds::PROP_NAME, PropValue::Str(self.name.clone())));
        if let Some(place) = self.place_id {
            out.push((graph_kinds::PROP_MODE_PLACE, PropValue::U64(place.0)));
        }
        out.push((graph_kinds::PROP_MODE_ACTIVE, PropValue::Bool(self.active)));
        if let Some(policy) = self.layout_policy {
            out.push((graph_kinds::PROP_MODE_LAYOUT_POLICY, PropValue::I64(policy)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut index = 0;
        let mut name = String::new();
        let mut place_id = None;
        let mut active = false;
        let mut layout_policy = None;

        for prop in props.iter().flatten() {
            match prop.0 {
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
        out.push((graph_kinds::PROP_NAME, PropValue::Str(self.name.clone())));
        if let Some(mode) = &self.layout_mode {
            out.push((graph_kinds::PROP_LAYOUT_MODE, PropValue::Str(mode.clone())));
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
}

impl Thing for Window {
    const KIND: &'static str = graph_kinds::KIND_WINDOW;
    const DESCRIPTION: &'static str = "A window owned by an app";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_PLACE_ID, PropValue::U64(self.place_id.0)));
        out.push((graph_kinds::PROP_WINDOW_X, PropValue::I64(self.x as i64)));
        out.push((graph_kinds::PROP_WINDOW_Y, PropValue::I64(self.y as i64)));
        out.push((
            graph_kinds::PROP_WINDOW_WIDTH,
            PropValue::I64(self.width as i64),
        ));
        out.push((
            graph_kinds::PROP_WINDOW_HEIGHT,
            PropValue::I64(self.height as i64),
        ));
        out.push((
            graph_kinds::PROP_Z_INDEX,
            PropValue::I64(self.z_index as i64),
        ));
        out.push((
            graph_kinds::PROP_WINDOW_ACTIVE,
            PropValue::Bool(self.active),
        ));
        out.push((graph_kinds::PROP_TITLE, PropValue::Str(self.title.clone())));
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

        for prop in props.iter().flatten() {
            match prop.0 {
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
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Surface {
    pub id: ThingId,
    pub window_id: ThingId,
    pub kind: String,
    pub text: String,
}

impl Thing for Surface {
    const KIND: &'static str = graph_kinds::KIND_SURFACE;
    const DESCRIPTION: &'static str = "Renderable surface attached to a window";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((
            graph_kinds::PROP_WINDOW_ID,
            PropValue::U64(self.window_id.0),
        ));
        out.push((
            graph_kinds::PROP_SURFACE_KIND,
            PropValue::Str(self.kind.clone()),
        ));
        out.push((
            graph_kinds::PROP_SURFACE_TEXT,
            PropValue::Str(self.text.clone()),
        ));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut window_id = ThingId(0);
        let mut kind = String::from("text");
        let mut text = String::new();

        for prop in props.iter().flatten() {
            match prop.0 {
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
                _ => {}
            }
        }

        Surface {
            id,
            window_id,
            kind,
            text,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_WINDOW_ID, PropType::U64),
            (graph_kinds::PROP_SURFACE_KIND, PropType::Str),
            (graph_kinds::PROP_SURFACE_TEXT, PropType::Str),
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
        out.push((
            graph_kinds::PROP_MODE_INDEX,
            PropValue::U64(self.mode_index as u64),
        ));
        out.push((graph_kinds::PROP_TIMESTAMP, PropValue::U64(self.timestamp)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut mode_index = 0;
        let mut timestamp = 0;
        for prop in props.iter().flatten() {
            match prop.0 {
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
