use fltk::prelude::*;
#[path="../constants/editor_text_size.rs"]
mod editor_text_size;
#[path="../constants/scrollbar_size.rs"]
mod scrollbar_size;

pub fn text_editor() -> (fltk::text::TextEditor, fltk::text::TextBuffer) {
    let mut buffer: fltk::text::TextBuffer = fltk::text::TextBuffer::default();
    buffer.set_tab_distance(4); // Set the tab distance to 4 spaces
    let mut text_editor: fltk::text::TextEditor = fltk::text::TextEditor::new(200, 20, 790, 380, None);
    text_editor.set_text_font(fltk::enums::Font::Courier);
    text_editor.set_text_size(editor_text_size::EDITOR_TEXT_SIZE);
    text_editor.set_scrollbar_size(scrollbar_size::SCROLLBAR_SIZE);
    text_editor.set_buffer(Some(buffer.clone())); // Set the text buffer for the text editor
    text_editor.show_cursor(true); // Show the cursor in the text editor
    text_editor.set_linenumber_width(25); // Set the width of the line numbers
    text_editor.set_linenumber_fgcolor(fltk::enums::Color::from_hex_str("#000000").unwrap()); // Set the color of the line numbers
    text_editor.set_linenumber_bgcolor(fltk::enums::Color::from_hex_str("#D3D3D3").unwrap()); // Set the background color of the line numbers
    text_editor.set_cursor_style(fltk::text::Cursor::Simple);
    text_editor.wrap_mode(fltk::text::WrapMode::AtBounds, 20);
    (text_editor, buffer) // Return the text editor and the text buffer as a tuple
}
