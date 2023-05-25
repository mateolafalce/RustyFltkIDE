//#![windows_subsystem = "windows"]

//TODO: fix path location when a new user run it
mod functions;
mod constants;
mod commands;
use functions::{
    window,
    text_editor,
    folders,
    terminal_output,
    terminal_input,
    options_windows,
};
#[path="./functions/sliders/horizontal_slider.rs"]
mod horizontal_slider;
#[path="./functions/sliders/vertical_slider.rs"]
mod vertical_slider;
#[path="./functions/root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="./functions/folders_functions/render_folder.rs"]
mod render_folder;
#[path="./functions/folders_functions/render_all_files_in_folders.rs"]
mod render_all_files_in_folders;
use fltk::prelude::*;

fn main() {
    let mut app: fltk::app::App = fltk::app::App::default();
    app.set_scheme(fltk::app::Scheme::Oxy);
    let mut window: fltk::window::Window = window();
    let (text_editor, text_buffer): (fltk::text::TextEditor, fltk::text::TextBuffer) = text_editor::text_editor();
    let (terminal_output, terminal_buffer): (fltk::text::TextDisplay, fltk::text::TextBuffer) = terminal_output::terminal_output();
    let (mut folders, prefix): (fltk::tree::Tree, Vec<String>) = folders::folders();
    let terminal_input: fltk::input::Input = terminal_input::terminal_input(terminal_output.clone(),terminal_buffer.clone());
    let right_slider: fltk::valuator::NiceSlider = vertical_slider::vertical_slider(text_editor.clone(),terminal_output.clone(),terminal_input.clone(),app);
    horizontal_slider::horizontal_slider(folders.clone(),text_editor.clone(),terminal_output.clone(),terminal_input,app.clone(),right_slider);
    render_all_files_in_folders::render_all_files_in_folders(folders.clone(),text_buffer.clone(),prefix);
    folders.handle(move |folders, event| {
        match event {
            fltk::enums::Event::Push => {
                if fltk::app::event_mouse_button() == fltk::app::MouseButton::Right {
                    options_windows::options_windows(app.clone(),folders,text_buffer.clone());
                }
                true
            },
            fltk::enums::Event::KeyDown => {
                if fltk::app::event_key() == fltk::enums::Key::ControlL {
                    println!("Ctrl pressed!");
                }
            true
            },
            fltk::enums::Event::DndEnter => true,
            fltk::enums::Event::DndDrag => true,
            fltk::enums::Event::DndRelease => true,
            fltk::enums::Event::Paste => {
                set_folders_roots::set_folders_roots(fltk::app::event_text()).unwrap();
                render_folder::render_folder(app.clone(),folders.clone(),text_buffer.clone());
                true
            }
            _ => false,
        }
    });
    window.end();
    window.show();
    app.run().unwrap();
}
