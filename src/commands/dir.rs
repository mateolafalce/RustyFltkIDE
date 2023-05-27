use std::process::Command;
use fltk::text::{
    TextDisplay,
    TextBuffer
};
#[path="../functions/write_terminal.rs"]
mod write_terminal;

pub fn dir(
    root_data: String,
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
    root: String
) {
    let output = Command::new("cmd")
        .args(&["/C", "dir", &root_data])
        .output()
        .expect("Error");
    let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
    write_terminal::write_terminal(
        &(root.clone() + " " + &input + "\n" + &result),
        text,
        terminal
    ).expect("Error");
}
