//! Audio capture via cpal. Records 10-second voice buffers for dictation.
//! Resamples to 16 kHz mono for Whisper input.
//! Enable with `--features audio` (requires libalsa on Linux).

#[cfg(feature = "audio")]
mod imp {
    use anyhow::{Context, Result};
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::sync::Mutex;
    use std::time::Duration;

    pub const TARGET_SAMPLE_RATE: u32 = 16000;
    pub const RECORD_DURATION_SECS: u64 = 10;

    /// f129=capture_audio. Record 10s via cpal, resample to 16kHz mono.
    pub fn capture_audio() -> Result<Vec<f32>> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .context("no default input device")?;
        let config = device
            .default_input_config()
            .context("no default input config")?;

        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        let samples = std::sync::Arc::new(Mutex::new(Vec::<f32>::new()));

        let err_fn = |e| eprintln!("cpal stream error: {}", e);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                let s = samples.clone();
                device.build_input_stream(
                    &config.into(),
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let mut v = s.lock().unwrap();
                        for frame in data.chunks(channels) {
                            let mono = frame.iter().sum::<f32>() / channels as f32;
                            v.push(mono);
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            cpal::SampleFormat::I16 => {
                let s = samples.clone();
                device.build_input_stream(
                    &config.into(),
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let mut v = s.lock().unwrap();
                        for frame in data.chunks(channels) {
                            let mono = frame.iter().map(|&x| x as f32 / 32768.0).sum::<f32>()
                                / channels as f32;
                            v.push(mono);
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            cpal::SampleFormat::U16 => {
                let s = samples.clone();
                device.build_input_stream(
                    &config.into(),
                    move |data: &[u16], _: &cpal::InputCallbackInfo| {
                        let mut v = s.lock().unwrap();
                        for frame in data.chunks(channels) {
                            let mono = frame
                                .iter()
                                .map(|&x| (x as f32 / 32768.0) - 1.0)
                                .sum::<f32>()
                                / channels as f32;
                            v.push(mono);
                        }
                    },
                    err_fn,
                    None,
                )?
            }
            _ => anyhow::bail!("unsupported sample format"),
        };

        stream.play().context("start stream")?;

        std::thread::sleep(Duration::from_secs(RECORD_DURATION_SECS));
        drop(stream);

        let mut samples = samples.lock().unwrap().clone();

        if sample_rate != TARGET_SAMPLE_RATE {
            samples = resample_to_16k(&samples, sample_rate)?;
        }

        Ok(samples)
    }

    /// f130=resample_to_16k. Linear interpolate to 16kHz.
    fn resample_to_16k(samples: &[f32], from_rate: u32) -> Result<Vec<f32>> {
        let from_rate = from_rate as f64;
        let to_rate = TARGET_SAMPLE_RATE as f64;

        if (from_rate - to_rate).abs() < 1.0 {
            return Ok(samples.to_vec());
        }

        let ratio = from_rate / to_rate;
        let out_len = (samples.len() as f64 / ratio) as usize;
        let mut out = Vec::with_capacity(out_len);

        for i in 0..out_len {
            let src_idx = i as f64 * ratio;
            let lo = src_idx.floor() as usize;
            let hi = (lo + 1).min(samples.len().saturating_sub(1));
            let frac = src_idx - lo as f64;
            let v = samples[lo] * (1.0 - frac as f32) + samples[hi] * frac as f32;
            out.push(v);
        }
        Ok(out)
    }
}

#[cfg(feature = "audio")]
pub use imp::*;

/// f129=capture_audio (stub when audio feature disabled).
#[cfg(not(feature = "audio"))]
pub fn capture_audio() -> anyhow::Result<Vec<f32>> {
    anyhow::bail!("audio feature not enabled; build with --features audio")
}

#[cfg(all(test, not(feature = "audio")))]
mod tests {
    use super::*;

    /// f129=capture_audio stub returns Err when audio disabled
    #[test]
    fn capture_audio_stub_returns_err() {
        let r = capture_audio();
        assert!(r.is_err());
        assert!(r.unwrap_err().to_string().contains("audio feature not enabled"));
    }
}
