use abi::ids::{RelationshipId, SymbolId, ThingId};
use abi::types::RelationshipRef;
use abi::syscall::nr;
use crate::syscall;
use alloc::vec::Vec;
use thing_codec::GraphClient;

pub struct SyscallGraphClient;

impl GraphClient for SyscallGraphClient {
    fn create_thing(&mut self, kind: SymbolId) -> Result<ThingId, i32> {
        // We use 0 for parent in the new API for now, or we might need to adjust this
        let res = unsafe {
            syscall(
                nr::SYS_THING_CREATE,
                kind.0,
                0, // No parent specified in trait API
                0,
                0,
                0,
                0,
            )
        };
        Ok(ThingId::from_parts(res.val0, res.val1))
    }

    fn set_body(&mut self, id: ThingId, body: &[u8]) -> Result<(), i32> {
        let res = unsafe {
            syscall(
                nr::SYS_THING_SET_BODY,
                id.low(),
                body.as_ptr() as u64,
                body.len() as u64,
                0,
                0,
                0,
            )
        };
        if res.status != 0 {
            return Err(res.status as i32);
        }
        Ok(())
    }

    fn get_body(&self, id: ThingId) -> Result<Vec<u8>, i32> {
        let res = unsafe { syscall(nr::SYS_THING_GET, id.low(), 0, 0, 0, 0, 0) };
        if res.val1 == 0 && res.status != 0 {
            return Err(res.status as i32);
        }

        let size = res.val1 as usize;
        let mut buf = Vec::with_capacity(size);
        unsafe {
            buf.set_len(size);
            let res2 = syscall(
                nr::SYS_THING_GET,
                id.low(),
                buf.as_mut_ptr() as u64,
                size as u64,
                0,
                0,
                0,
            );
            if res2.val1 == 0 && res2.status != 0 {
                return Err(res2.status as i32);
            }
        }
        Ok(buf)
    }
}

pub fn thing_create(kind: SymbolId, graph: ThingId) -> ThingId {
    let res = unsafe {
        syscall(
            nr::SYS_THING_CREATE,
            kind.0,
            graph.0 as u64,
            0,
            0,
            0,
            0,
        )
    };
    ThingId::from_parts(res.val0, res.val1)
}

pub fn thing_set_body(id: ThingId, body: &[u8]) -> Result<(), i32> {
    SyscallGraphClient.set_body(id, body)
}

pub fn thing_get_body(id: ThingId) -> Option<(Vec<u8>, u64)> {
    let res = unsafe { syscall(nr::SYS_THING_GET, id.low(), 0, 0, 0, 0, 0) };
    if res.val1 == 0 && res.status != 0 {
        return None;
    }

    let digest = res.val0;
    let size = res.val1 as usize;
    let mut buf = Vec::with_capacity(size);
    unsafe {
        buf.set_len(size);
        let res2 = syscall(
            nr::SYS_THING_GET,
            id.low(),
            buf.as_mut_ptr() as u64,
            size as u64,
            0,
            0,
            0,
        );
        if res2.val1 == 0 && res2.status != 0 {
            return None;
        }
    }
    Some((buf, digest))
}

pub fn thing_register_name(id: ThingId, name: &str) {
    unsafe {
        syscall(
            nr::SYS_THING_REGISTER_NAME,
            id.low(),
            name.as_ptr() as u64,
            name.len() as u64,
            0,
            0,
            0,
        );
    }
}

pub fn thing_find(name: &str) -> Option<ThingId> {
    let res = unsafe {
        syscall(
            nr::SYS_THING_FIND,
            name.as_ptr() as u64,
            name.len() as u64,
            0,
            0,
            0,
            0,
        )
    };
    if res.status != 0 {
        None
    } else {
        Some(ThingId::from_parts(res.val0, res.val1))
    }
}

pub fn symbol_intern(name: &str) -> SymbolId {
    let res = unsafe {
        syscall(
            nr::SYS_SYMBOL_INTERN,
            name.as_ptr() as u64,
            name.len() as u64,
            0,
            0,
            0,
            0,
        )
    };
    SymbolId(res.val0)
}

pub fn symbol_resolve(id: SymbolId) -> Option<Vec<u8>> {
    let res = unsafe { syscall(nr::SYS_SYMBOL_RESOLVE, id.0, 0, 0, 0, 0, 0) };
    if res.val1 == 0 {
        return None;
    }

    let size = res.val1 as usize;
    let mut buf = Vec::with_capacity(size);
    unsafe {
        buf.set_len(size);
        syscall(
            nr::SYS_SYMBOL_RESOLVE,
            id.0,
            buf.as_mut_ptr() as u64,
            size as u64,
            0,
            0,
            0,
        );
    }
    Some(buf)
}

pub fn relationship_create(kind: SymbolId, from: ThingId, to: ThingId) -> RelationshipId {
    let res = unsafe {
        syscall(
            nr::SYS_REL_CREATE,
            kind.0,
            from.low(),
            to.low(),
            0,
            0,
            0,
        )
    };
    RelationshipId::from_parts(res.val0, res.val1)
}

/// Fetch relationships originating from a Thing.
///
/// Returns `(count_returned, total_available)` on success.
pub fn relationships_from(
    from: ThingId,
    cursor: u64,
    out: &mut [RelationshipRef],
) -> Result<(u64, u64), i32> {
    if out.is_empty() {
        return Ok((0, 0));
    }
    let res = unsafe {
        syscall(
            nr::SYS_REL_GET_FROM,
            from.low(),
            cursor,
            out.as_mut_ptr() as u64,
            out.len() as u64,
            0,
            0,
        )
    };
    if res.status != 0 {
        return Err(res.status as i32);
    }
    Ok((res.val0, res.val1))
}
