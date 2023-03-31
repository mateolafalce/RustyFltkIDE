use fltk::{
    prelude::*,
    button::Button,
    enums::{
        Cursor,
        Event,
        FrameType
    },
    dialog::{
        NativeFileChooser,
        NativeFileChooserType
    },
    draw::set_cursor,
    app::App,
    tree::Tree
};
use crate::functions::{
    set_folders_roots::set_folders_roots,
    get_paths::get_all_paths_in_directory,
    get_folders_roots::get_folders_roots
};
use std::path::Path;

pub fn btn_add_folder(
    app: App,
    folders: Tree
) -> Button {
    let mut folders: Tree = folders.clone();
    let mut add_project_folder: Button = Button::new(0, 580, 200, 20, "🗃️ Add Project");
    add_project_folder.set_frame(FrameType::UpBox);
    add_project_folder.set_callback(move |_| {
        let mut dialog: NativeFileChooser = NativeFileChooser::new(NativeFileChooserType::BrowseDir);
        dialog.show();
        set_folders_roots(dialog.filename().display().to_string()).unwrap();
        let (raw_path, is_the_repository_clear): (String, bool) = get_folders_roots();
        let root = Path::new(&raw_path);
        let mut split_path: Vec<&str> = raw_path.as_str().split('\\').collect();
        split_path.pop();
        let prefix: String = split_path.join("/");
        let paths: Vec<String> = get_all_paths_in_directory(&root, prefix.clone(), is_the_repository_clear);
        for path in &paths {
            folders.add(&path);
        }
        app.redraw();
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
