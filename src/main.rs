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
    save_file
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

fn main() {
    let mut app: App = App::default();
    app.set_scheme(Scheme::Oxy);
    let mut window: Window = window::window();
    let (text_editor, text_buffer): (TextEditor, TextBuffer) = text_editor::text_editor();
    let (terminal_output, terminal_buffer): (TextDisplay, TextBuffer) = terminal_output::terminal_output();
    let (folders, prefix): (Tree, String) = folders::folders();
    let terminal_input: Input = terminal_input::terminal_input(terminal_output.clone(), terminal_buffer.clone());
    let btn_add_folder: Button = btn_add_folder::btn_add_folder(app.clone());
    horizontal_slider::horizontal_slider(
        folders.clone(),
        text_editor,
        terminal_output,
        terminal_input,
        btn_add_folder,
        app
    );
    render_file::render_file(
        folders.clone(),
        text_buffer,
        prefix
    );
    save_file(folders.clone());
    window.end();
    window.show();
    app.run().unwrap();
}
