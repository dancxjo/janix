#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod chime;
mod tone;

use abi::schema::keys::WRITE_PORT_HANDLE;
use abi::schema::kinds::{DEV_SOUND, DEV_SOUND_HDA_PCI_STUB};
use alloc::vec::Vec;
use stem::syscall::{channel_send, channel_wait, ChannelHandle};
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
                    // Port full, wait for space
                    let _ =
                        channel_wait(&[write_port_handle], abi::syscall::channel_wait::WRITABLE);
                }
            }
        }

        offset += to_write;
        // Rely on channel_wait(WRITABLE) for native backpressure instead of sleeping manually!
    }

    info!("Beeper: Finished.");
    loop {
        stem::time::sleep_ms(1000);
    }
}
