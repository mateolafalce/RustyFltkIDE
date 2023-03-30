use std::{
    thread,
    fs::metadata
};
use crate::functions::{
    commands::write_terminal,
    root,
    center,
    set_root
};
use fltk::{
        text::{
        TextDisplay,
        TextBuffer
    },
    dialog::alert,
};

pub fn cd_back(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay
){
    thread::spawn(move || {
        let root: String = root().unwrap();
        let mut split_string: Vec<&str> = root.split('\\').collect();
        split_string.pop();
        split_string.pop();
        let new_root: String = split_string.join("\\");
        match metadata(new_root.clone()) {
            Ok(_) => {
                set_root(new_root.clone()).expect("Error");
                write_terminal(
                    &(root.clone() + " " + &input + "\n"),
                    text.clone(),
                    terminal.clone()
                ).expect("Error");
            },
            Err(e) => {
                alert(center().0 - 100, center().1 - 100, &format!("Error: {}\n", e));
            }
        }
    });
}
