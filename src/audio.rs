use anyhow::{Result, Error};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample, FromSample, SizedSample};
use tracing::{info, error};

pub fn host_device_setup() -> Result<(cpal::Host, cpal::Device, cpal::SupportedStreamConfig), anyhow::Error> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::Error::msg("Default input device is not available"))?;
    info!("Input device : {}", device.name()?);

    let config = device.default_input_config()?;
    info!("Default input config : {:?}", config);

    Ok((host, device, config))
}

pub fn make_stream<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<cpal::Stream, Error> where T: SizedSample + FromSample<f32> + std::fmt::Debug {
    let num_channels = config.channels as usize;
    let err_fn = |err| eprintln!("Error building input sound stream: {}", err);

    let stream = device.build_input_stream(
        config.into(),
        move |input: &[T], _: &cpal::InputCallbackInfo| {
            process_frame(input, num_channels);
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

fn process_frame<SampleType>(
    input: &[SampleType],
    num_channels: usize,
) where
    SampleType: Sample + FromSample<f32> + std::fmt::Debug,
{
    println!("Input: {:?}", input);
}
