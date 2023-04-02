use std::thread;
use crate::functions::write_terminal;
use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    }
};

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
        write_terminal(
            "\n",
            text,
            terminal
        ).expect("Error");
    });
}
