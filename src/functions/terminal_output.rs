use fltk::prelude::*;
#[path="../constants/console_text_size.rs"]
mod console_text_size;
#[path="../constants/font.rs"]
mod font;
#[path="../constants/scrollbar_size.rs"]
mod scrollbar_size;

pub fn terminal_output() -> (fltk::text::TextDisplay, fltk::text::TextBuffer) {
    let terminal_buffer: fltk::text::TextBuffer = fltk::text::TextBuffer::default();
    let mut terminal: fltk::text::TextDisplay = fltk::text::TextDisplay::new(200, 400, 790, 170, None);
    let terminal_text: String = format!("Operating system: {}\n🦀 Rusty IDE console 💻\n\n", std::env::consts::OS);
    terminal.set_buffer(Some(terminal_buffer.clone())); // Set the text buffer for the text display widget
    terminal.buffer().unwrap().append(&terminal_text); // Add the welcome message to the text buffer
    terminal.set_scrollbar_align(
        fltk::enums::Align::Bottom | fltk::enums::Align::Right
    ); // Set the scrollbar alignment to bottom-right
    terminal.set_text_font(font::FONT);
    terminal.set_text_size(console_text_size::CONSOLE_TEXT_SIZE);
    terminal.set_scrollbar_size(scrollbar_size::SCROLLBAR_SIZE);
    terminal.set_frame(fltk::enums::FrameType::NoBox); // Set the frame type to no box (no visible border)
    (terminal, terminal_buffer)
}
