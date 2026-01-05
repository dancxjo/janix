//! Pure HID parsing logic.
//!
//! Handles Boot Protocol and (future) Report Descriptor parsing.



#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MouseReport {
    pub dx: i16,
    pub dy: i16,
    pub buttons: u16,
    pub wheel: i16,
}

/// Parse a Boot Protocol Mouse Report
///
/// Format:
/// Byte 0: Buttons (Bit 0: Left, 1: Right, 2: Middle)
/// Byte 1: X displacement (signed i8)
/// Byte 2: Y displacement (signed i8)
/// Byte 3: Wheel (optional, signed i8)
pub fn parse_boot_mouse(report: &[u8]) -> Option<MouseReport> {
    if report.len() < 3 {
        return None;
    }

    let buttons = report[0] as u16;
    let dx = report[1] as i8 as i16;
    let dy = report[2] as i8 as i16;
    
    let wheel = if report.len() >= 4 {
        report[3] as i8 as i16
    } else {
        0
    };

    Some(MouseReport {
        dx,
        dy,
        buttons, // Boot proto buttons map 1:1 to our normalized buttons for now
        wheel,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boot_mouse_3byte() {
        // Left click, dx=5, dy=-2
        let raw = [0b001, 5, 0xFE]; 
        let parsed = parse_boot_mouse(&raw).unwrap();
        assert_eq!(parsed.dx, 5);
        assert_eq!(parsed.dy, -2);
        assert_eq!(parsed.buttons, 1);
        assert_eq!(parsed.wheel, 0);
    }

    #[test]
    fn test_boot_mouse_4byte_wheel() {
        // Right click, dx=0, dy=0, wheel=1
        let raw = [0b010, 0, 0, 1];
        let parsed = parse_boot_mouse(&raw).unwrap();
        assert_eq!(parsed.wheel, 1);
        assert_eq!(parsed.buttons, 2);
    }
}
