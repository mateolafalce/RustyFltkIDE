//#![windows_subsystem = "windows"]
mod functions;
mod constants;
use functions::{
    window,
    text_editor,
    folders,
    terminal
};
use fltk::{
    prelude::*,
    window::Window,
    app::{
        App,
        Scheme
    },
};

fn main() {
    let mut app: App = App::default();
    app.set_scheme(Scheme::Oxy);
    let mut window: Window = window::window();
    folders::folders();
    text_editor::text_editor();
    terminal::terminal();
    window.end();
    window.show();
    app.run().unwrap();
}
