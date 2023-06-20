use fltk::prelude::*;
#[path="../event/error.rs"]
mod error;

pub fn render_file(
    mut folders: fltk::tree::Tree,
    mut text_buffer: fltk::text::TextBuffer,
    prefix: String
) -> bool {
    let folders_ = folders.clone();
    folders.set_callback(move |item| {
        if let Some(items) = item.get_selected_items() {  // Get the selected items from the Tree widget
            for i in items {  // Iterate over the selected items
                let path: String = prefix.clone() + "/"+ &item.item_pathname(&i).unwrap();  // Build the file path for the selected item
                let file_path: &std::path::Path = std::path::Path::new(&path);
                match text_buffer.load_file(file_path) {  // Load the file into the TextBuffer widget
                    Ok(_) => (), // If loading the file was successful, do nothing
                    Err(e) => {
                        error::error(&e.to_string()); //Return error
                        let tree_item: fltk::tree::TreeItem = fltk::tree::TreeItem::new(&folders_, "Avoid select error");
                        let _ = item.select_only(//Deselect item
                            &tree_item,
                            true
                        );
                    }
                };
            }
        }
    });
    true
}
