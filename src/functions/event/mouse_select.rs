pub fn mouse_select(event: fltk::enums::Event) -> bool {
    match event {
        fltk::enums::Event::Enter => {
            // Set the cursor to a hand when the mouse enters
            fltk::draw::set_cursor(fltk::enums::Cursor::Hand);
            true
        },
        fltk::enums::Event::Leave => {
            // Set the cursor to an arrow when the mouse leaves
            fltk::draw::set_cursor(fltk::enums::Cursor::Arrow);
            true
        },
        _ => false,
    }
}
