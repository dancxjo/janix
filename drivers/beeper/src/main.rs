#![feature(restricted_std)]
#![no_main]

extern crate alloc;

mod chime;
mod tone;

use abi::schema::keys::WRITE_PORT_HANDLE;
use abi::schema::kinds::{DEV_SOUND, DEV_SOUND_HDA_PCI_STUB};
use alloc::vec::Vec;
use stem::info;
use stem::syscall::{channel_send, channel_wait, ChannelHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

#[stem::main]
fn main(_arg: usize) -> ! {
    let tone_freq: Option<f64> = None; // Args not supported yet in stem
    let seconds = 10.0;

    let mut dev_buf = [ThingId::default(); 1];
    let mut write_port_handle = 0;

    info!("Beeper: Waiting for sound device write port handle...");

    while write_port_handle == 0 {
        let mut devices = [ThingId::default(); 1];
        let mut found_id = ThingId::default();

        if thingsys::find(DEV_SOUND_HDA_PCI_STUB, &mut devices).unwrap_or(0) > 0 {
            found_id = devices[0];
        } else if thingsys::find(DEV_SOUND, &mut devices).unwrap_or(0) > 0 {
            found_id = devices[0];
        }

        if found_id.to_u64_lossy() != 0 {
            if let Ok(h) = thingsys::prop_get(found_id, WRITE_PORT_HANDLE) {
                if h != 0 {
                    write_port_handle = h as ChannelHandle;
                    break;
                }
            }
        }
        stem::time::sleep_ms(100); // Backoff if property hasn't been set yet
    }

    let sample_rate = 44100;
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
                    let _ = channel_wait(&[write_port_handle], abi::syscall::channel_wait::WRITABLE);
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
