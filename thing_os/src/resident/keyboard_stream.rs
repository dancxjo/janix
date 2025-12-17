use super::Resident;
pub use abi::keyboard_stream::{KeyboardStreamHeader, KeyboardEntry};
use alloc::vec::Vec;
use crate::prelude::*;

/// Helper for accessing the ring buffer within a ResidentObject.
#[derive(Debug)]
pub struct KeyboardStreamMapped<T> {
    pub obj: Resident<T>,
}

impl<T> KeyboardStreamMapped<T> {
    pub fn new(obj: Resident<T>) -> Self {
        Self { obj }
    }
    
    // Reader helper
    pub fn read_entries_into(&self, start_tail: u64, out: &mut Vec<KeyboardEntry>) -> u64 {
        self.obj.with_read(|header, data| {
            unsafe {
                 let data_off = header.data_off as usize;
                 if data_off >= data.len() {
                     return start_tail;
                 }

                 let stream_header_ptr = data.as_ptr().add(data_off) as *const KeyboardStreamHeader;
                 let head = stream_header_ptr.read_unaligned().head;
                 let capacity = stream_header_ptr.read_unaligned().capacity;
                 
                 if capacity == 0 {
                     return start_tail;
                 }
                 
                 let entries_start = data_off + core::mem::size_of::<KeyboardStreamHeader>();
                 let entries_ptr = data.as_ptr().add(entries_start) as *const KeyboardEntry;
                 
                 let mut current = start_tail;
                 
                 // Handle Overrun
                 let diff = head.wrapping_sub(current);
                 if diff > capacity {
                      current = head.wrapping_sub(capacity);
                 }
                 
                 while current != head {
                      let idx = (current % capacity) as usize;
                      let entry_offset = entries_start + idx * core::mem::size_of::<KeyboardEntry>();
                      if entry_offset + core::mem::size_of::<KeyboardEntry>() > data.len() {
                           break; 
                      }

                      let entry = entries_ptr.add(idx).read_unaligned();
                      out.push(entry);
                      current = current.wrapping_add(1);
                 }
                 
                 current
            }
        })
    }
    
    // Writer helper
    pub fn append(&mut self, entry: KeyboardEntry) {
        self.obj.with_write(|header, data| {
             unsafe {
                let data_off = header.data_off as usize;
                if data_off >= data.len() { return; }

                let stream_header_ptr = data.as_mut_ptr().add(data_off) as *mut KeyboardStreamHeader;
                let head = stream_header_ptr.read_unaligned().head;
                let capacity = stream_header_ptr.read_unaligned().capacity;
                 
                if capacity == 0 { return; } 
                 
                let entries_start = data_off + core::mem::size_of::<KeyboardStreamHeader>();
                let entries_ptr = data.as_mut_ptr().add(entries_start) as *mut KeyboardEntry;
                
                let idx = (head % capacity) as usize;
                let entry_offset = entries_start + idx * core::mem::size_of::<KeyboardEntry>();
                
                if entry_offset + core::mem::size_of::<KeyboardEntry>() <= data.len() {
                    entries_ptr.add(idx).write_unaligned(entry);
                    let mut header_val = stream_header_ptr.read_unaligned();
                    header_val.head = head.wrapping_add(1);
                    stream_header_ptr.write_unaligned(header_val);
                }
             }
        })
    }
    
    // Initialize layout
    pub fn init(&mut self, capacity: u64) {
        self.obj.with_write(|header, data| {
              unsafe {
                  let data_off = header.data_off as usize;
                  // Ensure space
                  let needed = data_off + core::mem::size_of::<KeyboardStreamHeader>() + (capacity as usize * core::mem::size_of::<KeyboardEntry>());
                  if needed > data.len() {
                      let stream_header_ptr = data.as_mut_ptr().add(data_off) as *mut KeyboardStreamHeader;
                      let mut header_val = stream_header_ptr.read_unaligned();
                      header_val.capacity = 0;
                      stream_header_ptr.write_unaligned(header_val);
                      return;
                  }

                  let stream_header_ptr = data.as_mut_ptr().add(data_off) as *mut KeyboardStreamHeader;
                  // Read the full header (including ResidentHeader)
                  let mut header_val: KeyboardStreamHeader = stream_header_ptr.read_unaligned();
                  header_val.head = 0;
                  header_val.capacity = capacity;
                  header_val._pad = [0u8; 8];
                  stream_header_ptr.write_unaligned(header_val);
              }
        })
    }
}

use abi::{Thing, ThingId, PropKey, PropValue, PropType};

#[derive(Clone, Debug)]
pub struct KeyboardStreamThing {
    pub id: ThingId,
}

impl Thing for KeyboardStreamThing {
    const KIND: &'static str = "KeyboardStream";
    const DESCRIPTION: &'static str = "Resident ring buffer for keyboard input";

    fn to_props(&self, _out: &mut Vec<(PropKey, PropValue)>) {}
    
    fn from_props(id: ThingId, _props: &[Option<(PropKey, PropValue)>]) -> Self {
        Self { id }
    }
    
    fn schema() -> &'static [(&'static str, PropType)] {
        &[("head", PropType::U64), ("capacity", PropType::U64)]
    }
}
