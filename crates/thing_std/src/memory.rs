use super::*;

pub fn heap_grow(size: u64) -> u64 {
    let res = unsafe { syscall(nr::SYS_HEAP_GROW, size, 0, 0, 0, 0, 0) };
    if res.status == 0 {
        res.val0
    } else {
        0
    }
}

pub fn bytespace_create(size: u64) -> ThingId {
    let res = unsafe { syscall(nr::SYS_BYTESPACE_CREATE, size, 0, 0, 0, 0, 0) };
    ThingId::from_parts(res.val0, res.val1)
}

pub fn dma_bytespace_create(size: u64) -> (ThingId, u64) {
    let res = unsafe { syscall(nr::SYS_DMA_BYTESPACE_CREATE, size, 0, 0, 0, 0, 0) };
    if res.status != 0 {
        return (ThingId(0), 0);
    }
    let id = ThingId(res.val0 as u128);
    let phys_base = res.val1;
    (id, phys_base)
}

pub fn space_map(bs: ThingId, vaddr: u64, offset: u64, len: u64) -> u64 {
    unsafe {
        syscall(
            nr::SYS_SPACE_MAP,
            bs.low(),
            bs.high(),
            vaddr,
            offset,
            len,
            0,
        )
        .val0
    }
}

pub fn space_unmap(vaddr: u64, len: usize) -> i32 {
    unsafe {
        syscall(
            nr::SYS_SPACE_UNMAP,
            vaddr,
            len as u64,
            0,
            0,
            0,
            0,
        )
        .status as i32
    }
}

/// Bytespace creation and management
pub mod bytespace {
    use super::*;

    /// Create a RAM-backed bytespace of the given size
    pub fn create_ram(size: usize) -> ThingId {
        super::bytespace_create(size as u64)
    }
}
