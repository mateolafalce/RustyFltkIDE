use fltk::text::{
    TextDisplay,
    TextBuffer
};
use std::{
    process::Command,
    env::consts::OS,
    thread
};
use crate::functions::{
    root,
    commands_for_windows::commands_for_windows,
    write_terminal
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
            },
            _ => {
                match command_input {
                    "cargo build" => {
                        thread::spawn(move || {
                            let output = Command::new("cargo")
                                .args(&["build", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                                .output()
                                .expect("Error");
                            let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                                write_terminal(
                                    &(input + "\n" + &result),
                                    text.clone(),
                                    terminal.clone()
                                ).expect("Error");
                            }
                        );
                    },
                    "cargo b" => {
                        let output = Command::new("cargo")
                            .args(&["build", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo run" => {
                        let output = Command::new("cargo")
                            .args(&["run", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo r" => {
                        let output = Command::new("cargo")
                            .args(&["run", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                            .arg("run")
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo run --release" => {
                        thread::spawn(move || {
                            let output = Command::new("cargo")
                                .args(&["run", "--release", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                                .output()
                                .expect("Error");
                            let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                                write_terminal(
                                    &(input + "\n" + &result),
                                    text.clone(),
                                    terminal.clone()
                                ).expect("Error");
                            });
                    },
                    "cargo clean" => {
                        let output = Command::new("cargo")
                            .args(&["update", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo --version" => {
                        let output = Command::new("cargo")
                            .args(&["--version", &root.clone()])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo -V" => {
                        let output = Command::new("cargo")
                            .args(&["--version", &root.clone()])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo --help" => {
                        let output = Command::new("cargo")
                            .args(&["--help", &root.clone()])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo -h" => {
                        let output = Command::new("cargo")
                            .args(&["--help", &root.clone()])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
                            write_terminal(
                                &(input + "\n" + &result),
                                text.clone(),
                                terminal.clone()
                            ).expect("Error");
                    },
                    "cargo update" => {
                        let output = Command::new("cargo")
                            .args(&["update", "--manifest-path", &(root.clone() + "\\Cargo.toml")])
                            .output()
                            .expect("Error");
                        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
                            write_terminal(
                                &(input + "\n" + &result),
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
