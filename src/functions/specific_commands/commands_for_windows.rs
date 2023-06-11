use fltk::{
    text::{
        TextDisplay,
        TextBuffer
    },
};
use std::{
    fs::{
        File,
    },
    io::Read
};

#[path="../../commands/dir.rs"]
mod dir;
#[path="../../commands/cd_back.rs"]
mod cd_back;
#[path="../../commands/clear.rs"]
mod clear;
#[path="../../commands/cd_to.rs"]
mod cd_to;
#[path="../root/get_root.rs"]
mod get_root;
#[path="../write_terminal.rs"]
mod write_terminal;

pub fn commands_for_windows(
    input: &str,
    text: TextBuffer,
    terminal: TextDisplay,
) -> Result<(), std::io::Error> {
    let root: String = get_root::get_root();
    let mut file: File = File::open("src/constants/root.rs").expect("Error");
    let mut root_data: String = String::new();
    file.read_to_string(&mut root_data).unwrap();
    let raw_input: String = input.to_string();
    let split_raw_input: Vec<String> = raw_input.split(' ').map(|s| s.to_owned()).collect();
    let command_input: &str = raw_input.trim_start_matches(&root);
    if split_raw_input[0] == "cd" && split_raw_input[1] != ".." {
        cd_to::cd_to(input.to_owned(), text.clone(), terminal.clone(), split_raw_input, root.clone());
    }
    match command_input {
        "dir" => {
            dir::dir(root_data, input.to_owned(), text, terminal);
        }
        "cd .." => {
            cd_back::cd_back(input.to_owned(), text, terminal, root.clone());
        }
        "clear" => {
            clear::clear(text, terminal);
        }
        _ => {
            write_terminal::write_terminal(
                "\n",
                text,
                terminal
            ).expect("Error");
        }
    }
    Ok(())
}
