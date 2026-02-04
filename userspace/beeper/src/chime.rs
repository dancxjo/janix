use alloc::vec::Vec;
use core::f32::consts::PI;

pub fn generate_chime(sample_rate: u32) -> Vec<u8> {
    let duration_secs = 10.5; // Tripled length
    let total_samples = (sample_rate as f32 * duration_secs) as usize;
    let mut buffer = Vec::with_capacity(total_samples * 4); // Stereo S16LE = 4 bytes/sample

    // Parameters
    let f1 = 440.0;     // A4
    let f2 = 554.37;    // C#5
    let f3 = 659.25;    // E5
    let f4 = 880.0;     // A5 (shimmer)

    let attack = 0.2;   // Slightly slower attack
    let decay = 0.5;
    let sustain_level = 0.6;
    let release = 3.0; // Long, smooth fade out
    
    // Derived envelope timings
    let decay_start = (sample_rate as f32 * attack) as usize;
    let sustain_start = (sample_rate as f32 * (attack + decay)) as usize;
    let release_start = (sample_rate as f32 * (duration_secs - release)) as usize;

    for i in 0..total_samples {
        let t = i as f32 / sample_rate as f32;

        // Envelope
        let mut env = 0.0;
        if i < decay_start {
            // Attack
            env = t / attack;
        } else if i < sustain_start {
            // Decay
            let dt = (t - attack) / decay;
            env = 1.0 - (1.0 - sustain_level) * dt;
        } else if i < release_start {
            // Sustain
            env = sustain_level;
        } else {
            // Release
            let rt = (t - (duration_secs - release)) / release;
            if rt < 1.0 {
                env = sustain_level * (1.0 - rt);
            } else {
                env = 0.0;
            }
        }
        
        // Tremolo (gentle)
        env *= 0.98 + 0.02 * libm::sinf(2.0 * PI * 5.0 * t);

        // Synthesis
        let s1 = libm::sinf(2.0 * PI * f1 * t);
        let s2 = libm::sinf(2.0 * PI * f2 * t);
        let s3 = libm::sinf(2.0 * PI * f3 * t);
        let s4 = libm::sinf(2.0 * PI * f4 * t) * 0.12; // Shimmer

        let signal = (s1 + s2 + s3 + s4) / 3.2;
        let master_gain = 0.15;
        
        // Ensure no clicking - clamp
        let signal = if signal > 1.0 { 1.0 } else if signal < -1.0 { -1.0 } else { signal };

        let sample_l = signal * env * master_gain;
        let sample_r = signal * env * master_gain * 0.95; // Slight stereo width

        let pcm_l = (sample_l * 32000.0) as i16;
        let pcm_r = (sample_r * 32000.0) as i16;

        buffer.extend_from_slice(&pcm_l.to_le_bytes());
        buffer.extend_from_slice(&pcm_r.to_le_bytes());
    }

    buffer
}
