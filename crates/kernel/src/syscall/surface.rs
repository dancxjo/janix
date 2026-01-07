//! Surface and Graphics Syscalls

use crate::memory::bytespace::Bytespace;
use crate::syscall::{cap, user_mem};
use abi::cap::CapOp;
use abi::ids::ThingId;
use abi::syscall::err;
use abi::wire::SyscallResult;
use graph::store;
use graph::symbols::sym;

pub fn sys_surface_create(width: u64, height: u64, format: u64) -> SyscallResult {
    if let Err(code) = user_mem::require_current_cap(CapOp::GraphCreate, None) {
        return SyscallResult::new(code, 0, 0);
    }

    crate::log::klog(
        crate::log::Level::Trace,
        "SURFACE",
        &alloc::format!("Create {}x{} format={}", width, height, format),
    );
    // 1. Calculate size (assume 4 bytes per pixel for now)
    let bpp = 4;
    let stride = width * bpp;
    let size = stride * height;

    // 2. Create RAM Bytespace
    let bs = match Bytespace::new_ram(size as usize) {
        Ok(bs) => bs,
        Err(_) => return SyscallResult::new(err::ENOMEM, 0, 0),
    };

    // 3. Create Surface Thing
    let surface_id = store::thing_create(sym::KIND_SURFACE);

    // 4. Link Surface -> Bytespace
    store::relationship_create(sym::PRED_BACKS, surface_id, bs.id);

    // 5. Set Properties
    let format_val_id = store::thing_create(sym::KIND_VALUE_U32);
    store::thing_set_inline_payload(format_val_id, &(format as u32).to_le_bytes());
    store::relationship_create(sym::PRED_FORMAT, surface_id, format_val_id);

    // Grant creator ownership (read/write on the created surface)
    cap::grant_creator_ownership(surface_id);

    let res = SyscallResult::new(0, surface_id.high(), surface_id.low());
    crate::log::klog(
        crate::log::Level::Trace,
        "SURFACE",
        &alloc::format!("Created surface ID={:x}:{:x}", res.val0, res.val1),
    );
    res
}

pub fn sys_surface_draw(
    id_low: u64,
    id_high: u64,
    buf_ptr: u64,
    coords: u64,
    geom: u64,
    _format: u64,
) -> SyscallResult {
    let surface_id = ThingId(((id_high as u128) << 64) | (id_low as u128));

    if let Err(code) = user_mem::require_current_cap(CapOp::GraphWrite, Some(surface_id)) {
        return SyscallResult::new(code, 0, 0);
    }

    // unpack coords and geom
    let _x = (coords >> 32) as u32;
    let _y = (coords & 0xFFFFFFFF) as u32;
    let w = (geom >> 32) as u32;
    let h = (geom & 0xFFFFFFFF) as u32;

    // crate::log::klog(crate::log::Level::Trace, "SURFACE", &alloc::format!("Draw id={:x}:{:x} buf={:x} size={}x{}", id_high, id_low, buf_ptr, w, h));
    // 1. Find the backing bytespace
    let bs_id = match store::relationships_from(surface_id)
        .into_iter()
        .filter_map(|rid| store::get_relationship(rid))
        .find(|rel| rel.kind == sym::PRED_BACKS)
    {
        Some(rel) => rel.to,
        None => return SyscallResult::new(err::EINVAL, 0, 0),
    };

    // 2. Get physical base from registry
    let phys_base = if let Some(info) = Bytespace::lookup(bs_id) {
        if let Some(phys) = info.phys_base {
            phys
        } else {
            return SyscallResult::new(err::EINVAL, 0, 0);
        }
    } else {
        return SyscallResult::new(err::EINVAL, 0, 0);
    };

    // 3. Map to Kernel (HHDM)
    let hhdm = crate::boot::get_boot_ctx().hhdm_offset;
    let target_ptr = (phys_base + hhdm) as *mut u8;

    // 4. Copy
    // For now, simplify to a straight copy of w*h*4 bytes
    // TODO: Handle x, y, stride
    let size = (w as usize)
        .checked_mul(h as usize)
        .and_then(|v| v.checked_mul(4))
        .ok_or(err::EINVAL);
    let Ok(size) = size else {
        return SyscallResult::new(err::EINVAL, 0, 0);
    };

    let mut target = unsafe { core::slice::from_raw_parts_mut(target_ptr, size) };
    if let Err(code) = user_mem::copy_from_user(&mut target, buf_ptr, size) {
        return SyscallResult::new(code, 0, 0);
    }

    SyscallResult::new(0, 0, 0)
}
