use crate::cli::AudioSource;
use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Mp4,
    WebM,
}

fn detect_output_format(output: &str) -> OutputFormat {
    let ext = Path::new(output)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    
    match ext.as_str() {
        "webm" => OutputFormat::WebM,
        _ => OutputFormat::Mp4, // Default to MP4
    }
}

pub struct FfmpegRecorder;

impl FfmpegRecorder {
    pub fn new() -> Self { Self }

    pub fn start_recording(
        &self,
        output: &str,
        fps: u32,
        resolution: Option<&str>,
        duration: Option<u32>,
        audio: AudioSource,
        audio_device: Option<&str>,
    ) -> Result<()> {
        if !ffmpeg_available() {
            bail!(
                "ffmpeg not found in PATH.\n\
                Please install ffmpeg:\n\
                - macOS: brew install ffmpeg\n\
                - Windows: Download from https://www.gyan.dev/ffmpeg/builds/ and add to PATH"
            );
        }
        
        let format = detect_output_format(output);
        let mut cmd = build_ffmpeg_cmd(output, fps, resolution, duration, audio, audio_device, format);
        
        let mut child = cmd.spawn()
            .with_context(|| format!("Failed to start ffmpeg process. Command: {:?}", cmd))?;

        // Wait for ffmpeg to finish
        let status = child.wait()
            .context("Failed to wait for ffmpeg process to complete")?;
        
        if !status.success() {
            let exit_code = status.code().unwrap_or(-1);
            bail!(
                "ffmpeg recording failed with exit code: {}\n\
                This may indicate:\n\
                - Insufficient permissions (screen recording/microphone access)\n\
                - Unsupported codec or format\n\
                - Invalid audio device\n\
                - Hardware encoder unavailable\n\
                Check ffmpeg output above for detailed error messages.",
                exit_code
            );
        }
        Ok(())
    }
}

fn ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(target_os = "macos")]
fn build_ffmpeg_cmd(
    output: &str,
    fps: u32,
    resolution: Option<&str>,
    duration: Option<u32>,
    audio: AudioSource,
    audio_device: Option<&str>,
    format: OutputFormat,
) -> Command {
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y");
    
    // Video input from avfoundation
    cmd.arg("-f").arg("avfoundation");
    cmd.arg("-pixel_format").arg("uyvy422");
    cmd.arg("-framerate").arg(fps.to_string());
    if let Some(res) = resolution { cmd.arg("-video_size").arg(res); }

    // Determine audio input - always use avfoundation device directly
    let avfoundation_input = if matches!(audio, AudioSource::None) {
        // No audio
        "1:none".to_string()
    } else {
        // Audio from avfoundation device
        let aud = if let Some(dev) = audio_device {
            dev.to_string()
        } else {
            // Default: use reasonable defaults based on common setups
            // Typical setup: [0] BlackHole (system audio), [1] Built-in Mic, [2] External Mic
            match audio {
                AudioSource::System => ":0".to_string(), // Typically BlackHole or system audio loopback at index 0
                AudioSource::Mic => ":1".to_string(),    // Typically built-in mic at index 1
                _ => ":0".to_string(),
            }
        };
        format!("1{}", aud)
    };
    cmd.arg("-i").arg(avfoundation_input);

    if let Some(d) = duration { cmd.arg("-t").arg(d.to_string()); }
    
    // Increase probesize to help ffmpeg detect stream parameters
    cmd.args(["-probesize", "50M"]);
    
    // Video encoding
    cmd.args(["-map", "0:v:0"]); // Map video from first input
    
    // Apply resolution scaling if specified
    if let Some(res) = resolution {
        cmd.args(["-vf", &format!("scale={}", res)]);
    }
    
    match format {
        OutputFormat::WebM => {
            // WebM requires VP8/VP9/AV1 - use VP9 with real-time encoding settings
            // -deadline realtime: prioritize speed over quality for real-time encoding
            // -g: keyframe interval (2 seconds at 30fps = 60 frames)
            // -threads: use multiple threads for faster encoding
            // -row-mt: enable row-based multithreading
            cmd.args([
                "-vcodec", "libvpx-vp9",
                "-b:v", "5M",
                "-deadline", "realtime",
                "-speed", "6",
                "-g", &(fps * 2).to_string(), // Keyframe every 2 seconds
                "-threads", "4",
                "-row-mt", "1",
                "-pix_fmt", "yuv420p"
            ]);
        }
        OutputFormat::Mp4 => {
            // MP4 uses H.264
            cmd.args(["-vcodec", "h264_videotoolbox", "-allow_sw", "1", "-b:v", "5M", "-pix_fmt", "yuv420p"]);
        }
    }

    // Audio encoding
    if matches!(audio, AudioSource::None) {
        cmd.arg("-an");
    } else {
        // Map audio from first input (avfoundation)
        cmd.args(["-map", "0:a:0"]);
        
        // Apply audio filters for better quality:
        // - aresample: resample to consistent 48kHz (standard for quality recordings)
        // Use swr (software resampler) which is more widely available than soxr
        cmd.args(["-af", "aresample=48000"]);
        
        match format {
            OutputFormat::WebM => {
                // WebM requires Vorbis or Opus - use Opus with higher bitrate for better quality
                cmd.args(["-acodec", "libopus", "-b:a", "192k", "-ar", "48000"]);
            }
            OutputFormat::Mp4 => {
                // MP4 uses AAC with higher bitrate and explicit sample rate
                // Use higher bitrate (192k) for better quality, explicit 48kHz sample rate
                cmd.args(["-acodec", "aac", "-b:a", "192k", "-ar", "48000", "-ac", "2"]);
            }
        }
    }
    
    // WebM-specific muxing settings for proper streaming
    if matches!(format, OutputFormat::WebM) {
        cmd.args(["-f", "webm"]); // Explicitly set format
        // Use segment-based streaming for better real-time encoding
        cmd.args(["-cluster_size_limit", "0"]);
        cmd.args(["-cluster_time_limit", "0"]);
    }
    
    cmd.arg(output);
    cmd
}

#[cfg(target_os = "windows")]
fn build_ffmpeg_cmd(
    output: &str,
    fps: u32,
    resolution: Option<&str>,
    duration: Option<u32>,
    audio: AudioSource,
    audio_device: Option<&str>,
    format: OutputFormat,
) -> Command {
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y");
    cmd.args(["-f", "gdigrab", "-framerate", &fps.to_string(), "-i", "desktop"]);
    if let Some(d) = duration { cmd.arg("-t").arg(d.to_string()); }
    
    // Video encoding - map video stream first
    cmd.args(["-map", "0:v:0"]);
    
    // Apply resolution scaling if specified (after mapping)
    if let Some(res) = resolution {
        cmd.args(["-vf", &format!("scale={}", res)]);
    }
    
    match format {
        OutputFormat::WebM => {
            // WebM requires VP8/VP9/AV1 - use VP9 with real-time encoding settings
            cmd.args([
                "-vcodec", "libvpx-vp9",
                "-b:v", "5M",
                "-deadline", "realtime",
                "-speed", "6",
                "-g", &(fps * 2).to_string(), // Keyframe every 2 seconds
                "-threads", "4",
                "-row-mt", "1",
                "-pix_fmt", "yuv420p"
            ]);
        }
        OutputFormat::Mp4 => {
            // MP4 uses H.264
            cmd.args(["-vcodec", "h264_nvenc", "-b:v", "5M", "-pix_fmt", "yuv420p"]);
        }
    }
    
    // Audio encoding
    if matches!(audio, AudioSource::None) {
        cmd.arg("-an");
    } else {
        // Use device directly - default to virtual-audio-capturer for system audio
        let dev = if let Some(d) = audio_device {
            d.to_string()
        } else {
            match audio {
                AudioSource::System => "audio=virtual-audio-capturer".to_string(),
                AudioSource::Mic => "audio=default".to_string(), // Default mic input
                _ => "audio=virtual-audio-capturer".to_string(),
            }
        };
        cmd.args(["-f", "dshow", "-i", &dev]);
        
        // Apply audio filters for better quality - resample to 48kHz
        cmd.args(["-af", "aresample=48000"]);
        
        match format {
            OutputFormat::WebM => {
                // WebM requires Vorbis or Opus - use Opus with higher bitrate
                cmd.args(["-acodec", "libopus", "-b:a", "192k", "-ar", "48000"]);
            }
            OutputFormat::Mp4 => {
                // MP4 uses AAC with higher bitrate
                cmd.args(["-acodec", "aac", "-b:a", "192k", "-ar", "48000", "-ac", "2"]);
            }
        }
    }
    
    // WebM-specific muxing settings for proper streaming
    if matches!(format, OutputFormat::WebM) {
        cmd.args(["-f", "webm"]); // Explicitly set format
        // Use segment-based streaming for better real-time encoding
        cmd.args(["-cluster_size_limit", "0"]);
        cmd.args(["-cluster_time_limit", "0"]);
    }
    
    cmd.arg(output);
    cmd
}



