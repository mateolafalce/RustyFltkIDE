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
    }
};

// Function to create a new text editor with a text buffer
pub fn text_editor() -> (TextEditor, TextBuffer) {
    // Create a new text buffer
    let mut buffer: TextBuffer = TextBuffer::default();
    buffer.set_tab_distance(4); // Set the tab distance to 4 spaces
    // Create a new text editor
    let mut text_editor: TextEditor = TextEditor::new(200, 20, 790, 380, ""); // X position, Y position, Width, Height, Label
    text_editor.set_text_font(Font::Courier); // Set the text font to Courier
    text_editor.set_text_size(17); // Set the text size to 17
    text_editor.set_scrollbar_size(16); // Set the size of the scrollbar
    text_editor.set_buffer(Some(buffer.clone())); // Set the text buffer for the text editor
    text_editor.show_cursor(true); // Show the cursor in the text editor
    text_editor.set_linenumber_width(25); // Set the width of the line numbers
    text_editor.set_linenumber_fgcolor(Color::from_hex_str("#000000").unwrap()); // Set the color of the line numbers
    text_editor.set_linenumber_bgcolor(Color::from_hex_str("#D3D3D3").unwrap()); // Set the background color of the line numbers
    text_editor.set_cursor_style(Cursor::Simple); // Set the cursor style to a simple style
    // Return the text editor and the text buffer as a tuple
    (text_editor, buffer)
}
