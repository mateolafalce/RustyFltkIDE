use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
};

pub fn write_terminal(// A function that writes a response to a terminal display
    response: &str,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let mut new_buffer: TextBuffer = text.clone();
    let mut terminal: TextDisplay = terminal;
    let terminal_text: String = format!("{}{}", text.text(), response); // Combine the current text in the TextBuffer object with the response
    new_buffer.set_text(&terminal_text); // Set the TextBuffer object to the new combined text
    terminal.set_buffer(new_buffer);
    terminal.scroll(
        terminal.count_lines(1, terminal.buffer().unwrap().length(),true)
        ,0
    ); // Scroll to the end of the text in the TextDisplay object
    Ok(()) // Return a Result indicating success
}
