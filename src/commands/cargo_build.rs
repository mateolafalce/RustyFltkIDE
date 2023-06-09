#[path="../functions/event/command_result.rs"]
mod command_result;

// This function builds a Cargo project asynchronously
pub fn cargo_build(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
    root: String
) {
    // Spawn a new thread to run the build command
    std::thread::spawn(move || {
        // Execute the "cargo build" command with the given manifest path
        let output: std::process::Output = std::process::Command::new("cargo")
            .args(&["build", "--manifest-path", &((root + "\\Cargo.toml"))])
            .output()
            .expect("Error");
        // Update console
        command_result::command_result(output, input, text, terminal);
    });
}
