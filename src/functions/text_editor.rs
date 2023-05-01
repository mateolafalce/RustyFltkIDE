use fltk::{
    prelude::*,
    enums::{
        Font,
        Color,
    },
    text::{
        TextEditor,
        TextBuffer,
        Cursor,
        WrapMode
    }
};

pub fn text_editor() -> (TextEditor, TextBuffer) {
    let mut buffer: TextBuffer = TextBuffer::default();
    buffer.set_tab_distance(4); // Set the tab distance to 4 spaces
    let mut text_editor: TextEditor = TextEditor::new(200, 20, 790, 380, None);
    text_editor.set_text_font(Font::Courier);
    text_editor.set_text_size(17);
    text_editor.set_scrollbar_size(16);
    text_editor.set_buffer(Some(buffer.clone())); // Set the text buffer for the text editor
    text_editor.show_cursor(true); // Show the cursor in the text editor
    text_editor.set_linenumber_width(25); // Set the width of the line numbers
    text_editor.set_linenumber_fgcolor(Color::from_hex_str("#000000").unwrap()); // Set the color of the line numbers
    text_editor.set_linenumber_bgcolor(Color::from_hex_str("#D3D3D3").unwrap()); // Set the background color of the line numbers
    text_editor.set_cursor_style(Cursor::Simple);
    text_editor.wrap_mode(WrapMode::AtBounds, 20);
    (text_editor, buffer) // Return the text editor and the text buffer as a tuple
}
