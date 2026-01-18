#![no_std]
#![no_main]

extern crate alloc;
use stem::syscall;
use abi::types::{WatchSpec, WatchEvent, WatchMode};
use abi::query::{QueryStep};
use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_STR};
mod behavior;
use behavior::sniff::sniff;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let _ = main();
    syscall::exit(0)
}

fn main() -> Result<(), abi::errors::Errno> {
    syscall::log_write("INGESTD: Starting...", 1)?;

    let steps: [QueryStep; 0] = [];
    let spec = WatchSpec {
        mode: WatchMode::QueryThenStream as u32,
        query_ptr: steps.as_ptr() as u64,
        query_len: steps.len() as u64,
        start_seq: 0,
        ..Default::default()
    };
    
    // We expect this to fail if kernel is not updated yet or similar, but
    // assuming kernel is ready.
    let watch_id = match syscall::root_watch_open(&spec) {
        Ok(id) => id,
        Err(e) => {
            syscall::log_write("INGESTD: Failed to open watch", 1)?;
            return Err(e);
        }
    };
    
    // Pre-intern symbols to avoid allocation in loop
    // BUT root_intern takes &str. stem wrapper handles ptr/len.
    // For root_create_node, we need a SymbolRefWire (ptr or id).
    // We can use SYMBOL_REF_TAG_STR with ptr.
    
    syscall::log_write("INGESTD: Watch active. Loop start.", 1)?;
    
    let mut seq_out = 0u64;
    let mut watch_buf = [0u8; 4096];
    loop {
         let res = syscall::root_watch_next(watch_id, &mut seq_out, &mut watch_buf);
         match res {
             Ok(len) if len > 0 => {
                 // Parse WatchEvent from the returned batch payload
                 if len >= core::mem::size_of::<WatchEvent>() {
                     let evt: WatchEvent = unsafe {
                         core::ptr::read_unaligned(watch_buf.as_ptr() as *const _)
                     };
                     // MatchFound
                     if evt.kind == 1 { 
                         process_asset(evt.node_id);
                     }
                 }
             }
             Ok(0) => {
                 syscall::sleep_ms(100);
             }
             Err(_) => {
                 syscall::sleep_ms(1000);
             }
             _ => {}
         }
    }
}

fn process_asset(id: u64) {
    let mut buf = [0u8; 4096];
    let len_res = syscall::root_bytespace_read(id as usize, 0, &mut buf);
    
    if let Ok(len) = len_res {
        if len == 0 { return; }
        
        let signature = &buf[..len];
        if let Some(guess) = sniff(signature) {
             let _ = write_fact(id, guess.mime, guess.confidence);
        } else {
             let _ = write_fact(id, "application/octet-stream", 0);
        }
    }
}

fn write_fact(target_id: u64, mime: &str, confidence: u16) -> Result<(), abi::errors::Errno> {
    // 1. Create content_type_fact node
    // kind = "fact.content_type"
    let kind_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: "fact.content_type".as_ptr() as u64,
        len: "fact.content_type".len() as u64,
    };
    // Note: stem::root_create_node expects usize ptr to Wire.
    let kind_ptr = &kind_sym as *const _ as usize;
    let fact_id = syscall::root_create_node(kind_ptr)?;
    
    // 2. Set props
    // mime
    set_prop_str(fact_id as usize, "mime", mime)?;
    // confidence
    set_prop_u64(fact_id as usize, "confidence", confidence as u64)?;
    // detector
    set_prop_str(fact_id as usize, "detector", "magic-v0")?;
    
    // 3. Link
    // rel = "has_fact"
    let rel_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: "has_fact".as_ptr() as u64,
        len: "has_fact".len() as u64,
    };
    syscall::root_link(target_id as usize, &rel_sym as *const _ as usize, fact_id as usize)?;
    
    syscall::log_write("INGESTD: Tagged asset", 1)?;
    Ok(())
}

fn set_prop_str(id: usize, key: &str, val: &str) -> Result<(), abi::errors::Errno> {
    // Intern key? Or use STR tag?
    // root_prop_set expects key_ptr (SymbolRefWire).
    // Value is u64. Strings must be interned or passed as blob?
    // Props are currently u64 values?
    // `RootOp::PropSet { value: u64 }`.
    // So strings MUST be interned or we need PropSetStr?
    // The kernel `root_handlers.rs` `prop_set` takes value: usize.
    // If I want to store a string, I must intern it first and store the SymbolId.
    
    let key_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: key.as_ptr() as u64,
        len: key.len() as u64,
    };
    
    let val_id = syscall::root_intern(val)?; // Intern value
    
    syscall::root_prop_set(id, &key_sym as *const _ as usize, val_id as usize)?;
    Ok(())
}

fn set_prop_u64(id: usize, key: &str, val: u64) -> Result<(), abi::errors::Errno> {
    let key_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: key.as_ptr() as u64,
        len: key.len() as u64,
    };
    syscall::root_prop_set(id, &key_sym as *const _ as usize, val as usize)?;
    Ok(())
}
