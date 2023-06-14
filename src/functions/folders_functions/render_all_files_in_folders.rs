#[path="../render_file.rs"]
mod render_file;

pub fn render_all_files_in_folders(
    folders: fltk::tree::Tree,
    text_buffer: fltk::text::TextBuffer,
    prefix: Vec<String>
) -> bool {
    // Check if prefix has more than one element
    if prefix.len() > 0 {
        // Iterate over the elements of prefix (excluding the last one)
        for i in 0..prefix.len() - 1 {
            // Call render_file function for each prefix element
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    // Check if prefix has exactly one element
    if prefix.len() == 1 {
        // Iterate over the single element of prefix
        for i in 0..prefix.len() {
            // Call render_file function for the prefix element
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    true
}
