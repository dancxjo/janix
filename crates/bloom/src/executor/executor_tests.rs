#[cfg(test)]
mod tests {
    use crate::painter::CpuPainter;
    use crate::assets::bitmap::Bitmap;
    use crate::scene::{Rect, Point};
    use alloc::vec;

    #[test]
    fn test_tile_bitmap_basic() {
        let pixels = vec![1, 2, 3, 4];
        let bmp = Bitmap { w: 2, h: 2, pixels: pixels.into() };
        let mut target = vec![0u32; 15]; // 5x3
        // Pattern:
        // 1 2
        // 3 4
        // Expected:
        // 1 2 1 2 1
        // 3 4 3 4 3
        // 1 2 1 2 1
        
        {
            let mut painter = CpuPainter::new(&mut target, 5, 3);
            painter.draw_tiled_bitmap(Rect { x: 0, y: 0, w: 5, h: 3 }, &bmp, Point { x: 0, y: 0 });
        }
        
        assert_eq!(target[0..5], [1, 2, 1, 2, 1]);
        assert_eq!(target[5..10], [3, 4, 3, 4, 3]);
        assert_eq!(target[10..15], [1, 2, 1, 2, 1]);
    }

    #[test]
    fn test_tile_bitmap_clipping() {
        let pixels = vec![10];
        let bmp = Bitmap { w: 1, h: 1, pixels: pixels.into() };
        let mut target = vec![0u32; 4]; // 2x2
        // Draw in x=1, w=2 (which implies x=1, x=2). Target width 2 implies x=0, x=1 valid.
        // So only x=1 should be drawn.
        {
            let mut painter = CpuPainter::new(&mut target, 2, 2);
            painter.draw_tiled_bitmap(Rect { x: 1, y: 0, w: 2, h: 2 }, &bmp, Point { x: 0, y: 0 });
        }

        // target indices:
        // 0, 1 -> row 0
        // 2, 3 -> row 1
        // x=1 is index 1 and 3.
        assert_eq!(target, vec![0, 10, 0, 10]);
    }

    #[test]
    fn test_tile_bitmap_phase() {
        let pixels = vec![1, 2, 3, 4];
        let bmp = Bitmap { w: 2, h: 2, pixels: pixels.into() };
        let mut target = vec![0u32; 4]; // 2x2
        
        // origin (1,1) means target (1,1) corresponds to bitmap (0,0)=1
        // target (0,0) -> ( -1, -1 ) -> (1, 1) = 4
        // target (1,0) -> ( 0, -1 ) -> (0, 1) = 3
        // target (0,1) -> ( -1, 0 ) -> (1, 0) = 2
        // target (1,1) -> ( 0, 0 ) -> (0, 0) = 1
        // Result: 4 3 2 1
        
        {
            let mut painter = CpuPainter::new(&mut target, 2, 2);
            painter.draw_tiled_bitmap(Rect { x: 0, y: 0, w: 2, h: 2 }, &bmp, Point { x: 1, y: 1 });
        }
        
        assert_eq!(target, vec![4, 3, 2, 1]);
    }
}
