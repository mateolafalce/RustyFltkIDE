use fltk::prelude::*;
#[path="../root/get_root.rs"]
mod get_root;
#[path="../run_a_command.rs"]
mod run_a_command;

pub fn terminal_input_event(
    event: fltk::enums::Event,
    terminal_input: &mut fltk::input::Input,
    terminal_output: fltk::text::TextDisplay,
    terminal_buffer: fltk::text::TextBuffer,
)-> bool {
    match event {
        fltk::enums::Event::KeyUp => { // Handle the key up event
            if terminal_input.value().len() < get_root::get_root().len() {
                terminal_input.set_value(&(terminal_input.value()));
            }
            true
        },
        fltk::enums::Event::Push => { // Handle the push event
            terminal_input.set_readonly(false);
            true
        },
        fltk::enums::Event::KeyDown => {
            if fltk::app::event_key() == fltk::enums::Key::Enter { // Check if the key pressed is the Enter key
                run_a_command::run_a_command(terminal_input.value(),terminal_buffer.clone(),terminal_output.clone()).unwrap();
                terminal_input.set_value(&get_root::get_root()); // Set the input widget value back to the root directory
            }
            true
        },
        fltk::enums::Event::Leave => { // Handle the leave event
            terminal_input.set_readonly(true);
            true
        },
        _ => false
    }
}
