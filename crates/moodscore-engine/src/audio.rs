//! Audio output through cpal.

use std::fmt;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig};

use crate::{EngineHandle, Renderer};

/// A running audio output. Sound stops when this is dropped.
///
/// The underlying stream is not `Send` on every platform: keep this on the
/// thread that created it.
pub struct AudioOutput {
    _stream: cpal::Stream,
    pub device: String,
    pub sample_rate: u32,
    pub channels: u16,
}

#[derive(Debug)]
pub struct AudioError(String);

impl fmt::Display for AudioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for AudioError {}

fn err(context: &str, e: impl fmt::Display) -> AudioError {
    AudioError(format!("{context}: {e}"))
}

/// Open the system's default output device, create an engine at the device's
/// sample rate, and start playing.
///
/// `prepare` runs before the first sample is rendered: use it to set initial
/// targets so the engine starts on the right sound.
pub fn start_default_output(
    prepare: impl FnOnce(&EngineHandle),
) -> Result<(EngineHandle, AudioOutput), AudioError> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| AudioError("no default audio output device".into()))?;
    let device_name = device
        .description()
        .map(|d| d.to_string())
        .unwrap_or_else(|_| "default output".into());
    let supported = device
        .default_output_config()
        .map_err(|e| err("cannot read the output device configuration", e))?;
    let format = supported.sample_format();
    let config: StreamConfig = supported.into();

    let (handle, renderer) = crate::new(config.sample_rate);
    prepare(&handle);

    let stream = match format {
        SampleFormat::F32 => build::<f32>(&device, &config, renderer),
        SampleFormat::F64 => build::<f64>(&device, &config, renderer),
        SampleFormat::I16 => build::<i16>(&device, &config, renderer),
        SampleFormat::I32 => build::<i32>(&device, &config, renderer),
        SampleFormat::U16 => build::<u16>(&device, &config, renderer),
        SampleFormat::U8 => build::<u8>(&device, &config, renderer),
        other => return Err(AudioError(format!("unsupported sample format {other}"))),
    }?;
    stream
        .play()
        .map_err(|e| err("cannot start the audio stream", e))?;

    let output = AudioOutput {
        _stream: stream,
        device: device_name,
        sample_rate: config.sample_rate,
        channels: config.channels,
    };
    Ok((handle, output))
}

fn build<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    mut renderer: Renderer,
) -> Result<cpal::Stream, AudioError>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = config.channels as usize;
    device
        .build_output_stream(
            *config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                renderer.render_interleaved(data, channels);
            },
            |e| eprintln!("moodscore: audio stream error: {e}"),
            None,
        )
        .map_err(|e| err("cannot open the audio stream", e))
}
