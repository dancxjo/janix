extern crate alloc;

use alloc::vec::Vec;
use thing_os::prelude::ThingId;

use crate::flex::{AlignItems, FlexDirection, FlexWrap, JustifyContent};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: u32,
    pub h: u32,
}
impl Rect {
    pub const fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutSpec {
    Stack,
    Grid {
        rows: usize,
        cols: usize,
    },
    Flex {
        direction: FlexDirection,
        wrap: FlexWrap,
        justify: JustifyContent,
        align: AlignItems,
    },
}

#[derive(Clone, Debug)]
pub struct LayoutItem {
    pub id: ThingId,
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub flex_grow: f32,
    pub flex_shrink: f32,
}

impl Default for LayoutItem {
    fn default() -> Self {
        Self {
            id: ThingId(0),
            min_width: 0,
            min_height: 0,
            max_width: None,
            max_height: None,
            flex_grow: 0.0,
            flex_shrink: 1.0,
        }
    }
}

pub fn layout(
    container: Rect,
    spec: LayoutSpec,
    children: &[LayoutItem],
    gap: i32,
) -> Vec<(ThingId, Rect)> {
    let mut result = Vec::new();
    if children.is_empty() {
        return result;
    }

    match spec {
        LayoutSpec::Stack => {
            for child in children {
                result.push((child.id, container));
            }
        }

        LayoutSpec::Grid { rows, cols } => {
            let rows = (rows.max(1)) as u32;
            let cols = (cols.max(1)) as u32;

            let total_gap_x = (cols.saturating_sub(1) as i32) * gap;
            let total_gap_y = (rows.saturating_sub(1) as i32) * gap;

            let avail_w = (container.w as i32 - total_gap_x).max(0) as u32;
            let avail_h = (container.h as i32 - total_gap_y).max(0) as u32;

            let cell_w = avail_w / cols;
            let cell_h = avail_h / rows;

            for (i, child) in children.iter().enumerate() {
                let row = (i as u32) / cols;
                let col = (i as u32) % cols;
                if row >= rows {
                    break;
                }

                let x = container.x + (col as i32 * (cell_w as i32 + gap));
                let y = container.y + (row as i32 * (cell_h as i32 + gap));

                let mut w = if col == cols - 1 {
                    container.w - (cell_w * (cols - 1)) - (total_gap_x as u32)
                } else {
                    cell_w
                };

                let mut h = if row == rows - 1 {
                    container.h - (cell_h * (rows - 1)) - (total_gap_y as u32)
                } else {
                    cell_h
                };

                if let Some(max_w) = child.max_width {
                    w = w.min(max_w);
                }
                if let Some(max_h) = child.max_height {
                    h = h.min(max_h);
                }

                result.push((child.id, Rect::new(x, y, w, h)));
            }
        }

        LayoutSpec::Flex {
            direction,
            wrap,
            justify,
            align,
        } => {
            let is_row = direction == FlexDirection::Row;
            let main_size = if is_row { container.w } else { container.h };

            // 1) break into lines
            let mut lines: Vec<Vec<&LayoutItem>> = Vec::new();
            if wrap == FlexWrap::NoWrap {
                lines.push(children.iter().collect());
            } else {
                let mut current = Vec::new();
                let mut used: u32 = 0;

                for child in children {
                    let basis = if is_row {
                        child.min_width
                    } else {
                        child.min_height
                    };
                    let gap_needed = if current.is_empty() { 0 } else { gap as u32 };

                    if !current.is_empty() && used + gap_needed + basis > main_size {
                        lines.push(current);
                        current = Vec::new();
                        used = 0;
                    }
                    if !current.is_empty() {
                        used += gap as u32;
                    }
                    used += basis;
                    current.push(child);
                }
                if !current.is_empty() {
                    lines.push(current);
                }
            }

            // 2) layout each line
            let mut cross_pos: u32 = 0;
            for line in lines {
                let count = line.len();
                let total_gap = (count.saturating_sub(1) as i32 * gap).max(0) as u32;

                let mut total_basis: u32 = 0;
                let mut total_grow: f32 = 0.0;
                let mut line_cross: u32 = 0;

                for child in &line {
                    let basis = if is_row {
                        child.min_width
                    } else {
                        child.min_height
                    };
                    let cross = if is_row {
                        child.min_height
                    } else {
                        child.min_width
                    };
                    total_basis += basis;
                    total_grow += child.flex_grow;
                    line_cross = line_cross.max(cross);
                }

                // if nowrap, cross == container cross
                if wrap == FlexWrap::NoWrap {
                    line_cross = if is_row { container.h } else { container.w };
                }

                let available = main_size.saturating_sub(total_basis + total_gap);

                let mut main_pos: u32 = 0;

                if total_grow == 0.0 && available > 0 {
                    match justify {
                        JustifyContent::Start => {}
                        JustifyContent::Center => main_pos = available / 2,
                        JustifyContent::End => main_pos = available,
                        JustifyContent::SpaceBetween => {}
                        JustifyContent::SpaceAround => main_pos = available / (count as u32 * 2),
                    }
                }

                let extra_gap = if total_grow == 0.0
                    && available > 0
                    && justify == JustifyContent::SpaceBetween
                    && count > 1
                {
                    available / (count - 1) as u32
                } else if total_grow == 0.0
                    && available > 0
                    && justify == JustifyContent::SpaceAround
                {
                    available / count as u32
                } else {
                    0
                };

                let effective_gap: u32 = (gap.max(0) as u32) + extra_gap;

                for child in line {
                    let basis = if is_row {
                        child.min_width
                    } else {
                        child.min_height
                    };
                    let grow_share = if total_grow > 0.0 {
                        (available as f32 * (child.flex_grow / total_grow)) as u32
                    } else {
                        0
                    };

                    let mut item_main = basis + grow_share;

                    let mut item_cross = if align == AlignItems::Stretch {
                        line_cross
                    } else if is_row {
                        child.min_height
                    } else {
                        child.min_width
                    };

                    // max constraints
                    if is_row {
                        if let Some(max_w) = child.max_width {
                            item_main = item_main.min(max_w);
                        }
                        if let Some(max_h) = child.max_height {
                            item_cross = item_cross.min(max_h);
                        }
                    } else {
                        if let Some(max_h) = child.max_height {
                            item_main = item_main.min(max_h);
                        }
                        if let Some(max_w) = child.max_width {
                            item_cross = item_cross.min(max_w);
                        }
                    }

                    let cross_offset = match align {
                        AlignItems::Start | AlignItems::Stretch => 0,
                        AlignItems::Center => (line_cross.saturating_sub(item_cross)) / 2,
                        AlignItems::End => line_cross.saturating_sub(item_cross),
                    };

                    let (x, y, w, h) = if is_row {
                        (
                            container.x + main_pos as i32,
                            container.y + cross_pos as i32 + cross_offset as i32,
                            item_main,
                            item_cross,
                        )
                    } else {
                        (
                            container.x + cross_pos as i32 + cross_offset as i32,
                            container.y + main_pos as i32,
                            item_cross,
                            item_main,
                        )
                    };

                    result.push((child.id, Rect::new(x, y, w, h)));

                    main_pos += item_main + effective_gap;
                    if justify == JustifyContent::SpaceAround {
                        main_pos += extra_gap; // second half
                    }
                }

                cross_pos += line_cross + (gap.max(0) as u32);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flex_row_nowrap_positions_children_with_gap() {
        use crate::flex::*;

        let container = Rect::new(0, 0, 100, 20);
        let items = [
            LayoutItem {
                id: ThingId(1),
                min_width: 10,
                min_height: 5,
                ..Default::default()
            },
            LayoutItem {
                id: ThingId(2),
                min_width: 20,
                min_height: 5,
                ..Default::default()
            },
        ];

        let rects = layout(
            container,
            LayoutSpec::Flex {
                direction: FlexDirection::Row,
                wrap: FlexWrap::NoWrap,
                justify: JustifyContent::Start,
                align: AlignItems::Start,
            },
            &items,
            5,
        );

        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].1.x, 0);
        assert_eq!(rects[1].1.x, 10 + 5);
    }

    #[test]
    fn justify_center_offsets_start() {
        use crate::flex::*;

        let container = Rect::new(0, 0, 100, 20);
        let items = [LayoutItem {
            id: ThingId(1),
            min_width: 20,
            min_height: 5,
            ..Default::default()
        }];

        let rects = layout(
            container,
            LayoutSpec::Flex {
                direction: FlexDirection::Row,
                wrap: FlexWrap::NoWrap,
                justify: JustifyContent::Center,
                align: AlignItems::Start,
            },
            &items,
            0,
        );

        assert_eq!(rects[0].1.x, 40); // (100 - 20)/2
    }
}
