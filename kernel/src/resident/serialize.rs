use crate::graph::store::{self, StorageState, ArchiveRef};
use crate::graph::schema;
use abi::{ThingId, PropValue};
use abi::resident::{RestPolicy, RestResp, ResidentError, ResidentErrorCode};
use abi::resident_layout::{ResidentHeader, ResPropEntry, ResTag};
use crate::memory::hhdm;
use alloc::vec::Vec;
use alloc::string::String;

enum PropValueView<'a> {
    U64(u64),
    I64(i64),
    Bool(bool),
    Str(&'a str),
    Bytes(&'a [u8]),
}

impl<'a> PropValueView<'a> {
    fn to_prop_value(&self) -> PropValue {
        match self {
            Self::U64(v) => PropValue::U64(*v),
            Self::I64(v) => PropValue::I64(*v),
            Self::Bool(v) => PropValue::Bool(*v),
            Self::Str(s) => PropValue::Str(String::from(*s)),
            Self::Bytes(_) => PropValue::Str(String::from("<bytes>")), // Placeholder for now
        }
    }
}

fn cbor_encode_header(val: u64, major: u8, out: &mut Vec<u8>) {
    if val < 24 {
        out.push((major << 5) | (val as u8));
    } else if val <= 0xFF {
        out.push((major << 5) | 24);
        out.push(val as u8);
    } else if val <= 0xFFFF {
        out.push((major << 5) | 25);
        out.extend_from_slice(&(val as u16).to_be_bytes());
    } else if val <= 0xFFFF_FFFF {
        out.push((major << 5) | 26);
        out.extend_from_slice(&(val as u32).to_be_bytes());
    } else {
        out.push((major << 5) | 27);
        out.extend_from_slice(&val.to_be_bytes());
    }
}

fn cbor_encode_kv(out: &mut Vec<u8>, key: &str, val: &PropValueView) {
    // Encode Key (Text String, Maj 3)
    cbor_encode_header(key.len() as u64, 3, out);
    out.extend_from_slice(key.as_bytes());

    // Encode Value
    match val {
        PropValueView::U64(v) => cbor_encode_header(*v, 0, out),
        PropValueView::I64(v) => {
            if *v >= 0 {
                cbor_encode_header(*v as u64, 0, out);
            } else {
                cbor_encode_header((-1 - *v) as u64, 1, out);
            }
        },
        PropValueView::Bool(v) => {
            out.push(if *v { 0xF5 } else { 0xF4 });
        },
        PropValueView::Str(s) => {
            cbor_encode_header(s.len() as u64, 3, out);
            out.extend_from_slice(s.as_bytes());
        },
        PropValueView::Bytes(b) => {
            cbor_encode_header(b.len() as u64, 2, out); // Maj 2 = Byte String
            out.extend_from_slice(b);
        }
    }
}

pub fn snapshot_and_archive(thing_id: ThingId, policy: RestPolicy) -> Result<RestResp, ResidentError> {
    unsafe {
        let mut guard = store::things_slab();
        let slab = guard.as_mut().unwrap();
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
        
        // Seqlock Retry Loop + Encode
        let cbor_blob = loop {
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
             
             let mut cbor_out = Vec::with_capacity(count as usize * 16); // heuristic
             
             // Begin Map (Maj 5)
             cbor_encode_header(count as u64, 5, &mut cbor_out);

             let mut failed = false;

             for i in 0..count {
                  let entry = *table_ptr.add(i as usize);
                  
                  let val_view = match entry.tag {
                        ResTag::U64 => PropValueView::U64(entry.v), // U64
                        ResTag::I64 => PropValueView::I64(entry.v as i64), // I64
                        ResTag::Bool => PropValueView::Bool(entry.v != 0), // Bool
                        ResTag::Str => { // Str
                            let off = entry.a;
                            let len = entry.b;
                            if (off + len) as u32 > header.total_len { failed = true; break; }
                            let str_ptr = (header_ptr as *const u8).add(off as usize);
                            let slice = core::slice::from_raw_parts(str_ptr, len as usize);
                            if let Ok(s) = alloc::str::from_utf8(slice) {
                                PropValueView::Str(s)
                            } else {
                                PropValueView::Str("<invalid utf8>")
                            }
                        },
                        ResTag::Bytes => { // Bytes
                            let off = entry.a;
                            let len = entry.b;
                            if (off + len) as u32 > header.total_len { failed = true; break; }
                            let ptr = (header_ptr as *const u8).add(off as usize);
                            let slice = core::slice::from_raw_parts(ptr, len as usize);
                            PropValueView::Bytes(slice)
                        },
                        ResTag::ThingId => PropValueView::U64(entry.v), // ThingId -> U64 for cbor
                  };
                  
                  if let Some(k) = schema::get_key_from_id(thing.kind, entry.key_id) {
                       cbor_encode_kv(&mut cbor_out, k, &val_view);
                  }
             }
             
             core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
             let seq2 = (*header_ptr).seq;
             
             if seq1 == seq2 && !failed {
                   break cbor_out;
              }
             if failed {
                 // Bounds error, retry or fail
                 // For now, if bounds fail, we assume race and retry.
                 // Limit retries?
             }
        };

        
        // 2. Archive Blob
        let archive_ref = store::archive_store().as_mut().unwrap().store(cbor_blob);
        thing.archived_ref = Some(archive_ref);

        // 3. Update State based on Policy
        match policy {
            RestPolicy::SnapshotKeepResident => {
                thing.storage = StorageState::Both; 
            },
            RestPolicy::SnapshotEvictResident => {
                thing.resident = None;
                thing.storage = StorageState::Archived;
            }
        }
        
        let archived_ref = if let Some(r) = &thing.archived_ref {
             abi::resident::ArchiveRef {
                 id: r.0,
             }
        } else {
             abi::resident::ArchiveRef::default()
        };

        Ok(RestResp { thing_id, archived_ref })
    }
}
