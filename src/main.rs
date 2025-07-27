use cpal::{traits::StreamTrait, I24};
use std::io;
use tracing_subscriber;

use pbnjam::audio::{write_stream_to_file, host_device_setup};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let (_host, device, config) = host_device_setup()?;
    let input_stream = match config.sample_format() {
        cpal::SampleFormat::I8 => write_stream_to_file::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => write_stream_to_file::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => write_stream_to_file::<i32>(&device, &config.into()),
        cpal::SampleFormat::F32 => write_stream_to_file::<f32>(&device, &config.into()),
        sample_format => Err(anyhow::Error::msg(format!(
            "Unsupported sample format '{sample_format}'"
        ))),
    }?;

    input_stream.play()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    tracing::info!("Stopping input stream");
    drop(input_stream);


    Ok(())
}
