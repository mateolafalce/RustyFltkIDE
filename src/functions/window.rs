// Import the necessary modules from the fltk crate and the std library.
use fltk::{
    prelude::*,
    window::Window,
    image::IcoImage,
    enums::Color
};
use std::path::Path;

// Define a function called "window" that returns a Window object.
pub fn window() -> Window {
    // Load an icon from a file and assign it to a variable called "icon".
    let icon: IcoImage = IcoImage::load(&Path::new("src/rusty.ico")).unwrap();
    // Create a new window with the specified size and title.
    let mut window: Window = Window::new(100, 100, 1000, 600, "🪂 Rusty IDE 🪂");
    // Set the window's icon to the icon we loaded earlier.
    window.set_icon(Some(icon));
    // Enable the window's border.
    window.set_border(true);
    // Allow the user to resize the window.
    window.make_resizable(true);
    // Set the window's background color to white.
    window.set_color(Color::White);
    // Return the window object.
    window
}
