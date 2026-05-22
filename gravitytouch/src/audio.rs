// Generates a descending-chirp WAV (80 ms, 44100 Hz, 16-bit mono) in memory,
// so no external sound file is needed and the binary works in WASM unchanged.
pub fn generate_bounce_wav() -> Vec<u8> {
    const SAMPLE_RATE: u32 = 44100;
    const DURATION_MS: u32 = 80;
    let num_samples = (SAMPLE_RATE * DURATION_MS / 1000) as usize;

    let mut samples: Vec<i16> = Vec::with_capacity(num_samples);
    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32;
        let progress = i as f32 / num_samples as f32;
        let freq = 800.0 - 500.0 * progress;
        let env = (1.0 - progress).powi(2);
        let sample = (env * 0.7 * (2.0 * std::f32::consts::PI * freq * t).sin()
            * i16::MAX as f32) as i16;
        samples.push(sample);
    }

    let data_size = (num_samples * 2) as u32;
    let mut wav: Vec<u8> = Vec::with_capacity(44 + data_size as usize);
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());           // PCM
    wav.extend_from_slice(&1u16.to_le_bytes());           // mono
    wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    wav.extend_from_slice(&(SAMPLE_RATE * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());           // block align
    wav.extend_from_slice(&16u16.to_le_bytes());          // bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_size.to_le_bytes());
    for s in samples {
        wav.extend_from_slice(&s.to_le_bytes());
    }
    wav
}
