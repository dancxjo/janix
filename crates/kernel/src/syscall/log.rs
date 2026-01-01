//! Logging Syscall

use abi::wire::SyscallResult;
use abi::ids::ThingId;
use graph::symbols;
use graph::store;
use crate::log::Level;
use abi::syscall::err;

pub fn sys_log_emit(level_raw: u64, msg_ptr: u64, msg_len: u64) -> SyscallResult {
    // 1. Level
    let _level = match level_raw {
        0 => Level::Trace,
        1 => Level::Debug,
        2 => Level::Info,
        3 => Level::Warn,
        4 => Level::Error,
        _ => return SyscallResult::new(err::EINVAL, 0, 0),
    };
    
    // 2. Message
    // Safety: user ptr
    if msg_ptr == 0 || msg_len == 0 {
         return SyscallResult::new(err::EFAULT, 0, 0);
    }
    let msg = unsafe {
        core::slice::from_raw_parts(msg_ptr as *const u8, msg_len as usize)
    };

    // 3. Create Log Thing
    let kind_log = symbols::intern(b"kind.log_entry");
    let entry_id = store::thing_create(kind_log);
    
    // 4. Set Payload (Inline String)
    store::thing_set_inline_payload(entry_id, msg);
    
    // 5. Link to place.logs
    let place_logs = store::find_thing_by_name(symbols::intern(b"place.logs")).unwrap_or(ThingId(2)); // fallback root?
    let pred_contains = symbols::intern(b"predicate.contains");
    store::relationship_create(pred_contains, place_logs, entry_id);
    
    // 6. Link Level?
    // Optional.
    
    SyscallResult::new(0, entry_id.high(), entry_id.low())
}
