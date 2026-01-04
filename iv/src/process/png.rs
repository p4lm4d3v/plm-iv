// png.rs
use crate::QError;
use image;
use crate::models::image_type::ImageType;

pub fn process(image_buffer: &mut Vec<u8>) -> Result<(usize, usize, Vec<u32>), QError> {
    // Load image from buffer
    let img = image::load_from_memory(image_buffer)
        .map_err(|e| QError::ImageError(format!("Failed to load PNG: {}", e)))?;

    // Convert to RGBA8 (handles all PNG formats)
    let rgba_img = img.to_rgba8();

    // Get dimensions
    let width = rgba_img.width() as usize;
    let height = rgba_img.height() as usize;

    // Convert RGBA to ARGB (0xAARRGGBB) for display
    let window_buffer: Vec<u32> = ImageType::rgba_to_argb(rgba_img)?;
    Ok((width, height, window_buffer))
}