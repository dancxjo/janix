use alloc::string::String;
use alloc::vec::Vec;
use alloc::sync::Arc;
use crate::asset::Image;

use crate::damage::Rect;
use crate::geometry::Color;
use crate::ui::constants::{
    SHADE_BUTTON_PADDING, SHADE_BUTTON_SIZE, TITLE_BAR_HEIGHT, TITLE_BAR_ICON_SIZE,
    TITLE_BAR_PADDING,
};
use crate::ui::layout::{LayoutNode, LayoutTree, SymbolResolver};
use crate::ui::snapshot::{UiNodeKind, UiNodeSnapshot, UiSnapshot};
use abi::schema::keys;
use crate::render_state::{RenderState, RasterKey};
use stem::thing::ThingId;

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
    Raster {
        rect: Rect,
        image: Arc<Image>,
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
    pub fn build(
        snapshot: &UiSnapshot,
        layout: &LayoutTree,
        symbols: &impl SymbolResolver,
        render_state: &mut RenderState,
    ) -> PaintScene {
        let mut objects = Vec::new();
        let active_window = layout
            .root
            .as_ref()
            .and_then(Self::find_active_window);
        if let Some(root) = &layout.root {
            Self::build_recursive(snapshot, root, &mut objects, symbols, render_state, active_window);
        }

        let now_ms = crate::log_ratelimit::now_ms();
        if crate::log_ratelimit::log_every(1000, now_ms) {
            let text_count = objects
                .iter()
                .filter(|o| matches!(o, PaintObject::Text { .. }))
                .count();
            crate::log!("[bloom][paint] objs={} text={}", objects.len(), text_count);
        }

        PaintScene { objects }
    }

    fn build_recursive(
        snapshot: &UiSnapshot,
        layout_node: &LayoutNode,
        objects: &mut Vec<PaintObject>,
        symbols: &impl SymbolResolver,
        render_state: &mut RenderState,
        active_window: Option<ThingId>,
    ) {
        // Scope all drawing to this node's bounds before emitting content or children.
        objects.push(PaintObject::PushClip {
            rect: layout_node.rect.clone(),
        });

        if let Some(node_snapshot) = snapshot.nodes.get(&layout_node.id) {
            // Create paint object based on kind and properties
            Self::create_paint_objects(
                node_snapshot,
                layout_node,
                objects,
                symbols,
                render_state,
                active_window,
            );

            for child in &layout_node.children {
                Self::build_recursive(snapshot, child, objects, symbols, render_state, active_window);
            }
        }

        objects.push(PaintObject::PopClip);
    }

    fn create_paint_objects(
        node: &UiNodeSnapshot,
        layout: &LayoutNode,
        objects: &mut Vec<PaintObject>,
        symbols: &impl SymbolResolver,
        render_state: &mut RenderState,
        active_window: Option<ThingId>,
    ) {
        // Window special handling
        if node.kind == UiNodeKind::Window {
            // 1. Background
            let mut bg_color = Self::get_prop(node, keys::UI_BG_COLOR, symbols);
            if bg_color == 0 {
                bg_color = 0xFFF5F5F0;
            } // Default off-white if not set

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
                let bar_rect = Rect::new(layout.rect.x, layout.rect.y, layout.rect.w, title_h);
                let is_active = active_window.map(|id| id == layout.id).unwrap_or(false);
                let bar_color = if is_active { 0xFF2E7FD1 } else { 0xFF5A5A5A };
                objects.push(PaintObject::Rect {
                    rect: bar_rect,
                    color: Color::from_u32(bar_color),
                    radius: 0,
                });

                // Icon Background
                let icon_bg_rect = Rect::new(
                    layout.rect.x + icon_padding,
                    layout.rect.y + icon_padding,
                    icon_size,
                    icon_size,
                );
                objects.push(PaintObject::Rect {
                    rect: icon_bg_rect.clone(),
                    color: Color::from_u32(0xFF445566), // Slate blue/grey background
                    radius: 4,
                });

                // Icon
                let icon_rect = Rect::new(
                    layout.rect.x + icon_padding,
                    layout.rect.y + icon_padding,
                    icon_size,
                    icon_size,
                );
                if let Some(icon_cmds) = &node.window_icon_content {
                    objects.push(PaintObject::Commands {
                        cmds: icon_cmds.clone(),
                        rect: icon_rect,
                    });
                }

                let text_offset_x = icon_size + (icon_padding * 2);

                // Shade button
                let shade_x =
                    layout.rect.x + layout.rect.w - SHADE_BUTTON_PADDING - SHADE_BUTTON_SIZE;
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
                        rect: Rect::new(
                            layout.rect.x + text_offset_x,
                            layout.rect.y + 2,
                            text_w,
                            title_h,
                        ),
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
            if mode == 1 {
                // Svg
                if let Some(cmds) = &node.svg_content {
                    let w = layout.rect.w as u32;
                    let h = layout.rect.h as u32;
                    if w > 0 && h > 0 {
                        let content_hash =  cmds.as_ptr() as *const () as u64;
                        let key = RasterKey::Svg { node_id: node.id.to_u64_lossy(), w, h, content_hash };
                        
                        if let Some(image) = render_state.raster_cache.get(&key) {
                            crate::perf::add_counter("paint.svg.hit", 1);
                            objects.push(PaintObject::Raster {
                                rect: layout.rect.clone(),
                                image: image.clone(),
                            });
                            return;
                        }

                        crate::perf::add_counter("paint.svg.miss", 1);
                        // Rasterize
                        let stride = w as usize * 4;
                        let len = stride * h as usize;
                        let mut pixels = alloc::vec![0u32; len / 4];
                        
                        let mut surf = unsafe {
                            crate::surface::Surface::new(
                                pixels.as_mut_ptr() as *mut u8,
                                len,
                                w,
                                h,
                                stride as u32,
                            )
                        };
                        
                        let mut list = crate::drawlist::DrawList::new();
                        // Transform to 0,0 since we are rasterizing into a surface of exactly the node size
                        // The paint logic in 'lower' handles translation of the Raster to layout.x/y
                        // But wait, the original Objects::Commands logic pushed a translation of (rect.x, rect.y).
                        // If we rasterize 0..w, we don't need translation in the drawlist, we just draw at 0,0.
                        // BUT SVG commands might have their own coordinates?
                        // UiNodeKind::Inline SVG handling in `lower` (before my change) did:
                        // list.commands().push(DrawCmd::PushTransform { transform: Transform::translate(rect.x, rect.y) });
                        // Then appended commands.
                        // So the commands are local to 0,0? Or local to where they were defined?
                        // Usually SVG parser produces coords starting at 0,0.
                        // So if we rasterize into a w*h surface, we don't need any translation *if* the SVG fits in 0,0..w,h.
                        // The `rect` in `lower` translated them to the layout position on screen.
                        // So here we should NOT translate. We just draw them.
                        // The `Raster` object itself has `rect: layout.rect`. 
                        // In `lower` (raster handling), we call `list.blit_image(image, rect.x, rect.y)`.
                        // `blit_image` creates `DrawCmd::DrawImage` with `dest = Rect(x, y, w, h)`.
                        // So yes, we rasterize locally at 0,0.
                        
                        list.commands().extend(cmds.iter().cloned());
                        crate::raster::execute(&mut surf, &list, false);
                        
                        let image = Arc::new(Image {
                            width: w,
                            height: h,
                            pixels: Arc::from(pixels),
                            gen: crate::frame::AssetGeneration(0),
                        });
                        
                        render_state.raster_cache.insert(key, image.clone());
                        objects.push(PaintObject::Raster {
                            rect: layout.rect.clone(),
                            image,
                        });
                        return;
                    }

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

            // Phase 1: Raster Cache for Text
            let w = layout.rect.w as u32;
            let h = layout.rect.h as u32;
            
            // Simple DJB2-ish hash for text content and properties
            let mut hasher = 5381u64;
            for b in text.as_bytes() { hasher = ((hasher << 5).wrapping_add(hasher)).wrapping_add(*b as u64); }
            for b in font.as_bytes() { hasher = ((hasher << 5).wrapping_add(hasher)).wrapping_add(*b as u64); }
            // hash size (f32 bits), color (u32), font_debug (bool)
            hasher = ((hasher << 5).wrapping_add(hasher)).wrapping_add(size.to_bits() as u64);
            hasher = ((hasher << 5).wrapping_add(hasher)).wrapping_add(color.to_u32() as u64);
            hasher = ((hasher << 5).wrapping_add(hasher)).wrapping_add(if font_debug { 1 } else { 0 });

            if w > 0 && h > 0 {
                let key = RasterKey::Text { node_id: node.id.to_u64_lossy(), w, h, content_hash: hasher };
                if let Some(image) = render_state.raster_cache.get(&key) {
                     crate::perf::add_counter("paint.text.hit", 1);
                     objects.push(PaintObject::Raster {
                        rect: layout.rect.clone(),
                        image: image.clone(),
                    });
                    return;
                }
                
                crate::perf::add_counter("paint.text.miss", 1);
                // Rasterize Text
                 let stride = w as usize * 4;
                 let len = stride * h as usize;
                 let mut pixels = alloc::vec![0u32; len / 4];
                 
                 let mut surf = unsafe {
                    crate::surface::Surface::new(
                        pixels.as_mut_ptr() as *mut u8,
                        len,
                        w,
                        h,
                        stride as u32,
                    )
                 };
                 
                 let mut list = crate::drawlist::DrawList::new();
                 // Draw text at 0,0 locally
                 list.text_font_debug(&text, &font, 0, 0, size, color, font_debug);
                 
                 crate::raster::execute(&mut surf, &list, false);
                 
                 let image = Arc::new(Image {
                    width: w,
                    height: h,
                    pixels: Arc::from(pixels),
                    gen: crate::frame::AssetGeneration(0),
                 });
                 
                 render_state.raster_cache.insert(key, image.clone());
                 objects.push(PaintObject::Raster {
                    rect: layout.rect.clone(),
                    image,
                 });
                 return;
            }

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

    fn get_str_prop(
        node: &UiNodeSnapshot,
        key: &str,
        symbols: &impl SymbolResolver,
    ) -> Option<String> {
        if let Some(id) = symbols.resolve(key) {
            return node.strings.get(&id).cloned();
        }
        None
    }

    fn find_active_window(root: &LayoutNode) -> Option<ThingId> {
        let mut best: Option<(ThingId, i32, usize)> = None;
        let mut order = 0usize;
        Self::find_active_window_recursive(root, &mut order, &mut best);
        best.map(|(id, _, _)| id)
    }

    fn find_active_window_recursive(
        node: &LayoutNode,
        order: &mut usize,
        best: &mut Option<(ThingId, i32, usize)>,
    ) {
        if node.kind == UiNodeKind::Window {
            let current_order = *order;
            let should_replace = match best {
                None => true,
                Some((_, best_z, best_order)) => {
                    node.z_index > *best_z || (node.z_index == *best_z && current_order > *best_order)
                }
            };
            if should_replace {
                *best = Some((node.id, node.z_index, current_order));
            }
            *order = order.saturating_add(1);
        }

        for child in &node.children {
            Self::find_active_window_recursive(child, order, best);
        }
    }
}
