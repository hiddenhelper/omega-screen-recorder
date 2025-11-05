mod cli;
mod config;
mod capture;
mod encoder;
mod screenshot;
mod audio; // Available for future use if needed
mod error;

use anyhow::{Context, Result};
use crate::capture::ScreenRecorder;
use crate::error::AppError;

fn validate_resolution(res: &str) -> Result<(), AppError> {
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

fn validate_fps(fps: u32) -> Result<(), AppError> {
    if fps == 0 || fps > 120 {
        return Err(AppError::InvalidFps(fps));
    }
    Ok(())
}

fn validate_output_path(path: &str) -> Result<()> {
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

fn run() -> Result<()> {
    let args = cli::parse();
    
    match args.command {
        cli::Commands::Screenshot(sc) => {
            validate_output_path(&sc.output)
                .context("Validating screenshot output path")?;
            screenshot::capture_screenshot(sc.output, sc.monitor)?;
        }
        cli::Commands::Record(rc) => {
            validate_output_path(&rc.output)
                .context("Validating recording output path")?;
            
            // Load saved config
            let saved_config = config::load_config()
                .context("Loading saved configuration (using defaults if not found)")?;
            
            // Merge config with CLI args (CLI args override config)
            let fps = rc.fps.or(saved_config.fps).unwrap_or(30);
            validate_fps(fps)
                .with_context(|| format!("FPS validation failed: {}", fps))?;
            
            let resolution = rc.resolution.or(saved_config.resolution)
                .map(|r| r.replace('X', "x")); // Normalize uppercase X to lowercase x
            
            if let Some(ref res) = resolution {
                validate_resolution(res)
                    .with_context(|| format!("Resolution validation failed: {}", res))?;
            }
            
            let recorder = capture::create_recorder();
            recorder.start_recording(
                &rc.output,
                rc.duration,
                fps,
                resolution.as_deref(),
                rc.audio,
                rc.audio_device.as_deref(),
            )?;
        }
        cli::Commands::Config(cc) => {
            // Handle clear/reset
            if cc.clear {
                config::clear_config()?;
                return Ok(());
            }
            
            let mut cfg = config::load_config()
                .context("Loading existing configuration")?;
            
            // If no arguments provided, show current config
            if cc.fps.is_none() && cc.resolution.is_none() && cc.codec.is_none() {
                println!("Current configuration:");
                if let Some(fps) = cfg.fps {
                    println!("  FPS: {}", fps);
                } else {
                    println!("  FPS: (not set, defaults to 30)");
                }
                if let Some(ref res) = cfg.resolution {
                    println!("  Resolution: {}", res);
                } else {
                    println!("  Resolution: (not set, uses display resolution)");
                }
                if let Some(ref codec) = cfg.codec {
                    println!("  Codec: {}", codec);
                } else {
                    println!("  Codec: (not set, auto-detected from file extension)");
                }
                return Ok(());
            }
            
            // Update config with provided values (with validation)
            if let Some(fps) = cc.fps {
                validate_fps(fps)
                    .with_context(|| format!("Invalid FPS: {}", fps))?;
                cfg.fps = Some(fps);
            }
            if let Some(res) = cc.resolution {
                let normalized = res.replace('X', "x");
                validate_resolution(&normalized)
                    .with_context(|| format!("Invalid resolution: {}", res))?;
                cfg.resolution = Some(normalized);
            }
            if let Some(codec) = cc.codec {
                // Validate codec
                let codec_lower = codec.to_lowercase();
                if !["h264", "libvpx-vp9", "vp9"].contains(&codec_lower.as_str()) {
                    eprintln!("Warning: Unknown codec '{}'. Valid options: h264, libvpx-vp9, vp9", codec);
                }
                cfg.codec = Some(codec);
            }
            config::save_config(&cfg)
                .context("Saving configuration")?;
            println!("Configuration saved:");
            if let Some(fps) = cfg.fps {
                println!("  FPS: {}", fps);
            }
            if let Some(ref res) = cfg.resolution {
                println!("  Resolution: {}", res);
            }
            if let Some(ref codec) = cfg.codec {
                println!("  Codec: {}", codec);
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        // Show error chain if available
        let mut source = e.source();
        if source.is_some() {
            eprintln!("\nCaused by:");
            while let Some(err) = source {
                eprintln!("  {}", err);
                source = err.source();
            }
        }
        std::process::exit(1);
    }
}