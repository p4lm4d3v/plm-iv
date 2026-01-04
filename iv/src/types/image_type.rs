pub enum ImageType {
    PPM,
    PNG,
    JPG,
}

impl ImageType {
    pub fn get(extension: &str) -> Result<ImageType, ()> {
        match extension {
            "ppm" => Ok(ImageType::PPM),
            "png" => Ok(ImageType::PNG),
            "jpg" => Ok(ImageType::JPG),
            _ => Err(()),
        }
    }
}
