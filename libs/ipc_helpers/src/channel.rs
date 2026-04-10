//! Channel send/recv wrappers.
//!
//! These helpers sit on top of the raw `stem::syscall::channel` functions and
//! add framing, polling, and ergonomic error handling.

use abi::errors::Errno;
use stem::syscall::channel::{
    channel_close, channel_recv, channel_recv_handle, channel_send_all, channel_send_handle,
    channel_try_recv, ChannelHandle,
};

/// Send `data` over `handle`, retrying with `channel_try_recv` on `EAGAIN`
/// until the ring has space.  Yields the current task between retries to
/// avoid a busy-wait.
///
/// Returns `Err(Errno::EPIPE)` when the peer is gone.
pub fn send_all_blocking(handle: ChannelHandle, data: &[u8]) -> Result<(), Errno> {
    loop {
        match channel_send_all(handle, data) {
            Ok(_) => return Ok(()),
            Err(Errno::EAGAIN) => stem::syscall::yield_now(),
            Err(e) => return Err(e),
        }
    }
}

/// Attempt a non-blocking receive.  Returns `Ok(None)` when no data is ready
/// instead of `Err(Errno::EAGAIN)`.
pub fn try_recv_opt(
    handle: ChannelHandle,
    buf: &mut [u8],
) -> Result<Option<usize>, Errno> {
    match channel_try_recv(handle, buf) {
        Ok(n) => Ok(Some(n)),
        Err(Errno::EAGAIN) => Ok(None),
        Err(e) => Err(e),
    }
}

/// Block on `handle` until a complete message arrives, then return the byte
/// count.  The underlying `channel_recv` already blocks, so this is a thin
/// ergonomic wrapper.
pub fn recv_blocking(handle: ChannelHandle, buf: &mut [u8]) -> Result<usize, Errno> {
    channel_recv(handle, buf)
}

/// Send a capability handle over a channel and then send a control message.
///
/// The kernel queues the capability separately from the byte ring, so the
/// receiver must call `channel_recv_handle` before reading the control
/// message in order to correctly associate them.
pub fn send_cap_and_message(
    channel: ChannelHandle,
    cap: u32,
    message: &[u8],
) -> Result<(), Errno> {
    channel_send_handle(channel, cap)?;
    send_all_blocking(channel, message)
}

/// Receive a capability handle and an accompanying control message.
///
/// Returns `(new_fd, bytes_read)`.
pub fn recv_cap_and_message(
    channel: ChannelHandle,
    buf: &mut [u8],
) -> Result<(u32, usize), Errno> {
    let new_fd = channel_recv_handle(channel)?;
    let n = recv_blocking(channel, buf)?;
    Ok((new_fd, n))
}

/// A thin RAII wrapper that closes a channel handle on drop.
pub struct OwnedChannel(pub ChannelHandle);

impl OwnedChannel {
    pub fn new(handle: ChannelHandle) -> Self {
        Self(handle)
    }

    pub fn handle(&self) -> ChannelHandle {
        self.0
    }

    pub fn send(&self, data: &[u8]) -> Result<(), Errno> {
        send_all_blocking(self.0, data)
    }

    pub fn recv<'a>(&self, buf: &'a mut [u8]) -> Result<usize, Errno> {
        recv_blocking(self.0, buf)
    }

    /// Release the handle without closing it (e.g. to transfer ownership).
    pub fn into_inner(self) -> ChannelHandle {
        let h = self.0;
        core::mem::forget(self);
        h
    }
}

impl Drop for OwnedChannel {
    fn drop(&mut self) {
        channel_close(self.0).ok();
    }
}
