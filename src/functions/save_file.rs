use fltk::{
    prelude::*,
    enums::{
        Event,
        Key,
    },
    app::event_key,
    tree::Tree
};

pub fn save_file(
    folders: Tree
) {
    let mut folders: Tree = folders.clone();
    folders.handle(move |_, event| {
        match event {
            Event::Shortcut => {
                if event_key() == Key::AltL {
                    println!("foo");
                }
            true
        }
            _ => false
        }
    });
}
