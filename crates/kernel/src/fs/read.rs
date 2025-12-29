use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::ThingId;
use alloc::vec::Vec;
use thing_models::builtins::ids::{
    THING_BACKED_BY_KIND, THING_FILE_KIND, THING_LINK_KIND, THING_MODULE_KIND,
};

pub fn read_file_bytes<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    file_id: ThingId,
    offset: u64,
    len: u32,
) -> Result<Vec<u8>, ()> {
    // Allow reading either a File or a Module directly.
    let thing = kernel.graph.get(file_id).ok_or(())?;

    if thing.kind == THING_MODULE_KIND {
        return read_module_bytes(thing, offset, len);
    }

    if thing.kind != THING_FILE_KIND {
        kernel.bridge.log("Read failed: Not a file kind");
        return Err(());
    }

    // 2. Decode FileBody
    // ThingBody contains serialized TypedBytes (from main.rs)
    let tb = thing
        .body
        .decode::<abi::wire::typed::TypedBytes>()
        .map_err(|_| {
            kernel.bridge.log("Read failed: Decode TypedBytes failed");
            ()
        })?;

    let file_body =
        postcard::from_bytes::<thing_models::core::fs::FileBody>(&tb.bytes).map_err(|_| {
            kernel
                .bridge
                .log("Read failed: Decode file body from inner bytes failed");
            ()
        })?;

    // 3. Resolve Backing
    if (file_body.flags & 0x01) == 0 {
        kernel.bridge.log("Read failed: Provider not supported");
        return Err(());
    }

    // Prefer explicit BACKED_BY link
    let mut module_target: Option<ThingId> = None;
    for link in kernel.graph.list() {
        if link.kind != THING_LINK_KIND {
            continue;
        }
        if let Ok(tb_link) = link.body.decode::<abi::wire::typed::TypedBytes>() {
            if let Ok(link_body) =
                postcard::from_bytes::<thing_models::link::LinkBody>(&tb_link.bytes)
            {
                if link_body.from == file_id && link_body.predicate == THING_BACKED_BY_KIND {
                    module_target = Some(link_body.to);
                    break;
                }
            }
        }
    }

    // Fallback: name match against modules
    if module_target.is_none() {
        let mod_kind = THING_MODULE_KIND;
        let mut curr = ThingId(0);
        while let Some(mid) = kernel.graph.next_thing_of_kind(mod_kind, curr) {
            curr = mid;
            if let Some(mthing) = kernel.graph.get(mid) {
                if let Ok(mtb) = mthing.body.decode::<abi::wire::typed::TypedBytes>() {
                    if let Ok(mbody) = postcard::from_bytes::<
                        thing_models::builtins::core_kinds::ModuleBody,
                    >(&mtb.bytes)
                    {
                        if mbody.path == file_body.name {
                            module_target = Some(mid);
                            break;
                        }
                    }
                }
            }
        }
    }

    if let Some(mid) = module_target {
        if let Some(mthing) = kernel.graph.get(mid) {
            return read_module_bytes(mthing, offset, len);
        }
    }

    kernel.bridge.log("Read failed: Module not found for file");
    Err(())
}

fn read_module_bytes(module: &thing_models::Thing, offset: u64, len: u32) -> Result<Vec<u8>, ()> {
    let mtb = module
        .body
        .decode::<abi::wire::typed::TypedBytes>()
        .map_err(|_| ())?;
    let mbody = postcard::from_bytes::<thing_models::builtins::core_kinds::ModuleBody>(&mtb.bytes)
        .map_err(|_| ())?;

    let size = mbody.size_bytes;
    if offset >= size {
        return Ok(Vec::new());
    }
    let read_len = core::cmp::min(len as u64, size - offset) as usize;

    // Prefer embedded data, fallback to physical base.
    if !mbody.data.is_empty() {
        let start = offset as usize;
        let end = core::cmp::min(start.saturating_add(read_len), mbody.data.len());
        if start >= end || end > mbody.data.len() {
            return Ok(Vec::new());
        }
        return Ok(mbody.data[start..end].to_vec());
    }

    if mbody.base_phys != 0 {
        let src_ptr = mbody.base_phys as *const u8;
        let slice = unsafe { core::slice::from_raw_parts(src_ptr.add(offset as usize), read_len) };
        return Ok(slice.to_vec());
    }

    Err(())
}
