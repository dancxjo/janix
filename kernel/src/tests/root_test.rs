
use abi::wire::{ThingId, SymbolId};

pub fn run_selftest() {
    crate::kinfo!("ROOT SELFTEST: Starting...");

    // Create a stack buffer for ThingId output
    let mut bytespace_id_ptr = ThingId::default();

    // 1. Create Bytespace
    let bytespace_id = match crate::syscall::handlers::sys_root_bytespace_create(1024, 0, 0, &mut bytespace_id_ptr as *mut _ as usize) {
        Ok(_) => {
            // Bytespace ID is in bytespace_id_ptr
            // Wait, sys_root_bytespace_create returns Result<usize>, not ThingId.
            // The logic I wrote returns (i32, u64) in handler, but root_call converts.
            // But I updated `sys_root_bytespace_create` to accept `out_ptr`.
            // The handler should write to `out_ptr`.
            // So `bytespace_id_ptr` now contains the ID.
            crate::kinfo!("ROOT SELFTEST: Created bytespace");
            bytespace_id_ptr
        }
        Err(e) => {
            crate::kinfo!("ROOT SELFTEST: FAIL Bytespace create: {:?}", e);
            return;
        }
    };

    // 2. Get Kind
    // We need to pass ThingId pointer as ID, and SymbolId pointer as OUT.
    let mut kind_out = SymbolId::default();
    match crate::syscall::handlers::sys_root_get_kind(
        &bytespace_id as *const _ as usize,
        &mut kind_out as *mut _ as usize
    ) {
        Ok(_) => {
            crate::kinfo!(
                "ROOT SELFTEST: GetKind success"
            );
        }
        Err(e) => crate::kinfo!("FAIL GetKind: {:?}", e),
    }

    // 3. Watch Test
    let mask = 0xFFFF;
    let mut stream_id = ThingId::default();
    match crate::syscall::handlers::sys_root_watch_subscribe(
        &bytespace_id as *const _ as usize,
        mask,
        &mut stream_id as *mut _ as usize,
    ) {
        Ok(_) => {
            crate::kinfo!("ROOT SELFTEST: Subscribed stream");
        }
        Err(e) => {
            crate::kinfo!("ROOT SELFTEST: FAIL Subscribe: {:?}", e);
            return;
        }
    };

    // 5. Debug Print Test
    crate::kinfo!("ROOT SELFTEST: Testing GetKind Loop...");

    // Explicitly test signal propagation
    for i in 0..5 {
        crate::kinfo!("ROOT SELFTEST: Loop {}, sending GetKind...", i);
        match crate::syscall::handlers::sys_root_get_kind(
            &bytespace_id as *const _ as usize,
            &mut kind_out as *mut _ as usize
        ) {
            Ok(_) => {
                crate::kinfo!("ROOT SELFTEST: Loop {} Success", i);
            }
            Err(e) => {
                crate::kinfo!("ROOT SELFTEST: Loop {} FAIL: {:?}", i, e);
                break;
            }
        }
    }

    crate::kinfo!("ROOT SELFTEST: PASS");
}
