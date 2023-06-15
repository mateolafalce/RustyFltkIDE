use fltk::prelude::*;

#[path="../root/get_folders_roots.rs"]
mod get_folders_roots;
#[path="../event/mouse_select.rs"]
mod mouse_select;

pub fn btn_delete_folder(
    options_windows: fltk::window::Window,
) -> fltk::button::Button {
    let mut options_windows: fltk::window::Window = options_windows.clone();
    // Creating the delete_folder button
    let mut delete_folder: fltk::button::Button = fltk::button::Button::new(25, 35, 250, 20, "🗑️ Delete Project");
    delete_folder.set_frame(fltk::enums::FrameType::UpBox);
    // Setting the callback function for the delete_folder button
    delete_folder.set_callback(move |_| {
        // Hiding the options_windows window
        options_windows.hide();
        let icon: fltk::image::PngImage = fltk::image::PngImage::load(&std::path::Path::new("src/options.png")).unwrap();
        // Creating a new options_windows window
        let mut options_windows: fltk::window::Window = fltk::window::Window::new(
            fltk::app::event_x_root(),
            fltk::app::event_y_root(),
            300,
            200,
            "🗑️ Delete Project"
        );
        options_windows.set_icon(Some(icon));
        options_windows.set_border(true);
        options_windows.set_color(fltk::enums::Color::White);
        options_windows.end();
        options_windows.show();
    });
    // Handling events for the delete_folder button
    delete_folder.handle(move |_, event| {
        mouse_select::mouse_select(event) // Calling a function from the mouse_select module
    });
    delete_folder
}
