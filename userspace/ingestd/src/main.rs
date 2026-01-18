#![no_std]
#![no_main]

extern crate alloc;
use stem::syscall;
use abi::types::{WatchSpec, WatchEvent, WatchMode};
use abi::query::{QueryStep};
use abi::symbols::{SymbolRefWire, SYMBOL_REF_TAG_STR};
use abi::wire::ThingId;
use abi::symbols::SymbolId;
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
    
    let mut watch_id = ThingId::default();
    match syscall::root_watch_open(&spec, &mut watch_id) {
        Ok(_) => {},
        Err(e) => {
            syscall::log_write("INGESTD: Failed to open watch", 1)?;
            return Err(e);
        }
    };
    
    syscall::log_write("INGESTD: Watch active. Loop start.", 1)?;
    
    let mut seq_out = 0u64;
    let mut watch_buf = [0u8; 4096];
    loop {
         let res = syscall::root_watch_next(&watch_id, &mut seq_out, &mut watch_buf);
         match res {
             Ok(len) if len > 0 => {
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

fn process_asset(id: ThingId) {
    let mut buf = [0u8; 4096];
    let len_res = syscall::root_bytespace_read(&id, 0, &mut buf);
    
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

fn write_fact(target_id: ThingId, mime: &str, confidence: u16) -> Result<(), abi::errors::Errno> {
    let kind_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: "fact.content_type".as_ptr() as u64,
        len: "fact.content_type".len() as u64,
    };
    let kind_ptr = &kind_sym as *const _ as usize;
    let mut fact_id = ThingId::default();
    syscall::root_create_node(kind_ptr, &mut fact_id)?;
    
    set_prop_str(fact_id, "mime", mime)?;
    set_prop_u64(fact_id, "confidence", confidence as u64)?;
    set_prop_str(fact_id, "detector", "magic-v0")?;
    
    let rel_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: "has_fact".as_ptr() as u64,
        len: "has_fact".len() as u64,
    };
    syscall::root_link(&target_id, &rel_sym as *const _ as usize, &fact_id)?;
    
    syscall::log_write("INGESTD: Tagged asset", 1)?;
    Ok(())
}

fn set_prop_str(id: ThingId, key: &str, val: &str) -> Result<(), abi::errors::Errno> {
    let key_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: key.as_ptr() as u64,
        len: key.len() as u64,
    };
    
    let mut val_id = SymbolId::default();
    syscall::root_intern(val, &mut val_id)?;
    
    syscall::root_prop_set(&id, &key_sym as *const _ as usize, &val_id.0)?;
    Ok(())
}

fn set_prop_u64(id: ThingId, key: &str, val: u64) -> Result<(), abi::errors::Errno> {
    let key_sym = SymbolRefWire {
        tag: SYMBOL_REF_TAG_STR,
        ptr_or_id: key.as_ptr() as u64,
        len: key.len() as u64,
    };
    let mut val_buf = [0u8; 16];
    val_buf[0..8].copy_from_slice(&val.to_le_bytes());
    syscall::root_prop_set(&id, &key_sym as *const _ as usize, &val_buf)?;
    Ok(())
}
