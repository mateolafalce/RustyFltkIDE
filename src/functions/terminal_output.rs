use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
    enums::{
        Align,
        FrameType,
    },
};
use std::env::consts::OS;
use crate::{
    constants::{
        CONSOLE_TEXT_SIZE,
        FONT
    }
};

pub fn terminal_output() -> (TextDisplay, TextBuffer) {
    let terminal_buffer: TextBuffer = TextBuffer::default();
    let mut terminal: TextDisplay = TextDisplay::new(200, 400, 800, 170, None);
    let terminal_text: String = format!("Operating system: {}\n🦀 Rusty IDE console 💻\n\n", OS);
    terminal.set_buffer(Some(terminal_buffer.clone()));
    terminal.buffer().unwrap().append(&terminal_text);
    terminal.set_scrollbar_align(Align::Bottom | Align::Right);
    terminal.set_text_font(FONT);
    terminal.set_text_size(CONSOLE_TEXT_SIZE);
    terminal.set_scrollbar_size(18);
    terminal.set_frame(FrameType::NoBox);
    (terminal, terminal_buffer)
}
