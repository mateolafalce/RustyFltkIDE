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
use crate::{
    functions::run_a_command,
    constants::{
        CONSOLE_TEXT_SIZE,
        FONT
    }
};
use crate::functions::root;

pub fn terminal_input(
    terminal_output: TextDisplay,
    terminal_buffer: TextBuffer,
) -> Input {
    let mut terminal_input = Input::new(204, 570, 786, 30, None);
    terminal_input.set_text_font(FONT);
    terminal_input.set_text_size(CONSOLE_TEXT_SIZE);
    terminal_input.set_frame(FrameType::FlatBox);
    terminal_input.set_value(&root());
    terminal_input.set_readonly(true);
    let _terminal_input = terminal_output.clone();
    terminal_input.handle(move |terminal_input, event| {
        match event {
            Event::KeyUp => {
                if terminal_input.value().len() < root().len() {
                    terminal_input.set_value(&(terminal_input.value() + "🎯"));
                }
                true
            },
            Event::Push => {
                terminal_input.set_readonly(false);
                true
            },
            Event::KeyDown => {
                if event_key() == Key::Enter {
                    run_a_command(
                        terminal_input.value(),
                        terminal_buffer.clone(),
                        _terminal_input.clone()
                    ).expect("Error");
                    terminal_input.set_value(&root());
                }
                true
            },
            Event::Leave => {
                terminal_input.set_readonly(true);
                true
            },
            _ => false
        }
    });
    terminal_input
}
