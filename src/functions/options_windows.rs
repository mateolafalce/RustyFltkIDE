use fltk::{
    prelude::*,
    window::Window,
    button::Button,
    app::App,
    text::TextBuffer,
    tree::Tree
};
use crate::functions::{
    btn_add_folder,
};

pub fn options_windows(
    app: App,
    folders: &mut Tree,
    text_buffer: TextBuffer
) -> Window {
    let mut options_windows: Window = Window::new(10, 10, 380, 280, "Options");
    let btn_add_folder: Button = btn_add_folder::btn_add_folder(
        app.clone(),
        folders.clone(),
        text_buffer.clone()
    );
    options_windows.end();
    options_windows.show();
    options_windows
}
