#![cfg_attr(windows, windows_subsystem = "windows")]

pub mod types;
pub mod q_error;
mod util;
mod process;

use crate::types::image_type::ImageType;
use crate::q_error::QError;
use crate::process::ppm;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), QError> {
    // Read the arguments
    let args: Vec<String> = env::args().collect();

    // Arguments error management
    if args.len() < 2 {
        eprintln!("Usage: {} <image-path>", args[0]);
        return Err(QError::IncorrectUsageOfProgram);
    }

    // Parsing the image type, and getting the image path
    let image_path = &args[1]; 
    let extension = image_path.split('.').last().unwrap();
    let img_type = ImageType::get(extension);

    // Opening the file and reading it into the image buffer
    let mut file = File::open(image_path)?;
    let mut image_buffer = Vec::new();
    file.read_to_end(&mut image_buffer)?;

    // Running the appropriate process func for the image type
    let (width, height, window_buffer) = match img_type {
        Ok(ImageType::PPM) => ppm::process(&mut image_buffer),
        _ => Err(QError::WrongImageType),
    }?;

    // Initializing the application window
    let mut window = Window::new(
        image_path,
        width,
        height,
        WindowOptions {
            scale_mode: ScaleMode::Center,
            resize: true,
            ..WindowOptions::default()
        },
    )?;
    
    // Updating the window with the window buffer 
    // until the escape key is not pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&window_buffer, width, height)?;
    }

    Ok(())
}

