use fltk::prelude::*;

pub fn window() -> Window {
    let icon: fltk::image::IcoImage = fltk::image::IcoImage::load(&std::path::Path::new("src/rusty.ico")).unwrap();
    let mut window: fltk::window::Window = fltk::window::Window::new(100, 100, 1000, 600, "🪂 Rusty IDE 🪂");
    window.set_icon(Some(icon)); // Set the window's icon to the icon we loaded earlier.
    window.set_border(true); // Enable the window's border.
    window.make_resizable(true); // Allow the user to resize the window.
    window.set_color(fltk::enums::Color::White); // Set the window's background color to white.
    window // Return the window object.
}
