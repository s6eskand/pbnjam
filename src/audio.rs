use anyhow::{Result, Error};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, FromSample, SizedSample};
use hound::{WavWriter, WavSpec};
use std::{fs::File, io::BufWriter, sync::{Arc, Mutex, MutexGuard}};

// TODO: Make configurable
const WAV_SPEC: WavSpec = WavSpec {
    channels: 1,
    sample_rate: 48000,
    bits_per_sample: 32,
    sample_format: hound::SampleFormat::Float,
};

pub fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::Error::msg("Default input device is not available"))?;
    tracing::info!("Input device : {}", device.name()?);

    let config = device.default_input_config()?;
    tracing::info!("Input config : {:?}", config);

    Ok((host, device, config))
}

pub fn write_stream_to_file<T>(
    device: &cpal::Device, 
    config: &cpal::StreamConfig
) -> Result<cpal::Stream, Error> 
where 
    T: SizedSample + FromSample<f32> + std::fmt::Debug + hound::Sample + hound::Sample,
{
    let num_channels = config.channels as usize;
    let err_fn = |err| eprintln!("Error building input sound stream: {}", err);

    let wav_writer = Arc::new(Mutex::new(WavWriter::create("output.wav", WAV_SPEC)?));

    let wav_writer_cloned = Arc::clone(&wav_writer);
    let stream = device.build_input_stream(
        config.into(),
        move |input: &[T], _: &cpal::InputCallbackInfo| {
            let mut writer_guard = wav_writer_cloned.lock().unwrap();
            process_frame(input, num_channels, &mut writer_guard);
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(
    input: &[SampleType],
    num_channels: usize,
    wav_writer: &mut MutexGuard<WavWriter<BufWriter<File>>>,
) where
    SampleType: Sample + FromSample<f32> + std::fmt::Debug + hound::Sample,
{
    for &sample in input {
        match wav_writer.write_sample(sample) {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Error writing sample to file: {}", e);
            }
        }
    }
}
