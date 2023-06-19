use fltk::prelude::*;
#[path="../constants/console_text_size.rs"]
mod console_text_size;
#[path="../constants/font.rs"]
mod font;
#[path="./root/get_root.rs"]
mod get_root;
#[path="./event/terminal_input_event.rs"]
mod terminal_input_event;

pub fn terminal_input(
    terminal_output: fltk::text::TextDisplay,
    terminal_buffer: fltk::text::TextBuffer,
) -> fltk::input::Input {
    let mut terminal_input: fltk::input::Input = fltk::input::Input::new(204, 570, 786, 30, None);
    terminal_input.set_text_font(font::FONT);
    terminal_input.set_text_size(console_text_size::CONSOLE_TEXT_SIZE);
    terminal_input.set_frame(fltk::enums::FrameType::FlatBox);
    terminal_input.set_value(&get_root::get_root()); // Set the initial value for the input widget to a root directory
    terminal_input.set_readonly(true); // Set the input widget to be readonly by default
    terminal_input.handle(move |terminal_input, event| {
        terminal_input_event::terminal_input_event(event, terminal_input,terminal_output.clone(),terminal_buffer.clone())
    });
    terminal_input // Return the input widget
}
