use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::{ThingId, SymbolId};
use thing_models::payload::*;
use thing_models::link::LinkBody;
use thing_models::Thing;

#[derive(Debug)]
pub struct ResolvedRegion {
    pub bytespace_id: ThingId,
    pub backing_id: u64,
    pub offset: u64,
    pub len: u64,
}

#[derive(Debug)]
pub struct ResolvedImage2D {
    pub region: ResolvedRegion,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: SymbolId,
}

pub fn resolve_region<B: HardwareBridge>(kernel: &Kernel<B>, region_id: ThingId) -> Result<ResolvedRegion, ()> {
    // 1. Get Region Thing
    let region_thing = kernel.graph.get(region_id).ok_or(())?;
    if region_thing.kind != Region::KIND { return Err(()); }

    // 2. Decode Payload
    let region: Region = postcard::from_bytes(&region_thing.payload).map_err(|_| ())?;

    // 3. Find IN edge to ByteSpace
    let mut bytespace_id = None;
    for link in kernel.graph.iter_kind(LinkBody::KIND) {
        if let Ok(lb) = postcard::from_bytes::<LinkBody>(&link.payload) {
            if lb.from == region_id && lb.predicate == IN {
                bytespace_id = Some(lb.to);
                break;
            }
        }
    }
    let bytespace_id = bytespace_id.ok_or(())?;

    // 4. Validate ByteSpace
    let bs_thing = kernel.graph.get(bytespace_id).ok_or(())?;
    if bs_thing.kind != ByteSpace::KIND { return Err(()); }
    let bs: ByteSpace = postcard::from_bytes(&bs_thing.payload).map_err(|_| ())?;

    // 5. Get Backing
    let backing_id = kernel.bytespaces.get_backing_id(bytespace_id).ok_or(())?;

    // 6. Validate Bounds
    if region.offset + region.len > bs.len { return Err(()); }

    Ok(ResolvedRegion {
        bytespace_id,
        backing_id,
        offset: region.offset,
        len: region.len,
    })
}

pub fn resolve_image2d<B: HardwareBridge>(kernel: &Kernel<B>, owner_id: ThingId) -> Result<ResolvedImage2D, ()> {
    // 1. Find HAS_VIEW edge to Image2D
    let mut image_id = None;
    for link in kernel.graph.iter_kind(LinkBody::KIND) {
        if let Ok(lb) = postcard::from_bytes::<LinkBody>(&link.payload) {
            if lb.from == owner_id && lb.predicate == HAS_VIEW {
                // Check if target is Image2D
                if let Some(t) = kernel.graph.get(lb.to) {
                    if t.kind == Image2D::KIND {
                        image_id = Some(lb.to);
                        break;
                    }
                }
            }
        }
    }
    let image_id = image_id.ok_or(())?;

    // 2. Decode Image2D
    let img_thing = kernel.graph.get(image_id).ok_or(())?;
    let img: Image2D = postcard::from_bytes(&img_thing.payload).map_err(|_| ())?;

    // 3. Find DATA edge to Region
    let mut region_id = None;
    for link in kernel.graph.iter_kind(LinkBody::KIND) {
        if let Ok(lb) = postcard::from_bytes::<LinkBody>(&link.payload) {
            if lb.from == image_id && lb.predicate == DATA {
                region_id = Some(lb.to);
                break;
            }
        }
    }
    let region_id = region_id.ok_or(())?;

    // 4. Resolve Region
    let region = resolve_region(kernel, region_id)?;

    // 5. Validate Image fits in Region
    let required_len = (img.stride as u64) * (img.height as u64);
    if required_len > region.len { return Err(()); }

    Ok(ResolvedImage2D {
        region,
        width: img.width,
        height: img.height,
        stride: img.stride,
        format: img.pixel_format,
    })
}
