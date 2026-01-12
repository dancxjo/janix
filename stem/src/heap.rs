use abi::errors::Errno;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use spin::Mutex;

use crate::utils::align_up;
use crate::vm::vm_map;

const PAGE_SIZE: usize = 4096;
const HEAP_BASE: usize = 0x2000_0000;
const HEAP_GROW_MIN: usize = 256 * 1024;

struct HeapState {
    base: usize,
    size: usize,
    initialized: bool,
}

static HEAP_STATE: Mutex<HeapState> = Mutex::new(HeapState {
    base: HEAP_BASE,
    size: 0,
    initialized: false,
});

pub fn grow_heap(min_bytes: usize) -> Result<(), Errno> {
    let mut state = HEAP_STATE.lock();
    let grow_bytes = align_up(core::cmp::max(min_bytes, HEAP_GROW_MIN), PAGE_SIZE);
    let map_addr = state.base + state.size;

    let req = VmMapReq {
        addr_hint: map_addr,
        len: grow_bytes,
        prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
        flags: VmMapFlags::FIXED | VmMapFlags::PRIVATE,
        backing: VmBacking::Anonymous { zeroed: true },
    };
    let resp = vm_map(&req)?;
    if resp.addr != map_addr {
        return Err(Errno::ENOMEM);
    }

    if state.initialized {
        crate::allocator::extend_heap(grow_bytes);
    } else {
        crate::allocator::init_heap(map_addr, grow_bytes);
        state.initialized = true;
    }
    state.size += grow_bytes;
    Ok(())
}
