use fltk::{
    prelude::*,
    button::Button,
    window::Window,
    enums::{
        Cursor,
        Event,
        FrameType
    },
    dialog::{
        NativeFileChooser,
        NativeFileChooserType,
        alert
    },
    draw::set_cursor,
    app::App,
    tree::Tree,
    text::{
        TextBuffer,
    }
};
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


pub fn btn_add_folder(
    app: App,
    folders: Tree,
    text_buffer: TextBuffer,
    options_windows: Window
) -> Button {
    let mut options_windows: Window = options_windows.clone();
    let folders: Tree = folders.clone();
    let mut add_project_folder: Button = Button::new(25, 10, 250, 20, "@fileopen  Add Project");
    add_project_folder.set_frame(FrameType::UpBox);
    add_project_folder.set_callback(move |_| {
            options_windows.hide();
            let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseDir);
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
                        alert(center().0 - 100, center().1 - 100, &format!("Error: {}\n", e));
                    }
                }
            }
    });

    add_project_folder.handle(move |_, event| {
        match event {
            Event::Enter => {
                set_cursor(Cursor::Hand);
                true
            },
            Event::Leave => {
                set_cursor(Cursor::Arrow);
                true
            },
            _ => false,
        }
    });
    add_project_folder
}
