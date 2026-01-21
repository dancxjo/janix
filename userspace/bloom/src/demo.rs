#[cfg(feature = "svg-demo")]
pub fn run_svg_demo(mut surface: crate::surface::Surface, mut presenter: crate::present::PresenterImpl, width: u32, height: u32, format: u32) -> ! {
    use crate::drawlist::{DrawCmd};
    use crate::geometry::{Color, Transform};
    use crate::svg::SvgParser;
    use crate::frame::{FrameBuilder, FrameSpec};
    use alloc::vec;

    // Hardcoded simple SVGs for testing
    let svgs = [
        (r##"<svg><rect x="0" y="0" width="80" height="80" fill="#FF0000" stroke="white" stroke-width="5"/></svg>"##, "Rect"),
        (r##"<svg><circle cx="40" cy="40" r="40" fill="#0000FF" stroke="yellow" stroke-width="5"/></svg>"##, "Circle"),
        (r##"<svg><path d="M 10 80 L 40 10 L 70 80 Z" fill="#00FF00" stroke="white" stroke-width="5"/></svg>"##, "Triangle"),
        // Transformed group
        (r##"<svg><g transform="translate(10, 10) rotate(45)"><rect x="0" y="0" width="50" height="50" fill="purple"/></g></svg>"##, "Transform"),
        // Polyline
        (r##"<svg><polyline points="0,80 20,20 40,80 60,20 80,80" fill="none" stroke="orange" stroke-width="3"/></svg>"##, "Polyline"),
        // Bezier Heart
        (r##"<svg><path d="M 50 80 C 20 50 20 20 50 20 C 80 20 80 50 50 80 Z" fill="red" stroke="white" stroke-width="2"/></svg>"##, "Heart (Cubic)"),
        // Bezier Ghost
        (r##"<svg><path d="M 20 80 L 20 40 Q 20 20 50 20 Q 80 20 80 40 L 80 80 L 65 70 L 50 80 L 35 70 Z" fill="#EEE" stroke="#999" stroke-width="2"/></svg>"##, "Ghost (Quad)"),
        // Ellipse approximation
        (r##"<svg><g transform="scale(1.5, 0.8)"><circle cx="30" cy="50" r="25" fill="cyan"/></g></svg>"##, "Ellipse"),
    ];

    loop {
        // Build frame
        let spec = FrameSpec::new(width, height, format);
        // Using main ASSETS bank if available, or just dummy gen
        let asset_gen = crate::ASSETS.publish_pending();
        
        let token = presenter.acquire_frame(spec, asset_gen);
        let mut builder = FrameBuilder::new(token);
        
        {
            let list = builder.ops();
            list.clear(Color::new(30, 30, 30, 255));
            
            // Title
            list.text_font("SVG Demo - Bloom", "NotoSerif-Regular.ttf", 30, 20, 50.0, Color::WHITE);
            
            let mut x_off = 50;
            let mut y_off = 100;
            let cell_size = 150;
    
            for (xml, label) in svgs.iter() {
                let mut parser = SvgParser::new();
                
                // Layout transform
                list.commands().push(DrawCmd::PushTransform { transform: Transform::translate(x_off as f32, y_off as f32) });
                
                let cmds = parser.parse(xml);
                list.commands().extend(cmds);
                
                list.commands().push(DrawCmd::PopTransform);
                
                // Label
                list.text_font(label, "NotoSerif-Regular.ttf", 20, x_off, (y_off + 100) as f32, Color::new(200, 200, 200, 255));
                
                x_off += cell_size;
                if x_off + cell_size > width as i32 {
                    x_off = 50;
                    y_off += cell_size + 20;
                }
            }
        }
        
        builder.mark_full_damage();
        
        let token = builder.finish();
        
        // Rasterize
        crate::raster::execute_with_damage(&mut surface, &token.ops, &token.damage, false);
        
        // Present
        presenter.present_frame(token);
        presenter.pump();
        
        stem::sleep_ms(16); // ~60fps
    }
}
