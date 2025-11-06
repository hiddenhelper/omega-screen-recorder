use screenrec::config::AppConfig;

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.fps, None);
        assert_eq!(config.resolution, None);
        assert_eq!(config.codec, None);
    }

    #[test]
    fn test_app_config_serialization() {
        let config = AppConfig {
            fps: Some(30),
            resolution: Some("1920x1080".to_string()),
            codec: Some("h264".to_string()),
        };

        // Test that we can serialize to TOML
        let toml_string = toml::to_string_pretty(&config).unwrap();
        assert!(toml_string.contains("fps = 30"));
        assert!(toml_string.contains("resolution = \"1920x1080\""));
        assert!(toml_string.contains("codec = \"h264\""));
    }

    #[test]
    fn test_app_config_deserialization() {
        let toml_content = r#"
fps = 60
resolution = "3840x2160"
codec = "libvpx-vp9"
"#;
        let config: AppConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.fps, Some(60));
        assert_eq!(config.resolution, Some("3840x2160".to_string()));
        assert_eq!(config.codec, Some("libvpx-vp9".to_string()));
    }

    #[test]
    fn test_app_config_partial() {
        let toml_content = r#"
fps = 30
"#;
        let config: AppConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.fps, Some(30));
        assert_eq!(config.resolution, None);
        assert_eq!(config.codec, None);
    }

    #[test]
    fn test_app_config_empty() {
        let toml_content = r#""#;
        let config: AppConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.fps, None);
        assert_eq!(config.resolution, None);
        assert_eq!(config.codec, None);
    }
}

