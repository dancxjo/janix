#![cfg(test)]

use crate::sched::test_harness::{with_test_task, map_user_memory, set_current_caps};
use crate::memory::map::MapPerms;
use crate::syscall::{cpu, wait, watch};
use crate::watch as kernel_watch;
use abi::syscall::err;
use abi::cap::{Cap, CapOp, CapScope};
use abi::types::WatchKind;
use abi::ids::ThingId;
use alloc::vec;

#[test]
fn test_sys_cpu_features() {
    with_test_task(|tid| {
        let mut buf = [0u8; 128];
        let ptr = buf.as_mut_ptr() as u64;

        // Map user memory
        map_user_memory(tid, ptr, buf.len(), MapPerms::WRITE | MapPerms::USER);

        let res = cpu::sys_cpu_features(ptr, buf.len() as u64);
        assert_eq!(res.status, 0);
    });
}

#[test]
fn test_sys_wait_bad_ptr() {
    with_test_task(|tid| {
        // Must have CapOp::GraphWatch first
        set_current_caps(tid, vec![Cap {
            op: CapOp::GraphWatch,
            scope: CapScope::Global
        }]);

        // Pass bad pointer (0x1000 not mapped)
        let res = wait::sys_wait(0x1000, 1, 0, 0);
        assert_eq!(res.status as i32, err::EFAULT);
    });
}

#[test]
fn test_sys_watch_poll_write() {
    with_test_task(|tid| {
        let target = ThingId(1234);

        // Grant cap for create watch
        set_current_caps(tid, vec![Cap {
            op: CapOp::GraphWatch,
            scope: CapScope::Global
        }]);

        // Create watch
        let res = watch::sys_watch_create(WatchKind::Thing as u64, target.0 as u64);
        assert_eq!(res.status, 0);
        let wid = res.val0;

        // Trigger event
        kernel_watch::thing_updated(target);

        // Poll
        // WatchEvent size is 32 bytes (4 u64s)
        let mut buf = [0u8; 64];
        let ptr = buf.as_mut_ptr() as u64;
        map_user_memory(tid, ptr, buf.len(), MapPerms::WRITE | MapPerms::USER);

        let res = watch::sys_watch_poll(wid, ptr, 2); // try to read up to 2
        assert_eq!(res.status, 0);
        assert_eq!(res.val0, 1); // 1 event available

        // Verify write happened (non-zero bytes)
        assert!(buf.iter().any(|&b| b != 0));
    });
}

#[test]
fn test_boot_grants_application() {
    use crate::boot_grants;
    
    boot_grants::clear_grants();
    
    with_test_task(|tid| {
        // Register some boot grants for a fake module
        let module_id = ThingId(0xBEEF);
        let caps = vec![
            Cap { op: CapOp::Log, scope: CapScope::Global },
            Cap { op: CapOp::GraphRead, scope: CapScope::Global },
            Cap { op: CapOp::GraphWrite, scope: CapScope::Global },
        ];
        
        boot_grants::register_module_grants(module_id, Some("test_module".into()), caps.clone());
        
        // Verify the task starts with no caps (in test harness)
        set_current_caps(tid, vec![]);
        
        // Apply boot grants by name
        let mut task_caps = vec![];
        let count = boot_grants::apply_boot_grants_by_name("test_module", &mut task_caps);
        
        assert_eq!(count, 3, "Should have applied 3 capabilities");
        assert_eq!(task_caps.len(), 3);
        assert!(task_caps.contains(&Cap { op: CapOp::Log, scope: CapScope::Global }));
        assert!(task_caps.contains(&Cap { op: CapOp::GraphRead, scope: CapScope::Global }));
        assert!(task_caps.contains(&Cap { op: CapOp::GraphWrite, scope: CapScope::Global }));
    });
}

#[test]
fn test_creator_ownership_policy() {
    use crate::syscall::cap;
    
    with_test_task(|tid| {
        // Grant GraphCreate so we can create things
        set_current_caps(tid, vec![Cap {
            op: CapOp::GraphCreate,
            scope: CapScope::Global
        }]);
        
        // Create a fake "surface" Thing
        let surface_id = ThingId(0x1234_5678);
        
        // Apply creator ownership policy
        cap::grant_creator_ownership(surface_id);
        
        // Verify the current task now has read/write caps on the surface
        use crate::sched;
        sched::with_current_task(|task| {
            let has_read = task.caps.iter().any(|c| {
                c.op == CapOp::GraphRead && c.scope == CapScope::Thing(surface_id)
            });
            let has_write = task.caps.iter().any(|c| {
                c.op == CapOp::GraphWrite && c.scope == CapScope::Thing(surface_id)
            });
            
            assert!(has_read, "Creator should have GraphRead on created thing");
            assert!(has_write, "Creator should have GraphWrite on created thing");
        });
    });
}
