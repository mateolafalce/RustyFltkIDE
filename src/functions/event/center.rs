#[allow(dead_code)]
pub fn center() -> (i32, i32) {
    (
        (fltk::app::screen_size().0 / 2.0) as i32,  // Get the horizontal center of the screen
        (fltk::app::screen_size().1 / 2.0) as i32,  // Get the vertical center of the screen
    )
}
