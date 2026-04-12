//! Beeper — generates a PCM tone and sends it to the sound driver.
//!
//! # IPC note (legacy deviation)
//!
//! This driver sends raw PCM audio data over a **channel** handle published in
//! `AudioInfoPayload`.  Raw PCM is a continuous byte stream with no message
//! boundaries — according to the IPC doctrine (`docs/concepts/channels_vs_pipes.md`)
//! it should be transported via a **pipe** (or a memfd-backed ring for
//! zero-copy).  Channels are used here only because `AudioInfoPayload` embeds a
//! bare channel handle number that any process can copy from a VFS file; plain
//! pipe FDs cannot be shared cross-process without a prior capability transfer.
//!
//! A future version should use:
//! 1. A discovery channel at `/services/sound/connect`.
//! 2. `channel_send_msg` to pass a pipe write-end FD to the connecting client.
//! 3. `vfs_write` / `vfs_read` for the PCM byte stream.
//!
//! Tracked as part of <https://github.com/dancxjo/thing-os/issues/591>.

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod chime;
mod tone;

use abi::schema::keys::WRITE_PORT_HANDLE;
use abi::schema::kinds::{DEV_SOUND, DEV_SOUND_HDA_PCI_STUB};
use alloc::vec::Vec;
use stem::syscall::{channel_send, vfs_fd_from_handle, ChannelHandle};
use stem::{info, warn};

#[stem::main]
fn main(_arg: usize) -> ! {
    let tone_freq: Option<f64> = None;
    let seconds: f64 = 1.0;

    let mut write_port_handle = 0;
    let mut sample_rate = 44100;

    info!("Beeper: Waiting for sound device via /services/sound/main...");

    while write_port_handle == 0 {
        use abi::sound::AudioInfoPayload;
        use abi::syscall::vfs_flags::O_RDONLY;
        use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};

        if let Ok(fd) = vfs_open("/services/sound/main", O_RDONLY) {
            let mut payload = AudioInfoPayload {
                magic: 0,
                write_handle: 0,
                read_handle: 0,
                sample_rate: 0,
                channels: 0,
                bits_per_sample: 0,
            };
            let slice = unsafe {
                core::slice::from_raw_parts_mut(
                    &mut payload as *mut _ as *mut u8,
                    abi::sound::AUDIO_INFO_PAYLOAD_SIZE,
                )
            };

            if let Ok(n) = vfs_read(fd, slice) {
                if n == abi::sound::AUDIO_INFO_PAYLOAD_SIZE
                    && payload.magic == AudioInfoPayload::MAGIC
                {
                    write_port_handle = payload.write_handle as ChannelHandle;
                    sample_rate = payload.sample_rate;
                    info!(
                        "Beeper: Connected to sound device (rate={}Hz, handle={})",
                        sample_rate, write_port_handle
                    );
                    let _ = vfs_close(fd);
                    break;
                }
            }
            let _ = vfs_close(fd);
        }
        stem::time::sleep_ms(100);
    }

    let samples: Vec<u8> = if let Some(freq) = tone_freq {
        info!("Beeper: Generating {}Hz sine wave for {}s", freq, seconds);
        let mut gen = tone::ToneGenerator::new(freq, sample_rate as f64);
        let count = (seconds * sample_rate as f64) as usize;
        let mut v_i16 = Vec::with_capacity(count * 2);
        unsafe { v_i16.set_len(count * 2) };
        gen.fill_buffer(&mut v_i16);

        let mut v_u8 = Vec::with_capacity(count * 4);
        for s in v_i16 {
            v_u8.extend_from_slice(&s.to_le_bytes());
        }
        v_u8
    } else {
        info!("Beeper: Generating classic chime...");
        chime::generate_chime(sample_rate)
    };

    info!("Beeper: Playback started ({} bytes)", samples.len());

    // Bridge the write port handle to a VFS FD for FD-first write-readiness polling.
    let write_port_fd = vfs_fd_from_handle(write_port_handle).unwrap_or(0);

    let chunk_size = 4096;

    let mut offset = 0;
    while offset < samples.len() {
        let remaining = samples.len() - offset;
        let to_write = remaining.min(chunk_size);
        let buf = &samples[offset..offset + to_write];

        let mut sent_bytes = 0;
        while sent_bytes < buf.len() {
            match channel_send(write_port_handle, &buf[sent_bytes..]) {
                Ok(n) if n > 0 => {
                    sent_bytes += n;
                }
                _ => {
                    // Port full — poll for write readiness (FD-first backpressure).
                    let mut pollfds = [abi::syscall::PollFd {
                        fd: write_port_fd as i32,
                        events: abi::syscall::poll_flags::POLLOUT,
                        revents: 0,
                    }];
                    let _ = stem::syscall::vfs::vfs_poll(&mut pollfds, u64::MAX);
                }
            }
        }

        offset += to_write;
    }

    info!("Beeper: Finished.");
    loop {
        stem::time::sleep_ms(1000);
    }
}
