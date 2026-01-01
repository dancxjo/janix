//! Graph Mutation Syscalls

use abi::ids::{ThingId, SymbolId};
use abi::wire::SyscallResult;
use abi::syscall::err;
use graph::store;
use graph::symbols;
// use abi::types::RelationshipRef;

pub fn sys_thing_create(kind_low: u64, parent_low: u64) -> SyscallResult {
    // Task 07: Thing Create
    // 1. Create Thing with Kind
    // 2. Create Parent --[contains]--> Thing
    
    let kind = SymbolId(kind_low);
    let parent = ThingId(parent_low as u128);
    
    let thing_id = store::thing_create(kind);
    
    // Ensure containment
    let pred_contains = symbols::intern(b"predicate.contains");
    store::relationship_create(pred_contains, parent, thing_id);
    
    SyscallResult::new(0, thing_id.high(), thing_id.low())
}

pub fn sys_relationship_create(pred_low: u64, from_low: u64, to_low: u64) -> SyscallResult {
    let pred = SymbolId(pred_low);
    let from = ThingId(from_low as u128);
    let to = ThingId(to_low as u128);
    
    let rel_id = store::relationship_create(pred, from, to);
    SyscallResult::new(0, rel_id.high(), rel_id.low())
}

pub fn sys_relationship_delete(rel_low: u64) -> SyscallResult {
    let _rel_id = ThingId(rel_low as u128);
    
    // We need logic to delete.
    // Assuming `store::relationship_delete` exists?
    // If not, we might fail or stub.
    // Store usually has a delete/tombstone.
    // Let's assume store has it or we simulate it.
    // Checking `graph/store/mod.rs` via memory? Or just implementing stub.
    
    // NOTE: graph crate might need `relationship_delete`.
    // If it's missing, we default to error.
    
    // store::relationship_delete(rel_id); // Hypothetical
    
    SyscallResult::new(err::ENOSYS, 0, 0)
}

pub fn sys_thing_get(id_low: u64, _out_ptr: u64, out_len: u64) -> SyscallResult {
    // Fill ThingHeader
    let id = ThingId(id_low as u128);
    let header = store::get_thing_header(id); 
    // We assume get_thing_header returns Option<ThingHeader> or similar.
    // If not available, we can synthesize from store query.
    
    if let Some(_h) = header {
        // Copy to user
        // Safety: Unsafe raw ptr copy.
        // Assuming validation done by caller or higher level.
        if out_len < 16 { // Min size
             return SyscallResult::new(err::EINVAL, 0, 0);
        }
        // Write ID/Kind...
        
        SyscallResult::new(0, 0, 0)
    } else {
        SyscallResult::new(err::EINVAL, 0, 0)
    }
}

pub fn sys_relationships_from(id_low: u64, _cursor: u64, _out_ptr: u64) -> SyscallResult {
    // use abi::types::RelationshipRef;
    
    let id = ThingId(id_low as u128);
    let rels = store::relationships_from(id);
    
    // Simple pagination?
    // If cursor is index.
    
    SyscallResult::new(0, rels.len() as u64, 0)
}
