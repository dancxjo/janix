use super::Resident;
pub use abi::mouse_stream::{MouseStreamHeader, MouseEntry};
use alloc::vec::Vec;

/// Helper for accessing the ring buffer within a ResidentObject.
#[derive(Debug)]
pub struct MouseStreamMapped<T> {
    pub obj: Resident<T>,
}

impl<T> MouseStreamMapped<T> {
    pub fn new(obj: Resident<T>) -> Self {
        Self { obj }
    }
    
    // Reader helper
    pub fn read_entries_into(&self, start_tail: u32, out: &mut Vec<MouseEntry>) -> u32 {
        self.obj.with_read(|header, data| {
            unsafe {
                 let data_off = header.data_off as usize;
                 if data_off >= data.len() {
                     return start_tail;
                 }

                 let stream_header_ptr = data.as_ptr().add(data_off) as *const MouseStreamHeader;
                 let head = (*stream_header_ptr).head;
                 let capacity = (*stream_header_ptr).capacity;
                 
                 if capacity == 0 {
                     return start_tail;
                 }
                 
                 let entries_start = data_off + core::mem::size_of::<MouseStreamHeader>();
                 let entries_ptr = data.as_ptr().add(entries_start) as *const MouseEntry;

                 let mut current = start_tail;
                 
                 // Handle Overrun
                 // If reader is behind by more than capacity, skip to oldest available.
                 let diff = head.wrapping_sub(current);
                 if diff > capacity {
                      current = head.wrapping_sub(capacity);
                 }
                 
                 // If reader is ahead of head (shouldn't happen unless reset), clamp.
                 // The wrapping_sub check above handles huge diffs (negative) as huge positive > capacity.
                 // Wait, wrapping_sub of `head < current` is huge positive.
                 // So `diff > capacity` catches both "Way behind" and "Ahead".
                 // If Ahead, `head - current` is near u32::MAX.
                 // So valid range is `[head - capacity, head]`.
                 
                 // However, valid check:
                 // if diff > capacity...
                 // Case 1: normal overrun. `head` is 100, `current` is 10. `capacity` is 20.
                 // `100 - 10 = 90 > 20`. Correct. `current` -> 80.
                 
                 // Case 2: reset/ahead. `head` is 10, `current` is 20.
                 // `10 - 20 = FFFFFFFO`. Huge. > 20. `current` -> `10 - 20 = FFFFFFFO` (still bad?)
                 // No, `current = head - capacity` = `10 - 20 = FFFFFFFO`.
                 // We want `current = head`?
                 
                 // Actually if `current > head`, we should just jump to `head` (or 0?).
                 // But strictly with wrapping logic, we can't distinguish "very old" from "future".
                 // Assuming we don't jump into future, treating it as overrun is safest: jump to window.
                 
                 while current != head {
                      let idx = (current % capacity) as usize;
                      let entry_offset = entries_start + idx * core::mem::size_of::<MouseEntry>();
                      if entry_offset + core::mem::size_of::<MouseEntry>() > data.len() {
                           break; 
                      }

                      let entry = *entries_ptr.add(idx);
                      out.push(entry);
                      current = current.wrapping_add(1);
                 }
                 
                 current
            }
        })
    }
    
    // Writer helper
    pub fn append(&mut self, entry: MouseEntry) {
        self.obj.with_write(|header, data| {
             unsafe {
                let data_off = header.data_off as usize;
                if data_off >= data.len() { return; }

                let stream_header_ptr = data.as_mut_ptr().add(data_off) as *mut MouseStreamHeader;
                let head = (*stream_header_ptr).head;
                let capacity = (*stream_header_ptr).capacity;
                 
                if capacity == 0 { return; } 
                 
                let entries_start = data_off + core::mem::size_of::<MouseStreamHeader>();
                let entries_ptr = data.as_mut_ptr().add(entries_start) as *mut MouseEntry;
                
                let idx = (head % capacity) as usize;
                let entry_offset = entries_start + idx * core::mem::size_of::<MouseEntry>();
                
                if entry_offset + core::mem::size_of::<MouseEntry>() <= data.len() {
                    *entries_ptr.add(idx) = entry;
                    (*stream_header_ptr).head = head.wrapping_add(1);
                }
             }
        })
    }
    
    // Initialize layout
    pub fn init(&mut self, capacity: u32) {
        self.obj.with_write(|header, data| {
              unsafe {
                  let data_off = header.data_off as usize;
                  // Ensure space
                  let needed = data_off + core::mem::size_of::<MouseStreamHeader>() + (capacity as usize * core::mem::size_of::<MouseEntry>());
                  if needed > data.len() {
                      // Cannot init, too small.
                      // Adjust capacity? Or just fail silently (v1)?
                      // Set capacity to 0.
                      let stream_header_ptr = data.as_mut_ptr().add(data_off) as *mut MouseStreamHeader;
                      (*stream_header_ptr).capacity = 0;
                      return;
                  }

                  let stream_header_ptr = data.as_mut_ptr().add(data_off) as *mut MouseStreamHeader;
                  (*stream_header_ptr).head = 0;
                  (*stream_header_ptr).capacity = capacity;
                  // Zero alignment padding?
                  (*stream_header_ptr)._pad = [0u8; 8];
              }
        })
    }
}

use abi::ThingId;
use crate::{Thing, PropKey, PropValue, PropType};

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
    use crate::resident::Resident;
    use abi::resident_layout::ResidentHeader;
    use alloc::vec;
    use core::alloc::Layout;

    struct TestMouseStream {
        _backing: Vec<u8>,
        stream: MouseStreamMapped<()>,
    }

    impl TestMouseStream {
        fn new(capacity: u32) -> Self {
             let header_size = core::mem::size_of::<ResidentHeader>();
             let stream_header_size = core::mem::size_of::<MouseStreamHeader>();
             let entries_size = capacity as usize * core::mem::size_of::<MouseEntry>();
             let total_size = header_size + stream_header_size + entries_size;
             
             let mut backing = vec![0u8; total_size];
             let ptr = backing.as_mut_ptr();
             
             unsafe {
                 let header = ptr as *mut ResidentHeader;
                 (*header).version = 1;
                 (*header).seq = 0;
                 (*header).magic = ResidentHeader::MAGIC;
                 (*header).data_off = header_size as u32; 
             }

             let obj = unsafe { Resident::new(ThingId(0), ptr, total_size) };
             let mut stream = MouseStreamMapped::new(obj);
             stream.init(capacity);
             
             Self { _backing: backing, stream }
        }
    }

    #[test]
    fn test_append_read() {
        let mut t = TestMouseStream::new(10);
        
        // MouseEntry fields: dx, dy, buttons, flags
        let entry1 = MouseEntry { buttons: 1, dx: 10, dy: 20, flags: 0, ..Default::default() };
        t.stream.append(entry1);
        
        let mut events = Vec::new();
        let head = t.stream.read_entries_into(0, &mut events);
        assert_eq!(head, 1);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].buttons, 1);
        assert_eq!(events[0].dx, 10);
        
        events.clear();
        let head2 = t.stream.read_entries_into(head, &mut events);
        assert_eq!(head2, 1);
        assert!(events.is_empty());
    }
}
