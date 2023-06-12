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
    text_buffer: fltk::text::TextBuffer
) -> bool {
    match event {
        fltk::enums::Event::Push => {
            if fltk::app::event_mouse_button() == fltk::app::MouseButton::Right {
                options_windows::options_windows(app.clone(),folders,text_buffer.clone());
            }
            true
        },
        fltk::enums::Event::DndEnter => true,
        fltk::enums::Event::DndDrag => true,
        fltk::enums::Event::DndRelease => true,
        fltk::enums::Event::Paste => {
            set_folders_roots::set_folders_roots(fltk::app::event_text()).unwrap();
            render_folder::render_folder(app.clone(),folders.clone(),text_buffer.clone());
            true
        }
        _ => false,
    }
}
