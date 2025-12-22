use kernel::log;

#[test]
fn log_stats_reset_on_init() {
    let _guard = kernel::test_lock();
    log::init();

    let stats = log::log_stats();
    assert_eq!(stats.stored_entries, 0);
    assert_eq!(stats.total_written, 0);
    assert_eq!(stats.overwritten, 0);
    assert_eq!(stats.truncated, 0);
}

#[test]
fn log_stats_track_truncation_and_overwrite() {
    let _guard = kernel::test_lock();
    log::init();

    // First message is intentionally long to trigger truncation.
    let long_msg = "x".repeat(300);
    log::log_message(&long_msg);

    // Now write enough messages to wrap the ring buffer and cause overwrites.
    for _ in 0..150 {
        log::log_message("short");
    }

    let stats = log::log_stats();
    assert_eq!(
        stats.stored_entries, 100,
        "ring buffer should retain its max size"
    );
    assert_eq!(stats.total_written, 151); // 1 long + 150 short
    assert_eq!(
        stats.overwritten, 51,
        "entries beyond capacity should be counted as overwrites"
    );
    assert_eq!(
        stats.truncated, 1,
        "only the first long message should be truncated"
    );
}
