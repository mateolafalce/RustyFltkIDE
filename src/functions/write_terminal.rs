use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
};

pub fn write_terminal(
    response: &str,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let mut new_buffer: TextBuffer = text.clone();
    let mut terminal: TextDisplay = terminal;
    let terminal_text: String = format!("{}{}", text.text(), response);
    new_buffer.set_text(&terminal_text);
    terminal.set_buffer(new_buffer);
    terminal.scroll(terminal.count_lines(1, terminal.buffer().unwrap().length(),true),0);
    Ok(())
}
