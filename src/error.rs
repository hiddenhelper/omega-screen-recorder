#![allow(dead_code)]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid resolution format: {0}. Expected format: WIDTHxHEIGHT (e.g., 1920x1080)")]
    InvalidResolutionFormat(String),

    #[error("Invalid FPS: {0}. FPS must be between 1 and 120")]
    InvalidFps(u32),

    #[error("Output path is invalid or not writable: {0}")]
    InvalidOutputPath(String),

    #[error("Monitor index {0} is out of range. Available monitors: 0-{1}")]
    InvalidMonitorIndex(u32, usize),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("FFmpeg error: {0}")]
    FfmpegError(String),
}

