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
        StyleTableEntry
    }
};

const STYLES: &[StyleTableEntry] = &[
    StyleTableEntry {
        color: Color::Green,
        font: Font::Courier,
        size: 16,
}];

pub fn text_style(text_editor: TextEditor) {
    let mut buffer: TextBuffer = TextBuffer::default();
    let mut sbuf: TextBuffer = TextBuffer::default();
    buffer.set_tab_distance(4);
    let mut text_editor: TextEditor = TextEditor::new(200, 20, 790, 380, "");
    sbuf.set_text(&"A".repeat("::".len()));
    text_editor.set_text_font(Font::Courier);
    text_editor.set_text_size(17);
    text_editor.set_scrollbar_size(16);
    text_editor.set_buffer(Some(buffer.clone()));
    text_editor.show_cursor(true);
    text_editor.set_linenumber_width(25);
    text_editor.set_linenumber_fgcolor(Color::from_hex_str("#000000").unwrap());
    text_editor.set_linenumber_bgcolor(Color::from_hex_str("#D3D3D3").unwrap());
    text_editor.set_cursor_style(Cursor::Simple);
    text_editor.set_highlight_data(sbuf, STYLES.to_vec());
}
