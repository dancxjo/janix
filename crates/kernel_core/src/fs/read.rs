use crate::Kernel;
use abi::ThingId;
use alloc::vec::Vec;
use hw::HardwareBridge;
use thing_models::builtins::ids::{THING_BACKED_BY_KIND, THING_FILE_KIND, THING_MODULE_KIND};

pub fn read_file_bytes<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    file_id: ThingId,
    offset: u64,
    len: u32,
) -> Result<Vec<u8>, ()> {
    // 1. Verify it's a File
    let thing = kernel.graph.get(file_id).ok_or(())?;
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
    if (file_body.flags & 0x01) != 0 {
        // RAMDISK: Find the module that backs this file

        let mod_kind = THING_MODULE_KIND;
        let mut curr = ThingId(0);

        while let Some(mid) = kernel.graph.next_thing_of_kind(mod_kind, curr) {
            curr = mid;
            if let Some(mthing) = kernel.graph.get(mid) {
                // Modules use TypedBytes wrapper too!
                if let Ok(mtb) = mthing.body.decode::<abi::wire::typed::TypedBytes>() {
                    if let Ok(mbody) = postcard::from_bytes::<
                        thing_models::builtins::core_kinds::ModuleBody,
                    >(&mtb.bytes)
                    {
                        // Match by name
                        if mbody.path == file_body.name {
                            let base = mbody.base_phys;
                            let size = mbody.size_bytes;

                            if offset >= size {
                                return Ok(Vec::new());
                            }

                            let read_len = core::cmp::min(len as u64, size - offset) as usize;

                            // SAFETY: Phsyical memory for modules is identity mapped or HHDM mapped.
                            let src_ptr = base as *const u8;
                            // kernel.bridge.log(alloc::format!("Reading file {} offset {} len {} from {:p}", file_body.name, offset, read_len, src_ptr).as_str());
                            let slice = unsafe {
                                core::slice::from_raw_parts(src_ptr.add(offset as usize), read_len)
                            };
                            let vec = slice.to_vec();
                            kernel.bridge.log("Read success");
                            return Ok(vec);
                        }
                    }
                }
            }
        }
        kernel.bridge.log("Read failed: Module not found for file");
        Err(())
    } else {
        kernel.bridge.log("Read failed: Provider not supported");
        Err(())
    }
}
