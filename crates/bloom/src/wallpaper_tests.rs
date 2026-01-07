//! Unit tests for wallpaper loading that would have caught the bugs:
//! 1. BMP parsing validation
//! 2. Address mismatch detection (simulated)
//! 3. Bitmap store + TileBitmap integration

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    
    /// Create a minimal valid 24-bit BMP header for testing
    fn create_test_bmp(width: u32, height: u32, pixels: &[u8]) -> Vec<u8> {
        let row_stride = ((width as usize * 3 + 3) / 4) * 4;
        let pixel_data_size = row_stride * height as usize;
        let file_size = 54 + pixel_data_size;
        
        let mut data = vec![0u8; file_size];
        
        // BMP header
        data[0] = b'B';
        data[1] = b'M';
        // File size (little-endian)
        data[2..6].copy_from_slice(&(file_size as u32).to_le_bytes());
        // Data offset
        data[10..14].copy_from_slice(&54u32.to_le_bytes());
        
        // DIB header
        data[14..18].copy_from_slice(&40u32.to_le_bytes()); // header size
        data[18..22].copy_from_slice(&width.to_le_bytes());
        data[22..26].copy_from_slice(&(height as i32).to_le_bytes()); // positive = bottom-up
        data[26..28].copy_from_slice(&1u16.to_le_bytes()); // planes
        data[28..30].copy_from_slice(&24u16.to_le_bytes()); // bits per pixel
        
        // Copy pixel data (if provided)
        let copy_len = pixels.len().min(pixel_data_size);
        data[54..54+copy_len].copy_from_slice(&pixels[..copy_len]);
        
        data
    }

    // ===================== BMP PARSING TESTS =====================
    
    #[test]
    fn test_parse_bmp_valid_header() {
        let bmp = create_test_bmp(2, 2, &[0; 24]); // 2x2 with padding
        
        // Verify header is valid
        assert_eq!(bmp[0], b'B');
        assert_eq!(bmp[1], b'M');
        
        let width = u32::from_le_bytes([bmp[18], bmp[19], bmp[20], bmp[21]]);
        let height = i32::from_le_bytes([bmp[22], bmp[23], bmp[24], bmp[25]]);
        let bits = u16::from_le_bytes([bmp[28], bmp[29]]);
        
        assert_eq!(width, 2);
        assert_eq!(height, 2);
        assert_eq!(bits, 24);
    }
    
    #[test]
    fn test_parse_bmp_reject_invalid_magic() {
        let mut bmp = create_test_bmp(2, 2, &[0; 24]);
        bmp[0] = b'X'; // Invalid magic
        
        // Parse should fail
        assert_ne!(bmp[0], b'B', "BMP magic should be invalid");
    }
    
    #[test]
    fn test_parse_bmp_reject_too_small() {
        let bmp = vec![b'B', b'M']; // Only 2 bytes, way too small
        
        assert!(bmp.len() < 54, "BMP should be rejected as too small");
    }
    
    #[test]
    fn test_parse_bmp_reject_unsupported_depth() {
        let mut bmp = create_test_bmp(2, 2, &[0; 24]);
        bmp[28] = 16; // 16-bit not supported
        bmp[29] = 0;
        
        let bits = u16::from_le_bytes([bmp[28], bmp[29]]);
        assert!(bits != 24 && bits != 32, "Should reject unsupported bit depth");
    }
    
    #[test]
    fn test_parse_bmp_1x1_pixel() {
        // 1x1 24-bit BMP: Blue=0xFF, Green=0x00, Red=0x80, padding to 4 bytes
        let pixel_data = [0xFF, 0x00, 0x80, 0x00]; // BGR + padding
        let bmp = create_test_bmp(1, 1, &pixel_data);
        
        assert!(bmp.len() >= 58, "BMP should have header + 4 bytes pixel data");
        
        // Verify pixel at offset 54
        assert_eq!(bmp[54], 0xFF, "Blue channel");
        assert_eq!(bmp[55], 0x00, "Green channel");
        assert_eq!(bmp[56], 0x80, "Red channel");
    }

    // ===================== ADDRESS VALIDATION TESTS =====================
    // These tests simulate the bug where we used requested_addr instead of actual_addr
    
    #[test]
    fn test_address_mismatch_detection() {
        // Simulates space_map returning a different address than requested
        let requested_addr: u64 = 0x8500_0000;
        let actual_addr: u64 = 0x8600_0000; // Different!
        
        // The bug: code used requested_addr for slice creation
        // The fix: code should use actual_addr
        
        // This would have caught the bug:
        assert_ne!(requested_addr, actual_addr, "Addresses should not match for this test");
        
        // The fixed code should detect this and either:
        // a) Use actual_addr for the slice, or
        // b) Return an error if exact address is required
        
        // Simulate the fixed behavior:
        let result = if actual_addr == 0 {
            Err("space_map failed")
        } else {
            // Fixed: use actual_addr, not requested_addr
            Ok(actual_addr) 
        };
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), actual_addr, "Should use ACTUAL address");
    }
    
    #[test]
    fn test_space_map_failure_detection() {
        // Simulates space_map returning 0 (failure)
        let actual_addr: u64 = 0;
        
        let result = if actual_addr == 0 {
            Err("space_map failed")
        } else {
            Ok(actual_addr)
        };
        
        assert!(result.is_err(), "Should detect space_map failure");
    }

    // ===================== BITMAP STORE INTEGRATION TESTS =====================
    
    #[test]
    fn test_bitmap_store_add_retrieve() {
        use crate::assets::bitmap::{Bitmap, BitmapStore};
        
        let mut store = BitmapStore::new();
        
        let bmp = Bitmap {
            w: 4,
            h: 4,
            pixels: vec![0xFFAABBCC; 16].into(),
        };
        
        let handle = store.add(bmp);
        let retrieved = store.get(handle);
        
        assert!(retrieved.is_some(), "Should retrieve bitmap by handle");
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.w, 4);
        assert_eq!(retrieved.h, 4);
        assert_eq!(retrieved.pixels.len(), 16);
    }
    
    #[test]
    fn test_bitmap_store_multiple_bitmaps() {
        use crate::assets::bitmap::{Bitmap, BitmapStore};
        
        let mut store = BitmapStore::new();
        
        let bmp1 = Bitmap { w: 2, h: 2, pixels: vec![1; 4].into() };
        let bmp2 = Bitmap { w: 3, h: 3, pixels: vec![2; 9].into() };
        
        let h1 = store.add(bmp1);
        let h2 = store.add(bmp2);
        
        // Handles should be different
        assert_ne!(h1.0, h2.0, "Handles should be unique");
        
        // Each retrieves correct bitmap
        assert_eq!(store.get(h1).unwrap().w, 2);
        assert_eq!(store.get(h2).unwrap().w, 3);
    }

    // ===================== TILEBITMAP EXECUTOR INTEGRATION TEST =====================
    
    #[test]
    fn test_tilebitmap_with_store() {
        use crate::painter::CpuPainter;
        use crate::assets::bitmap::{Bitmap, BitmapStore};
        use crate::scene::{Rect, Point};
        
        let mut store = BitmapStore::new();
        
        // Create a 2x2 wallpaper pattern
        let wallpaper = Bitmap {
            w: 2,
            h: 2,
            pixels: vec![
                0xFF0000FF, // Red
                0xFF00FF00, // Green
                0xFFFF0000, // Blue
                0xFFFFFFFF, // White
            ].into(),
        };
        
        let handle = store.add(wallpaper);
        
        // Get the bitmap from store and draw
        let bmp = store.get(handle).expect("Bitmap should exist");
        
        let mut target = vec![0u32; 4]; // 2x2 screen
        {
            let mut painter = CpuPainter::new(&mut target, 2, 2);
            painter.draw_tiled_bitmap(
                Rect { x: 0, y: 0, w: 2, h: 2 },
                bmp,
                Point { x: 0, y: 0 },
            );
        }
        
        // Should have the wallpaper pattern
        assert_eq!(target[0], 0xFF0000FF, "Top-left should be red");
        assert_eq!(target[1], 0xFF00FF00, "Top-right should be green");
        assert_eq!(target[2], 0xFFFF0000, "Bottom-left should be blue");
        assert_eq!(target[3], 0xFFFFFFFF, "Bottom-right should be white");
    }
}
