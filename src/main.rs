mod cli;
mod config;
mod capture;
mod encoder;
mod screenshot;
mod audio; // Available for future use if needed
mod error;
mod validation;

use anyhow::{Context, Result};
use crate::capture::ScreenRecorder;

fn run() -> Result<()> {
    let args = cli::parse();
    
    match args.command {
        cli::Commands::Screenshot(sc) => {
            validation::validate_output_path(&sc.output)
                .context("Validating screenshot output path")?;
            screenshot::capture_screenshot(sc.output, sc.monitor)?;
        }
        cli::Commands::Record(rc) => {
            validation::validate_output_path(&rc.output)
                .context("Validating recording output path")?;
            
            // Load saved config
            let saved_config = config::load_config()
                .context("Loading saved configuration (using defaults if not found)")?;
            
            // Merge config with CLI args (CLI args override config)
            let fps = rc.fps.or(saved_config.fps).unwrap_or(30);
            validation::validate_fps(fps)
                .with_context(|| format!("FPS validation failed: {}", fps))?;
            
            let resolution = rc.resolution.or(saved_config.resolution)
                .map(|r| r.replace('X', "x")); // Normalize uppercase X to lowercase x
            
            if let Some(ref res) = resolution {
                validation::validate_resolution(res)
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
                validation::validate_fps(fps)
                    .with_context(|| format!("Invalid FPS: {}", fps))?;
                cfg.fps = Some(fps);
            }
            if let Some(res) = cc.resolution {
                let normalized = res.replace('X', "x");
                validation::validate_resolution(&normalized)
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