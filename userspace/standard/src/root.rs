pub use abi::root::{JournalOp, JournalIndex, WatchEvent, OpKind};
use stem::{sys_graph_append, sys_watch_create, sys_watch_next};

pub type Errno = i64;

pub fn graph_append(op: JournalOp) -> Result<JournalIndex, Errno> {
    let ret = unsafe { sys_graph_append(&op) };
    if ret == u64::MAX {
        Err(-1) // Generic error
    } else {
        Ok(ret)
    }
}

pub fn watch_create() -> Result<u64, Errno> {
    let ret = unsafe { sys_watch_create() };
    Ok(ret)
}

pub fn watch_next(id: u64) -> Result<Option<WatchEvent>, Errno> {
    let mut event: WatchEvent = unsafe { core::mem::MaybeUninit::zeroed().assume_init() };
    let code = unsafe { sys_watch_next(id, &mut event) };
    
    if code == 0 {
        Ok(Some(event))
    } else if code == -11 { // EAGAIN
        Ok(None)
    } else {
        Err(code)
    }
}
