use crate::geometry::Transform;

use super::parser::SvgParser;

/// Render SVG string to a raw buffer (ARGB pre-allocation)
/// Useful for asset loading.
///
/// # Examples
/// ```rust,no_run
/// let pixels = crate::svg::render_to_buffer(
///     "<svg width=\"2\" height=\"2\"><rect width=\"2\" height=\"2\" fill=\"red\"/></svg>",
///     2,
///     2,
///     1.0,
/// );
/// assert_eq!(pixels.len(), 4);
/// ```
pub fn render_to_buffer(xml: &str, width: i32, height: i32, scale: f32) -> alloc::vec::Vec<u32> {
    let mut pixels = alloc::vec![0u32; (width * height) as usize];
    let ptr = pixels.as_mut_ptr() as *mut u8;
    let mut surface = unsafe {
        crate::surface::PixelBuffer::new(
            ptr,
            (width * height * 4) as usize,
            width as u32,
            height as u32,
            (width * 4) as u32,
        )
    };

    let mut parser = SvgParser::new();
    parser.set_viewport(width, height);
    parser.set_initial_transform(Transform::scale(scale, scale));

    let mut list = crate::drawlist::DrawList::new();
    let cmds = parser.parse(xml);
    list.commands().extend(cmds);

    crate::raster::execute(&mut surface, &list, false);

    pixels
}
