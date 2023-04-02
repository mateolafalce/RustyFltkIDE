//#![windows_subsystem = "windows"]
mod functions;
mod constants;
mod commands;
use functions::{
    window,
    text_editor,
    folders,
    terminal_output,
    terminal_input,
    horizontal_slider,
    btn_add_folder,
    render_file,
    save_file,
    vertical_slider,
    options_windows
};
use fltk::{
    prelude::*,
    window::Window,
    app::{
        App,
        Scheme
    },
    tree::Tree,
    text::{
        TextEditor,
        TextDisplay,
        TextBuffer
    },
    input::Input,
    button::Button,
};
use fltk::draw::set_cursor;
use fltk::enums::{Cursor, Event};
use fltk::app::{event_mouse_button, MouseButton};

fn main() {
    let mut app: App = App::default();
    app.set_scheme(Scheme::Oxy);
    let mut window: Window = window::window();
    let (text_editor, text_buffer): (TextEditor, TextBuffer) = text_editor::text_editor();
    let (terminal_output, terminal_buffer): (TextDisplay, TextBuffer) = terminal_output::terminal_output();
    let (mut folders, prefix): (Tree, Vec<String>) = folders::folders();
    let terminal_input: Input = terminal_input::terminal_input(
        terminal_output.clone(),
        terminal_buffer.clone()
    );
    /*horizontal_slider::horizontal_slider(
        folders.clone(),
        text_editor.clone(),
        terminal_output.clone(),
        terminal_input,
        btn_add_folder,
        app.clone()
    );*/
    if prefix.len() > 0 {
        for i in 0..prefix.len() - 1 {
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    if prefix.len() == 1 {
        for i in 0..prefix.len() {
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    save_file(folders.clone());
    vertical_slider::vertical_slider(
        text_editor,
        terminal_output,
        app
    );
    folders.handle(move |folders, event| {
        match event {
            Event::Push => {
                if event_mouse_button() == MouseButton::Right {
                    options_windows::options_windows(
                        app.clone(),
                        folders,
                        text_buffer.clone()
                    );
                }
                true
            },
            _ => false,
        }
    });
    window.end();
    window.show();
    app.run().unwrap();
}
