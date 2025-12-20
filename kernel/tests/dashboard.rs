#[test]
#[ignore]
fn test_dashboard_snapshot_from_boot_graph() {
    let _guard = kernel::test_lock();
    kernel::init();
    kernel::create_builtin_things();

    // Seed some data
    kernel::model::create_frame_pool(0x1000, 0x9000, 4096);
    kernel::model::create_cpu_core(0);

    let snapshot = kernel::model::dashboard_snapshot();

    // Verify counts
    assert!(snapshot.counts.total_things > 0);
    assert!(snapshot.counts.frame_pools == 1);
    assert!(snapshot.counts.cpu_cores == 1);

    // Verify memory summary consistency
    assert!(snapshot.memory.total_frames >= snapshot.memory.used_frames);
    assert_eq!(
        snapshot.memory.total_frames,
        snapshot.memory.used_frames + snapshot.memory.free_frames
    );
}
