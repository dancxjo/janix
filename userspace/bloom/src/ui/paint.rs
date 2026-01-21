use alloc::vec::Vec;
use alloc::string::String;

use crate::geometry::Color;
use crate::damage::Rect;
use crate::ui::constants::{SHADE_BUTTON_PADDING, SHADE_BUTTON_SIZE, TITLE_BAR_HEIGHT, TITLE_BAR_ICON_SIZE, TITLE_BAR_PADDING};
use crate::ui::layout::{LayoutTree, LayoutNode, SymbolResolver};
use crate::ui::snapshot::{UiSnapshot, UiNodeSnapshot, UiNodeKind};
use abi::schema::keys;
use abi::ids::HandleId; // Need HandleId for ThingId::from (Wait, paint.rs uses abi::WireType::ThingId)
use abi::WireType::ThingId;

#[derive(Debug, Clone, PartialEq)]
pub enum PaintObject {
    PushClip {
        rect: Rect,
    },
    PopClip,
    Rect {
        rect: Rect,
        color: Color,
        radius: u32,
    },
    Text {
        rect: Rect,
        text: String,
        font: String,
        size: f32,
        color: Color,
        font_debug: bool,
    },
    Image {
        rect: Rect,
    },
    Commands {
        cmds: alloc::sync::Arc<alloc::vec::Vec<crate::drawlist::DrawCmd>>,
        rect: Rect,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaintScene {
    pub objects: Vec<PaintObject>,
}

pub struct PaintBuilder;

impl PaintBuilder {
    pub fn build(snapshot: &UiSnapshot, layout: &LayoutTree, symbols: &impl SymbolResolver) -> PaintScene {
        let mut objects = Vec::new();
        if let Some(root) = &layout.root {
            Self::build_recursive(snapshot, root, &mut objects, symbols);
        }
        
        let now_ms = crate::log_ratelimit::now_ms();
        if crate::log_ratelimit::log_every(1000, now_ms) {
            let text_count = objects.iter().filter(|o| matches!(o, PaintObject::Text { .. })).count();
            crate::log!("[bloom][paint] objs={} text={}", objects.len(), text_count);
        }
        
        PaintScene { objects }
    }

    fn build_recursive(snapshot: &UiSnapshot, layout_node: &LayoutNode, objects: &mut Vec<PaintObject>, symbols: &impl SymbolResolver) {
        // Scope all drawing to this node's bounds before emitting content or children.
        objects.push(PaintObject::PushClip {
            rect: layout_node.rect.clone(),
        });

        if let Some(node_snapshot) = snapshot.nodes.get(&layout_node.id) {
            // Create paint object based on kind and properties
            Self::create_paint_objects(node_snapshot, layout_node, objects, symbols);

            for child in &layout_node.children {
                Self::build_recursive(snapshot, child, objects, symbols);
            }
        }

        objects.push(PaintObject::PopClip);
    }

    fn create_paint_objects(node: &UiNodeSnapshot, layout: &LayoutNode, objects: &mut Vec<PaintObject>, symbols: &impl SymbolResolver) {
        // Window special handling
        if node.kind == UiNodeKind::Window {
            // 1. Background
            let mut bg_color = Self::get_prop(node, keys::UI_BG_COLOR, symbols);
            if bg_color == 0 { bg_color = 0xFF000000; } // Default black if not set
            
            objects.push(PaintObject::Rect {
                rect: layout.rect.clone(),
                color: Color::from_u32(bg_color as u32),
                radius: Self::get_prop(node, keys::UI_RADIUS, symbols) as u32,
            });

            // 2. Title Bar logic
            let title_h = TITLE_BAR_HEIGHT;
            let is_shaded = Self::get_prop(node, keys::UI_WINDOW_SHADED, symbols) != 0;
            if layout.rect.h >= title_h {
                // Determine icon area
                let icon_size = TITLE_BAR_ICON_SIZE;
                let icon_padding = TITLE_BAR_PADDING;
                let _bar_rect = Rect::new(layout.rect.x, layout.rect.y, layout.rect.w, title_h);
                
                // Icon Background
                let icon_bg_rect = Rect::new(layout.rect.x + icon_padding, layout.rect.y + icon_padding, icon_size, icon_size);
                objects.push(PaintObject::Rect {
                    rect: icon_bg_rect.clone(),
                    color: Color::from_u32(0xFF445566), // Slate blue/grey background
                    radius: 4,
                });

                // Icon
                let icon_rect = Rect::new(layout.rect.x + icon_padding, layout.rect.y + icon_padding, icon_size, icon_size);
                if let Some(icon_cmds) = &node.window_icon_content {
                     objects.push(PaintObject::Commands {
                         cmds: icon_cmds.clone(),
                         rect: icon_rect,
                     });
                }
                
                let text_offset_x = icon_size + (icon_padding * 2);

                // Shade button
                let shade_x = layout.rect.x + layout.rect.w - SHADE_BUTTON_PADDING - SHADE_BUTTON_SIZE;
                let shade_y = layout.rect.y + (title_h - SHADE_BUTTON_SIZE) / 2;
                let shade_rect = Rect::new(shade_x, shade_y, SHADE_BUTTON_SIZE, SHADE_BUTTON_SIZE);
                let shade_bg = if is_shaded { 0xFF2F3C4A } else { 0xFF4A5B6C };
                objects.push(PaintObject::Rect {
                    rect: shade_rect.clone(),
                    color: Color::from_u32(shade_bg),
                    radius: 6,
                });
                let shade_glyph = if is_shaded { "v" } else { "^" };
                objects.push(PaintObject::Text {
                    rect: shade_rect.clone(),
                    text: shade_glyph.into(),
                    font: "NotoSans-Regular.ttf".into(),
                    size: 18.0,
                    color: Color::from_u32(0xFFFFFFFF),
                    font_debug: false,
                });

                // Title Text
                let title = Self::get_str_prop(node, keys::UI_TITLE, symbols);
                if let Some(t) = title {
                    let text_w = (shade_x - (layout.rect.x + text_offset_x)).max(0);
                    objects.push(PaintObject::Text {
                        rect: Rect::new(layout.rect.x + text_offset_x, layout.rect.y + 2, text_w, title_h),
                        text: t,
                        font: "NotoSans-Regular.ttf".into(),
                        size: 14.0,
                        color: Color::from_u32(0xFFFFFFFF),
                        font_debug: false,
                    });
                }
            }
                

            return;
        }

        // UI_INLINE special handling
        if node.kind == UiNodeKind::Inline {
             let mode = Self::get_prop(node, keys::UI_INLINE_MODE, symbols);
             if mode == 1 { // Svg
                 if let Some(cmds) = &node.svg_content {
                     objects.push(PaintObject::Commands {
                         cmds: cmds.clone(),
                         rect: layout.rect.clone(),
                     });
                     return;
                 } else {
                     // Fallback: Red Box
                     objects.push(PaintObject::Rect {
                         rect: layout.rect.clone(),
                         color: Color::new(255, 0, 0, 255),
                         radius: 0,
                     });
                     return;
                 }
             }
             // If mode == 0, fallthrough to Text logic
        }

        // Check for UI_TEXT using symbol resolver
        let text_key_id = symbols.resolve(keys::UI_TEXT);
        let has_text = if let Some(id) = text_key_id {
            // V0: check BOTH props (raw) and strings (parsed)
            node.props.contains_key(&id) || node.strings.contains_key(&id)
        } else {
            false
        };

        if has_text {
            let text = Self::get_str_prop(node, keys::UI_TEXT, symbols).unwrap_or_default();
            let font_stack = Self::get_str_prop(node, keys::UI_FONT_STACK, symbols);
            let font = font_stack
                .or_else(|| Self::get_str_prop(node, keys::UI_FONT, symbols))
                .unwrap_or_else(|| "Noto Sans".into());
            let size = Self::get_prop(node, keys::UI_FONT_SIZE, symbols) as f32;
            let font_debug = Self::get_prop(node, keys::UI_FONT_DEBUG, symbols) != 0;

            let mut color_val = Self::get_prop(node, keys::UI_FG_COLOR, symbols);
            if color_val == 0 {
                color_val = Self::get_prop(node, keys::UI_COLOR, symbols);
            }
            // Use white by default for text if no color specified
            if color_val == 0 {
                color_val = 0xFFFFFFFF;
            }
            let color = Color::from_u32(color_val as u32);
            
            objects.push(PaintObject::Text {
                rect: layout.rect.clone(),
                text,
                font,
                size: if size == 0.0 { 16.0 } else { size },
                color,
                font_debug,
            });
            return;
        }

        // Default to a colored rect if it has dimensions or a color
        let mut color_val = Self::get_prop(node, keys::UI_BG_COLOR, symbols);
        if color_val == 0 {
            color_val = Self::get_prop(node, keys::UI_COLOR, symbols);
        }

        // Only emit Rect if color is non-transparent OR it's been intentionally sized
        // Note: Window nodes with no color shouldn't necessarily emit a transparent black box.
        if color_val != 0 {
            objects.push(PaintObject::Rect {
                rect: layout.rect.clone(),
                color: Color::from_u32(color_val as u32),
                radius: Self::get_prop(node, keys::UI_RADIUS, symbols) as u32,
            });
            return;
        }


    }

    fn get_prop(node: &UiNodeSnapshot, key: &str, symbols: &impl SymbolResolver) -> u64 {
        if let Some(id) = symbols.resolve(key) {
            return *node.props.get(&id).unwrap_or(&0);
        }
        0
    }

    fn get_str_prop(node: &UiNodeSnapshot, key: &str, symbols: &impl SymbolResolver) -> Option<String> {
        if let Some(id) = symbols.resolve(key) {
            return node.strings.get(&id).cloned();
        }
        None
    }
}
