#[path="../functions/root/set_root.rs"]
mod set_root;
#[path="../functions/write_terminal.rs"]
mod write_terminal;
#[path="../functions/event/center.rs"]
mod center;
#[path="../functions/event/error.rs"]
mod error;

pub fn cd_to(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
    split_raw_input: Vec<String>,
    root: String
) -> bool {
    let mut split_string: Vec<&str> = root.split('\\').collect();
    split_string.pop();  // Remove the last element (emoji)
    split_string.push(&split_raw_input[1]);
    let new_root: String = split_string.join("\\");

    match std::fs::metadata(new_root.clone()) {  // Check if the new root path exists
        Ok(_) => {
            set_root::set_root(new_root.clone()).expect("Error");
            write_terminal::write_terminal(
                &(root.clone() + " " + &input + "\n"),
                text.clone(),
                terminal.clone()
            );  // Write the terminal command to the text buffer using the `write_terminal` module
        },
        Err(e) => {
            error::error(&e.to_string());  // Display the error message using the `error` module
        }
    }
    true
}
