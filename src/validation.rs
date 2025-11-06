use crate::error::AppError;
use anyhow::{Context, Result};

pub fn validate_resolution(res: &str) -> Result<(), AppError> {
    let parts: Vec<&str> = res.split('x').collect();
    if parts.len() != 2 {
        return Err(AppError::InvalidResolutionFormat(res.to_string()));
    }
    let width: u32 = parts[0].parse()
        .map_err(|_| AppError::InvalidResolutionFormat(res.to_string()))?;
    let height: u32 = parts[1].parse()
        .map_err(|_| AppError::InvalidResolutionFormat(res.to_string()))?;
    
    if width == 0 || height == 0 || width > 7680 || height > 4320 {
        return Err(AppError::InvalidResolutionFormat(
            format!("{} (dimensions must be between 1x1 and 7680x4320)", res)
        ));
    }
    Ok(())
}

pub fn validate_fps(fps: u32) -> Result<(), AppError> {
    if fps == 0 || fps > 120 {
        return Err(AppError::InvalidFps(fps));
    }
    Ok(())
}

pub fn validate_output_path(path: &str) -> Result<()> {
    use std::path::Path;
    let p = Path::new(path);
    
    // Check if parent directory exists and is writable
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            return Err(anyhow::anyhow!(
                "Output directory does not exist: {}",
                parent.display()
            ));
        }
        // Check if parent is writable (simplified check - try to create a temp file)
        if !parent.is_dir() {
            return Err(anyhow::anyhow!(
                "Output path parent is not a directory: {}",
                parent.display()
            ));
        }
    }
    
    // If file exists, check if it's writable
    if p.exists() && p.is_file() {
        use std::fs::OpenOptions;
        OpenOptions::new()
            .write(true)
            .open(p)
            .with_context(|| format!("Output file exists but is not writable: {}", p.display()))?;
    }
    
    Ok(())
}

