use screenrec::error::AppError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_resolution_format_error() {
        let error = AppError::InvalidResolutionFormat("1920 1080".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Invalid resolution format"));
        assert!(error_msg.contains("1920 1080"));
        assert!(error_msg.contains("WIDTHxHEIGHT"));
    }

    #[test]
    fn test_invalid_fps_error() {
        let error = AppError::InvalidFps(200);
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Invalid FPS"));
        assert!(error_msg.contains("200"));
        assert!(error_msg.contains("1 and 120"));
    }

    #[test]
    fn test_invalid_output_path_error() {
        let error = AppError::InvalidOutputPath("/invalid/path".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Output path is invalid"));
        assert!(error_msg.contains("/invalid/path"));
    }

    #[test]
    fn test_invalid_monitor_index_error() {
        let error = AppError::InvalidMonitorIndex(5, 2);
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Monitor index"));
        assert!(error_msg.contains("5"));
        assert!(error_msg.contains("0-2"));
    }

    #[test]
    fn test_config_error() {
        let error = AppError::ConfigError("Failed to parse".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("Configuration error"));
        assert!(error_msg.contains("Failed to parse"));
    }

    #[test]
    fn test_ffmpeg_error() {
        let error = AppError::FfmpegError("Not found".to_string());
        let error_msg = format!("{}", error);
        assert!(error_msg.contains("FFmpeg error"));
        assert!(error_msg.contains("Not found"));
    }

    #[test]
    fn test_error_debug() {
        let error = AppError::InvalidFps(150);
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidFps"));
        assert!(debug_str.contains("150"));
    }
}

