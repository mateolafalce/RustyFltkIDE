pub fn mouse_select(event: fltk::enums::Event) -> bool {
    match event {
        fltk::enums::Event::Enter => {
            fltk::draw::set_cursor(fltk::enums::Cursor::Hand);
            true
        },
        fltk::enums::Event::Leave => {
            fltk::draw::set_cursor(fltk::enums::Cursor::Arrow);
            true
        },
        _ => false,
    }
}
