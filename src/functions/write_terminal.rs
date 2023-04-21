// Import necessary modules from the fltk library
use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
};
// A function that writes a response to a terminal display
// Takes in a reference to the response, a TextBuffer object, and a TextDisplay object
pub fn write_terminal(
    response: &str,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    // Clone the existing TextBuffer object and create a new TextDisplay object
    let mut new_buffer: TextBuffer = text.clone();
    let mut terminal: TextDisplay = terminal;
    // Combine the current text in the TextBuffer object with the response
    let terminal_text: String = format!("{}{}", text.text(), response);
    // Set the TextBuffer object to the new combined text
    new_buffer.set_text(&terminal_text);
    // Set the TextDisplay object to the new TextBuffer object
    terminal.set_buffer(new_buffer);
    // Scroll to the end of the text in the TextDisplay object
    terminal.scroll(terminal.count_lines(1, terminal.buffer().unwrap().length(),true),0);
    // Return a Result indicating success
    Ok(())
}
