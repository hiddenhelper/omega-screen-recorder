use screenrec::validation::{validate_resolution, validate_fps};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_resolution_valid() {
        assert!(validate_resolution("1920x1080").is_ok());
        assert!(validate_resolution("3840x2160").is_ok());
        assert!(validate_resolution("1280x720").is_ok());
        assert!(validate_resolution("1x1").is_ok());
        assert!(validate_resolution("7680x4320").is_ok());
    }

    #[test]
    fn test_validate_resolution_invalid_format() {
        assert!(validate_resolution("1920 1080").is_err());
        assert!(validate_resolution("1920X1080").is_err());
        assert!(validate_resolution("invalid").is_err());
        assert!(validate_resolution("").is_err());
        assert!(validate_resolution("1920").is_err());
        assert!(validate_resolution("1920x1080x720").is_err());
    }

    #[test]
    fn test_validate_resolution_invalid_numbers() {
        assert!(validate_resolution("0x1080").is_err());
        assert!(validate_resolution("1920x0").is_err());
        assert!(validate_resolution("0x0").is_err());
        assert!(validate_resolution("abcx1080").is_err());
        assert!(validate_resolution("1920xabc").is_err());
    }

    #[test]
    fn test_validate_resolution_out_of_range() {
        assert!(validate_resolution("7681x4320").is_err());
        assert!(validate_resolution("7680x4321").is_err());
        assert!(validate_resolution("10000x10000").is_err());
    }

    #[test]
    fn test_validate_fps_valid() {
        assert!(validate_fps(1).is_ok());
        assert!(validate_fps(30).is_ok());
        assert!(validate_fps(60).is_ok());
        assert!(validate_fps(120).is_ok());
    }

    #[test]
    fn test_validate_fps_invalid() {
        assert!(validate_fps(0).is_err());
        assert!(validate_fps(121).is_err());
        assert!(validate_fps(200).is_err());
    }
}

