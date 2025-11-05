use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "screenrec", version, about = "High-performance cross-platform screen recording CLI tool")] 
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Capture a screenshot and save it to a file
    Screenshot(ScreenshotArgs),

    /// Record the screen to a video file (uses ffmpeg if available)
    Record(RecordArgs),

    /// Configure defaults (saved to a config file)
    Config(ConfigArgs),
}

#[derive(Args, Debug)]
pub struct ScreenshotArgs {
    /// Output image path (.png or .jpg)
    #[arg(short, long)]
    pub output: String,

    /// Monitor index (0-based). Defaults to primary display
    #[arg(long)]
    pub monitor: Option<u32>,

}

#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
pub enum AudioSource {
    /// No audio
    None,

    /// System audio (may require loopback device on macOS/Windows)
    System,

    /// Microphone input
    Mic,
}

#[derive(Args, Debug)]
pub struct RecordArgs {
    /// Output video path (.mp4 or .webm)
    #[arg(short, long)]
    pub output: String,

    /// Target frames per second (overrides config if specified)
    #[arg(long)]
    pub fps: Option<u32>,

    /// Resolution like 1920x1080 (overrides config if specified). Defaults to current display resolution
    #[arg(long)]
    pub resolution: Option<String>,

    /// Duration in seconds (if omitted, recording continues until interrupted)
    #[arg(long)]
    pub duration: Option<u32>,

    /// Audio source
    #[arg(long, value_enum, default_value_t = AudioSource::None)]
    pub audio: AudioSource,

    /// Force use of a specific ffmpeg device string for audio (advanced)
    #[arg(long)]
    pub audio_device: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// Default FPS
    #[arg(long)]
    pub fps: Option<u32>,

    /// Default resolution like 1920x1080
    #[arg(long)]
    pub resolution: Option<String>,
    
    /// Default codec (h264 or libvpx-vp9)
    #[arg(long)]
    pub codec: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}


