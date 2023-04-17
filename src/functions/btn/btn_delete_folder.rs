use fltk::{
    prelude::*,
    button::Button,
    window::Window,
    enums::{
        Cursor,
        Event,
        FrameType
    },
    draw::set_cursor,
    app::App,
    tree::Tree,
    text::TextBuffer,
};
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;

pub fn btn_delete_folder(
    app: App,
    folders: Tree,
    text_buffer: TextBuffer,
    options_windows: Window,
) -> Button {
    let _options_windows: Window = options_windows.clone();
    let _folders: Tree = folders.clone();
    let mut delete_folder: Button = Button::new(25, 35, 250, 20, "🗑️ Delete Project");
    delete_folder.set_frame(FrameType::UpBox);
    delete_folder.set_callback(move |_| {
        let (raw_path, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
        //group.clear();
    });
    delete_folder.handle(move |_, event| {
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
    delete_folder
}
