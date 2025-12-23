#[cfg(test)]
mod tests {
    use crate::graph;
    use crate::sched::{Scheduler, ThreadState, graph_sync};
    use crate::sched::MAX_THREADS;
    use abi::{ProcessId, ThreadId};
    use thing_models::PropValue;
    use alloc::string::String;
    use alloc::vec::Vec;

    #[test]
    #[ignore]
    fn test_run_queue_rebuild_equivalence() {
        let _guard = crate::test_lock();
        // Initialize subsystems
        // We need to be careful not to double-init if other tests ran.
        // But test_lock guards it.
        // crate::init() calls graph::store::init() which overwrites the slab.
        // This is fine for isolated tests.
        // However, crate::init calls log::init which might be safe.
        // Let's call minimal inits.

        crate::graph::store::init();
        crate::symbols::init();
        crate::graph::schema::init(); // needed for symbols/interning potentially

        // Setup Scheduler
        let mut scheduler = Scheduler::new();
        scheduler.init_graph_mirror();

        // 1. Create threads
        // Mock entry point
        let entry = unsafe { core::mem::transmute(0usize) };
        let pid = scheduler.add_process("test_proc");

        let tid1 = scheduler.add_thread(pid, "t1", entry, 0, 0, 10);
        let tid2 = scheduler.add_thread(pid, "t2", entry, 0, 0, 20);
        let tid3 = scheduler.add_thread(pid, "t3", entry, 0, 0, 5);

        // 2. Set threads to Runnable and Sync to Graph
        // add_thread puts them in New state in Graph.
        // We simulate running them and yielding them.

        // Update local state
        scheduler.thread_mut(tid1).unwrap().state = ThreadState::Runnable;
        scheduler.thread_mut(tid2).unwrap().state = ThreadState::Runnable;
        scheduler.thread_mut(tid3).unwrap().state = ThreadState::Runnable;

        // Manually sync to graph to establish "Graph Truth"
        for tid in [tid1, tid2, tid3] {
            let thread = scheduler.thread_mut(tid).unwrap();
            let thing = thread.thing_id.unwrap();
             let props = alloc::vec![
                 (crate::symbols::intern("state"), PropValue::Str(String::from(thread.state.as_str()))),
                 (crate::symbols::intern("priority"), PropValue::U64(thread.priority)),
             ];
             crate::graph::update_thing(thing, props);
        }

        // 3. Clear Cache (simulate crash or invalidation)
        scheduler.cache.run_queue.clear();

        // 4. Rebuild Cache
        let rebuilt = graph_sync::rebuild_cache_from_graph(&mut scheduler.threads);

        // 5. Verify
        assert_eq!(rebuilt.run_queue.len(), 3);

        // Expected order: Descending Priority: t2(20), t1(10), t3(5)
        assert_eq!(rebuilt.run_queue[0], tid2, "Highest priority should be first");
        assert_eq!(rebuilt.run_queue[1], tid1, "Medium priority should be second");
        assert_eq!(rebuilt.run_queue[2], tid3, "Lowest priority should be third");

        // 6. Test Sleep Queue Rebuild
        // Mark t1 as Sleeping
        scheduler.thread_mut(tid1).unwrap().state = ThreadState::Sleeping;
        scheduler.thread_mut(tid1).unwrap().sleep_until_ns = 1000;

        let thread1 = scheduler.thread_mut(tid1).unwrap();
        let thing1 = thread1.thing_id.unwrap();
        let props = alloc::vec![
             (crate::symbols::intern("state"), PropValue::Str(String::from(ThreadState::Sleeping.as_str()))),
             (crate::symbols::intern("sleep_until_ns"), PropValue::U64(1000)),
        ];
        crate::graph::update_thing(thing1, props);

        let rebuilt_sleep = graph_sync::rebuild_cache_from_graph(&mut scheduler.threads);

        assert_eq!(rebuilt_sleep.run_queue.len(), 2); // t2, t3
        assert_eq!(rebuilt_sleep.sleep_queue.len(), 1);
        assert_eq!(rebuilt_sleep.sleep_queue[0].thread_id, tid1);
        assert_eq!(rebuilt_sleep.sleep_queue[0].wake_at_ns, 1000);
    }
}
