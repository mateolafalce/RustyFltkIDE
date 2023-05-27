#[path="../functions/write_terminal.rs"]
mod write_terminal;
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
    terminal: TextDisplay,
    root: String
){
    thread::spawn(move || {
        let output = Command::new("cargo")
            .args(&["build", "--release", "--manifest-path", &((root + "\\Cargo.toml"))])
            .output()
            .expect("Error");
        let result: String = format!("{}", String::from_utf8_lossy(&output.stderr));
            write_terminal::write_terminal(
                &(input + "\n" + &result),
                text.clone(),
                terminal.clone()
            ).expect("Error");
        });
}
