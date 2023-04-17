use std::fs::metadata;
use crate::functions::{
    write_terminal,
    center,
};
use fltk::{
    text::{
        TextDisplay,
        TextBuffer
    },
    dialog::alert
};
#[path="../functions/root/set_root.rs"]
mod set_root;

pub fn cd_to(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
    split_raw_input: Vec<String>,
    root: String
){
    let mut split_string: Vec<&str> = root.split('\\').collect();
    split_string.pop();
    split_string.push(&split_raw_input[1]);
    let new_root: String = split_string.join("\\");
    match metadata(new_root.clone()) {
        Ok(_) => {
            set_root::set_root(new_root.clone()).expect("Error");
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
}
