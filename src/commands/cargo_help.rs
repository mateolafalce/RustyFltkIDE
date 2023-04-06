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

pub fn cargo_help(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay
) {
    thread::spawn(move || {
        let output = Command::new("cargo")
            .args(&["--help", &(root.clone())()])
            .output()
            .expect("Error");
        let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
            write_terminal(
                &(input + "\n" + &result),
                text.clone(),
                terminal.clone()
            ).expect("Error");
        }
    );
}
