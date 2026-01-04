use crate::process::{jpeg, png, ppm};
use crate::q_error::QError;

pub enum ImageType {
    PPM,
    PNG,
    JPEG,
}

impl ImageType {
    pub fn get(extension: &str) -> Result<ImageType, QError> {
        match extension {
            "ppm" => Ok(ImageType::PPM),
            "png" => Ok(ImageType::PNG),
            "jpg" | "jpeg" => Ok(ImageType::JPEG),
            _ => Err(QError::InvalidImageType),
        }
    }

    pub fn process(self, image_buffer: &mut Vec<u8>) -> Result<(usize, usize, Vec<u32>), QError> {
        match self {
            ImageType::PPM => ppm::process(image_buffer),
            ImageType::JPEG => jpeg::process(image_buffer),
            ImageType::PNG => png::process(image_buffer),
        }
    }
}

