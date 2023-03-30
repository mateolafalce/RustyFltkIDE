use fltk::{
    text::{
        TextDisplay,
        TextBuffer
    },
    dialog::alert,
};
use std::{
    fs::{
        File,
        metadata
    },
    io::Read
};
use crate::functions::{
    commands::write_terminal,
    root,
    set_root,
    center,
};
use crate::commands::{
    dir,
    cd_back,
    clear
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
    let split_raw_input: Vec<&str> = raw_input.split(' ').collect();
    let command_input: &str = raw_input.trim_start_matches(&root().unwrap());
    if split_raw_input[0] == "cd" && split_raw_input[1] != ".." {
        let root: String = root().unwrap();
        let mut split_string: Vec<&str> = root.split('\\').collect();
        split_string.pop();
        split_string.push(split_raw_input[1]);
        let new_root: String = split_string.join("\\");
        match metadata(new_root.clone()) {
            Ok(_) => {
                set_root(new_root.clone()).expect("Error");
                write_terminal(
                    &(root.clone() + " " + input + "\n"),
                    text.clone(),
                    terminal.clone()
                ).expect("Error");
                return Ok(());
            },
            Err(e) => {
                alert(center().0 - 100, center().1 - 100, &format!("Error: {}\n", e));
            }
        }
    }
    match command_input {
        "dir" => {
            dir::dir(root_data, input.to_owned(), text, terminal);
        }
        "cd .." => {
            cd_back::cd_back(input.to_owned(), text, terminal);
        }
        "clear" => {
            clear::clear(text, terminal);
        }
        _ => {
            write_terminal(
                "Command not found. Verify that it exists.\n",
                text,
                terminal
            ).expect("Error");
        }
    }
    Ok(())
}
