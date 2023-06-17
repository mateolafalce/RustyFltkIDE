#[path="../functions/event/command_result.rs"]
mod command_result;

pub fn cargo_clean(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
    root: String
) {
    std::thread::spawn(move || {
        let output: std::process::Output = std::process::Command::new("cargo")// Create a new cargo command process
            .args(&["clean", "--manifest-path", &((root + "\\Cargo.toml"))])// Specify the 'clean' command with manifest path
            .output()
            .expect("Error");
        command_result::command_result(output, input, text, terminal);// Process the command result
    });
}
