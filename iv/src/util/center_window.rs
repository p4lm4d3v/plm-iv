use minifb::Window;

pub fn center_window(window: &mut Window, sc_w: usize, sc_h: usize) -> Result<(), minifb::Error> {
    // Get window dimensions
    let (window_width, window_height) = window.get_size();

    // Calculate centered position
    let pos_x = ((sc_w - window_width) / 2) as isize;
    let pos_y = ((sc_h - window_height) / 2) as isize;

    // Set window position
    window.set_position(pos_x, pos_y);
    Ok(())
}
