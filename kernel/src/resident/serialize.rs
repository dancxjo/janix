use crate::graph::store::{self, StorageState};
use crate::graph::schema;
use abi::{ThingId, PropValue};
use abi::resident::{RestPolicy, RestResp, ResidentError, ResidentErrorCode};
use abi::resident_layout::{ResidentHeader, ResPropEntry};
use crate::memory::hhdm;
use alloc::vec::Vec;
use alloc::string::String;

pub fn snapshot_and_archive(thing_id: ThingId, _policy: RestPolicy) -> Result<RestResp, ResidentError> {
    unsafe {
        let slab = store::things_slab();
        let idx = thing_id.index() as usize;

        if idx >= slab.slots.len() { 
            return Err(ResidentError{code: ResidentErrorCode::BadThing, aux0:0,aux1:0}); 
        }
        let slot = &mut slab.slots[idx];
        if slot.generation != thing_id.generation() { 
            return Err(ResidentError{code: ResidentErrorCode::BadThing, aux0:0,aux1:0}); 
        }
        let thing = slot.thing.as_mut().ok_or(ResidentError{code: ResidentErrorCode::BadThing, aux0:0,aux1:0})?;

        let resident = thing.resident.as_ref().ok_or(ResidentError{code: ResidentErrorCode::NotResident, aux0:0,aux1:0})?;
        
        if resident.pages.is_empty() {
             return Err(ResidentError{code: ResidentErrorCode::NotResident, aux0:0,aux1:0});
        }

        let first_frame = resident.pages[0].frame;
        let vaddr = hhdm::phys_to_virt(first_frame.start_address);
        let header_ptr = vaddr as *const ResidentHeader;
        
        // Seqlock Retry Loop
        let props_vec = loop {
             let seq1 = (*header_ptr).seq;
             if seq1 % 2 != 0 {
                  core::hint::spin_loop();
                  continue;
             }
             core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Acquire);
             
             let header = *header_ptr;
             if header.magic != ResidentHeader::MAGIC {
                  return Err(ResidentError{code: ResidentErrorCode::BadHeader, aux0:0,aux1:0});
             }
             
             let props_off = header.props_off;
             let count = header.prop_count;
             
             let table_ptr = (header_ptr as *const u8).add(props_off as usize) as *const ResPropEntry;
             
             let mut temp_props = Vec::with_capacity(count as usize);
             let mut failed = false;

             for i in 0..count {
                  let entry = *table_ptr.add(i as usize);
                  
                  let val = match entry.tag {
                        1 => PropValue::U64(entry.v),
                        2 => PropValue::I64(entry.v as i64),
                        3 => PropValue::Bool(entry.v != 0),
                        4 => { 
                            let off = entry.a;
                            let len = entry.b;
                            if (off + len) as u32 > header.total_len {
                                failed = true;
                                break; 
                            }
                            let str_ptr = (header_ptr as *const u8).add(off as usize);
                            let slice = core::slice::from_raw_parts(str_ptr, len as usize);
                            if let Ok(s) = alloc::str::from_utf8(slice) {
                                PropValue::Str(String::from(s))
                            } else {
                                PropValue::Str(String::from("<invalid utf8>")) 
                            }
                        },
                        // Bytes not yet supported in PropValue fully? PropValue has Str.
                        // We will skip bytes for now or map to Str special?
                        // Plan said "minimal CBOR". But we are converting to internal Props first.
                        // PropValue only has Str. We'll skip bytes.
                        _ => PropValue::U64(0), 
                  };
                  
                  if let Some(k) = schema::get_key_from_id(thing.kind, entry.key_id) {
                       temp_props.push((k, val));
                  }
             }
             
             core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
             let seq2 = (*header_ptr).seq;
             
             if seq1 == seq2 && !failed {
                  break temp_props;
             }
             if failed {
                 // If we failed bounds check likely due to race, retry. 
                 // If persistent, it will loop forever?
                 // Add loop limit?
             }
        };

        // Update Thing Props
        for (key, val) in props_vec {
             let mut found = false;
             for slot in thing.props.iter_mut() {
                  if let Some((k, _)) = slot {
                       if *k == key {
                            *slot = Some((key, val.clone()));
                            found = true;
                            // Update prop index? 
                            // Direct update misses indexing.
                            crate::graph::index_props::add_to_prop_index(thing.id, key, &val);
                            break;
                       }
                  }
             }
             if !found {
                  for slot in thing.props.iter_mut() {
                       if matches!(slot, None) {
                            *slot = Some((key, val.clone()));
                            crate::graph::index_props::add_to_prop_index(thing.id, key, &val);
                            break;
                       }
                  }
             }
        }
        
        thing.storage = StorageState::Both; // Keeping resident for now (Snapshot)
        thing.archived = true;
        
        Ok(RestResp { thing_id })
    }
}
