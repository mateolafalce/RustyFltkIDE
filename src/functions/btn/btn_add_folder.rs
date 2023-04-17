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
    render_file::render_file,
    center
};
#[path="../root/get_all_paths_in_directory.rs"]
mod get_all_paths_in_directory;
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;


use std::path::Path;

pub fn btn_add_folder(
    app: App,
    folders: Tree,
    text_buffer: TextBuffer,
    options_windows: Window
) -> Button {
    let mut options_windows: Window = options_windows.clone();
    let mut folders: Tree = folders.clone();
    let mut add_project_folder: Button = Button::new(25, 10, 250, 20, "@fileopen  Add Project");
    add_project_folder.set_frame(FrameType::UpBox);
    add_project_folder.set_callback(move |_| {
            let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseDir);
            dialog.show();
            let folder_input: String = dialog.filename().display().to_string();
            if folder_input != "" {
                options_windows.set_label("Loading ...");
                match set_folders_roots::set_folders_roots(folder_input) {
                    Ok(_) => {
                        let mut prefix: Vec<String> = vec![];
                        let mut close_tree: Vec<String> = vec![];
                        let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
                        for i in 0..raw_path.len() - 1 {
                            let mut split_path: Vec<&str> = raw_path[i].as_str().split('\\').collect();
                            close_tree.push(split_path.last().unwrap().to_string());
                            split_path.pop();
                            prefix.push(split_path.join("/"));
                            let paths: Vec<String> = get_all_paths_in_directory::get_all_paths_in_directory(
                                &Path::new(&raw_path[i]),
                                prefix[i].clone(),
                                is_the_repository_clear
                            );
                            for path in &paths {
                                folders.add(&path);
                            }
                        }
                        for i in 0..raw_path.len() - 1 {
                            let _ = folders.close(&close_tree[i], true);
                        }
                        if prefix.len() == 1 {
                            for i in 0..prefix.len() {
                                render_file(
                                    folders.clone(),
                                    text_buffer.clone(),
                                    prefix[i].clone()
                                );
                            }
                        } else if prefix.len() > 1 {
                            for i in 0..prefix.len() - 1 {
                                render_file(
                                    folders.clone(),
                                    text_buffer.clone(),
                                    prefix[i].clone()
                                );
                            }
                        }
                        app.redraw();
                        options_windows.hide();
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
