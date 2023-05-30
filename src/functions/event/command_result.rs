#[path="../write_terminal.rs"]
mod write_terminal;

pub fn command_result(
    output: std::process::Output,
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay
) -> bool {
    let result: String = format!("{}", String::from_utf8_lossy(&output.stdout));
    write_terminal::write_terminal(
        &(input + "\n" + &result),
        text.clone(),
        terminal.clone()
    ).unwrap();
    true
}
