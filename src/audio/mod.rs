use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, SampleRate, Stream, StreamConfig};
use crossbeam_channel::{unbounded, Sender};
use std::io::Write;
use std::thread;

pub struct AudioHandle {
    // These fields keep the resources alive - dropping the handle stops capture
    #[allow(dead_code)]
    stream: Stream,
    #[allow(dead_code)]
    writer_thread: thread::JoinHandle<()>,
}

impl AudioHandle {
    /// Stop audio capture and wait for the writer thread to finish
    pub fn stop(self) {
        // Dropping stream stops the audio capture
        drop(self.stream);
        // Wait for writer thread to finish writing any remaining data
        let _ = self.writer_thread.join();
    }
}

pub fn start_default_input_capture<W>(mut writer: W, sample_rate: u32, channels: u16) -> Result<AudioHandle>
where
    W: Write + Send + 'static,
{
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .context("No default input audio device available")?;

    let mut supported = device
        .supported_input_configs()
        .context("Query input configs failed")?;

    let config_range = supported
        .find(|c| {
            c.channels() == channels
                && c.min_sample_rate().0 <= sample_rate
                && c.max_sample_rate().0 >= sample_rate
        })
        .unwrap_or_else(|| supported.next().expect("At least one supported input config"));

    let sample_format = config_range.sample_format();

    let cfg = cpal::StreamConfig {
        channels: config_range.channels(),
        sample_rate: SampleRate(sample_rate),
        buffer_size: BufferSize::Default,
    };

    // Channel for non-blocking communication
    let (tx, rx) = unbounded::<Vec<i16>>();

    // Writer thread: writes PCM data to file or FFmpeg stdin
    let writer_thread = thread::spawn(move || {
        for chunk in rx.iter() {
            let _ = writer.write_all(bytemuck::cast_slice(&chunk));
        }
    });

    // Build and start CPAL stream
    let stream = match sample_format {
        SampleFormat::I16 => build_stream_i16(&device, &cfg, tx)?,
        SampleFormat::U16 => build_stream_u16(&device, &cfg, tx)?,
        SampleFormat::F32 => build_stream_f32(&device, &cfg, tx)?,
        _ => build_stream_f32(&device, &cfg, tx)?,
    };

    stream.play()?;
    Ok(AudioHandle { stream, writer_thread })
}

fn build_stream_i16(device: &cpal::Device, config: &StreamConfig, tx: Sender<Vec<i16>>) -> Result<Stream> {
    let err_fn = |e| eprintln!("audio input stream error: {e}");
    let stream = device.build_input_stream(
        config,
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
            let chunk: Vec<i16> = data.to_vec();
            let _ = tx.try_send(chunk);
        },
        err_fn,
        None,
    )?;
    Ok(stream)
}

fn build_stream_u16(device: &cpal::Device, config: &StreamConfig, tx: Sender<Vec<i16>>) -> Result<Stream> {
    let err_fn = |e| eprintln!("audio input stream error: {e}");
    let stream = device.build_input_stream(
        config,
        move |data: &[u16], _: &cpal::InputCallbackInfo| {
            let mut chunk = Vec::with_capacity(data.len());
            for &sample in data {
                // Convert u16 to i16: u16 range [0, 65535] -> i16 range [-32768, 32767]
                let val_i16 = sample as i32 - 32768;
                chunk.push(val_i16 as i16);
            }
            let _ = tx.try_send(chunk);
        },
        err_fn,
        None,
    )?;
    Ok(stream)
}

fn build_stream_f32(device: &cpal::Device, config: &StreamConfig, tx: Sender<Vec<i16>>) -> Result<Stream> {
    let err_fn = |e| eprintln!("audio input stream error: {e}");
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut chunk = Vec::with_capacity(data.len());
            for &sample in data {
                // Convert f32 [-1.0, 1.0] to i16
                let s16 = (sample.clamp(-1.0, 1.0) * i16::MAX as f32).round() as i16;
                chunk.push(s16);
            }
            let _ = tx.try_send(chunk);
        },
        err_fn,
        None,
    )?;
    Ok(stream)
}
