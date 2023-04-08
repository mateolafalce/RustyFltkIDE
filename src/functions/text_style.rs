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

fn find_colon(text: &str) -> Option<usize> {
    if let Some(pattern) = text.find("::") {
        return text[..pattern].find(":");
    }
    None
}

pub fn text_style(
    mut text_editor: TextEditor,
    text: &str
) {
    let mut sbuf: TextBuffer = TextBuffer::default();
    sbuf.set_text(&"A".repeat(find_colon(text).unwrap()));
    text_editor.set_highlight_data(sbuf, STYLES.to_vec());
}
