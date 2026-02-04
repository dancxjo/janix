
#![no_std]
#![no_main]

extern crate alloc;

mod chime;
mod tone;

use alloc::vec::Vec;
use abi::schema::keys::WRITE_PORT_HANDLE;
use abi::schema::kinds::DEV_SOUND;
use stem::syscall::port::{port_send, port_wait, PortHandle};
use stem::{error, info};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;

#[stem::main]
fn main(_arg: usize) -> ! {
    let tone_freq: Option<f64> = None; // Args not supported yet in stem
    let seconds = 30.0;
    
    let mut dev_buf = [ThingId::default(); 1];
    let mut device_id = None;
    let mut write_port_handle = 0;
    
    for _ in 0..50 {
        if let Ok(count) = thingsys::find(DEV_SOUND, &mut dev_buf) {
            if count > 0 {
                let id = dev_buf[0];
                if let Ok(h) = thingsys::prop_get(id, WRITE_PORT_HANDLE) {
                    if h != 0 {
                        device_id = Some(id);
                        write_port_handle = h as PortHandle;
                        break;
                    }
                }
            }
        }
        stem::time::sleep_ms(100);
    }
    if write_port_handle == 0 {
        error!("Beeper: Device has no write port handle");
        loop { stem::yield_now(); }
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
            match port_send(write_port_handle, &buf[sent_bytes..]) {
                Ok(n) if n > 0 => {
                    sent_bytes += n;
                }
                _ => {
                    // Port full, wait for space
                    let _ = port_wait(&[write_port_handle], abi::syscall::port_wait::WRITABLE);
                }
            }
        }
        
        offset += to_write;
    }

    info!("Beeper: Finished.");
    loop { stem::time::sleep_ms(1000); }
}
