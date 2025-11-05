use anyhow::{bail, Context, Result};
use image::{ImageBuffer, Rgb, Rgba};
use screenshots::Screen;
use std::path::Path;

pub fn capture_screenshot<P: AsRef<Path>>(output_path: P, monitor_index: Option<u32>) -> Result<()> {
    let displays = Screen::all().context("Failed to enumerate displays. Check screen permissions.")?;
    if displays.is_empty() {
        bail!("No displays found. Please ensure you have at least one display connected.");
    }

    let idx = monitor_index.unwrap_or(0) as usize;
    let screen = displays
        .get(idx)
        .with_context(|| {
            format!(
                "Monitor index {} is out of range. Available monitors: 0-{}",
                idx,
                displays.len().saturating_sub(1)
            )
        })?;

    let image = screen.capture()
        .context("Failed to capture screen. Ensure you have screen recording permissions.")?;
    let (width, height) = (image.width(), image.height());
    
    if width == 0 || height == 0 {
        bail!("Captured image has invalid dimensions: {}x{}", width, height);
    }
    
    let buffer = image.as_raw(); // already RGBA
    let expected_size = (width * height * 4) as usize;
    
    if buffer.len() < expected_size {
        bail!("Image buffer size mismatch: expected {} bytes, got {}", expected_size, buffer.len());
    }

    let buffer_clone = buffer.clone();
    let img: ImageBuffer<Rgba<u8>, _> = ImageBuffer::from_raw(width, height, buffer_clone)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to build image buffer from captured data ({}x{} pixels, {} bytes)",
                width, height, buffer.len()
            )
        })?;

    let path = output_path.as_ref();
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "png" => {
            img.save_with_format(path, image::ImageFormat::Png)
                .with_context(|| format!("Failed to save PNG to {}", path.display()))?
        }
        "jpg" | "jpeg" => {
            // JPEG doesn't support alpha channel, so convert RGBA to RGB
            let rgb_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
                let pixel = img.get_pixel(x, y);
                Rgb([pixel[0], pixel[1], pixel[2]])
            });
            rgb_img
                .save_with_format(path, image::ImageFormat::Jpeg)
                .with_context(|| format!("Failed to save JPEG to {}", path.display()))?
        }
        _ => {
            img.save_with_format(path, image::ImageFormat::Png)
                .with_context(|| format!("Failed to save PNG to {}", path.display()))?
        }
    }

    Ok(())
}
