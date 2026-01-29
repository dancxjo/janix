//! Core VIR types for vector graphics

use serde::{Deserialize, Serialize};
use alloc::vec::Vec;

/// A 2D point with floating point coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct VirPoint {
    pub x: f32,
    pub y: f32,
}

impl VirPoint {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

/// A path segment (verb + data)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum VirSegment {
    /// Move to a point (start new contour)
    MoveTo(VirPoint),
    /// Line to a point
    LineTo(VirPoint),
    /// Quadratic bezier curve (control point, end point)
    QuadTo(VirPoint, VirPoint),
    /// Cubic bezier curve (control point 1, control point 2, end point)
    CubicTo(VirPoint, VirPoint, VirPoint),
    /// Close current contour
    Close,
}

/// A complete path consisting of multiple segments
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VirPath {
    pub segments: Vec<VirSegment>,
}

impl VirPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn move_to(&mut self, x: f32, y: f32) {
        self.segments.push(VirSegment::MoveTo(VirPoint::new(x, y)));
    }

    pub fn line_to(&mut self, x: f32, y: f32) {
        self.segments.push(VirSegment::LineTo(VirPoint::new(x, y)));
    }

    pub fn quad_to(&mut self, cx: f32, cy: f32, x: f32, y: f32) {
        self.segments.push(VirSegment::QuadTo(
            VirPoint::new(cx, cy),
            VirPoint::new(x, y),
        ));
    }

    pub fn cubic_to(&mut self, c1x: f32, c1y: f32, c2x: f32, c2y: f32, x: f32, y: f32) {
        self.segments.push(VirSegment::CubicTo(
            VirPoint::new(c1x, c1y),
            VirPoint::new(c2x, c2y),
            VirPoint::new(x, y),
        ));
    }

    pub fn close(&mut self) {
        self.segments.push(VirSegment::Close);
    }
}

impl Default for VirPath {
    fn default() -> Self {
        Self::new()
    }
}

/// Color with RGBA channels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl VirColor {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const BLACK: Self = Self::rgb(0, 0, 0);
    pub const WHITE: Self = Self::rgb(255, 255, 255);
}

/// Paint types (currently only solid colors)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Paint {
    Solid(VirColor),
}

/// Fill rule for path filling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FillRule {
    /// Non-zero winding rule
    NonZero,
    /// Even-odd rule
    EvenOdd,
}

/// Fill style (paint + rule)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FillStyle {
    pub paint: Paint,
    pub rule: FillRule,
}

/// Line cap style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineCap {
    /// Flat cap at endpoint
    Butt,
    /// Rounded cap extending beyond endpoint
    Round,
    /// Square cap extending beyond endpoint
    Square,
}

/// Line join style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineJoin {
    /// Sharp corner (limited by miter_limit)
    Miter,
    /// Rounded corner
    Round,
    /// Beveled corner
    Bevel,
}

/// Stroke style
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StrokeStyle {
    pub paint: Paint,
    pub width: f32,
    pub line_cap: LineCap,
    pub line_join: LineJoin,
    pub miter_limit: f32,
}

impl StrokeStyle {
    pub fn new(paint: Paint, width: f32) -> Self {
        Self {
            paint,
            width,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            miter_limit: 4.0,
        }
    }
}

/// ViewBox defining the coordinate system
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ViewBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl ViewBox {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
}
