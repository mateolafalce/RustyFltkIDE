use fltk::prelude::*;
#[path="./folders_functions/btn_add_folder.rs"]
mod btn_add_folder;
#[path="./folders_functions/btn_delete_folder.rs"]
mod btn_delete_folder;

pub fn options_windows(
    app: fltk::app::App,
    folders: &mut fltk::tree::Tree,
    text_buffer: fltk::text::TextBuffer,
) -> fltk::window::Window {
    // Load icon
    let icon: fltk::image::PngImage = fltk::image::PngImage::load(&std::path::Path::new("src/options.png")).unwrap();
    // Create options window
    let mut options_windows: fltk::window::Window = fltk::window::Window::new(fltk::app::event_x_root(),fltk::app::event_y_root(),300,200,"Options");
    //Style
    options_windows.set_icon(Some(icon));
    options_windows.set_border(true);
    options_windows.set_color(fltk::enums::Color::White);
    // Call the btn_add_folder function
    btn_add_folder::btn_add_folder(
        app.clone(),
        folders.clone(),
        text_buffer.clone(),
        options_windows.clone(),
    );
    // Call the btn_delete_folder function
    btn_delete_folder::btn_delete_folder(
        options_windows.clone(),
    );
    options_windows.end();
    options_windows.show();
    options_windows
}
