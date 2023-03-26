use fltk::{
    prelude::*,
    text::{
        TextDisplay,
        TextBuffer
    },
    enums::{
        Align,
        FrameType,
        Event,
        Key
    },
    input::Input,
    app::event_key
};
use std::{
    fs::File,
    io::{
        Read,
        Write
    },
    env::consts::OS,
};
use crate::{
    functions::commands::run_a_command,
    constants::{
        CONSOLE_TEXT_SIZE,
        FONT
    }
};

pub fn terminal() {
    let terminal_buffer: TextBuffer = TextBuffer::default();
    let mut terminal: TextDisplay = TextDisplay::new(151, 450, 849, 120, None);
    let terminal_text: String = format!("Operating system: {}\n🦀 Rusty IDE console 💻\n\n", OS);
    terminal.set_buffer(Some(terminal_buffer.clone()));
    terminal.buffer().unwrap().append(&terminal_text);
    terminal.set_scrollbar_align(Align::Right);
    terminal.set_text_font(FONT);
    terminal.set_text_size(CONSOLE_TEXT_SIZE);
    terminal.set_scrollbar_size(18);
    terminal.set_frame(FrameType::NoBox);
    let mut input = Input::new(154, 570, 849, 30, None);
    input.set_text_font(FONT);
    input.set_text_size(CONSOLE_TEXT_SIZE);
    input.set_frame(FrameType::FlatBox);
    input.set_value(&root().unwrap());
    input.set_readonly(true);
    input.handle(move |input, event| {
        match event {
            Event::KeyUp => {
                if input.value().len() < root().unwrap().len() {
                    input.set_value(&(input.value() + "🎯"));
                }
                true
            },
            Event::Push => {
                input.set_readonly(false);
                true
            },
            Event::KeyDown => {
                if event_key() == Key::Enter {
                    run_a_command(
                        input.value(),
                        terminal_buffer.clone(),
                        terminal.clone()
                    ).expect("Error");
                    input.set_value(&root().unwrap());
                }
                true
            },
            Event::Leave => {
                input.set_readonly(true);
                true
            },
            _ => false
        }
    });
}

pub fn root() -> Result<String, std::io::Error> {
    let mut file: File = File::open("src/constants/root.rs").expect("Error");
    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            contents.push_str(r"\🎯");
            Ok(contents)
        },
        Err(_) => {
            Ok("root".to_string())//TODO
        }
    }
}
pub fn set_root(root: String) -> Result<(), std::io::Error> {
    let mut file: File = File::create("src/constants/root.rs")?;
    file.write_all(root.as_bytes())?;
    Ok(())
}
