use fltk::{
    prelude::*,
    window::Window,
    image::IcoImage,
    enums::Color
};
use std::path::Path;

pub fn window() -> Window {
    let icon: IcoImage = IcoImage::load(&Path::new("src/rusty.ico")).unwrap();
    let mut window: Window = Window::new(100, 100, 1000, 600, "🪂 Rusty IDE 🪂");
    window.set_icon(Some(icon)); // Set the window's icon to the icon we loaded earlier.
    window.set_border(true); // Enable the window's border.
    window.make_resizable(true); // Allow the user to resize the window.
    window.set_color(Color::White); // Set the window's background color to white.
    window // Return the window object.
}
