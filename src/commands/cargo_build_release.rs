use crate::functions::{
    write_terminal,
    root
};
use fltk::text::{
    TextDisplay,
    TextBuffer
};
use std::{
    thread,
    process::Command
};

pub fn cargo_build_release(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay
){
    thread::spawn(move || {
        let output = Command::new("cargo")
            .args(&["build", "--release", "--manifest-path", &((root.clone())().unwrap() + "\\Cargo.toml")])
            .output()
            .expect("Error");
        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
            write_terminal(
                &(input + "\n" + &result),
                text.clone(),
                terminal.clone()
            ).expect("Error");
        });
}
