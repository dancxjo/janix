//! Path tessellation and flattening
//!
//! Converts VIR paths with curves into flat polygons suitable for rasterization.
//! This module handles:
//! - Curve flattening (quadratic and cubic beziers → line segments)
//! - Stroke expansion (converting strokes to filled outlines)
//! - Fill rule application

use crate::vir::{VirPath, VirPoint, VirSegment, VirTransform, StrokeStyle, LineCap, LineJoin};
use alloc::vec::Vec;

pub mod flatten;
pub mod stroke;

pub use flatten::*;
pub use stroke::*;

/// Tessellation configuration
#[derive(Debug, Clone, Copy)]
pub struct TessellateConfig {
    /// Tolerance for curve flattening in internal units
    /// Smaller = more segments = higher quality
    pub tolerance: f32,
    
    /// Whether to apply transform before tessellation
    pub apply_transform: bool,
}

impl Default for TessellateConfig {
    fn default() -> Self {
        Self {
            tolerance: 0.25, // Default tolerance
            apply_transform: true,
        }
    }
}

impl TessellateConfig {
    /// Adjust tolerance based on transform scale
    pub fn adjust_for_transform(&self, transform: &VirTransform) -> Self {
        let scale = transform.max_scale();
        Self {
            tolerance: self.tolerance / scale as f32,
            apply_transform: self.apply_transform,
        }
    }
}

/// A tessellated path (all curves flattened to line segments)
#[derive(Debug, Clone)]
pub struct TessellatedPath {
    /// Flattened vertices
    pub vertices: Vec<VirPoint>,
    /// Contour boundaries (indices into vertices)
    /// Each contour is a closed loop
    pub contours: Vec<Contour>,
}

/// A single closed contour
#[derive(Debug, Clone)]
pub struct Contour {
    /// Start index in vertices array
    pub start: usize,
    /// Number of vertices in this contour
    pub count: usize,
}

impl TessellatedPath {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            contours: Vec::new(),
        }
    }
}

impl Default for TessellatedPath {
    fn default() -> Self {
        Self::new()
    }
}

/// Tessellate a VIR path for filling
pub fn tessellate_fill(
    path: &VirPath,
    transform: &VirTransform,
    config: &TessellateConfig,
) -> TessellatedPath {
    let adjusted_config = config.adjust_for_transform(transform);
    let mut result = TessellatedPath::new();
    
    let mut current_contour_start = 0;
    let mut contour_has_points = false;
    let mut current_point = VirPoint::new(0.0, 0.0);
    
    for segment in &path.segments {
        match segment {
            VirSegment::MoveTo(p) => {
                // Close previous contour if it has points
                if contour_has_points {
                    let count = result.vertices.len() - current_contour_start;
                    result.contours.push(Contour {
                        start: current_contour_start,
                        count,
                    });
                }
                
                // Start new contour
                current_contour_start = result.vertices.len();
                let tp = if adjusted_config.apply_transform {
                    transform.transform_point(*p)
                } else {
                    *p
                };
                result.vertices.push(tp);
                current_point = tp;
                contour_has_points = true;
            }
            VirSegment::LineTo(p) => {
                let tp = if adjusted_config.apply_transform {
                    transform.transform_point(*p)
                } else {
                    *p
                };
                result.vertices.push(tp);
                current_point = tp;
            }
            VirSegment::QuadTo(cp, p) => {
                // Flatten quadratic bezier
                let tcp = if adjusted_config.apply_transform {
                    transform.transform_point(*cp)
                } else {
                    *cp
                };
                let tp = if adjusted_config.apply_transform {
                    transform.transform_point(*p)
                } else {
                    *p
                };
                
                flatten_quad(
                    current_point,
                    tcp,
                    tp,
                    adjusted_config.tolerance,
                    &mut result.vertices,
                );
                current_point = tp;
            }
            VirSegment::CubicTo(cp1, cp2, p) => {
                // Flatten cubic bezier
                let tcp1 = if adjusted_config.apply_transform {
                    transform.transform_point(*cp1)
                } else {
                    *cp1
                };
                let tcp2 = if adjusted_config.apply_transform {
                    transform.transform_point(*cp2)
                } else {
                    *cp2
                };
                let tp = if adjusted_config.apply_transform {
                    transform.transform_point(*p)
                } else {
                    *p
                };
                
                flatten_cubic(
                    current_point,
                    tcp1,
                    tcp2,
                    tp,
                    adjusted_config.tolerance,
                    &mut result.vertices,
                );
                current_point = tp;
            }
            VirSegment::Close => {
                // Implicitly close the contour
                // The rasterizer will handle the closing
            }
        }
    }
    
    // Close final contour if needed
    if contour_has_points {
        let count = result.vertices.len() - current_contour_start;
        result.contours.push(Contour {
            start: current_contour_start,
            count,
        });
    }
    
    result
}

/// Tessellate a VIR path for stroking
/// Returns a filled path representing the stroke outline
pub fn tessellate_stroke(
    path: &VirPath,
    transform: &VirTransform,
    stroke: &StrokeStyle,
    config: &TessellateConfig,
) -> TessellatedPath {
    let adjusted_config = config.adjust_for_transform(transform);
    
    // First, flatten the path
    let flattened = tessellate_fill(path, transform, &adjusted_config);
    
    // Then expand to stroke outline
    expand_stroke(&flattened, stroke, &adjusted_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vir::VirPath;

    #[test]
    fn test_tessellate_simple_rect() {
        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 0.0);
        path.line_to(10.0, 10.0);
        path.line_to(0.0, 10.0);
        path.close();
        
        let transform = VirTransform::identity();
        let config = TessellateConfig::default();
        
        let result = tessellate_fill(&path, &transform, &config);
        
        assert_eq!(result.contours.len(), 1);
        assert_eq!(result.vertices.len(), 4);
    }

    #[test]
    fn test_tessellate_with_transform() {
        let mut path = VirPath::new();
        path.move_to(0.0, 0.0);
        path.line_to(10.0, 0.0);
        
        let transform = VirTransform::scale(2.0, 2.0);
        let config = TessellateConfig::default();
        
        let result = tessellate_fill(&path, &transform, &config);
        
        // Check that transform was applied
        assert_eq!(result.vertices[1].x, 20.0); // 10 * 2
        assert_eq!(result.vertices[1].y, 0.0);
    }
}
