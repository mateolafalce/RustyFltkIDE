use std::thread;
use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    }
};
#[path="../functions/write_terminal.rs"]
mod write_terminal;

pub fn clear(
    text: TextBuffer,
    terminal: TextDisplay,
) {
    thread::spawn(move || {
        let mut text: TextBuffer = text.clone();
        text.set_text("");
        let mut terminal: TextDisplay = terminal.clone();
        terminal.set_buffer(Some(text.clone()));
        terminal.buffer().unwrap().append("");
        write_terminal::write_terminal(
            "\n",
            text,
            terminal
        ).expect("Error");
    });
}
