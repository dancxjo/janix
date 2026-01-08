#![cfg(test)]

use super::memory;
use crate::sched;
use crate::sched::test_harness::{
    set_current_caps,
    set_current_task,
    spawn_sibling_task,
    translate_user_virt,
    with_test_task,
};
use abi::cap::{Cap, CapOp, CapScope};
use abi::syscall::err;
use alloc::vec;
use alloc::vec::Vec;

const PAGE_SIZE: u64 = 4096;

fn mem_caps() -> Vec<Cap> {
    vec![Cap {
        op: CapOp::MemManage,
        scope: CapScope::Global,
    }]
}

fn assert_disjoint(ranges: &[(u64, u64)]) {
    for i in 0..ranges.len() {
        for j in (i + 1)..ranges.len() {
            let (s0, e0) = ranges[i];
            let (s1, e1) = ranges[j];
            assert!(e0 <= s1 || e1 <= s0, "ranges overlap: {s0:#x}-{e0:#x} vs {s1:#x}-{e1:#x}");
        }
    }
}

#[test]
fn threads_share_brk_identity() {
    with_test_task(|tid0| {
        set_current_caps(tid0, mem_caps());
        let tid1 = spawn_sibling_task(tid0);
        set_current_caps(tid1, mem_caps());

        set_current_task(tid0);
        let a0 = memory::sys_heap_grow(0).val0;

        set_current_task(tid1);
        let b0 = memory::sys_heap_grow(0).val0;

        assert_eq!(a0, b0);
    });
}

#[test]
fn concurrent_sbrk_returns_disjoint_ranges() {
    with_test_task(|tid0| {
        set_current_caps(tid0, mem_caps());
        let tid1 = spawn_sibling_task(tid0);
        set_current_caps(tid1, mem_caps());

        let mut allocations: Vec<u64> = Vec::new();
        let rounds = 8;

        for _ in 0..rounds {
            set_current_task(tid0);
            let a = memory::sys_heap_grow(PAGE_SIZE);
            assert_eq!(a.status, 0);
            allocations.push(a.val0);

            set_current_task(tid1);
            let b = memory::sys_heap_grow(PAGE_SIZE);
            assert_eq!(b.status, 0);
            allocations.push(b.val0);
        }

        allocations.sort_unstable();
        for pair in allocations.windows(2) {
            let start = pair[0];
            let next = pair[1];
            assert!(next >= start + PAGE_SIZE, "overlap detected: {start:#x} then {next:#x}");
        }
    });
}

#[test]
fn sbrk_failure_does_not_advance_break() {
    with_test_task(|tid| {
        set_current_caps(tid, mem_caps());
        set_current_task(tid);

        let base = sched::with_current_task(|task| {
            let state = task.address_space.heap_state();
            task.address_space.set_heap_state(state.base, 0, state.base);
            state.base
        })
        .expect("current task required");

        let b0 = memory::sys_heap_grow(0);
        assert_eq!(b0.status, 0);
        assert_eq!(b0.val0, base);

        let fail = memory::sys_heap_grow(PAGE_SIZE * 4);
        assert_eq!(fail.status as i32, err::ENOMEM);

        let b1 = memory::sys_heap_grow(0);
        assert_eq!(b1.status, 0);
        assert_eq!(b1.val0, b0.val0);
    });
}

#[test]
fn brk_grow_maps_visible_to_other_thread() {
    with_test_task(|tid0| {
        set_current_caps(tid0, mem_caps());
        let tid1 = spawn_sibling_task(tid0);
        set_current_caps(tid1, mem_caps());

        set_current_task(tid0);
        let grow = memory::sys_heap_grow(PAGE_SIZE);
        assert_eq!(grow.status, 0);
        let addr = grow.val0;

        let ptr_a = translate_user_virt(tid0, addr).expect("translate from parent");
        unsafe { core::ptr::write_bytes(ptr_a, 0xAB, PAGE_SIZE as usize) };

        set_current_task(tid1);
        let ptr_b = translate_user_virt(tid1, addr).expect("translate from sibling");
        let slice = unsafe { core::slice::from_raw_parts(ptr_b, PAGE_SIZE as usize) };
        assert!(slice.iter().all(|&b| b == 0xAB));
    });
}

#[test]
fn thread_spawn_does_not_reset_or_desync_heap() {
    with_test_task(|tid0| {
        set_current_caps(tid0, mem_caps());
        let tid1 = spawn_sibling_task(tid0);
        set_current_caps(tid1, mem_caps());

        set_current_task(tid0);
        let p = memory::sys_heap_grow(PAGE_SIZE);
        assert_eq!(p.status, 0);

        set_current_task(tid1);
        let seen = memory::sys_heap_grow(0);
        assert_eq!(seen.status, 0);
        assert!(seen.val0 >= p.val0 + PAGE_SIZE);

        let q = memory::sys_heap_grow(PAGE_SIZE);
        assert_eq!(q.status, 0);

        set_current_task(tid0);
        let r = memory::sys_heap_grow(PAGE_SIZE);
        assert_eq!(r.status, 0);

        let ranges = [
            (p.val0, p.val0 + PAGE_SIZE),
            (q.val0, q.val0 + PAGE_SIZE),
            (r.val0, r.val0 + PAGE_SIZE),
        ];
        assert_disjoint(&ranges);
    });
}
