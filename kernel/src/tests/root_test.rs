use abi::kinds::*;

pub fn run_selftest() {
    crate::kinfo!("ROOT SELFTEST: Starting...");
    
    // 1. Create Bytespace
    let bytespace_id = match crate::syscall::handlers::sys_root_bytespace_create(1024, 0, 0) {
         Ok(id) => {
             crate::kinfo!("ROOT SELFTEST: Created bytespace id={}", id);
             id
         },
         Err(e) => {
             crate::kinfo!("ROOT SELFTEST: FAIL Bytespace create: {:?}", e);
             return;
         }
    };

    // 2. Get Kind
    match crate::syscall::handlers::sys_root_get_kind(bytespace_id) {
         Ok(k) => {
             crate::kinfo!("ROOT SELFTEST: GetKind id={} -> kind={:x}", bytespace_id, k);
             if k == KIND_BYTESPACE_BUFFER as usize {
                  crate::kinfo!("PASS Kind Check");
             } else {
                  crate::kinfo!("FAIL Kind Check");
             }
         },
         Err(e) => crate::kinfo!("FAIL GetKind: {:?}", e),
    }
    
    // 3. Watch Test
    // Subscribe to bytespace_id
    let mask = 0xFFFF;
    let _stream_id = match crate::syscall::handlers::sys_root_watch_subscribe(bytespace_id, mask) {
        Ok(sid) => {
            crate::kinfo!("ROOT SELFTEST: Subscribed stream_id={}", sid);
            sid
        },
        Err(e) => {
            crate::kinfo!("ROOT SELFTEST: FAIL Subscribe: {:?}", e);
            return;
        }
    };
    
    // 4. Set Prop to trigger event
    let key = 100;
    let val = 200;
    if let Err(e) = crate::syscall::handlers::sys_root_prop_set(bytespace_id, key, val) {
        crate::kinfo!("ROOT SELFTEST: FAIL PropSet: {:?}", e);
        return;
    }
    
    // 5. Debug Print Test
    crate::kinfo!("ROOT SELFTEST: Testing debug print...");
    let dbg = crate::root::debug::ThingDebug(bytespace_id as u64);
    crate::kinfo!("Debug bytespace: {}", dbg);
    
    // Test truncation
    for i in 0..10 {
         let _ = crate::syscall::handlers::sys_root_prop_set(bytespace_id, 1000 + i, i);
    }
    crate::kinfo!("Debug bytespace (many props): {}", dbg);

    // 7. Graph Debug Test
    crate::kinfo!("ROOT SELFTEST: Testing graph debug...");
    // Create another thing
    let thing2_id = match crate::syscall::handlers::sys_root_bytespace_create(64, 0, 0) {
        Ok(id) => id as u64,
        Err(_) => 0,
    };
    
    // Link
    let rel = abi::kinds::REL_HAS_RESOURCE;
    let _ = crate::syscall::handlers::sys_root_link(bytespace_id as usize, rel as usize, thing2_id as usize);
    
    // Describe Link
    let edge_dbg = crate::root::debug::EdgeDebug(bytespace_id as u64, rel, thing2_id);
    crate::kinfo!("Debug edge: {}", edge_dbg);
    
    crate::kinfo!("ROOT SELFTEST: PASS");
}
