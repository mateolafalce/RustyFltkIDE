use fltk::text::{
    TextDisplay,
    TextBuffer
};
use std::env::consts::OS;
use crate::functions::{
    root,
    commands_for_windows::commands_for_windows,
};


pub fn run_a_command(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let raw_input: String = input.to_string();
    let command_input: &str = raw_input.trim_start_matches(&root());
    let mut root: String = root();
    root.pop();
    root.pop();
    if OS == "windows" {
            commands_for_windows(
                command_input,
                text.clone(),
                terminal.clone()
            ).expect("Windows console error");
        } else {
            print!("");
    }
    Ok(())
}
