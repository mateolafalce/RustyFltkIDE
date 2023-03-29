use fltk::{
    text::{
        TextDisplay,
        TextBuffer
    },
};
use std::{
    process::Command,
    fs::File,
    io::Read
};
use crate::functions::{
    commands::write_terminal,
    root,
    set_root
};

pub fn commands_for_windows(
    input: &str,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let mut file: File = File::open("src/constants/root.rs").expect("Error");
    let mut root_data: String = String::new();
    file.read_to_string(&mut root_data).unwrap();
    let raw_input: String = input.to_string();
    let command_input: &str = raw_input.trim_start_matches(&root().unwrap());
    let _split_string: Vec<&str> = command_input.split(' ').collect();
    match command_input {
        "dir" => {
            let output = Command::new("cmd")
                .args(&["/C", "dir", &root_data])
                .output()
                .expect("Error");
            let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
            write_terminal(
                &result,
                text,
                terminal
            ).expect("Error");
        }
        "cd .." => {
            let root: String = root().unwrap();
            let mut split_string: Vec<&str> = root.split('\\').collect();
            split_string.pop();
            split_string.pop();
            let new_root: String = split_string.join("\\");
            set_root(new_root).expect("Error");
            write_terminal(
                &root,
                text,
                terminal
            ).expect("Error");
        }
        _ => {
            write_terminal(
                "Command not found. Verify that it exists.",
                text.clone(),
                terminal.clone()
            ).expect("Error");
        }
    }
    Ok(())
}
