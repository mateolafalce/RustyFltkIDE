use fltk::{
    tree::Tree,
    text::TextBuffer,
};

#[path="../render_file.rs"]
mod render_file;

pub fn render_all_files_in_folders(
    folders: Tree,
    text_buffer: TextBuffer,
    prefix: Vec<String>
) -> bool {
    if prefix.len() > 0 {
        for i in 0..prefix.len() - 1 {
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    if prefix.len() == 1 {
        for i in 0..prefix.len() {
            render_file::render_file(
                folders.clone(),
                text_buffer.clone(),
                prefix[i].clone()
            );
        }
    }
    true
}
