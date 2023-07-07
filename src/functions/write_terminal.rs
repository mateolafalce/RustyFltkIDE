use fltk::prelude::*;

pub fn write_terminal(
    response: &str,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
) -> bool {
    let mut new_buffer: fltk::text::TextBuffer = text.clone();
    let mut terminal: fltk::text::TextDisplay = terminal;
    // Combine the current text in the TextBuffer object with the response
    let terminal_text: String = format!("{}{}", text.text(), response);
    // Set the TextBuffer object to the new combined text
    new_buffer.set_text(&terminal_text);
    terminal.set_buffer(new_buffer);
    // Scroll to the end of the text in the TextDisplay object
    terminal.scroll(
        terminal.count_lines(1, terminal.buffer().unwrap().length(),true),
        0
    );
    true
}
