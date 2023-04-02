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
    let command_input: &str = raw_input.trim_start_matches(&root().unwrap());
    let mut root: String = root().unwrap();
    root.pop();
    root.pop();
    /*let split_string: Vec<&str> = command_input.split(' ').collect();
    if split_string[0] == "cargo" {
        match split_string[1] {
            /*"add" => {
                let output = Command::new("cargo")
                    .args(&["add", &split_string[2], "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                    .output()
                    .expect("Error");
                let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                    write_terminal(
                        &(input + "\n" + &result),
                        text.clone(),
                        terminal.clone()
                    ).expect("Error");
            },
            "new" => {
                let output = Command::new("cargo")
                    .args(&["new", &(root.clone() + "\\" + split_string[2])])
                    .output()
                    .expect("Error");
                let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                    write_terminal(
                        &(input + "\n" + &result),
                        text.clone(),
                        terminal.clone()
                    ).expect("Error");
            },*/
            _ => {

                }
            }
        }
    } else {*/
        if OS == "windows" {
            commands_for_windows(
                command_input,
                text.clone(),
                terminal.clone()
            ).expect("Windows console error");
        } else {
            print!("");
        }
    //}
    Ok(())
}
