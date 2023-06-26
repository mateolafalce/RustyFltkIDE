use std::io::Read;
#[path="../../commands/dir.rs"]
mod dir;
#[path="../../commands/cd_back.rs"]
mod cd_back;
#[path="../../commands/clear.rs"]
mod clear;
#[path="../../commands/cd_to.rs"]
mod cd_to;
#[path="../root/get_root.rs"]
mod get_root;
#[path="../write_terminal.rs"]
mod write_terminal;

pub fn commands_for_windows(
    input: &str,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
) -> Result<(), std::io::Error> {
    let root: String = get_root::get_root();
    // Open and read the file containing the root data
    let mut file: std::fs::File = std::fs::File::open("src/constants/root.rs").expect("Error");
    let mut root_data: String = String::new();
    file.read_to_string(&mut root_data).unwrap();
    // Process the input
    let raw_input: String = input.to_string();
    let split_raw_input: Vec<String> = raw_input.split(' ').map(|s| s.to_owned()).collect();
    let command_input: &str = raw_input.trim_start_matches(&root);
    // Handle different commands
    if split_raw_input[0] == "cd" && split_raw_input[1] != ".." {
        cd_to::cd_to(input.to_owned(), text.clone(), terminal.clone(), split_raw_input, root.clone());
    }
    match command_input {
        "dir" => {
            // Call the dir module
            dir::dir(root_data, input.to_owned(), text, terminal);
        }
        "cd .." => {
            // Call the cd_back module
            cd_back::cd_back(input.to_owned(), text, terminal, root.clone());
        }
        "clear" => {
            // Call the clear module
            clear::clear(text, terminal);
        }
        _ => {
            // Call the write_terminal module with a new line
            write_terminal::write_terminal("\n", text, terminal);
        }
    }
    Ok(())
}
