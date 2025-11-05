use crate::cli::AudioSource;
use crate::encoder::FfmpegRecorder;
use anyhow::Result;

pub trait ScreenRecorder {
    fn start_recording(
        &self,
        output: &str,
        duration: Option<u32>,
        fps: u32,
        resolution: Option<&str>,
        audio: AudioSource,
        audio_device: Option<&str>,
    ) -> Result<()>;
}

impl ScreenRecorder for FfmpegRecorder {
    fn start_recording(
        &self,
        output: &str,
        duration: Option<u32>,
        fps: u32,
        resolution: Option<&str>,
        audio: AudioSource,
        audio_device: Option<&str>,
    ) -> Result<()> {
        FfmpegRecorder::start_recording(self, output, fps, resolution, duration, audio, audio_device)
    }
}

pub fn create_recorder() -> impl ScreenRecorder {
    FfmpegRecorder::new()
}


