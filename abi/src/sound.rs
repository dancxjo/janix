//! Sound Protocol and Payload Types

/// Service discovery payload published at `/services/sound/main`.
///
/// # IPC note (legacy deviation)
///
/// `write_handle` and `read_handle` are **channel** handle numbers.  They are
/// stored as plain integers so any process can read them from a VFS file and
/// use the handles directly, without a capability transfer.
///
/// Raw PCM audio is a continuous byte stream — according to the IPC doctrine
/// (`docs/concepts/channels_vs_pipes.md`) it should be transported via a
/// **pipe** (or a memfd-backed ring for zero-copy).  The channel is used here
/// only because of the plain-integer discovery mechanism.
///
/// The correct long-term design is:
/// 1. Publish a **channel** handle for the control/discovery endpoint.
/// 2. Use `channel_send_msg` to deliver a pipe write-end FD to each client.
/// 3. Use `vfs_write` / `vfs_read` for the raw PCM byte stream.
///
/// Tracked as part of <https://github.com/dancxjo/thing-os/issues/591>.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioInfoPayload {
    /// Magic number for identification: 0x50434d31 ("PCM1")
    pub magic: u32,
    /// Channel handle — write end (app → driver PCM byte stream).
    ///
    /// Legacy: this is a channel handle repurposed as a byte-stream endpoint.
    /// New code should obtain a pipe write-end FD via `channel_send_msg` on a
    /// dedicated connect channel instead.
    pub write_handle: u32,
    /// Channel handle — read end (driver ← app PCM byte stream).
    ///
    /// Legacy: see `write_handle`.
    pub read_handle: u32,
    /// Sample rate in Hz (e.g., 44100, 48000)
    pub sample_rate: u32,
    /// Number of channels (e.g., 1 for Mono, 2 for Stereo)
    pub channels: u32,
    /// Bits per sample (e.g., 16)
    pub bits_per_sample: u32,
}

impl AudioInfoPayload {
    pub const MAGIC: u32 = 0x50434d31;
}

pub const AUDIO_INFO_PAYLOAD_SIZE: usize = core::mem::size_of::<AudioInfoPayload>();
