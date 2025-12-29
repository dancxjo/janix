use abi::memory::PhysFrame;
use abi::ThingId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::alloc::Layout;
use core::ptr;

#[derive(Debug)]
pub struct ByteSpaceBacking {
    pub id: u64,
    pub len: u64,
    pub flags: u32,
    pub pages: Vec<PhysFrame>,
    /// Fallback pointers for kernel-heap allocated pages (anonymous backing).
    /// Used when we don't have direct physical frame allocation.
    pub kernel_ptrs: Vec<*mut u8>,
}

// Send/Sync because we might share store (though usually locked)
unsafe impl Send for ByteSpaceBacking {}
unsafe impl Sync for ByteSpaceBacking {}

impl Drop for ByteSpaceBacking {
    fn drop(&mut self) {
        if !self.kernel_ptrs.is_empty() {
            let layout = Layout::from_size_align(4096, 4096).unwrap();
            for &ptr in &self.kernel_ptrs {
                unsafe { alloc::alloc::dealloc(ptr, layout) };
            }
        }
    }
}

pub struct ByteSpaceStore {
    backings: BTreeMap<u64, ByteSpaceBacking>,
    pub thing_to_backing: BTreeMap<ThingId, u64>,
    next_id: u64,
}

impl ByteSpaceStore {
    pub fn new() -> Self {
        Self {
            backings: BTreeMap::new(),
            thing_to_backing: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn bind_thing(&mut self, thing_id: ThingId, backing_id: u64) {
        self.thing_to_backing.insert(thing_id, backing_id);
    }

    pub fn get_backing_id(&self, thing_id: ThingId) -> Option<u64> {
        self.thing_to_backing.get(&thing_id).copied()
    }

    pub fn create_backing(&mut self, len: u64, flags: u32) -> Result<u64, ()> {
        let num_pages = (len + 4095) / 4096;
        let mut kernel_ptrs = Vec::new();

        // Fallback: allocate kernel pages
        for _ in 0..num_pages {
            unsafe {
                let layout = Layout::from_size_align(4096, 4096).map_err(|_| ())?;
                let ptr = alloc::alloc::alloc_zeroed(layout);
                if ptr.is_null() {
                    return Err(());
                }
                kernel_ptrs.push(ptr);
            }
        }

        let id = self.next_id;
        self.next_id += 1;

        let backing = ByteSpaceBacking {
            id,
            len,
            flags,
            pages: Vec::new(), // Empty means purely virtual/fallback
            kernel_ptrs,
        };
        self.backings.insert(id, backing);
        Ok(id)
    }

    pub fn create_from_slice(&mut self, data: &[u8], flags: u32) -> Result<u64, ()> {
        let id = self.create_backing(data.len() as u64, flags)?;
        self.write(id, 0, data);
        Ok(id)
    }

    pub fn create_backing_with_frames(&mut self, len: u64, flags: u32, pages: Vec<PhysFrame>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let backing = ByteSpaceBacking {
            id,
            len,
            flags,
            pages,
            kernel_ptrs: Vec::new(),
        };
        self.backings.insert(id, backing);
        id
    }

    pub fn get(&self, id: u64) -> Option<&ByteSpaceBacking> {
        self.backings.get(&id)
    }

    pub fn read(&self, id: u64, offset: u64, buf: &mut [u8]) -> usize {
        if let Some(backing) = self.backings.get(&id) {
            let mut read_len = 0;
            let end = core::cmp::min(offset + buf.len() as u64, backing.len);
            let mut curr = offset;

            // Handle fallback pointers
            if !backing.kernel_ptrs.is_empty() {
                 while curr < end {
                    let page_idx = (curr / 4096) as usize;
                    let page_off = (curr % 4096) as usize;
                    let chunk_len = core::cmp::min(end - curr, 4096 - page_off as u64) as usize;

                    if let Some(&ptr) = backing.kernel_ptrs.get(page_idx) {
                        unsafe {
                            let src = ptr.add(page_off);
                            let dst = &mut buf[read_len..read_len + chunk_len];
                            ptr::copy_nonoverlapping(src, dst.as_mut_ptr(), chunk_len);
                        }
                    } else {
                        break;
                    }
                    curr += chunk_len as u64;
                    read_len += chunk_len;
                }
            } else if !backing.pages.is_empty() {
                // If we have physical frames but no virtual mapping, we can't read directly.
                // Assuming kernel runs in a direct map or we can't read.
            }

            read_len
        } else {
            0
        }
    }

    pub fn write(&mut self, id: u64, offset: u64, buf: &[u8]) -> usize {
        if let Some(backing) = self.backings.get_mut(&id) {
             let mut written_len = 0;
            let end = core::cmp::min(offset + buf.len() as u64, backing.len);
            let mut curr = offset;

            if !backing.kernel_ptrs.is_empty() {
                 while curr < end {
                    let page_idx = (curr / 4096) as usize;
                    let page_off = (curr % 4096) as usize;
                    let chunk_len = core::cmp::min(end - curr, 4096 - page_off as u64) as usize;

                    if let Some(&ptr) = backing.kernel_ptrs.get(page_idx) {
                        unsafe {
                            let dst = ptr.add(page_off);
                            let src = &buf[written_len..written_len + chunk_len];
                            ptr::copy_nonoverlapping(src.as_ptr(), dst, chunk_len);
                        }
                    } else {
                        break;
                    }
                    curr += chunk_len as u64;
                    written_len += chunk_len;
                }
            }
            written_len
        } else {
            0
        }
    }
}
