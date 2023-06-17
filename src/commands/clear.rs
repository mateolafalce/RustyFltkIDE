use fltk::prelude::*;
#[path="../functions/write_terminal.rs"]
mod write_terminal;

pub fn clear(
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
) -> bool {
    std::thread::spawn(move || {
        let mut text: fltk::text::TextBuffer = text.clone();
        text.set_text("");
        let mut terminal: fltk::text::TextDisplay = terminal.clone();
        terminal.set_buffer(Some(text.clone()));
        terminal.buffer().unwrap().append("");
        // Call the write_terminal function from the write_terminal module
        write_terminal::write_terminal("\n",text,terminal);
    });
    true
}
