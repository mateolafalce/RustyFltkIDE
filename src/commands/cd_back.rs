use std::fs::metadata;
use fltk::{
        text::{
        TextDisplay,
        TextBuffer
    },
    dialog::alert,
};
#[path="../functions/root/set_root.rs"]
mod set_root;
#[path="../functions/write_terminal.rs"]
mod write_terminal;
#[path="../functions/event/center.rs"]
mod center;

pub fn cd_back(
    input: String,
    text: TextBuffer,
    terminal: TextDisplay,
    root: String
){
    let mut split_string: Vec<&str> = root.split('\\').collect();
    split_string.pop();
    split_string.pop();
    let new_root: String = split_string.join("\\");
    match metadata(new_root.clone()) {
        Ok(_) => {
            set_root::set_root(new_root.clone()).expect("Error");
            write_terminal::write_terminal(
                &(root.clone() + " " + &input + "\n"),
                text.clone(),
                terminal.clone()
            ).expect("Error");
        },
        Err(e) => {
            alert(center::center().0 - 100, center::center().1 - 100, &format!("Error: {}\n", e));
        }
    }
}
