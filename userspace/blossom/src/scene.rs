extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;

use abi::ui_scene::{
    CheckboxMeta, EdgeInsets, FlexMeta, IconMeta, ImageMeta, LineMeta, NodeKind, RectMeta,
    SizeSpec, StringRef, TextMeta, UiScene, WindowMeta,
};

#[derive(Clone, Debug)]
pub struct SceneGraph {
    pub nodes: Vec<SceneNode>,
    pub(crate) strings: Vec<u8>,
    pub root: usize,
}

impl SceneGraph {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, abi::ui_scene::DecodeError> {
        let scene = UiScene::decode(bytes)?;
        let count = scene.node_count();
        let mut nodes: Vec<SceneNode> = Vec::with_capacity(count);
        let mut id_to_index: Vec<usize> = vec![usize::MAX; count.max(1)];

        for i in 0..count {
            let view = scene.node(i).ok_or(abi::ui_scene::DecodeError::Truncated)?;
            let id = view.id() as usize;
            if id < id_to_index.len() {
                id_to_index[id] = i;
            }
            let min_w = normalize_bound(view.min_width());
            let min_h = normalize_bound(view.min_height());
            let max_w = normalize_bound(view.max_width());
            let max_h = normalize_bound(view.max_height());
            nodes.push(SceneNode {
                id: view.id(),
                parent: view.parent().map(|p| p as usize),
                children: Vec::new(),
                kind: view.kind(),
                width: view.width(),
                height: view.height(),
                flex_basis: view.flex_basis(),
                min_width: min_w,
                min_height: min_h,
                max_width: max_w,
                max_height: max_h,
                margin: view.margin(),
                padding: view.padding(),
                flex_grow: view.flex_grow(),
                flex_shrink: view.flex_shrink(),
                window_meta: view.window_meta(),
                flex_meta: view.flex_meta(),
                text_meta: view.text_meta(),
                rect_meta: view.rect_meta(),
                image_meta: view.image_meta(),
                line_meta: view.line_meta(),
                icon_meta: view.icon_meta(),
                checkbox_meta: view.checkbox_meta(),
            });
        }

        for idx in 0..nodes.len() {
            if let Some(parent_id) = nodes[idx].parent {
                if parent_id < id_to_index.len() {
                    let parent_index = id_to_index[parent_id];
                    if parent_index != usize::MAX {
                        nodes[parent_index].children.push(idx);
                    }
                }
            }
        }

        let root = nodes
            .iter()
            .position(|node| node.parent.is_none())
            .unwrap_or(0);

        Ok(Self {
            nodes,
            strings: scene.strings_bytes().to_vec(),
            root,
        })
    }

    pub fn string(&self, r: StringRef) -> Option<&str> {
        if r.len == 0 {
            return Some("");
        }
        let start = r.offset as usize;
        let end = start + r.len as usize;
        if end > self.strings.len() {
            return None;
        }
        core::str::from_utf8(&self.strings[start..end]).ok()
    }
}

fn normalize_bound(value: i32) -> Option<i32> {
    if value < 0 {
        None
    } else {
        Some(value)
    }
}

#[derive(Clone, Debug)]
pub struct SceneNode {
    pub id: u32,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub kind: NodeKind,
    pub width: SizeSpec,
    pub height: SizeSpec,
    pub flex_basis: SizeSpec,
    pub min_width: Option<i32>,
    pub min_height: Option<i32>,
    pub max_width: Option<i32>,
    pub max_height: Option<i32>,
    pub margin: EdgeInsets,
    pub padding: EdgeInsets,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub window_meta: Option<WindowMeta>,
    pub flex_meta: Option<FlexMeta>,
    pub text_meta: Option<TextMeta>,
    pub rect_meta: Option<RectMeta>,
    pub image_meta: Option<ImageMeta>,
    pub line_meta: Option<LineMeta>,
    pub icon_meta: Option<IconMeta>,
    pub checkbox_meta: Option<CheckboxMeta>,
}
