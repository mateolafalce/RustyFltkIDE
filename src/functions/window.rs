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
    window.set_icon(Some(icon));
    window.set_border(true);
    window.make_resizable(true);
    window.set_color(Color::White);
    window
}
