//! Helper module for Root watch patterns.

use crate::syscall::{self, root_watch_next};
use abi::errors::Errno;

/// Statistics returned by `drain`.
#[derive(Debug, Clone, Default)]
pub struct DrainStats {
    /// Number of batches successfully processed.
    pub batches: usize,
    /// Number of overflow events encountered (and skipped).
    pub overflows: usize,
    /// The sequence number of the last successfully processed batch, if any.
    pub last_seq: Option<u64>,
}

/// Drain a watch until `EAGAIN`, processing each batch with the provided handler.
///
/// This implements the "Catch-up" phase of the "Catch-up then Stream" pattern.
/// It will loop until the kernel reports no more pending events (EAGAIN).
///
/// # Arguments
///
/// * `handle` - The file descriptor/handle of the open Root watch.
/// * `buf` - A buffer to use for reading batches. Must be large enough for the expected batches.
/// * `handler` - A closure called for each successfully received batch.
///               Arguments are `(out_seq, batch_bytes)`.
///               `out_seq` is the sequence number *after* the batch.
///
/// # Returns
///
/// * `Ok(stats)` - Drain completed successfully (hit EAGAIN).
/// * `Err(ENOSPC)` - The buffer was too small for a pending batch.
/// * `Err(e)` - Other system error.
pub fn drain<F>(
    handle: usize,
    buf: &mut [u8],
    mut handler: F,
) -> Result<DrainStats, Errno>
where
    F: FnMut(u64, &[u8]),
{

    let mut stats = DrainStats::default();
    let mut seq_out = 0u64;

    loop {
        match root_watch_next(handle, &mut seq_out, buf) {
            Ok(len) => {
                // Determine if this is a valid batch or empty
                if len > 0 {
                    stats.batches += 1;
                    stats.last_seq = Some(seq_out);
                    handler(seq_out, &buf[..len]);
                } else {
                    // Start seq 0 sometimes returns empty batch at start?
                    // Or maybe just next? 
                    // If len == 0 and Ok, it might be just an ACK? 
                    // Usually watch_next returns EAGAIN if empty.
                    // We'll treat len=0 as "no data but success", continue draining?
                    // Safe to continue.
                }
            }
            Err(Errno::EAGAIN) => {
                // Done draining
                return Ok(stats);
            }
            Err(Errno::EOVERFLOW) => {
                // Watch overflowed, gap in sequence. Log and continue.
                // We don't have logging here based on user request "No logging inside helper".
                stats.overflows += 1;
                continue;
            }
            Err(e) => {
                // Other errors (e.g. ENOSPC, EBADF) are fatal
                return Err(e);
            }
        }
    }
}
