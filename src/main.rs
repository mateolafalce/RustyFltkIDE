// #![windows_subsystem = "windows"]
// TODO: Fix path location when a new user runs it

#[path = "./functions/sliders/horizontal_slider.rs"]
mod horizontal_slider;
#[path = "./functions/sliders/vertical_slider.rs"]
mod vertical_slider;
#[path = "./functions/folders_functions/render_all_files_in_folders.rs"]
mod render_all_files_in_folders;
#[path = "./functions/terminal_input.rs"]
mod terminal_input;
#[path = "./functions/terminal_output.rs"]
mod terminal_output;
#[path = "./functions/folders.rs"]
mod folders;
#[path = "./functions/window.rs"]
mod window;
#[path = "./functions/text_editor.rs"]
mod text_editor;
#[path = "./functions/event/folders_events.rs"]
mod folders_events;
#[path = "./functions/event/app_events.rs"]
mod app_events;
use fltk::prelude::*;

fn main() {
    let mut app: fltk::app::App = fltk::app::App::default();
    app.set_scheme(fltk::app::Scheme::Oxy);
    // Create the main window
    let mut window: fltk::window::Window = window::window();
    // Create the text editor and its associated text buffer
    let (text_editor, text_buffer): (fltk::text::TextEditor, fltk::text::TextBuffer) =
        text_editor::text_editor();
    // Create the terminal output display and its associated text buffer
    let (terminal_output, terminal_buffer): (fltk::text::TextDisplay, fltk::text::TextBuffer) =
        terminal_output::terminal_output();
    // Create the folders tree and a prefix vector
    let (mut folders, prefix): (fltk::tree::Tree, Vec<String>) = folders::folders();
    // Create the terminal input field
    let terminal_input: fltk::input::Input =
        terminal_input::terminal_input(terminal_output.clone(), terminal_buffer.clone());
    // Create the right slider
    let right_slider: fltk::valuator::NiceSlider =
        vertical_slider::vertical_slider(text_editor.clone(), terminal_output.clone(), terminal_input.clone(), app);
    // Set up the horizontal slider
    horizontal_slider::horizontal_slider(
        folders.clone(),
        text_editor.clone(),
        terminal_output.clone(),
        terminal_input,
        app.clone(),
        right_slider,
    );
    // Render all files in folders
    render_all_files_in_folders::render_all_files_in_folders(
        folders.clone(),
        text_buffer.clone(),
        prefix,
    );
    // Handle events for the folders tree
    folders.handle(move |folders, event| {
        folders_events::folders_events(
            folders,
            event,
            app.clone(),
            text_buffer.clone(),
            text_editor.clone(),
        )
    });
    // Create a channel for sending and receiving application events
    let (sender, receiver): (fltk::app::Sender<app_events::Message>, fltk::app::Receiver<app_events::Message>) =
        fltk::app::channel::<app_events::Message>();
    // Create the system menu bar
    let menu: fltk::menu::SysMenuBar = fltk::menu::SysMenuBar::default().with_size(0, 0);
    window.end();
    window.show();
    // Start the main event loop
    while app.wait() {
        app_events::app_events(menu.clone(), sender, receiver);
    }
}
