//! Kernel-side advisory file locking (flock semantics).
//!
//! This module implements a simple in-kernel advisory lock table indexed by
//! inode number.  It supports the four POSIX `flock(2)` operations:
//!
//! * `LOCK_SH` — shared (read) lock
//! * `LOCK_EX` — exclusive (write) lock
//! * `LOCK_NB` — non-blocking flag (can be OR-ed with `LOCK_SH`/`LOCK_EX`)
//! * `LOCK_UN` — unlock
//!
//! # Semantics
//!
//! Advisory locking is per-open-file.  In ThingOS, the lock state is tracked
//! per **inode number** (from [`VfsStat::ino`]).  When a process calls
//! `flock(fd, LOCK_UN)`, or the fd is closed while a lock is held, the entry
//! is removed from the table.
//!
//! If a blocking lock (`LOCK_SH` or `LOCK_EX` without `LOCK_NB`) cannot be
//! acquired immediately, the current implementation returns `EAGAIN`
//! (`EWOULDBLOCK`).  Full sleeping-wait support is deferred to a future
//! iteration.

use alloc::collections::BTreeMap;
use abi::errors::{Errno, SysResult};
use abi::syscall::flock_flags::{LOCK_EX, LOCK_SH, LOCK_UN};
use spin::Mutex;

/// State of the advisory lock held on a single inode.
#[derive(Default)]
struct FlockEntry {
    /// Number of active shared (read) lock holders.
    shared_count: usize,
    /// Whether an exclusive (write) lock is currently held.
    exclusive: bool,
}

impl FlockEntry {
    fn is_idle(&self) -> bool {
        self.shared_count == 0 && !self.exclusive
    }
}

/// Global advisory lock table: inode number → lock state.
static FLOCK_TABLE: Mutex<BTreeMap<u64, FlockEntry>> = Mutex::new(BTreeMap::new());

/// Apply a `flock(2)`-style lock operation to the inode identified by `ino`.
///
/// `how` is a combination of [`abi::syscall::flock_flags`] values.
///
/// # Errors
/// Returns [`Errno::EAGAIN`] (`EWOULDBLOCK`) when the lock cannot be acquired
/// immediately and `LOCK_NB` was specified (or when blocking would be required,
/// pending a full sleep-wait implementation).
pub fn flock(ino: u64, how: u32) -> SysResult<()> {
    if how & LOCK_UN != 0 {
        // Release any lock held on this inode.
        let mut table = FLOCK_TABLE.lock();
        if let Some(entry) = table.get_mut(&ino) {
            if entry.exclusive {
                entry.exclusive = false;
            } else if entry.shared_count > 0 {
                entry.shared_count -= 1;
            }
            if entry.is_idle() {
                table.remove(&ino);
            }
        }
        return Ok(());
    }

    let mut table = FLOCK_TABLE.lock();
    let entry = table.entry(ino).or_default();

    if how & LOCK_SH != 0 {
        // Shared lock: allowed when there is no exclusive holder.
        if entry.exclusive {
            // EAGAIN == EWOULDBLOCK on POSIX.
            return Err(Errno::EAGAIN);
        }
        entry.shared_count += 1;
        return Ok(());
    }

    if how & LOCK_EX != 0 {
        // Exclusive lock: only allowed when there are no other lock holders.
        if entry.exclusive || entry.shared_count > 0 {
            // EAGAIN == EWOULDBLOCK on POSIX.
            // TODO: sleep the calling thread and retry once the lock is released.
            return Err(Errno::EAGAIN);
        }
        entry.exclusive = true;
        return Ok(());
    }

    // Unknown operation — treat as invalid.
    Err(Errno::EINVAL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::errors::Errno;
    use abi::syscall::flock_flags::{LOCK_EX, LOCK_NB, LOCK_SH, LOCK_UN};

    /// Clean up any state for a test inode so tests don't interfere.
    fn release(ino: u64) {
        let mut table = FLOCK_TABLE.lock();
        table.remove(&ino);
    }

    #[test]
    fn shared_lock_succeeds_when_no_lock_held() {
        let ino = 0xF001;
        release(ino);
        assert_eq!(flock(ino, LOCK_SH), Ok(()));
        flock(ino, LOCK_UN).unwrap();
    }

    #[test]
    fn multiple_shared_locks_are_allowed() {
        let ino = 0xF002;
        release(ino);
        assert_eq!(flock(ino, LOCK_SH), Ok(()));
        assert_eq!(flock(ino, LOCK_SH), Ok(()));
        flock(ino, LOCK_UN).unwrap();
        flock(ino, LOCK_UN).unwrap();
    }

    #[test]
    fn exclusive_lock_succeeds_when_no_lock_held() {
        let ino = 0xF003;
        release(ino);
        assert_eq!(flock(ino, LOCK_EX), Ok(()));
        flock(ino, LOCK_UN).unwrap();
    }

    #[test]
    fn exclusive_lock_fails_when_shared_lock_held() {
        let ino = 0xF004;
        release(ino);
        flock(ino, LOCK_SH).unwrap();
        assert_eq!(flock(ino, LOCK_EX | LOCK_NB), Err(Errno::EAGAIN));
        flock(ino, LOCK_UN).unwrap();
    }

    #[test]
    fn shared_lock_fails_when_exclusive_lock_held() {
        let ino = 0xF005;
        release(ino);
        flock(ino, LOCK_EX).unwrap();
        assert_eq!(flock(ino, LOCK_SH | LOCK_NB), Err(Errno::EAGAIN));
        flock(ino, LOCK_UN).unwrap();
    }

    #[test]
    fn unlock_when_no_lock_held_is_noop() {
        let ino = 0xF006;
        release(ino);
        assert_eq!(flock(ino, LOCK_UN), Ok(()));
    }

    #[test]
    fn exclusive_lock_fails_when_another_exclusive_held() {
        let ino = 0xF007;
        release(ino);
        flock(ino, LOCK_EX).unwrap();
        assert_eq!(flock(ino, LOCK_EX | LOCK_NB), Err(Errno::EAGAIN));
        flock(ino, LOCK_UN).unwrap();
    }

    #[test]
    fn unlock_removes_entry_from_table() {
        let ino = 0xF008;
        release(ino);
        flock(ino, LOCK_EX).unwrap();
        flock(ino, LOCK_UN).unwrap();
        assert!(!FLOCK_TABLE.lock().contains_key(&ino));
    }

    #[test]
    fn invalid_how_returns_einval() {
        let ino = 0xF009;
        release(ino);
        assert_eq!(flock(ino, 0), Err(Errno::EINVAL));
    }
}
