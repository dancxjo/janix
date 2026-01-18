use crate::root::graph::{Graph, ThingId, Commit, MAX_PENDING_COMMITS, MAX_PENDING_BYTES};
use crate::root::handlers::HandlerResult;
use abi::root::{BATCH_MAGIC, BATCH_VERSION, OP_CREATE_NODE, OP_PUT_EDGE, REF_ABSOLUTE, REF_LOCAL};
use abi::symbols::SymbolId;
use core::sync::atomic::Ordering;
use alloc::vec::Vec;

/// Handle SYS_ROOT_APPLY_BATCH
/// 
/// 1. Copy batch from user (conceptually, here we access raw ptr assuming access).
/// 2. Validate.
/// 3. Apply mutations.
/// 4. Notify watches.
use crate::root::symbols::Interner;
use alloc::string::String;

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    const HEX: &[u8] = b"0123456789abcdef";
    for &b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0xf) as usize] as char);
    }
    s
}

pub fn handle_apply_batch(
    graph: &mut Graph,
    interner: &mut Interner,
    batch: &[u8],
) -> (i32, u64) {

    
    let magic = u32::from_le_bytes(batch[0..4].try_into().unwrap());
    let version = u16::from_le_bytes(batch[4..6].try_into().unwrap());
    let op_count = u16::from_le_bytes(batch[6..8].try_into().unwrap());

    if magic != BATCH_MAGIC || version != BATCH_VERSION {
        return (-1, 0); 
    }
    
    let mut cursor = 8usize;
    let mut local_refs: Vec<ThingId> = Vec::with_capacity(16);
    let mut created_nodes: Vec<ThingId> = Vec::new();

    for _ in 0..op_count {
        if cursor >= batch.len() { return (-1, 0); }
        let tag = batch[cursor];
        cursor += 1;
        
        match tag {
            OP_CREATE_NODE => {
                if cursor + 16 > batch.len() { return (-1, 0); }
                let kind_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                cursor += 16;
                let kind_str = bytes_to_hex(&kind_bytes);
                let kind = interner.intern(&kind_str);
                
                if cursor + 2 > batch.len() { return (-1, 0); }
                let out_idx = u16::from_le_bytes(batch[cursor..cursor+2].try_into().unwrap()) as usize;
                cursor += 2;
                
                let new_id = graph.alloc(kind);
                created_nodes.push(new_id);
                if out_idx >= local_refs.len() {
                    local_refs.resize(out_idx + 1, 0);
                }
                local_refs[out_idx] = new_id;
            }
            OP_PUT_EDGE => {
                 let parse_ref = |cur: &mut usize, d: &[u8], locals: &[u64]| -> Option<u64> {
                     if *cur >= d.len() { return None; }
                     let k = d[*cur];
                     *cur += 1;
                     match k {
                         REF_ABSOLUTE => {
                             if *cur + 16 > d.len() { return None; }
                             let val = u64::from_le_bytes(d[*cur..*cur+8].try_into().unwrap());
                             *cur += 16;
                             Some(val)
                         }
                         REF_LOCAL => {
                             if *cur + 2 > d.len() { return None; }
                             let idx = u16::from_le_bytes(d[*cur..*cur+2].try_into().unwrap()) as usize;
                             *cur += 2;
                             if idx < locals.len() { Some(locals[idx]) } else { None }
                         }
                         _ => None
                     }
                 };
                 
                 let src = match parse_ref(&mut cursor, &batch, &local_refs) { Some(id) => id, None => return (-2, 0) };
                 
                 if cursor + 16 > batch.len() { return (-3, 0); }
                 let rel_bytes: [u8; 16] = batch[cursor..cursor + 16].try_into().unwrap();
                 cursor += 16;
                 let rel_str = bytes_to_hex(&rel_bytes);
                 let rel = interner.intern(&rel_str);
                 
                 let dst = match parse_ref(&mut cursor, &batch, &local_refs) { Some(id) => id, None => return (-4, 0) };
                 
                 graph.link(src, rel, dst);
            }
            _ => { return (-5, 0); }
        }
    }
    
    // 4. Commit Success
    let new_seq = graph.root_seq.fetch_add(1, Ordering::SeqCst) + 1;
    
    // 5. Notify Watches
    // Iterate all global watches.
    // If a watch matches the batch (simple filter: "all" or specific), push data.
    // Optimized: For v0 we push to ALL watches that subscribe to updates.
    // Filter logic: `kind_filter` etc.
    // Plan: "Filtering is computed during apply... if commit includes matching op".
    // For now, we just push everything to everyone for v0, unless we implement the filter scan.
    
    let commit = Commit {
        seq: new_seq,
        data: batch.to_vec(), // Move the buffer
    };
    
    // We need to clone the commit for each watcher (or Rc).
    // `GlobalWatch` stores `VecDeque<Commit>`. Commit owns data.
    // Deep copy is expensive.
    // Better: `Commit` should use `Arc<Vec<u8>>` or similar.
    // But `GlobalWatch` definition is `Vec<u8>`. 
    // We will clone for now (v0). Optimization: Rc<CommitData>.
    
    for watch in graph.global_watches.values_mut() {
        // Enforce limits
        while watch.pending.len() >= MAX_PENDING_COMMITS || watch.pending_bytes >= MAX_PENDING_BYTES {
            if let Some(dropped) = watch.pending.pop_front() {
                watch.pending_bytes -= dropped.data.len();
                watch.overflowed = true;
            } else {
                break;
            }
        }
        
        // Push
        let data_clone = commit.data.clone();
        watch.pending_bytes += data_clone.len();
        watch.pending.push_back(Commit { seq: new_seq, data: data_clone });
    }
    
    (0, new_seq)
}
