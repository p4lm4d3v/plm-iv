use crate::q_error::QError;
use crate::util::u8_to_rgb::u8_to_rgb;

pub fn process(image_buffer: &mut Vec<u8>) -> Result<(usize, usize, Vec<u32>), QError> {
    // Read the lines of the file from the image buffer
    let image_buffer_string = image_buffer.iter().map(|&b| b as char).collect::<String>();
    let image_buffer_string_split = image_buffer_string.lines().collect::<Vec<&str>>();

    // Read the magic number
    let magic_number = image_buffer_string_split[0];

    // Read the size
    let size_string = image_buffer_string_split[1];
    let size_split = size_string.split(' ').collect::<Vec<&str>>();
    // and parse it into the width and height
    let width = size_split[0].parse::<usize>()?;
    let height = size_split[1].parse::<usize>()?;

    // Read the color range
    let color_range = image_buffer_string_split[2].parse::<usize>()?;

    // Calculate the number of bytes to skip to get to the pixel data
    let to_skip = magic_number.len() + size_string.len() + color_range.to_string().len() + 3;
    
    // Remove the bytes from the image buffer before the pixel data
    let image_buffer = image_buffer
        .iter()
        .skip(to_skip)
        // .map(|&b| color_range as u8 - b) INVERSE
        .map(|&b| b)
        // .filter(|&b| (b as char) != ' ')
        .collect::<Vec<_>>();

    // Initialize the window buffer
    let mut window_buffer = vec![0x00; width * height];

    let mut win_buff_idx = 0;
    for img_buff_idx in (1..image_buffer.len() - 1).step_by(3) {
        // Take every 3 bytes from the image buffer  
        let (r, g, b) = (
            image_buffer[img_buff_idx - 1],
            image_buffer[img_buff_idx],
            image_buffer[img_buff_idx + 1],
        );
        // Turn them into the u32 hex color 
        // and set the pixel in the window buffer to the color
        window_buffer[win_buff_idx] = u8_to_rgb(r, g, b);
        win_buff_idx += 1;
    }

    Ok((width, height, window_buffer))
}
