use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::alloc::Layout;
// use crate::bridge::FullMachineBridge; // Not needed for store logic yet if we don't map vertically

#[derive(Debug)]
pub struct ByteSpace {
    pub id: u64,
    pub len: u64,
    pub flags: u32,
    pub pages: Vec<PageChunk>,
}

#[derive(Debug)]
pub struct PageChunk {
    ptr: *mut u8,
}

// Send/Sync for PageChunk?
unsafe impl Send for PageChunk {}
unsafe impl Sync for PageChunk {}

impl PageChunk {
    pub fn new() -> Option<Self> {
        unsafe {
            let layout = Layout::from_size_align(4096, 4096).ok()?;
            let ptr = alloc::alloc::alloc_zeroed(layout);
            if ptr.is_null() {
                None
            } else {
                Some(Self { ptr })
            }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, 4096) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, 4096) }
    }
}

impl Drop for PageChunk {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align(4096, 4096).unwrap();
            alloc::alloc::dealloc(self.ptr, layout);
        }
    }
}

pub struct ByteSpaceStore {
    spaces: BTreeMap<u64, ByteSpace>,
    next_id: u64,
}

impl ByteSpaceStore {
    pub fn new() -> Self {
        Self {
            spaces: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn create(&mut self, len: u64, flags: u32) -> Result<u64, ()> {
        let mut pages = Vec::new();
        let num_pages = (len + 4095) / 4096;

        for _ in 0..num_pages {
            let chunk = PageChunk::new().ok_or(())?;
            pages.push(chunk);
        }

        let id = self.next_id;
        self.next_id += 1;

        let space = ByteSpace {
            id,
            len,
            flags,
            pages,
        };
        self.spaces.insert(id, space);
        Ok(id)
    }

    pub fn create_from_slice(&mut self, data: &[u8], flags: u32) -> Result<u64, ()> {
        let len = data.len() as u64;
        let mut pages = Vec::new();

        let mut offset = 0;
        while offset < data.len() {
            let mut chunk = PageChunk::new().ok_or(())?;
            let slice = chunk.as_mut_slice();
            let copy_len = core::cmp::min(data.len() - offset, 4096);
            slice[..copy_len].copy_from_slice(&data[offset..offset + copy_len]);
            pages.push(chunk);
            offset += copy_len;
        }

        let id = self.next_id;
        self.next_id += 1;

        let space = ByteSpace {
            id,
            len,
            flags,
            pages,
        };
        self.spaces.insert(id, space);
        Ok(id)
    }

    pub fn get(&self, id: u64) -> Option<&ByteSpace> {
        self.spaces.get(&id)
    }

    pub fn get_mut(&mut self, id: u64) -> Option<&mut ByteSpace> {
        self.spaces.get_mut(&id)
    }
}
