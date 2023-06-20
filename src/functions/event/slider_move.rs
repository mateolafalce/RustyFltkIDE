pub fn slider_move(
    event: fltk::enums::Event
) -> bool {
    match event {
        fltk::enums::Event::Push => { // Set the cursor to "Move" when the slider is pushed
            fltk::draw::set_cursor(fltk::enums::Cursor::Move);
            true
        },
        fltk::enums::Event::NoEvent => { // Set the cursor to "Arrow" when there's no event on the slider
            fltk::draw::set_cursor(fltk::enums::Cursor::Arrow);
            true
        },
        _ => false,
    }
}
