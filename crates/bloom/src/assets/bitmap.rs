use alloc::sync::Arc;
use alloc::collections::BTreeMap;


pub struct Bitmap {
    pub w: u32,
    pub h: u32,
    pub pixels: Arc<[u32]>, // ARGB or premultiplied ARGB
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BitmapHandle(pub u64);

pub struct BitmapStore {
    bitmaps: BTreeMap<u64, Bitmap>,
    next_id: u64,
}

impl BitmapStore {
    pub fn new() -> Self {
        Self {
            bitmaps: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, bitmap: Bitmap) -> BitmapHandle {
        let id = self.next_id;
        self.next_id += 1;
        self.bitmaps.insert(id, bitmap);
        BitmapHandle(id)
    }

    pub fn get(&self, handle: BitmapHandle) -> Option<&Bitmap> {
        self.bitmaps.get(&handle.0)
    }
    
    pub fn remove(&mut self, handle: BitmapHandle) {
        self.bitmaps.remove(&handle.0);
    }
}
