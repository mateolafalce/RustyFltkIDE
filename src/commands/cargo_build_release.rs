#[path="../functions/event/command_result.rs"]
mod command_result;

// This function builds a Cargo project in release mode asynchronously
pub fn cargo_build_release(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
    root: String
){
    // Spawn a new thread to run the build command
    std::thread::spawn(move || {
        // Execute the "cargo build --release" command with the given manifest path
        let output: std::process::Output  = std::process::Command::new("cargo")
            .args(&["build", "--release", "--manifest-path", &((root + "\\Cargo.toml"))])
            .output()
            .expect("Error");
        // Update Console
        command_result::command_result(output, input, text, terminal);
    });
}
