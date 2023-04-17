use fltk::text::{
    TextDisplay,
    TextBuffer
};
use std::env::consts::OS;
#[path="./specific_commands/commands_for_windows.rs"]
mod commands_for_windows;
#[path="./specific_commands/commands_for_cargo.rs"]
mod commands_for_cargo;
#[path="./root/get_root.rs"]
mod root;

pub fn run_a_command(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let raw_input: String = input.to_string();
    let command_input: &str = raw_input.trim_start_matches(&root::root());
    let mut root: String = root::root();
    root.pop();
    root.pop();
    if OS == "windows" {
            commands_for_windows::commands_for_windows(
                command_input,
                text.clone(),
                terminal.clone()
            ).unwrap();
        } else {
            print!("todo");
            //TODO
    };
    commands_for_cargo::commands_for_cargo(
        input,
        text.clone(),
        terminal.clone(),
        command_input,
    );
    Ok(())
}
