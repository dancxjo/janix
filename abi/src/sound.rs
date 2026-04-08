//! Sound Protocol and Payload Types

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AudioInfoPayload {
    /// Magic number for identification: 0x50434d31 ("PCM1")
    pub magic: u32,
    /// Handle to the write port (app -> driver)
    pub write_handle: u32,
    /// Handle to the read port (driver -> app)
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
