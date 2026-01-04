use crate::QError;
use crate::models::image_type::ImageType;
use image;

pub fn process(image_buffer: &mut Vec<u8>) -> Result<(usize, usize, Vec<u32>), QError> {
    let img = image::load_from_memory(image_buffer).map_err(|_| QError::FailedToLoadImage)?;

    let rgba_img = img.to_rgba8();

    let width = rgba_img.width() as usize;
    let height = rgba_img.height() as usize;

    let window_buffer: Vec<u32> = ImageType::rgba_to_argb(rgba_img)?;

    Ok((width, height, window_buffer))
}
