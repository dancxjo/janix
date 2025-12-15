use super::ResidentObject;
use abi::resident_layout::ResidentHeader;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MouseEntry {
    pub buttons: u8,
    pub _pad0: u8, // alignment
    pub x: i16,    // Delta X
    pub y: i16,    // Delta Y
    pub z: i16,    // Scroll Z (optional)
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct MouseRingLayout {
    pub head: u32,
    pub capacity: u32,
    // Add more metadata if needed
    pub _reserved: [u64; 2], 
    pub entries: [MouseEntry; 0], // Flexible array member-like
}

/// Helper for accessing the ring buffer within a ResidentObject.
#[derive(Debug)]
pub struct MouseStream<T> {
    pub obj: ResidentObject<T>,
}

impl<T> MouseStream<T> {
    pub fn new(obj: ResidentObject<T>) -> Self {
        Self { obj }
    }
    
    // Reader helper
    pub fn read_entries(&self, start_tail: u32) -> (u32, Vec<MouseEntry>) {
        self.obj.with_read(|_header, data| {
            unsafe {
                 let layout_ptr = data.as_ptr().add(core::mem::size_of::<ResidentHeader>()) as *const MouseRingLayout;
                 let head = (*layout_ptr).head;
                 let capacity = (*layout_ptr).capacity;
                 
                 // If capacity is 0, nothing to do
                 if capacity == 0 {
                     return (start_tail, Vec::new());
                 }
                 
                 let entries_ptr = (layout_ptr as *const u8).add(core::mem::size_of::<MouseRingLayout>()) as *const MouseEntry;
                 
                 let mut results = Vec::new();
                 let mut current = start_tail;
                 
                 // Limit read to avoid excessively large allocations in one go?
                 // Or just read all.
                 while current != head {
                      let idx = (current % capacity) as usize;
                      let entry = *entries_ptr.add(idx);
                      results.push(entry);
                      current = current.wrapping_add(1);
                 }
                 
                 (current, results)
            }
        })
    }
    
    // Writer helper
    pub fn append(&mut self, entry: MouseEntry) {
        self.obj.with_write(|_header, data| {
             unsafe {
                let layout_ptr = data.as_mut_ptr().add(core::mem::size_of::<ResidentHeader>()) as *mut MouseRingLayout;
                let head = (*layout_ptr).head;
                let capacity = (*layout_ptr).capacity;
                 
                if capacity == 0 { return; } // Safety
                 
                let entries_ptr = (layout_ptr as *mut u8).add(core::mem::size_of::<MouseRingLayout>()) as *mut MouseEntry;
                let idx = (head % capacity) as usize;
                
                *entries_ptr.add(idx) = entry;
                
                (*layout_ptr).head = head.wrapping_add(1);
             }
        })
    }
    
    // Initialize layout
    pub fn init(&mut self, capacity: u32) {
        self.obj.with_write(|_header, data| {
              unsafe {
                  let layout_ptr = data.as_mut_ptr().add(core::mem::size_of::<ResidentHeader>()) as *mut MouseRingLayout;
                  (*layout_ptr).head = 0;
                  (*layout_ptr).capacity = capacity;
                  // Zero entries? Optional.
              }
        })
    }
}

use abi::{Thing, ThingId, PropKey, PropValue, PropType};
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct MouseStreamThing {
    pub id: ThingId,
}

impl Thing for MouseStreamThing {
    const KIND: &'static str = "MouseStream";
    const DESCRIPTION: &'static str = "Resident ring buffer for mouse input";

    fn to_props(&self, _out: &mut Vec<(PropKey, PropValue)>) {}
    
    fn from_props(id: ThingId, _props: &[Option<(PropKey, PropValue)>]) -> Self {
        Self { id }
    }
    

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("head", PropType::U64), ("capacity", PropType::U64)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resident::ResidentObject;
    use alloc::vec::Vec;
    use core::alloc::Layout;

    // Helper to create a backed MouseStream
    struct TestMouseStream {
        _backing: Vec<u8>,
        stream: MouseStream<()>,
        ptr: *mut u8,
    }

    impl TestMouseStream {
        fn new(capacity: u32) -> Self {
             let layout_size = core::mem::size_of::<MouseRingLayout>();
             let entries_size = capacity as usize * core::mem::size_of::<MouseEntry>();
             let header_size = core::mem::size_of::<ResidentHeader>();
             let total_size = header_size + layout_size + entries_size;
             
             let mut backing = vec![0u8; total_size];
             let ptr = backing.as_mut_ptr();
             
             unsafe {
                 // Init header (minimal for with_read/write)
                 let header = ptr as *mut ResidentHeader;
                 (*header).version = 1;
                 (*header).seq = 0;
                 (*header).magic = ResidentHeader::MAGIC;
                 (*header).props_off = header_size as u32; // irrelevant for mouse stream direct access?
                 // MouseStream accesses data after header.
                 // In code: `data.as_ptr().add(...)` where data starts at prop_off?
                 // Wait. `with_read` passes `slice::from_raw_parts(self.ptr, ...)` as data?
                 // `ResidentObject::with_read` implementation:
                 // `f(&*header_ptr, core::slice::from_raw_parts(self.ptr, self.byte_len));`
                 // So `data` argument to closure IS the whole buffer (including header).
                 
                 // `MouseStream` logic:
                 // `let layout_ptr = data.as_ptr().add(core::mem::size_of::<ResidentHeader>()) ...`
                 // This assumes `data` points to base of resident object.
                 // Yes, `with_read` passes `self.ptr`.
                 
                 // So we just need backing buffer.
             }

             let obj = unsafe { ResidentObject::new(ThingId(0), ptr, total_size) };
             let mut stream = MouseStream::new(obj);
             stream.init(capacity);
             
             Self { _backing: backing, stream, ptr }
        }
    }

    #[test]
    fn test_append_read() {
        let mut t = TestMouseStream::new(10);
        
        let entry1 = MouseEntry { buttons: 1, _pad0:0, x: 10, y: 20, z: 0, timestamp: 100 };
        t.stream.append(entry1);
        
        let (head, events) = t.stream.read_entries(0);
        assert_eq!(head, 1);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].buttons, 1);
        assert_eq!(events[0].x, 10);
        
        // Read again, no new events
        let (head2, events2) = t.stream.read_entries(head);
        assert_eq!(head2, 1);
        assert!(events2.is_empty());
    }

    #[test]
    fn test_wrapping() {
        let cap = 5;
        let mut t = TestMouseStream::new(cap);
        
        // Fill buffer
        for i in 0..cap {
             t.stream.append(MouseEntry { buttons: 0, _pad0:0, x: i as i16, y: 0, z:0, timestamp: i as u64 });
        }
        
        let (head, events) = t.stream.read_entries(0);
        assert_eq!(head, 5);
        assert_eq!(events.len(), 5);
        
        // Append one more (overwrite index 0)
        t.stream.append(MouseEntry { buttons: 1, _pad0:0, x: 99, y: 0, z:0, timestamp: 99 });
        
        // Reader reads from 5
        let (head2, events2) = t.stream.read_entries(head);
        assert_eq!(head2, 6);
        assert_eq!(events2.len(), 1);
        assert_eq!(events2[0].x, 99);
        
        // Simulating slow reader: reader was at 0.
        // head is 6. capacity is 5.
        // ring buffer has [5, 1, 2, 3, 4] (indices are wrapped)
        // actually indices: 0->5 (wrapped 0), 1->1 ...
        // Index 0 was overwritten by entry 6 (idx 0).
        // If reader reads from 0 to 6:
        // 0%5=0 (New entry), 1%5=1 (Old), ...
        // Reader will see mixed data?
        // Standard ring buffer overwrite issues apply.
        // But logic is valid for test.
        
        let (head_late, events_late) = t.stream.read_entries(0);
        assert_eq!(head_late, 6);
        // It reads 6 entries?
        assert_eq!(events_late.len(), 6);
        // Entry 0 is x=99 (new)
        // Entry 1 is x=1 (old)
        // ...
        assert_eq!(events_late[0].x, 99);
        assert_eq!(events_late[1].x, 1);
    }
}

