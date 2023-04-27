use fltk::{
    prelude::*,
    button::Button,
    window::Window,
    enums::{
        Cursor,
        Color,
        Event,
        FrameType
    },
    draw::set_cursor,
    image::PngImage,
    app::{
        event_x_root,
        event_y_root
    },
    text::{
        TextBuffer,
        TextDisplay
    }
};
use std::path::Path;

/*#[path="../root/set_folders_roots.rs"]
mod set_folders_roots;*/
#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;

pub fn btn_delete_folder(
    options_windows: Window,
) -> Button {
    let mut options_windows: Window = options_windows.clone();
    let mut delete_folder: Button = Button::new(25, 35, 250, 20, "🗑️ Delete Project");
    delete_folder.set_frame(FrameType::UpBox);
    delete_folder.set_callback(move |_| {
        options_windows.hide();
        let icon: PngImage = PngImage::load(&Path::new("src/options.png")).unwrap();
        let mut options_windows: Window = Window::new(
            event_x_root(),
            event_y_root(),
            300,
            200,
            "🗑️ Delete Project"
        );
        options_windows.set_icon(Some(icon));
        options_windows.set_border(true);
        options_windows.set_color(Color::White);
        options_windows.end();
        options_windows.show();

        
        let (_paths, is_the_repository_clear): (Vec<String>, bool) = get_folders_roots::get_folders_roots();
        /*if is_the_repository_clear {
            let mut buf: TextBuffer = TextBuffer::default();
            buf.set_text("You don't own any repositories on Rusty");
            let mut txt_clear_of_repositories: TextDisplay = TextDisplay::new(25, 10, 200, 100, None);
            txt_clear_of_repositories.set_buffer(buf);
        }*/
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
