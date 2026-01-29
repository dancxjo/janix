use super::SvgParser;
use crate::drawlist::DrawCmd;

#[test]
fn pin_svg_pipeline_basic() {
    let xml = r##"<svg width="16" height="16" xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="16" height="16" fill="#ff0000"/></svg>"##;
    let mut parser = SvgParser::new();
    let cmds = parser.parse(xml);

    assert!(
        !cmds.is_empty(),
        "Parser should produce commands for simple rect"
    );

    let mut found_rect = false;
    for cmd in cmds {
        if let DrawCmd::FillRect { rect, color, .. } = cmd {
            assert_eq!(rect.width(), 16);
            assert_eq!(rect.height(), 16);
            assert_eq!(color.r, 255);
            found_rect = true;
        }
    }
    assert!(found_rect, "Should have produced a FillRect command");
}
