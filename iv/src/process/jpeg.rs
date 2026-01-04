use crate::q_error::QError;
use jpeg_decoder as jpeg;
use jpeg_decoder::PixelFormat;

pub fn process(image_buffer: &mut Vec<u8>) -> Result<(usize, usize, Vec<u32>), QError> {
    let mut decoder = jpeg::Decoder::new(image_buffer.as_slice());

    // Decode the JPEG
    let pixels = decoder.decode()?;

    // Get metadata
    let metadata = decoder.info().ok_or(QError::NoMetadata)?;
    let width = metadata.width as usize;
    let height = metadata.height as usize;

    let window_buffer: Vec<u32> = match metadata.pixel_format {
        PixelFormat::L8 => {
            // Grayscale to RGB
            pixels.iter().map(|&l| {
                let l = l as u32;
                0xFF000000 | (l << 16) | (l << 8) | l
            }).collect()
        }
        PixelFormat::L16 => {
            pixels.chunks_exact(2).map(|bytes| {
                let l16 = ((bytes[0] as u16) << 8) | bytes[1] as u16;
                let l8 = (l16 >> 8) as u8;

                let l = l8 as u32;
                0xFF000000 | (l << 16) | (l << 8) | l
            }).collect()
        }
        PixelFormat::RGB24 => {
            pixels.chunks_exact(3).map(|rgb| {
                0xFF000000 |
                    ((rgb[0] as u32) << 16) |
                    ((rgb[1] as u32) << 8) |
                    (rgb[2] as u32)
            }).collect()
        }
        PixelFormat::CMYK32 => {
            pixels.chunks_exact(4).map(|cmyk| {
                let c = cmyk[0] as f32 / 255.0;
                let m = cmyk[1] as f32 / 255.0;
                let y = cmyk[2] as f32 / 255.0;
                let k = cmyk[3] as f32 / 255.0;

                let r = (255.0 * (1.0 - c) * (1.0 - k)).clamp(0.0, 255.0) as u8;
                let g = (255.0 * (1.0 - m) * (1.0 - k)).clamp(0.0, 255.0) as u8;
                let b = (255.0 * (1.0 - y) * (1.0 - k)).clamp(0.0, 255.0) as u8;

                0xFF000000 | ((r as u32) << 16) |
                    ((g as u32) << 8) |
                    (b as u32)
            }).collect()
        }
    };

    Ok((width, height, window_buffer))
}
