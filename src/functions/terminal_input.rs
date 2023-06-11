use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
    enums::{
        FrameType,
        Event,
        Key
    },
    input::Input,
    app::event_key
};

#[path="../constants/text_size.rs"]
mod text_size;
#[path="../constants/font.rs"]
mod font;
#[path="./root/get_root.rs"]
mod get_root;
#[path="./run_a_command.rs"]
mod run_a_command;

pub fn terminal_input(
    terminal_output: TextDisplay,
    terminal_buffer: TextBuffer,
) -> Input {
    let mut terminal_input = Input::new(204, 570, 786, 30, None);
    terminal_input.set_text_font(font::FONT);
    terminal_input.set_text_size(text_size::CONSOLE_TEXT_SIZE);
    terminal_input.set_frame(FrameType::FlatBox);
    terminal_input.set_value(&get_root::get_root()); // Set the initial value for the input widget to a root directory
    terminal_input.set_readonly(true); // Set the input widget to be readonly by default
    let _terminal_input = terminal_output.clone();
    terminal_input.handle(move |terminal_input, event| {
        match event {
            Event::KeyUp => { // Handle the key up event
                if terminal_input.value().len() < get_root::get_root().len() {
                    terminal_input.set_value(&(terminal_input.value()));
                }
                true
            },
            Event::Push => { // Handle the push event
                terminal_input.set_readonly(false);
                true
            },
            Event::KeyDown => {
                if event_key() == Key::Enter { // Check if the key pressed is the Enter key
                    run_a_command::run_a_command(terminal_input.value(),terminal_buffer.clone(),terminal_output.clone()).unwrap();
                    terminal_input.set_value(&get_root::get_root()); // Set the input widget value back to the root directory
                }
                true
            },
            Event::Leave => { // Handle the leave event
                terminal_input.set_readonly(true);
                true
            },
            _ => false
        }
    });
    terminal_input // Return the input widget
}
