#[path="../functions/event/command_result.rs"]
mod command_result;

pub fn cargo_build_release(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
    root: String
){
    std::thread::spawn(move || {
        let output = std::process::Command::new("cargo")
            .args(&["build", "--release", "--manifest-path", &((root + "\\Cargo.toml"))])
            .output()
            .expect("Error");
        command_result::command_result(output, input, text, terminal);
    });
}
