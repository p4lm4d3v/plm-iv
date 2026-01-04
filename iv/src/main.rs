#![cfg_attr(windows, windows_subsystem = "windows")]

mod models;
mod process;
mod q_error;
mod util;

use crate::models::image_type::ImageType;
use crate::q_error::QError;

use crate::util::center_window::center_window;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use winapi::um::winuser::SM_CXSCREEN;

fn main() -> Result<(), QError> {
    // Read the arguments
    let args: Vec<String> = env::args().collect();

    // Arguments error management
    if args.len() < 2 {
        eprintln!("Usage: {} <image-path>", args[0]);
        return Err(QError::NotEnoughArguments);
    }

    // Parsing the image type, and getting the image path
    let image_path = &args[1];
    let extension = image_path.split('.').last().unwrap();
    let img_type = ImageType::get(extension)?;

    // Opening the file and reading it into the image buffer
    let mut file = File::open(image_path)?;
    let mut image_buffer = Vec::new();
    file.read_to_end(&mut image_buffer)?;

    // Running the appropriate process func for the image type
    let (width, height, window_buffer) = img_type.process(&mut image_buffer)?;

    // Read the file name
    let file_name = Path::new(image_path)
        .file_name()
        .ok_or(QError::PathError)?
        .to_str()
        .ok_or(QError::PathError)?;

    // Initializing the application window
    let mut window = Window::new(
        file_name,
        width,
        height,
        WindowOptions {
            scale_mode: ScaleMode::Center,
            resize: true,
            ..WindowOptions::default()
        },
    )?;

    use winapi::um::winuser::GetSystemMetrics;

    let sc_w = unsafe { GetSystemMetrics(0) } as usize;
    let sc_h = unsafe { GetSystemMetrics(1) } as usize;

    center_window(&mut window, sc_w, sc_h)?;

    // Updating the window with the window buffer
    // until the escape key is not pressed
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&window_buffer, width, height)?;
        println!("Image buffer len: {}", image_buffer.len());
    }

    Ok(())
}
