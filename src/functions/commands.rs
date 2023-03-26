use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
};
use std::{
    process::Command,
    env::consts::OS
};
use crate::functions::root;
use crate::functions::commands_for_windows::commands_for_windows;

pub fn run_a_command(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let raw_input: String = input.to_string();
    let command_input: &str = raw_input.trim_start_matches(&root().unwrap());
    let _split_string: Vec<&str> = command_input.split(' ').collect();
    if OS == "windows" {
        commands_for_windows(
            command_input,
            text,
            terminal
        ).expect("Windows console error");
    } else {
        print!("");
    }
    match command_input {
        "ls" => {
            let _output = Command::new("ls")
                .arg("-l")
                .output()
                .expect("failed to execute process");
        }
        _ => {
            print!("");
        }
    }
    Ok(())
}

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
