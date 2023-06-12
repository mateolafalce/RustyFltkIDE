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

#[path="../constants/text_size.rs"]
mod text_size;
#[path="../constants/font.rs"]
mod font;

pub fn terminal_output() -> (TextDisplay, TextBuffer) {
    let terminal_buffer: TextBuffer = TextBuffer::default();
    let mut terminal: TextDisplay = TextDisplay::new(200, 400, 790, 170, None);
    let terminal_text: String = format!("Operating system: {}\n🦀 Rusty IDE console 💻\n\n", OS);
    terminal.set_buffer(Some(terminal_buffer.clone())); // Set the text buffer for the text display widget
    terminal.buffer().unwrap().append(&terminal_text); // Add the welcome message to the text buffer
    terminal.set_scrollbar_align(Align::Bottom | Align::Right); // Set the scrollbar alignment to bottom-right
    terminal.set_text_font(font::FONT);
    terminal.set_text_size(text_size::CONSOLE_TEXT_SIZE);
    terminal.set_scrollbar_size(18);
    terminal.set_frame(FrameType::NoBox); // Set the frame type to no box (no visible border)
    (terminal, terminal_buffer) 
}
