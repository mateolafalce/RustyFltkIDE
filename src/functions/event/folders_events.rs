use fltk::prelude::*;
#[path="../options_windows.rs"]
mod options_windows;
#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;
#[path="../folders_functions/render_folder.rs"]
mod render_folder;

pub fn folders_events(
    folders: &mut fltk::tree::Tree,
    event: fltk::enums::Event,
    app: fltk::app::App,
    mut text_buffer: fltk::text::TextBuffer,
    mut text_editor: fltk::text::TextEditor
) -> bool {
    match event {
        fltk::enums::Event::Push => {
            // Handle click
            if fltk::app::event_mouse_button() == fltk::app::MouseButton::Right {
                options_windows::options_windows(app.clone(),folders,text_buffer.clone());
            } else {
                match folders.get_item_focus() {
                    Some(val) => {
                        println!("{}", &val.label().unwrap());
                        text_buffer.set_text(&val.label().unwrap());
                        text_editor.set_buffer(Some(text_buffer.clone()));
                    },
                    None => (),
                }
            };
            true
        },
        fltk::enums::Event::DndEnter => true, // Handle drag and drop enter event
        fltk::enums::Event::DndDrag => true, // Handle drag and drop drag event
        fltk::enums::Event::DndRelease => true, // Handle drag and drop release event
        fltk::enums::Event::Paste => {
            // Handle paste event
            set_folders_roots::set_folders_roots(fltk::app::event_text()).unwrap();
            render_folder::render_folder(app.clone(),folders.clone(),text_buffer.clone());
            true
        }
        _ => false,
    }
}
