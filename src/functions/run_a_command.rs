#[path="./specific_commands/commands_for_windows.rs"]
mod commands_for_windows;
#[path="./specific_commands/commands_for_cargo.rs"]
mod commands_for_cargo;
#[path="./root/get_root.rs"]
mod get_root;

pub fn run_a_command(
    input: String,
    text: fltk::text::TextBuffer,
    terminal: fltk::text::TextDisplay,
) -> Result<(), std::io::Error> {
    let raw_input: String = input.to_string();
    let command_input: &str = raw_input.trim_start_matches(&get_root::get_root());
    let mut root: String = get_root::get_root();
    root.pop();
    root.pop();
    // Check the operating system
    if std::env::consts::OS == "windows" {
        // Execute Windows-specific commands
        commands_for_windows::commands_for_windows(
            command_input,
            text.clone(),
            terminal.clone()
        ).unwrap();
    } else {
        // TODO: Implement commands for other operating systems
        print!("todo");
    }
    // Execute Cargo-specific commands
    commands_for_cargo::commands_for_cargo(
        input,
        text.clone(),
        terminal.clone(),
        command_input,
    );
    Ok(())
}
