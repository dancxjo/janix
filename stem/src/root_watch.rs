//! Helper module for Root watch patterns.

use crate::syscall::root_watch_try_next;
use abi::errors::Errno;

/// Root watch handle type (syscall handle id).
pub type HandleId = usize;

/// Statistics returned by `watch_drain`.
#[derive(Debug, Clone, Default)]
pub struct DrainStats {
    /// Number of watch payloads successfully processed.
    pub batches: usize,
    /// Number of overflow events encountered (and skipped).
    pub overflows: usize,
    /// The sequence number of the last successfully processed batch, if any.
    pub last_seq: Option<u64>,
}

/// Drain a watch until `EAGAIN`, processing each watch payload with the provided handler.
///
/// This implements the "Catch-up" phase of the "Catch-up then Stream" pattern.
/// It will loop until the kernel reports no more pending events (EAGAIN).
///
/// # Arguments
///
/// * `handle` - The file descriptor/handle of the open Root watch.
/// * `buf` - A buffer to use for reading watch payloads. Must be large enough for the expected payloads.
/// * `handler` - A closure called for each successfully received watch payload.
///               Arguments are `(out_seq, batch_bytes)`.
///               `out_seq` is the sequence number *after* the payload.
///
/// # Returns
///
/// * `Ok(stats)` - Drain completed successfully (hit EAGAIN).
/// * `Err(ENOSPC)` - The buffer was too small for a pending batch.
/// * `Err(e)` - Other system error.
pub fn watch_drain<F>(handle: HandleId, buf: &mut [u8], mut handler: F) -> Result<DrainStats, Errno>
where
    F: FnMut(u64, &[u8]),
{
    let mut stats = DrainStats::default();
    let mut seq_out = 0u64;

    loop {
        match root_watch_try_next(handle, &mut seq_out, buf) {
            Ok(len) => {
                stats.batches += 1;
                stats.last_seq = Some(seq_out);
                handler(seq_out, &buf[..len]);
            }
            Err(Errno::EAGAIN) => {
                // Done draining
                return Ok(stats);
            }
            Err(Errno::EOVERFLOW) => {
                stats.overflows += 1;
                continue;
            }
            Err(Errno::ENOSPC) => {
                return Err(Errno::ENOSPC);
            }
            Err(e) => {
                // Other errors (e.g. ENOSPC, EBADF) are fatal
                return Err(e);
            }
        }
    }
}

#[deprecated(note = "use watch_drain instead")]
pub fn drain<F>(handle: HandleId, buf: &mut [u8], handler: F) -> Result<DrainStats, Errno>
where
    F: FnMut(u64, &[u8]),
{
    watch_drain(handle, buf, handler)
}
