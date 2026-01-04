use crate::q_error::QError;
use crate::util::u8_to_rgb::u8_to_rgb;

pub fn process(
    image_buffer: &mut Vec<u8>,
) -> Result<(usize, usize, Vec<u32>), QError> {
    let image_buffer_string = image_buffer.iter().map(|&b| b as char).collect::<String>();
    let image_buffer_string_split = image_buffer_string.split('\n').collect::<Vec<&str>>();

    let magic_number = image_buffer_string_split[0];

    let size_string = image_buffer_string_split[1];
    let size_split = size_string.split(' ').collect::<Vec<&str>>();
    let width = size_split[0].parse::<usize>()?;
    let height = size_split[1].parse::<usize>()?;

    let color_range = image_buffer_string_split[2].parse::<usize>()?;

    let to_skip = magic_number.len() + size_string.len() + color_range.to_string().len() + 3;

    let image_buffer = image_buffer
        .iter()
        .skip(to_skip)
        // .map(|&b| color_range as u8 - b) INVERSE
        .map(|&b| b)
        // .filter(|&b| (b as char) != ' ')
        .collect::<Vec<_>>();

    let mut window_buffer = vec![0x00; width * height];

    let mut win_buff_idx = 0;
    for img_buff_idx in (1..image_buffer.len() - 1).step_by(3) {
        let (r, g, b) = (
            image_buffer[img_buff_idx - 1],
            image_buffer[img_buff_idx],
            image_buffer[img_buff_idx + 1],
        );
        window_buffer[win_buff_idx] = u8_to_rgb(r, g, b);
        win_buff_idx += 1;
    }

    Ok((width, height, window_buffer))
}
