use fltk::prelude::*;
use crate::functions::{
    center
};
#[path="../root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;
#[path="./render_folder.rs"]
mod render_folder;
#[path="../event/mouse_select.rs"]
mod mouse_select;

pub fn btn_add_folder(
    app: fltk::app::App,
    folders: fltk::tree::Tree,
    text_buffer: fltk::text::TextBuffer,
    options_windows: fltk::window::Window
) -> fltk::button::Button {
    let mut options_windows: fltk::window::Window = options_windows.clone();
    let folders: fltk::tree::Tree = folders.clone();
    let mut add_project_folder: fltk::button::Button = fltk::button::Button::new(25, 10, 250, 20, "@fileopen  Add Project");
    add_project_folder.set_frame(fltk::enums::FrameType::UpBox);
    add_project_folder.set_callback(move |_| {
        options_windows.hide();
        let mut dialog: fltk::dialog::NativeFileChooser = fltk::dialog::NativeFileChooser::new(
            fltk::dialog::NativeFileChooserType::BrowseDir
        );
        dialog.show();
        let folder_input: String = dialog.filename().display().to_string();
        if folder_input != "" {
            options_windows.set_label("Loading ...");
            match set_folders_roots::set_folders_roots(folder_input) {
                Ok(_) => {
                    render_folder::render_folder(
                        app.clone(),
                        folders.clone(),
                        text_buffer.clone(),
                    );
                }
                Err(e) => {
                    options_windows.set_label("Options");
                    fltk::dialog::alert(center().0 - 100, center().1 - 100, &format!("Error: {}\n", e));
                }
            }
        }
    });
    //Manage the mouse event
    add_project_folder.handle(move |_, event| {
        mouse_select(event).unrwarp();
    });
    add_project_folder
}
