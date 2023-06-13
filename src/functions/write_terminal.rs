use fltk::prelude::*;

pub fn write_terminal(
    response: &str,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
) -> bool {
    let mut new_buffer: fltk::text::TextBuffer = text.clone();
    let mut terminal: fltk::text::TextDisplay = terminal;
    let terminal_text: String = format!("{}{}", text.text(), response); // Combine the current text in the TextBuffer object with the response
    new_buffer.set_text(&terminal_text); // Set the TextBuffer object to the new combined text
    terminal.set_buffer(new_buffer);
    terminal.scroll(
        terminal.count_lines(1, terminal.buffer().unwrap().length(),true),
        0
    ); // Scroll to the end of the text in the TextDisplay object
    true
}
