#[path="../functions/event/command_result.rs"]
mod command_result;

pub fn dir(
    root_data: String,
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
) {
    let output: std::process::Output = std::process::Command::new("cmd")
        .args(&["/C", "dir", &root_data])
        .output()
        .expect("Error");
    command_result::command_result(output, input, text, terminal);
}
