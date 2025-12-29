use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::{ThingId, SymbolId};
use alloc::vec::Vec;
use thing_models::payload::{Module, ByteSpace, File, ThingPayload, HAS_BYTES, BACKED_BY, HAS_MODULE};
use thing_models::link::LinkBody;

pub fn read_file_bytes<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    id: ThingId,
    offset: u64,
    len: u32,
) -> Result<Vec<u8>, ()> {
    // 1. Resolve to ByteSpace Thing
    let bytespace_id = resolve_to_bytespace(kernel, id)?;

    // 2. Get Backing ID
    let backing_id = kernel.bytespaces.get_backing_id(bytespace_id).ok_or(())?;

    // 3. Read
    let mut buf = alloc::vec![0u8; len as usize];
    let n = kernel.bytespaces.read(backing_id, offset, &mut buf);
    buf.truncate(n);
    Ok(buf)
}

fn resolve_to_bytespace<B: HardwareBridge>(kernel: &Kernel<B>, id: ThingId) -> Result<ThingId, ()> {
    let thing = kernel.graph.get(id).ok_or(())?;

    if thing.kind == ByteSpace::KIND {
        return Ok(id);
    }

    if thing.kind == Module::KIND {
        // Find HAS_BYTES link
        for link in kernel.graph.iter_kind(LinkBody::KIND) {
             if let Ok(lb) = postcard::from_bytes::<LinkBody>(&link.payload) {
                 if lb.from == id && lb.predicate == HAS_BYTES {
                     return Ok(lb.to);
                 }
             }
        }
        return Err(());
    }

    if thing.kind == File::KIND {
        // Find BACKED_BY or HAS_MODULE
        for link in kernel.graph.iter_kind(LinkBody::KIND) {
             if let Ok(lb) = postcard::from_bytes::<LinkBody>(&link.payload) {
                 if lb.from == id && (lb.predicate == BACKED_BY || lb.predicate == HAS_MODULE) {
                     return resolve_to_bytespace(kernel, lb.to);
                 }
             }
        }
    }

    Err(())
}
