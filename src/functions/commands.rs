use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
};
use std::{
    thread,
    process::Command,
    env::consts::OS
};
use crate::functions::{
    root,
    commands_for_windows::commands_for_windows
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
    let split_string: Vec<&str> = command_input.split(' ').collect();
    if split_string[0] == "cargo" {
        match split_string[1] {
            "add" => {
                let output = Command::new("cargo")
                    .args(&["add", &split_string[2], "--manifest-path", &(root + "\\Cargo.toml")])
                    .output()
                    .expect("Error");
                let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                    write_terminal(
                        &result,
                        text.clone(),
                        terminal.clone()
                    ).expect("Error");
            },
            "new" => {
                let output = Command::new("cargo")
                    .args(&["new", &(root + "\\" + split_string[2])])
                    .output()
                    .expect("Error");
                let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                    write_terminal(
                        &result,
                        text.clone(),
                        terminal.clone()
                    ).expect("Error");
            },
            _ => {
                match command_input {
                    "cargo build" => {
                        let output = Command::new("cargo")
                            .args(&["build", "--manifest-path", &(root + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo b" => {
                        let output = Command::new("cargo")
                            .args(&["build", "--manifest-path", &(root + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo run" => {
                        let output = Command::new("cargo")
                            .args(&["run", "--manifest-path", &(root + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo r" => {
                        let output = Command::new("cargo")
                            .args(&["run", "--manifest-path", &(root + "\\Cargo.toml")])
                            .arg("run")
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo run --release" => {
                        write_terminal(
                            "cargo run --release",
                            text.clone(),
                            terminal.clone()
                        ).expect("Error");
                        thread::spawn(move || {
                            let output = Command::new("cargo")
                                .args(&["run", "--release", "--manifest-path", &(root + "\\Cargo.toml")])
                                .output()
                                .expect("Error");
                            let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                                write_terminal(
                                    &result,
                                    text.clone(),
                                    terminal.clone()
                                ).expect("Error");
                            });
                    },
                    "cargo clean" => {
                        let output = Command::new("cargo")
                            .args(&["update", "--manifest-path", &(root + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo --version" => {
                        let output = Command::new("cargo")
                            .args(&["--version", &root])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo -V" => {
                        let output = Command::new("cargo")
                            .args(&["--version", &root])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo --help" => {
                        let output = Command::new("cargo")
                            .args(&["--help", &root])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo -h" => {
                        let output = Command::new("cargo")
                            .args(&["--help", &root])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo update" => {
                        let output = Command::new("cargo")
                            .args(&["update", "--manifest-path", &(root + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &result,
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    _ => {
                        println!("Error");
                    }
                }
            }
        }
    } else {
        if OS == "windows" {
            commands_for_windows(
                command_input,
                text.clone(),
                terminal.clone()
            ).expect("Windows console error");
        } else {
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