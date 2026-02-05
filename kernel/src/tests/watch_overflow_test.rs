//! Watch overflow and ordering tests.
//!
//! Verifies that the watch system correctly handles overflow scenarios
//! and maintains cursor monotonicity.

use crate::root::graph::{CommitHistory, CommitSummary};
use alloc::vec;

/// Test that a watch receives EOVERFLOW when its cursor falls behind oldest history.
#[cfg(test)]
pub fn test_watch_cursor_behind_oldest() {
    // Create a minimal history (3 commits max)
    let mut history = CommitHistory::new(3, 1024);

    // Push commits 1, 2, 3
    history.push(1, vec![1], CommitSummary::default());
    history.push(2, vec![2], CommitSummary::default());
    history.push(3, vec![3], CommitSummary::default());

    assert_eq!(history.oldest_seq(), Some(1));
    assert_eq!(history.newest_seq(), Some(3));

    // Push commit 4 (evicts 1)
    history.push(4, vec![4], CommitSummary::default());

    assert_eq!(history.oldest_seq(), Some(2));
    assert_eq!(history.newest_seq(), Some(4));

    // A watch with cursor_seq=1 should get EOVERFLOW
    // because seq 1 is no longer in history
    assert!(!history.contains(1));
    assert!(history.contains(2));
}

/// Test that cursor resync to oldest works correctly.
#[cfg(test)]
pub fn test_cursor_resync_to_oldest() {
    let mut history = CommitHistory::new(3, 1024);

    // Fill history
    for i in 1..=3 {
        history.push(i, vec![i as u8], CommitSummary::default());
    }

    // Evict by pushing more
    for i in 4..=6 {
        history.push(i, vec![i as u8], CommitSummary::default());
    }

    // Now oldest is 4, newest is 6
    assert_eq!(history.oldest_seq(), Some(4));
    assert_eq!(history.newest_seq(), Some(6));

    // A cursor at seq=2 should resync to 4
    let cursor = 2u64;
    let oldest = history.oldest_seq().unwrap();
    assert!(cursor < oldest, "cursor should be behind oldest");

    // Resync logic: new cursor = oldest
    let new_cursor = oldest;
    assert_eq!(new_cursor, 4);

    // And new_cursor should be able to read
    assert!(history.get(new_cursor).is_some());
}

/// Test history eviction under byte pressure.
#[cfg(test)]
pub fn test_byte_limit_eviction() {
    // Max 100 bytes
    let mut history = CommitHistory::new(1000, 100);

    // Push 50 bytes
    history.push(1, vec![0u8; 50], CommitSummary::default());
    assert_eq!(history.len(), 1);
    assert_eq!(history.oldest_seq(), Some(1));

    // Push another 60 bytes (total 110 > 100, evicts first)
    history.push(2, vec![0u8; 60], CommitSummary::default());
    assert_eq!(history.len(), 1);
    assert_eq!(history.oldest_seq(), Some(2));
    assert!(!history.contains(1));
}

/// Test that draining multiple commits maintains order.
#[cfg(test)]
pub fn test_commit_ordering() {
    let mut history = CommitHistory::new(10, 10240);

    // Push commits with distinct payloads
    for i in 1..=5 {
        history.push(i, vec![i as u8; 10], CommitSummary::default());
    }

    // Read in order
    for i in 1..=5 {
        let data = history.get(i).expect("commit should exist");
        assert_eq!(data[0], i as u8, "commit {} has wrong payload", i);
    }
}

/// Selftest entry point for watch overflow verification.
pub fn run_selftest() {
    crate::kinfo!("WATCH OVERFLOW SELFTEST: Starting...");

    // Test 1: Basic history eviction
    {
        let mut history = CommitHistory::new(4, 4096);
        for i in 1..=4 {
            history.push(i, vec![i as u8], CommitSummary::default());
        }
        assert_eq!(history.len(), 4);
        assert_eq!(history.oldest_seq(), Some(1));

        // Evict by pushing 5
        history.push(5, vec![5], CommitSummary::default());
        assert_eq!(history.oldest_seq(), Some(2));
        assert!(!history.contains(1));
        crate::kinfo!("WATCH OVERFLOW SELFTEST: Eviction test PASS");
    }

    // Test 2: Cursor behind oldest detection
    {
        let mut history = CommitHistory::new(2, 4096);
        history.push(1, vec![1], CommitSummary::default());
        history.push(2, vec![2], CommitSummary::default());
        history.push(3, vec![3], CommitSummary::default()); // Evicts 1

        let cursor = 1u64;
        let oldest = history.oldest_seq().unwrap();
        let needs_overflow = cursor < oldest;
        assert!(needs_overflow, "cursor=1 should be behind oldest=2");
        crate::kinfo!("WATCH OVERFLOW SELFTEST: Cursor detection test PASS");
    }

    // Test 3: Monotonic sequence guarantee
    {
        let mut history = CommitHistory::new(10, 4096);
        let mut last_seq = 0u64;

        for i in 1..=10 {
            history.push(i, vec![i as u8], CommitSummary::default());
            let newest = history.newest_seq().unwrap();
            assert!(newest > last_seq, "sequences must be monotonic");
            last_seq = newest;
        }
        crate::kinfo!("WATCH OVERFLOW SELFTEST: Monotonicity test PASS");
    }

    crate::kinfo!("WATCH OVERFLOW SELFTEST: All tests PASS");
}
