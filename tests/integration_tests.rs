// Integration tests for the screen recorder
// These tests may require actual system access, so they might be skipped in CI

use screenrec::cli::AudioSource;
use screenrec::config::AppConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_audio_source_enum() {
        // Test AudioSource enum values
        assert_eq!(AudioSource::None as u8, 0);
        assert_eq!(AudioSource::System as u8, 1);
        assert_eq!(AudioSource::Mic as u8, 2);
    }

    #[test]
    fn test_app_config_clone() {
        let config = AppConfig {
            fps: Some(30),
            resolution: Some("1920x1080".to_string()),
            codec: Some("h264".to_string()),
        };
        let cloned = config.clone();
        assert_eq!(cloned.fps, config.fps);
        assert_eq!(cloned.resolution, config.resolution);
        assert_eq!(cloned.codec, config.codec);
    }

    #[test]
    fn test_app_config_debug() {
        let config = AppConfig {
            fps: Some(60),
            resolution: Some("3840x2160".to_string()),
            codec: None,
        };
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("AppConfig"));
        assert!(debug_str.contains("60"));
        assert!(debug_str.contains("3840x2160"));
    }
}

