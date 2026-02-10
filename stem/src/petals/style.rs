extern crate alloc;

use alloc::string::String;

use crate::errors::Result;
use crate::thing::sys::{bytespace_create, bytespace_write, create_node, link, prop_set};
use crate::thing::ThingId;
use abi::schema::{keys, kinds, rels, ui_kind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelectorKind {
    Text,
    Column,
    Row,
    WindowRoot,
    TextInput,
}

impl SelectorKind {
    fn as_ui_kind(self) -> u64 {
        match self {
            SelectorKind::Text => ui_kind::TEXT,
            SelectorKind::Column => ui_kind::COLUMN,
            SelectorKind::Row => ui_kind::ROW,
            SelectorKind::WindowRoot => ui_kind::WINDOW,
            SelectorKind::TextInput => ui_kind::TEXT_INPUT,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StyleSelector<'a> {
    pub kind: Option<SelectorKind>,
    pub class: Option<&'a str>,
    pub key: Option<&'a str>,
    pub focused: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Declarations<'a> {
    pub color: Option<u32>,
    pub background: Option<u32>,
    pub font_name: Option<&'a str>,
    pub font_size_px: Option<u64>,
    pub padding_px: Option<u64>,
    pub gap_px: Option<u64>,
    pub border_width_px: Option<u64>,
    pub border_color: Option<u32>,
    pub min_width_px: Option<u64>,
    pub min_height_px: Option<u64>,
    pub cursor_color: Option<u32>,
}

pub fn create_stylesheet() -> Result<ThingId> {
    create_node(kinds::CSS_STYLESHEET).map_err(crate::errors::Error::Errno)
}

pub fn set_default_stylesheet(ui_crown: ThingId, stylesheet: ThingId) -> Result<()> {
    prop_set(ui_crown, keys::UI_STYLESHEET_DEFAULT, stylesheet.to_u64_lossy())
        .map_err(crate::errors::Error::Errno)
}

pub fn attach_window_stylesheet(window_id: ThingId, stylesheet: ThingId) -> Result<()> {
    prop_set(window_id, keys::UI_STYLESHEET, stylesheet.to_u64_lossy())
        .map_err(crate::errors::Error::Errno)
}

pub fn add_rule(
    stylesheet: ThingId,
    selector: &StyleSelector<'_>,
    declarations: &Declarations<'_>,
) -> Result<ThingId> {
    let rule = create_node(kinds::CSS_RULE).map_err(crate::errors::Error::Errno)?;
    link(stylesheet, rels::HAS_CHILD, rule).map_err(crate::errors::Error::Errno)?;
    link(rule, rels::CHILD_OF, stylesheet).map_err(crate::errors::Error::Errno)?;

    if let Some(kind) = selector.kind {
        prop_set(rule, keys::UI_STYLE_MATCH_KIND, kind.as_ui_kind())
            .map_err(crate::errors::Error::Errno)?;
    }
    if let Some(class) = selector.class {
        set_string_prop(rule, keys::UI_STYLE_MATCH_CLASS, class)?;
    }
    if let Some(key) = selector.key {
        set_string_prop(rule, keys::UI_STYLE_MATCH_KEY, key)?;
    }
    if selector.focused {
        prop_set(rule, keys::UI_STYLE_MATCH_FOCUSED, 1).map_err(crate::errors::Error::Errno)?;
    }

    if let Some(color) = declarations.color {
        prop_set(rule, keys::UI_STYLE_COLOR, color as u64).map_err(crate::errors::Error::Errno)?;
    }
    if let Some(background) = declarations.background {
        prop_set(rule, keys::UI_STYLE_BACKGROUND, background as u64)
            .map_err(crate::errors::Error::Errno)?;
    }
    if let Some(name) = declarations.font_name {
        set_string_prop(rule, keys::UI_STYLE_FONT_NAME, name)?;
    }
    if let Some(size) = declarations.font_size_px {
        prop_set(rule, keys::UI_STYLE_FONT_SIZE, size).map_err(crate::errors::Error::Errno)?;
    }
    if let Some(padding) = declarations.padding_px {
        prop_set(rule, keys::UI_STYLE_PADDING, padding).map_err(crate::errors::Error::Errno)?;
    }
    if let Some(gap) = declarations.gap_px {
        prop_set(rule, keys::UI_STYLE_GAP, gap).map_err(crate::errors::Error::Errno)?;
    }
    if let Some(border_width) = declarations.border_width_px {
        prop_set(rule, keys::UI_STYLE_BORDER_WIDTH, border_width)
            .map_err(crate::errors::Error::Errno)?;
    }
    if let Some(border_color) = declarations.border_color {
        prop_set(rule, keys::UI_STYLE_BORDER_COLOR, border_color as u64)
            .map_err(crate::errors::Error::Errno)?;
    }
    if let Some(min_w) = declarations.min_width_px {
        prop_set(rule, keys::UI_STYLE_MIN_WIDTH, min_w).map_err(crate::errors::Error::Errno)?;
    }
    if let Some(min_h) = declarations.min_height_px {
        prop_set(rule, keys::UI_STYLE_MIN_HEIGHT, min_h).map_err(crate::errors::Error::Errno)?;
    }
    if let Some(cursor_color) = declarations.cursor_color {
        prop_set(rule, keys::UI_STYLE_CURSOR_COLOR, cursor_color as u64)
            .map_err(crate::errors::Error::Errno)?;
    }

    Ok(rule)
}

pub fn set_node_classes(node: ThingId, classes: &[&str]) -> Result<()> {
    if classes.is_empty() {
        return prop_set(node, keys::UI_CLASS, 0).map_err(crate::errors::Error::Errno);
    }
    let joined = classes
        .iter()
        .filter(|c| !c.is_empty())
        .fold(String::new(), |mut acc, class| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(class);
            acc
        });
    if joined.is_empty() {
        return prop_set(node, keys::UI_CLASS, 0).map_err(crate::errors::Error::Errno);
    }
    set_string_prop(node, keys::UI_CLASS, &joined)
}

fn set_string_prop(id: ThingId, key_name: &str, value: &str) -> Result<()> {
    if value.is_empty() {
        return prop_set(id, key_name, 0).map_err(crate::errors::Error::Errno);
    }
    let bs_id = bytespace_create(value.len(), 0, 0).map_err(crate::errors::Error::Errno)?;
    bytespace_write(bs_id, 0, value.as_bytes())
        .map(|_| ())
        .map_err(crate::errors::Error::Errno)?;
    prop_set(id, key_name, bs_id.to_u64_lossy()).map_err(crate::errors::Error::Errno)
}
