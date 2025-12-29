use crate::client::GraphClient;
use crate::abi::{ThingId, SymbolId};
use crate::abi::wire::graph::{GraphOp, GraphReply};
use models::payload::*;

pub struct ResolvedImage2D {
    pub bytespace_id: ThingId,
    pub offset: u64,
    pub len: u64,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: SymbolId,
}

pub fn resolve_image2d(client: &GraphClient, owner_id: ThingId) -> Result<ResolvedImage2D, ()> {
    let mut buf = [0u8; 1024];

    // 1. Scan for HAS_VIEW from owner
    let op = GraphOp::ScanLinks { from: Some(owner_id), to: None, kind: Some(HAS_VIEW) };
    let reply = client.call_op(&op, &mut buf).map_err(|_| ())?;

    let view_id = if let GraphReply::Links(links) = reply {
        let mut found = None;
        for (_, to, _) in links {
             let mut buf2 = [0u8; 512];
             let op_get = GraphOp::GetThing { id: to };
             if let Ok(GraphReply::Thing { bytes }) = client.call_op(&op_get, &mut buf2) {
                 if postcard::from_bytes::<Image2D>(&bytes).is_ok() {
                     found = Some(to);
                     break;
                 }
             }
        }
        found.ok_or(())?
    } else { return Err(()); };

    // 2. Get Image2D payload
    let op_img = GraphOp::GetThing { id: view_id };
    let img = if let Ok(GraphReply::Thing { bytes }) = client.call_op(&op_img, &mut buf) {
        postcard::from_bytes::<Image2D>(&bytes).map_err(|_| ())?
    } else { return Err(()); };

    // 3. Scan for DATA from view_id to Region
    let op = GraphOp::ScanLinks { from: Some(view_id), to: None, kind: Some(DATA) };
    let region_id = if let Ok(GraphReply::Links(links)) = client.call_op(&op, &mut buf) {
        links.first().map(|(_, t, _)| *t).ok_or(())?
    } else { return Err(()); };

    // 4. Get Region payload
    let op_reg = GraphOp::GetThing { id: region_id };
    let region = if let Ok(GraphReply::Thing { bytes }) = client.call_op(&op_reg, &mut buf) {
        postcard::from_bytes::<Region>(&bytes).map_err(|_| ())?
    } else { return Err(()); };

    // 5. Scan for IN from region_id to ByteSpace
    let op = GraphOp::ScanLinks { from: Some(region_id), to: None, kind: Some(IN) };
    let bs_id = if let Ok(GraphReply::Links(links)) = client.call_op(&op, &mut buf) {
        links.first().map(|(_, t, _)| *t).ok_or(())?
    } else { return Err(()); };

    Ok(ResolvedImage2D {
        bytespace_id: bs_id,
        offset: region.offset,
        len: region.len,
        width: img.width,
        height: img.height,
        stride: img.stride,
        format: img.pixel_format,
    })
}
