use fltk::{
    prelude::*,
    tree::Tree,
    text::{
        TextBuffer,
        TextEditor,
    },
    dialog::alert,
    app::screen_size
};
use std::path::Path;
use crate::functions::text_style;

pub fn center() -> (i32, i32) {
    (
        (screen_size().0 / 2.0) as i32,
        (screen_size().1 / 2.0) as i32,
    )
}

pub fn render_file(
    folders: Tree,
    text_buffer: TextBuffer,
    text_editor: TextEditor,
    prefix: String
) {
    let mut text_editor: TextEditor = text_editor.clone();
    let mut folders: Tree = folders.clone();
    let mut text_buffer: TextBuffer = text_buffer.clone();
    folders.set_callback(move |item| {
        if let Some(items) = item.get_selected_items() {
            for i in items {
                let path: String = prefix.clone() + "/"+ &item.item_pathname(&i).unwrap();
                let file_path: &Path = Path::new(&path);
                match text_buffer.load_file(file_path) {
                    Ok(_) => {
                        text_style(text_editor.clone(), &text_buffer.text());
                    },
                    Err(e) => {
                        alert(center().0 - 100, center().1 - 100, &format!("Error: {}", e));
                        let _ = item.deselect(
                            &path,
                            true
                        );
                    }
                }
            }
        }}
    );
}
