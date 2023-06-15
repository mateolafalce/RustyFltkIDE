use fltk::prelude::*;

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
#[path="../event/center.rs"]
mod center;
#[path="../event/error.rs"]
mod error;

pub fn btn_add_folder(
    app: fltk::app::App,
    folders: fltk::tree::Tree,
    text_buffer: fltk::text::TextBuffer,
    options_windows: fltk::window::Window
) -> fltk::button::Button {
    let mut options_windows: fltk::window::Window = options_windows.clone();
    let folders: fltk::tree::Tree = folders.clone();
    // Create a button for adding a project folder
    let mut add_project_folder: fltk::button::Button = fltk::button::Button::new(25, 10, 250, 20, "@fileopen  Add Project");
    add_project_folder.set_frame(fltk::enums::FrameType::UpBox);
    // Set a callback for the button to handle the add project folder functionality
    add_project_folder.set_callback(move |_| {
        options_windows.hide();
        // Show a native file chooser dialog to select a folder
        let mut dialog: fltk::dialog::NativeFileChooser = fltk::dialog::NativeFileChooser::new(
            fltk::dialog::NativeFileChooserType::BrowseDir
        );
        dialog.show();
        let folder_input: String = dialog.filename().display().to_string();
        if folder_input != "" {
            options_windows.set_label("Loading ...");
            // Call the set_folders_roots function to set the root folder for the project
            match set_folders_roots::set_folders_roots(folder_input) {
                Ok(_) => {
                    // If successful, render the folder in the tree view
                    render_folder::render_folder(app.clone(), folders.clone(), text_buffer.clone());
                }
                Err(e) => {
                    options_windows.set_label("Options");
                    error::error(&e.to_string());
                }
            }
        }
    });
    // Handle the mouse events for the button
    add_project_folder.handle(move |_, event| {
        mouse_select::mouse_select(event)
    });
    add_project_folder
}
