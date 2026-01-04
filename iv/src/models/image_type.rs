use crate::process::{png_jpeg_jpg, ppm};
use crate::q_error::QError;
use image::RgbaImage;

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
            ImageType::JPEG => png_jpeg_jpg::process(image_buffer),
            ImageType::PNG => png_jpeg_jpg::process(image_buffer),
        }
    }

    pub fn rgba_to_argb(img: RgbaImage) -> Result<Vec<u32>, QError> {
        Ok(img
            .pixels()
            .map(|p| {
                // p is &[u8; 4] with RGBA bytes
                ((p[3] as u32) << 24) |  // Alpha
                    ((p[0] as u32) << 16) |  // Red
                    ((p[1] as u32) << 8) |   // Green
                    (p[2] as u32) // Blue
            })
            .collect::<Vec<_>>()
        )
    }
}
